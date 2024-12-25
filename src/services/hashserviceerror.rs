use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashServiceError {
    #[error("service connection error")]
    ConnectionError(#[from] mongodb::error::Error),
    #[error("service connection error")]
    RedisConnectionError(#[from] redis::RedisError),
    #[error("service connection error")]
    FirestoreConnectionError(#[from] firestore::errors::FirestoreError),
    #[error("Missing configuration '{configuration}' in '{mode}' mode.")]
    MissingConfiguration {
        mode: String,
        configuration: String,
    },
    #[error("Internal error")]
    IOError(#[from] io::Error),
    #[error("unknown data store error")]
    InternalHttpClientError(#[from] reqwest::Error),
    #[error("unknown data store error")]
    Unknown,
}

pub fn build_configuration_error(mode: &str, configuration: &str) -> HashServiceError {
    HashServiceError::MissingConfiguration{ mode: mode.to_string(), configuration: configuration.to_string() }
}