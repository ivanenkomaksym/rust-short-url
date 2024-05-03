use crate::{services::hashservice, models::{linkinfo::LinkInfo, queryparams::QueryParams}, configuration};

use async_trait::async_trait;

use super::hashserviceerror::HashServiceError;

pub struct RedisHashService {
    _database_config: configuration::settings::Database
}

impl RedisHashService {
    pub fn new(config: &configuration::settings::Database) -> impl hashservice::HashService {
        RedisHashService {
            _database_config: config.clone()
        }
    }
}

#[async_trait]
impl hashservice::HashService for RedisHashService {
    async fn insert(&mut self, _value: &str) -> Result<String, HashServiceError> {
        todo!()
    }

    async fn find(&mut self, _key: &str) -> Option<LinkInfo> {
        todo!()
    }

    async fn init(&mut self) -> Result<(), HashServiceError> {
        todo!()
    }

    async fn get_links(&self, _query_params: Option<QueryParams>) -> Result<Vec<LinkInfo>, HashServiceError> {
        todo!()
    }
}