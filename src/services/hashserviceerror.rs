use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashServiceError {
    #[error("service connection error")]
    ConnectionError(#[from] mongodb::error::Error),
    #[error("Missing configuration '{configuraiton}' in '{mode}' mode.")]
    MissingConfiguration {
        mode: String,
        configuraiton: String,
    },
    #[error("Internal error")]
    IOError(#[from] io::Error),
    #[error("unknown data store error")]
    InternalHttpClientError(#[from] reqwest::Error),
    #[error("unknown data store error")]
    Unknown,
}