use crate::constants::APPLICATION_JSON;

use actix_web::{middleware, App, HttpResponse};
use actix_web::HttpServer;

use std::io;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    message: String
}

pub async fn start_http_server() -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(hello)
    })
    .bind("localhost:8000")?
    .run()
    .await
}

#[get("/hello")]
async fn hello() -> HttpResponse {
HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(Response { message: String::from("hello")})
}