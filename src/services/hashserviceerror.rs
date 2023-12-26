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
    #[error("unknown data store error")]
    Unknown,
}