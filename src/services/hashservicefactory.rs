use crate::configuration::settings::Settings;
use crate::configuration::settings::Mode;
use crate::name_of;

use super::coordinatorhashservice::CoordinatorHashService;
use super::hashservice::{self};
use super::hashserviceerror::build_configuration_error;
use super::hashserviceerror::HashServiceError;
use super::inmemoryhashservice::InMemoryHashService;
use super::mongohashservice::MongoHashService;
use super::redishashservice::RedisHashService;

pub async fn create_hash_service(settings: &Settings) -> Result<Box<dyn hashservice::HashService>, HashServiceError> {
    let mut hash_service: Box<dyn hashservice::HashService> = match &settings.mode {
        Mode::InMemory => {
            Box::new(InMemoryHashService::new())
        },
        Mode::Mongo => {
            match &settings.mongo_config {
                None => return Err(build_configuration_error(Mode::Mongo.to_string().as_str(), name_of!(mongo_config in Settings))),
                Some(mongo_config) => {
                    Box::new(MongoHashService::new(mongo_config))
                }
            }
        },
        Mode::Coordinator => {
            match &settings.coordinator {
                None => return Err(build_configuration_error(Mode::Coordinator.to_string().as_str(), name_of!(coordinator in Settings))),
                Some(coordinator_config) => {
                    Box::new(CoordinatorHashService::new(coordinator_config))
                }
            }
        },
        Mode::Redis => {
            match &settings.redis_config {
                None => return Err(build_configuration_error(Mode::Redis.to_string().as_str(), name_of!(redis_config in Settings))),
                Some(redis_config) => {
                    Box::new(RedisHashService::new(redis_config))
                }
            }
        }
    };

    hash_service.init().await?;
    Ok(hash_service)
}