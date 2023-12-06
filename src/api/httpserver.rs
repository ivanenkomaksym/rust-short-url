use crate::configuration::settings::Settings;
use crate::constants::{APPLICATION_JSON, TEXT_HTML};
use crate::services::hashservice::HashService;

use actix_web::{middleware, App, HttpResponse, web};
use actix_web::HttpServer;
use std::io;
use std::sync::{Arc, Mutex};

use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct ShortenRequest {
   long_url: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    message: String
}

pub async fn start_http_server(settings: &Settings, hash_service: Arc<Mutex<dyn HashService>>) -> io::Result<()> {
    HttpServer::new(move|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(hello)
            .service(shorten)
            .service(redirect)
            .service(summary)
            .app_data(web::Data::new(hash_service.clone()))
    })
    .bind(&settings.apiserver.application_url)?
    .run()
    .await
}

#[get("/hello")]
async fn hello() -> HttpResponse {
HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(Response { message: String::from("hello")})
}

#[get("/shorten")]
async fn shorten(info: web::Query<ShortenRequest>, hash_service: web::Data<Arc<Mutex<dyn HashService>>>) -> actix_web::Result<String> {
    dbg!(&info.long_url);
    let hostname = "localhost:8000";
    let hash = hash_service.lock().unwrap().insert(&info.long_url);

    Ok(format!("{}/{}", hostname, hash))
}

#[get("/{short_url}")]
async fn redirect(path: web::Path<String>, hash_service: web::Data<Arc<Mutex<dyn HashService>>>) -> HttpResponse {
    let short_url = path.into_inner();
    if short_url.is_empty() {
        return HttpResponse::BadRequest()
            .finish();
    }

    dbg!(&short_url);

    let long_url: String = match hash_service.lock().unwrap().find(&short_url){
        None => {
            return HttpResponse::NotFound()
                .finish();
        }
        Some(value) => value.long_url.clone()
    };

    HttpResponse::PermanentRedirect()
        .append_header(("location", long_url))
        .content_type(TEXT_HTML)
        .finish()
}

#[get("/{short_url}/summary")]
async fn summary(path: web::Path<String>, hash_service: web::Data<Arc<Mutex<dyn HashService>>>) -> HttpResponse {
    let short_url = path.into_inner();
    if short_url.is_empty() {
        return HttpResponse::BadRequest()
            .finish();
    }

    dbg!(&short_url);

    let linfinfo = match hash_service.lock().unwrap().find(&short_url){
        None => {
            return HttpResponse::NotFound()
                .finish();
        }
        Some(value) => value.clone()
    };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(linfinfo)
}