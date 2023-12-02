mod constants;
mod api;

#[macro_use]
extern crate actix_web;

use std::{env,io};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    
    api::httpserver::start_http_server().await
}
