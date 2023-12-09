use crate::configuration::settings::Settings;

use super::hashservice::{self};
use super::inmemoryhashservice::InMemoryHashService;
use super::persistenthashservice::PersistentHashService;

pub fn create_hash_service(settings: &Settings) -> Box<dyn hashservice::HashService> {
    match &settings.database {
        None => return Box::new(InMemoryHashService::new()),
        Some(database_config) => {
            return Box::new(PersistentHashService::new(database_config))
        }
    }    
}