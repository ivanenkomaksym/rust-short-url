#[cfg(test)]
mod tests {
    use rust_short_url::configuration::settings::{Settings, ApiServer};
    
    #[test]
    fn test1() {
        let _settings = setup_settings();
    }

    fn setup_settings() -> Settings {
        return Settings { debug: true, apiserver: ApiServer { application_url: String::from("localhost") }, database: None }
    }
}