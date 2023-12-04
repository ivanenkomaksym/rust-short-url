use crate::configuration::settings::Settings;

use super::hashservice::HashService;
use super::inmemoryhashservice::InMemoryHashService;

pub fn create_hash_service(settings: &Settings) -> impl HashService {
    match &settings.database {
        None => return InMemoryHashService::new(),
        Some(_value) => {
            todo!("Build hash service with real connection.");
        }
    }    
}