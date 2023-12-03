mod api;
mod configuration;
mod constants;
mod services;

#[macro_use]
extern crate actix_web;

use std::{env,io, sync::{Arc, Mutex}};
use configuration::settings::Settings;
use services::inmemoryhashservice::InMemoryHashService;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let settings_result: Result<Settings, config::ConfigError> = Settings::new();

    let settings = match settings_result  {
        Err(e) => panic!("Problem loading settings: {:?}", e),
        Ok(s) => s,
    };

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let hash_service = InMemoryHashService::new();
    let hash_service_arc = Arc::new(Mutex::new(hash_service));
    
    api::httpserver::start_http_server(&settings, hash_service_arc).await
}
