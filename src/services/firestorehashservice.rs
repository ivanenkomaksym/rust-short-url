use crate::{services::hashservice, models::{linkinfo::LinkInfo, queryparams::QueryParams}, configuration};

use firestore::*;
use async_trait::async_trait;
use redis::{Commands, JsonCommands};

use super::{hashfunction, hashserviceerror::HashServiceError};

pub struct FirestoreHashService {
    firestore_config: configuration::settings::FirestoreConfig,
    db: Option<FirestoreDb>,
}

impl FirestoreHashService {
    pub fn new(config: &configuration::settings::FirestoreConfig) -> impl hashservice::HashService {
        FirestoreHashService {
            firestore_config: config.clone(),
            db: None
        }
    }
}

#[async_trait]
impl hashservice::HashService for FirestoreHashService {
    async fn init(&mut self) -> Result<(), HashServiceError> {
        self.db = Some(FirestoreDb::new(self.firestore_config.project_id.clone()).await?);

        Ok(())
    }

    async fn get_links(&mut self, query_info: Option<QueryParams>) -> Result<Vec<LinkInfo>, HashServiceError> {
        // Implement the logic for get_links
        unimplemented!()
    }

    async fn insert(&mut self, value: &str) -> Result<LinkInfo, HashServiceError> {
        // Implement the logic for insert
        unimplemented!()
    }

    async fn find(&mut self, key: &str) -> Result<Option<LinkInfo>, HashServiceError> {
        // Implement the logic for find
        unimplemented!()
    }

    async fn delete(&mut self, key: &str) -> Result<bool, HashServiceError> {
        // Implement the logic for delete
        unimplemented!()
    }
}