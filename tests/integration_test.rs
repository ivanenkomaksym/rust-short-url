#[cfg(test)]
mod tests {
    use rust_short_url::{configuration::settings::{Settings, ApiServer}, services::{hashservicefactory::create_hash_service, hashservice::HashService}};
    
    #[test]
    fn test_successful_hashing() {
        // Arrange
        let settings = setup_settings();
        let mut hash_service = create_hash_service(&settings);

        let str1 = "string1";
        let str2 = "string2";

        // Act
        let key1 = hash_service.insert(str1);
        let key2 = hash_service.insert(str2);

        // Assert
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_successful_find_inserted_long_url() {
        // Arrange
        let settings = setup_settings();
        let mut hash_service = create_hash_service(&settings);

        let expected_long_url = "https://doc.rust-lang.org/";

        // Act
        let key = hash_service.insert(expected_long_url);
        let linkinfo_result = hash_service.find(&key);

        // Assert
        assert_eq!(linkinfo_result.is_none(), false);
        let actual_long_url = &linkinfo_result.unwrap().long_url;

        assert_eq!(expected_long_url, actual_long_url);
    }

    #[test]
    fn test_failed_find_not_inserted_long_url() {
        // Arrange
        let settings = setup_settings();
        let mut hash_service = create_hash_service(&settings);

        // Act
        let key = "non_existing_key";
        let linkinfo_result = hash_service.find(key);

        // Assert
        assert_eq!(linkinfo_result.is_none(), true);
    }

    fn setup_settings() -> Settings {
        return Settings { debug: true, apiserver: ApiServer { application_url: String::from("localhost") }, database: None }
    }
}