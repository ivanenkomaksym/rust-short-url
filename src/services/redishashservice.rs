use crate::{services::hashservice, models::{linkinfo::LinkInfo, queryparams::QueryParams}, configuration};

use async_trait::async_trait;
use redis::JsonCommands;
use serde_json::json;

use super::{hashfunction, hashserviceerror::HashServiceError};

pub struct RedisHashService {
    database_config: configuration::settings::Database,
    connection: Option<redis::Connection>,
}

impl RedisHashService {
    pub fn new(config: &configuration::settings::Database) -> impl hashservice::HashService {
        RedisHashService {
            database_config: config.clone(),
            connection: None
        }
    }
}

#[async_trait]
impl hashservice::HashService for RedisHashService {
    async fn init(&mut self) -> Result<(), HashServiceError> {
        let client = redis::Client::open(self.database_config.connection_string.clone()).unwrap();
        self.connection = Some(client.get_connection().unwrap());

        Ok(())
    }

    async fn insert(&mut self, value: &str) -> Result<String, HashServiceError> {
        let hash_value = hashfunction::hash(value);

        let new_link = LinkInfo{
            short_url: hash_value.clone(),
            long_url: String::from(value),
            clicks: 0
        };

        self.connection.as_mut().unwrap().json_set(&hash_value, "$", &json!(new_link).to_string())?;

        Ok(hash_value)
    }

    async fn find(&mut self, _key: &str) -> Option<LinkInfo> {
        return None
    }

    async fn get_links(&self, _query_params: Option<QueryParams>) -> Result<Vec<LinkInfo>, HashServiceError> {
        todo!()
    }
}