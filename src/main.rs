use rust_short_url::{configuration, services, api};

use std::{env,io};
use configuration::settings::Settings;
use services::hashservicefactory;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web = debug, actix_server =info");
    env_logger::init();
    
    let settings_result: Result<Settings, config::ConfigError> = Settings::new();

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