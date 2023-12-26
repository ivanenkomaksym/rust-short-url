use crate::configuration::settings::Settings;
use crate::configuration::settings::Mode;

use super::coordinatorhashservice::CoordinatorHashService;
use super::hashservice::{self};
use super::inmemoryhashservice::InMemoryHashService;
use super::persistenthashservice::PersistentHashService;

pub async fn create_hash_service(settings: &Settings) -> Box<dyn hashservice::HashService> {
    let mut hash_service: Box<dyn hashservice::HashService> = match &settings.mode {
        Mode::InMemory => {
            Box::new(InMemoryHashService::new())
        },
        Mode::Persistent => {
            match &settings.database {
                None => panic!(),
                Some(database_config) => {
                    Box::new(PersistentHashService::new(database_config))
                }
            }
        },
        Mode::Coordinator => {
            match &settings.coordinator {
                None => panic!(),
                Some(coordinator_config) => {
                    Box::new(CoordinatorHashService::new(coordinator_config))
                }
            }
        },
    };

    match hash_service.init().await {
        Ok(_) => return hash_service,
        Err(e) => panic!("Problem initializing hash service: {:?}", e),
    };
}