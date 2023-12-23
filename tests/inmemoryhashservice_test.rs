#[cfg(test)]
mod tests {
    use rust_short_url::{configuration::settings::{Settings, ApiServer}, services::hashservicefactory::create_hash_service};
    
    #[actix_rt::test]
    async fn test_successful_hashing() {
        // Arrange
        let settings = setup_settings();
        let mut hash_service = create_hash_service(&settings).await;

        let str1 = "string1";
        let str2 = "string2";

        // Act
        let key1 = hash_service.insert(str1).await;
        let key2 = hash_service.insert(str2).await;

        // Assert
        assert_ne!(key1, key2);
    }

    #[actix_rt::test]
    async fn test_successful_find_inserted_long_url() {
        // Arrange
        let settings = setup_settings();
        let mut hash_service = create_hash_service(&settings).await;

        let expected_long_url = "https://doc.rust-lang.org/";

        // Act
        let key = hash_service.insert(expected_long_url).await;
        let linkinfo_result = hash_service.find(&key).await;

        // Assert
        assert_eq!(linkinfo_result.is_none(), false);
        let actual_long_url = &linkinfo_result.unwrap().long_url;

        assert_eq!(expected_long_url, actual_long_url);
    }

    #[actix_rt::test]
    async fn test_failed_find_not_inserted_long_url() {
        // Arrange
        let settings = setup_settings();
        let mut hash_service = create_hash_service(&settings).await;

        // Act
        let key = "non_existing_key";
        let linkinfo_result = hash_service.find(key).await;

        // Assert
        assert_eq!(linkinfo_result.is_none(), true);
    }

    #[actix_rt::test]
    async fn test_summary() {
        // Arrange
        let settings = setup_settings();
        let mut hash_service = create_hash_service(&settings).await;

        let expected_long_url = "https://doc.rust-lang.org/";

        // Act
        let key = hash_service.insert(expected_long_url).await;

        let mut linkinfo_result = None;
        let expected_clicks = 20;
        for _i in 0..expected_clicks {
            linkinfo_result = hash_service.find(&key).await;
        }

        // Assert
        assert_eq!(linkinfo_result.is_none(), false);
        let actual_linkinfo = &linkinfo_result.unwrap();
        
        assert_eq!(actual_linkinfo.short_url, key);
        assert_eq!(actual_linkinfo.long_url, expected_long_url);
        assert_eq!(actual_linkinfo.clicks, expected_clicks);
    }

    #[actix_rt::test]
    async fn test_get_links() {
        // Arrange
        let settings = setup_settings();
        let mut hash_service = create_hash_service(&settings).await;

        let url1 = "https://doc.rust-lang.org/";
        let url2 = "https://crates.io/";

        hash_service.insert(url1).await;
        hash_service.insert(url2).await;

        // Act
        let links = hash_service.get_links(None).await;

        // Assert
        assert_eq!(links.len(), 2);
        assert!(links.iter().any(|e| e.long_url == url1));
        assert!(links.iter().any(|e| e.long_url == url2));
    }

    fn setup_settings() -> Settings {
        return Settings { debug: true, apiserver: ApiServer { application_url: String::from("localhost"), hostname: String::from("localhost") }, database: None, ratelimit: None }
    }
}