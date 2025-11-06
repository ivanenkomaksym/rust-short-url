#[cfg(test)]
mod tests {
    use rust_short_url::{configuration::settings::{Settings, ApiServer}, services::hashservicefactory::create_hash_service, models::queryparams::QueryParams};
    
    #[actix_rt::test]
    async fn test_successful_hashing() {
        // Arrange
        let settings = setup_settings();
        let mut hash_service = create_hash_service(&settings).await.unwrap();

        let str1 = "string1";
        let str2 = "string2";

        // Act
        let key1_result = hash_service.insert(str1).await;
        let key2_result = hash_service.insert(str2).await;

        // Assert
        assert!(key1_result.is_ok());
        assert!(key2_result.is_ok());

        assert_ne!(key1_result.unwrap(), key2_result.unwrap());
    }

    #[actix_rt::test]
    async fn test_successful_find_inserted_long_url() {
        // Arrange
        let settings = setup_settings();
        let mut hash_service = create_hash_service(&settings).await.unwrap();

        let expected_long_url = "https://doc.rust-lang.org/";

        // Act
        let inserted_result = hash_service.insert(expected_long_url).await;
        assert!(inserted_result.is_ok());
        let inserted = inserted_result.unwrap();

        let linkinfo_result = hash_service.find(&inserted.short_url).await.unwrap();

        // Assert
        assert_eq!(linkinfo_result.is_none(), false);
        let actual_long_url = &linkinfo_result.unwrap().long_url;

        assert_eq!(expected_long_url, actual_long_url);
    }

    #[actix_rt::test]
    async fn test_failed_find_not_inserted_long_url() {
        // Arrange
        let settings = setup_settings();
        let mut hash_service = create_hash_service(&settings).await.unwrap();

        // Act
        let key = "non_existing_key";
        let linkinfo_result = hash_service.find(key).await.unwrap();

        // Assert
        assert_eq!(linkinfo_result.is_none(), true);
    }

    #[actix_rt::test]
    async fn test_summary() {
        // Arrange
        let settings = setup_settings();
        let mut hash_service = create_hash_service(&settings).await.unwrap();

        let expected_long_url = "https://doc.rust-lang.org/";

        // Act
        let inserted_result = hash_service.insert(expected_long_url).await;
        assert!(inserted_result.is_ok());
        let inserted = inserted_result.unwrap();

        let mut linkinfo_result = None;
        let expected_clicks = 20;
        for _i in 0..expected_clicks {
            linkinfo_result = hash_service.find(&inserted.short_url).await.unwrap();
        }

        // Assert
        assert_eq!(linkinfo_result.is_none(), false);
        let actual_linkinfo = &linkinfo_result.unwrap();
        
        assert_eq!(actual_linkinfo.short_url, inserted.short_url);
        assert_eq!(actual_linkinfo.long_url, expected_long_url);
    }

    #[actix_rt::test]
    async fn test_get_links() {
        // Arrange
        let settings = setup_settings();
        let mut hash_service = create_hash_service(&settings).await.unwrap();

        let url1 = "https://doc.rust-lang.org/";
        let url2 = "https://crates.io/";

        let result1 = hash_service.insert(url1).await;
        let result2 = hash_service.insert(url2).await;

        // Act
        let links_result = hash_service.get_links(None).await;

        // Assert
        assert!(links_result.is_ok());
        let links = links_result.unwrap();

        assert!(result1.is_ok());
        assert!(result2.is_ok());

        assert_eq!(links.len(), 2);
        assert!(links.iter().any(|e| e.long_url == url1));
        assert!(links.iter().any(|e| e.long_url == url2));
    }

    #[actix_rt::test]
    async fn test_top_skip() {
        // Arrange
        let settings = setup_settings();
        let mut hash_service = create_hash_service(&settings).await.unwrap();

        let urls = [ "https://doc.rust-lang.org/",
                                "https://crates.io/",
                                "https://en.wikipedia.org/wiki/Rust_(programming_language)",
                                "https://github.com/rust-lang",
                                "https://www.reddit.com/r/rust/" ];

        for url in urls.iter() {
            let result = hash_service.insert(url).await;
            assert!(result.is_ok());
        };

        // Act
        let all_links = hash_service.get_links(None).await.unwrap();
        let top2_links = hash_service.get_links(Some(QueryParams{ top: Some(2), skip: None })).await.unwrap();
        let top6_links = hash_service.get_links(Some(QueryParams{ top: Some(6), skip: None })).await.unwrap();
        let skip3_links = hash_service.get_links(Some(QueryParams{ top: None, skip: Some(3) })).await.unwrap();
        let skip6_links = hash_service.get_links(Some(QueryParams{ top: None, skip: Some(6) })).await.unwrap();
        let top2_skip2_links = hash_service.get_links(Some(QueryParams{ top: Some(2), skip: Some(2) })).await.unwrap();

        // Assert
        assert_eq!(all_links.len(), 5);

        assert_eq!(top2_links.len(), 2);
        assert!(top2_links[0].long_url == all_links[0].long_url);
        assert!(top2_links[1].long_url == all_links[1].long_url);

        assert_eq!(top6_links.len(), 5);

        assert_eq!(skip3_links.len(), 2);
        assert!(skip3_links[0].long_url == all_links[3].long_url);
        assert!(skip3_links[1].long_url == all_links[4].long_url);

        assert_eq!(skip6_links.len(), 0);

        assert_eq!(top2_skip2_links.len(), 2);
        assert!(top2_skip2_links[0].long_url == all_links[2].long_url);
        assert!(top2_skip2_links[1].long_url == all_links[3].long_url);

    }

    fn setup_settings() -> Settings {
        return Settings {
            debug: true,
            apiserver: ApiServer {
                application_url: String::from("localhost"),
                hostname: String::from("localhost"),
                allow_origin: String::from("localhost"),
                api_key: None,
                google_application_credentials: None,
            },
            mongo_config: None,
            redis_config: None,
            firestore_config: None,
            ratelimit: None,
            mode: rust_short_url::configuration::settings::Mode::InMemory,
            coordinator: None
        }
    }
}