use rust_short_url::{api, configuration::{self, settings::read_settings}, services};

use std::{env,io};
use configuration::settings::Settings;
use services::hashservicefactory;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "debug, actix_server =info");
    env_logger::init();

    // Print all environment variables.
    for (key, value) in std::env::vars() {
        println!("{key}: {value}");
    }
    
    let settings_result: Result<Settings, config::ConfigError> = read_settings();

    let settings = match settings_result  {
        Err(e) => panic!("Problem loading settings: {:?}", e),
        Ok(s) => s,
    };

    let hash_service = match hashservicefactory::create_hash_service(&settings).await {
        Err(e) => panic!("Problem constructing hash service: {:?}", e),
        Ok(s) => s,
    };
    
    api::httpserver::start_http_server(settings, hash_service).await
}