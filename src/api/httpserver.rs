use crate::configuration::settings::Settings;
use crate::constants::{APPLICATION_JSON, TEXT_HTML};

use actix_web::{middleware, App, HttpResponse, web};
use actix_web::HttpServer;
use std::io;

use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct ShortenRequest {
   long_url: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    message: String
}

pub async fn start_http_server(settings: &Settings) -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(hello)
            .service(shorten)
            .service(redirect)
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
async fn shorten(info: web::Query<ShortenRequest>) -> actix_web::Result<String> {
    dbg!(&info.long_url);
    let hostname = "localhost:8000";
    let hash = "zn9edcu";
    Ok(format!("{}/{}", hostname, hash))
}

#[get("/{short_url}")]
async fn redirect(path: web::Path<String>) -> HttpResponse {
    let short_url = path.into_inner();
    dbg!(short_url);
    HttpResponse::PermanentRedirect()
        .append_header(("location", "https://docs.rs/actix-web/latest/actix_web/"))
        .content_type(TEXT_HTML)
        .finish()
}