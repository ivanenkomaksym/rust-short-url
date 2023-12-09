use crate::configuration::settings::Settings;

use super::hashservice::{self};
use super::inmemoryhashservice::InMemoryHashService;
use super::persistenthashservice::PersistentHashService;

pub async fn create_hash_service(settings: &Settings) -> Box<dyn hashservice::HashService> {
    let mut hash_service: Box<dyn hashservice::HashService> = match &settings.database {
        None => Box::new(InMemoryHashService::new()),
        Some(database_config) => {
            Box::new(PersistentHashService::new(database_config))
        }
    };

    match hash_service.init().await {
        Ok(_) => return hash_service,
        Err(e) => panic!("Problem initializing hash service: {:?}", e),
    };
}