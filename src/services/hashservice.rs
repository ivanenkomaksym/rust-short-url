use crate::models::linkinfo::LinkInfo;

use async_trait::async_trait;

use super::hashserviceerror::HashServiceError;

#[async_trait]
pub trait HashService: Send + Sync {
    async fn init(&mut self) -> Result<(), HashServiceError>;

    fn insert(&mut self, value: &str) -> String;

    fn find(&mut self, key: &str) -> Option<&LinkInfo>;
}