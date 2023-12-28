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
    #[error("Invalid URI")]
    InvalidURI(#[from] hyper::http::uri::InvalidUri),
    #[error("HTTP client error")]
    HttpStreamError(#[from] hyper::Error),
    #[error("HTTP client error")]
    HttpConnectionError(#[from] hyper::http::Error),
    #[error("unknown data store error")]
    Unknown,
}