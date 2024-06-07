use crate::configuration::settings::Settings;
use crate::constants::{APPLICATION_JSON, TEXT_HTML};
use crate::models::queryparams::QueryParams;
use crate::services::hashservice::HashService;

use actix_web::dev::Service;
use actix_web::{middleware, App, HttpResponse, web};
use actix_web::HttpServer;
use std::io;
use std::sync::{Mutex, Arc};
use serde::{Serialize, Deserialize};

use super::ratelimiter::RateLimiter;
use super::ratelimitermiddleware::RateLimiterMiddlewareService;

#[derive(Debug, Deserialize)]
pub struct ShortenRequest {
   long_url: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    message: String
}

pub struct AppData {
    pub settings: Settings,
    pub hash_service: Box<dyn HashService>
}

pub async fn start_http_server(settings: Settings, hash_service: Box<dyn HashService>) -> io::Result<()> {
    let application_url = settings.apiserver.application_url.clone();
    let rate_limiter = Arc::new(Mutex::new(RateLimiter::new(settings.ratelimit)));

    let appdata = web::Data::new(Mutex::new(AppData { settings, hash_service }));

    HttpServer::new(move|| {
        let rate_limiter = rate_limiter.clone();

        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(hello)
            .service(urls)
            //.service(shorten)
            .service(web::resource("/shorten").wrap_fn(move|req, srv| 
                {
                    let rate_limiter = rate_limiter.clone();
                    RateLimiterMiddlewareService::new(srv, rate_limiter).call(req)
                }).route(web::get().to(shorten)))
            .service(redirect)
            .service(summary)
            .service(delete)
            .app_data(web::Data::clone(&appdata))
    })
    .bind(application_url)?
    .run()
    .await
}

#[get("/hello")]
async fn hello() -> HttpResponse {
HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(Response { message: String::from("hello")})
}

#[get("/urls")]
async fn urls(query_params: web::Query<QueryParams>, appdata: web::Data<Mutex<AppData>>) -> HttpResponse {
    match appdata.lock().unwrap().hash_service.get_links(Some(query_params.0)).await {
        Err(err) => {
            log::error!("{}", err);
            return HttpResponse::InternalServerError()
                .finish();
        }
        Ok(urls) => {
            HttpResponse::Ok()
                .content_type(APPLICATION_JSON)
                .json(urls)
        }
    }
}

pub async fn shorten(info: web::Query<ShortenRequest>, appdata: web::Data<Mutex<AppData>>) -> HttpResponse {
    let mut data = appdata.lock().unwrap();
    match data.hash_service.insert(&info.long_url).await {
        Err(err) => {
            log::error!("{}", err);
            return HttpResponse::InternalServerError()
                .finish();
        }
        Ok(value) => {
            HttpResponse::Ok()
                .content_type(APPLICATION_JSON)
                .json(value)
        }
    }
}

#[get("/{short_url}")]
async fn redirect(path: web::Path<String>, appdata: web::Data<Mutex<AppData>>) -> HttpResponse {
    let short_url = path.into_inner();
    if short_url.is_empty() {
        return HttpResponse::BadRequest()
            .finish();
    }

    let mut data = appdata.lock().unwrap();
    let long_url: String = match data.hash_service.find(&short_url).await {
        Ok(v) => {
            match v {
                None => {
                    return HttpResponse::NotFound()
                        .finish();
                }
                Some(value) => value.long_url.clone()
            }
        },
        Err(err) => {
            log::error!("{}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::PermanentRedirect()
        .append_header(("location", long_url))
        .content_type(TEXT_HTML)
        .finish()
}

#[get("/{short_url}/summary")]
async fn summary(path: web::Path<String>, appdata: web::Data<Mutex<AppData>>) -> HttpResponse {
    let short_url = path.into_inner();
    if short_url.is_empty() {
        return HttpResponse::BadRequest()
            .finish();
    }

    let mut data = appdata.lock().unwrap();
    let linkinfo = match data.hash_service.find(&short_url).await{
        Ok(v) => {
            match v {
                None => {
                    return HttpResponse::NotFound().finish();
                }
                Some(value) => value.clone()
            }
        },
        Err(err) => {
            log::error!("{}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(linkinfo)
}

#[delete("/{short_url}")]
async fn delete(path: web::Path<String>, appdata: web::Data<Mutex<AppData>>) -> HttpResponse {
    let short_url = path.into_inner();
    if short_url.is_empty() {
        return HttpResponse::BadRequest()
            .finish();
    }

    let mut data = appdata.lock().unwrap();
    let result: bool = match data.hash_service.delete(&short_url).await {
        Ok(v) => v,
        Err(err) => {
            log::error!("{}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    if result {
        return HttpResponse::NoContent()
            .finish();
    } else {
        return HttpResponse::NotFound()
            .finish();
    }
}