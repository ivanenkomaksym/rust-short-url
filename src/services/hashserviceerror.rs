use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashServiceError {
    #[error("service connection error")]
    ConnectionError(#[from] mongodb::error::Error),
    #[error("unknown data store error")]
    Unknown,
}