use crate::configuration::settings::Settings;
use crate::configuration::settings::Mode;

use super::coordinatorhashservice::CoordinatorHashService;
use super::hashservice::{self};
use super::hashserviceerror::HashServiceError;
use super::inmemoryhashservice::InMemoryHashService;
use super::mongohashservice::MongoHashService;

pub async fn create_hash_service(settings: &Settings) -> Result<Box<dyn hashservice::HashService>, HashServiceError> {
    let mut hash_service: Box<dyn hashservice::HashService> = match &settings.mode {
        Mode::InMemory => {
            Box::new(InMemoryHashService::new())
        },
        Mode::Mongo => {
            match &settings.database {
                None => return Err(HashServiceError::MissingConfiguration{ mode: String::from("Persistent"), configuraiton: String::from("Database") }),
                Some(database_config) => {
                    Box::new(MongoHashService::new(database_config))
                }
            }
        },
        Mode::Coordinator => {
            match &settings.coordinator {
                None => return Err(HashServiceError::MissingConfiguration{ mode: String::from("Coordinator"), configuraiton: String::from("Coordinator") }),
                Some(coordinator_config) => {
                    Box::new(CoordinatorHashService::new(coordinator_config))
                }
            }
        },
    };

    hash_service.init().await?;
    Ok(hash_service)
}