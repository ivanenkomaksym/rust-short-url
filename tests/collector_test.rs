#[cfg(test)]
mod tests {
    use actix_web::http::header::{self, HeaderMap};
    use reqwest::header::HeaderValue;
    use rust_short_url::stats::collector;
    
    #[actix_rt::test]
    async fn test_successful_collect_stats() {
        // Arrange
        let mut header_map: HeaderMap = HeaderMap::new();
        header_map.append(header::ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.5"));
        header_map.append(header::USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3"));

        // Act
        let analytic = collector::collect_stats(&header_map).await;

        // Assert
        assert_eq!(analytic.language, Some("en-US".to_string()));
        assert_eq!(analytic.os, Some("Windows".to_string()));
    }
}