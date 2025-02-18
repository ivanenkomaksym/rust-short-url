use crate::{configuration, models::{linkinfo::LinkInfo, queryparams::QueryParams}, services::{hashfunction, hashservice}};
use futures_util::TryStreamExt;
use mongodb::{ bson::doc, options::{ ClientOptions, ServerApi, ServerApiVersion }, Client, Collection };

use async_trait::async_trait;

use super::hashserviceerror::HashServiceError;

pub struct MongoHashService {
    mongo_config: configuration::settings::MongoConfig,
    collection: Option<Collection<LinkInfo>>
}

impl MongoHashService {
    pub fn new(config: &configuration::settings::MongoConfig) -> impl hashservice::HashService {
        MongoHashService {
            mongo_config: config.clone(),
            collection: None
        }
    }
}

#[async_trait]
impl hashservice::HashService for MongoHashService {
    async fn insert(&mut self, value: &str) -> Result<LinkInfo, HashServiceError> {
        let hash_value = hashfunction::hash(value);

        let find_result = self.find(&hash_value).await?;
        if find_result.is_some() {
            return Ok(find_result.unwrap());
        }
        
        let new_link = LinkInfo{
            short_url: hash_value.clone(),
            long_url: String::from(value),
            clicks: 0
        };

        self.collection.as_mut().unwrap().insert_one(new_link.clone(), None).await?;
        Ok(new_link)
    }

    async fn update(&mut self, key: &str, value: &LinkInfo) -> Result<bool, HashServiceError> {
        let update_result = self.collection.as_mut().unwrap().replace_one(
            doc! { "short_url": key }, value, None
        ).await?;

        Ok(update_result.modified_count > 0)
    }

    async fn find(&mut self, key: &str) -> Result<Option<LinkInfo>, HashServiceError> {
        let find_result = self.collection.as_mut().unwrap().find_one(
            doc! { "short_url": key }, None
        ).await?;
        
        return Ok(find_result)
    }

    async fn delete(&mut self, key: &str) -> Result<bool, HashServiceError> {
        let delete_result = self.collection.as_mut().unwrap().delete_one(
            doc! { "short_url": key }, None
        ).await?;

        return Ok(delete_result.deleted_count > 0)
    }

    async fn init(&mut self) -> Result<(), HashServiceError> {
        let mut client_options = ClientOptions::parse(&self.mongo_config.connection_string).await?;
        // Set the server_api field of the client_options object to Stable API version 1
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        // Create a new client and connect to the server
        let client = Client::with_options(client_options)?;
        // Send a ping to confirm a successful connection
        client.database("admin").run_command(doc! { "ping": 1 }, None).await?;
        log::debug!("Pinged your deployment. You successfully connected to MongoDB!");

        self.collection = Some(client.database(self.mongo_config.database_name.as_str()).collection::<LinkInfo>(self.mongo_config.collection_name.as_str()));

        Ok(())
    }

    async fn get_links(&mut self, query_params: Option<QueryParams>) -> Result<Vec<LinkInfo>, HashServiceError>
    {
        let coll = match &self.collection {
            Some(value) => value,
            None => return Ok([].to_vec())
        };
        
        let cursor = coll.find(
            doc! {}, None
        ).await?;
        
        let urls: Vec<LinkInfo> = cursor.try_collect().await.expect("");

        let query_params = match query_params {
            Some(value) => value,
            None => return Ok(urls)
        };

        let top = query_params.top.unwrap_or(urls.len());
        let skip = query_params.skip.unwrap_or(0);
        
        Ok(urls.into_iter().skip(skip).take(top).collect())
    }
}