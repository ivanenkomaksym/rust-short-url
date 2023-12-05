#[cfg(test)]
mod tests {
    use rust_short_url::{configuration::settings::{Settings, ApiServer}, services::{hashservicefactory::create_hash_service, hashservice::HashService}};
    
    #[test]
    fn test_successful_find_inserted_long_url() {
        // Arrange
        let settings = setup_settings();
        let mut hash_service = create_hash_service(&settings);

        let expected_long_url = String::from("https://doc.rust-lang.org/");

        // Act
        let key = hash_service.insert(&expected_long_url);
        let actual_long_url_result = hash_service.find(&key);

        // Assert
        assert_ne!(actual_long_url_result, Option::None);
        let actual_long_url = actual_long_url_result.unwrap();

        assert_eq!(&expected_long_url, actual_long_url);
    }

    #[test]
    fn test_failed_find_not_inserted_long_url() {
        // Arrange
        let settings = setup_settings();
        let hash_service = create_hash_service(&settings);

        // Act
        let key = String::from("non_existing_key");
        let actual_long_url_result = hash_service.find(&key);

        // Assert
        assert_eq!(actual_long_url_result, Option::None);
    }

    fn setup_settings() -> Settings {
        return Settings { debug: true, apiserver: ApiServer { application_url: String::from("localhost") }, database: None }
    }
}