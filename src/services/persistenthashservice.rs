use crate::{services::hashservice, models::linkinfo::LinkInfo, configuration};
use mongodb::{ bson::doc, options::{ ClientOptions, ServerApi, ServerApiVersion }, Client };

use async_trait::async_trait;

use super::hashserviceerror::HashServiceError;

pub struct PersistentHashService {
    database_config: configuration::settings::Database
}

impl PersistentHashService {
    pub fn new(config: &configuration::settings::Database) -> impl hashservice::HashService {
        PersistentHashService {
            database_config: config.clone()
        }
    }
}

#[async_trait]
impl hashservice::HashService for PersistentHashService {
    fn insert(&mut self, _value: &str) -> String {
        todo!("Implement it")
    }

    fn find(&mut self, _key: &str) -> Option<&LinkInfo> {
        todo!("Implement it")
    }

    async fn init(&mut self) -> Result<(), HashServiceError> {
        let mut client_options = ClientOptions::parse(&self.database_config.connection_string).await?;
        // Set the server_api field of the client_options object to Stable API version 1
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        // Create a new client and connect to the server
        let client = Client::with_options(client_options)?;
        // Send a ping to confirm a successful connection
        client.database("admin").run_command(doc! { "ping": 1 }, None).await?;
        println!("Pinged your deployment. You successfully connected to MongoDB!");

        Ok(())
    }
}
