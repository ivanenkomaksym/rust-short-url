#[cfg(test)]
mod tests {
    use rust_short_url::{configuration::settings::{Settings, ApiServer, MongoConfig}, services::hashservicefactory::create_hash_service};

    #[actix_rt::test]
    #[should_panic(expected = "connection string contains no scheme")]
    async fn test_failed_connection() {
        // Arrange
        let settings = setup_settings();
        let _hash_service = create_hash_service(&settings).await.unwrap();
    }

    fn setup_settings() -> Settings {
        return Settings { 
            debug: true,
            apiserver: ApiServer { application_url: String::from("localhost"), hostname: String::from("localhost") },
            mongo_config: Some(MongoConfig { connection_string: String::from("invalid_string"), database_name: String::from("database"), collection_name: String::from("collection") }),
            redis_config: None,
            ratelimit: None,
            mode: rust_short_url::configuration::settings::Mode::Mongo,
            coordinator: None,
        }
    }
}