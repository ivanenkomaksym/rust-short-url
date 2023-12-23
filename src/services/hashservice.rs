use crate::models::linkinfo::LinkInfo;

use async_trait::async_trait;

use super::hashserviceerror::HashServiceError;

#[async_trait]
pub trait HashService: Send + Sync {
    async fn init(&mut self) -> Result<(), HashServiceError>;

    async fn get_links(&self) -> Vec<LinkInfo>;

    async fn insert(&mut self, value: &str) -> String;

    async fn find(&mut self, key: &str) -> Option<LinkInfo>;
}