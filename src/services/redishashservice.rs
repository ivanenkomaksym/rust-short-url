use crate::{services::hashservice, models::{linkinfo::LinkInfo, queryparams::QueryParams}, configuration};

use async_trait::async_trait;
use redis::JsonCommands;

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
        
        self.connection.as_mut().unwrap().json_set(&hash_value, "$", &new_link)?;

        Ok(hash_value)
    }

    async fn find(&mut self, key: &str) -> Result<Option<LinkInfo>, HashServiceError> {
        let result = self.connection.as_mut().unwrap().json_get::<&str, &str, String>(key, "$")?;
        
        let found_links: Vec<LinkInfo> = match serde_json::from_str(result.as_str()) {
            Ok(v) => v,
            Err(err) => panic!("{}", err)
        };

        if found_links.is_empty() {
            return Ok(None);
        }

        let mut found_link = found_links.first().unwrap().clone();
        found_link.clicks += 1;

        self.connection.as_mut().unwrap().json_set(key.to_string(), "$", &found_link)?;

        return Ok(Some(found_link));
    }

    async fn get_links(&self, _query_params: Option<QueryParams>) -> Result<Vec<LinkInfo>, HashServiceError> {
        todo!()
    }
}