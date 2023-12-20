#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    
    use rust_short_url::api::ratelimiter::RateLimiter;
    use rust_short_url::configuration::settings::RateLimit;

    #[test]
    fn test_ratelimiter() {
        let ratelimit_options = setup_ratelimit_settings();

        let mut rate_limiter = RateLimiter::new(Some(ratelimit_options));

        assert!(rate_limiter.consume(5)); // Consume 5 tokens, should succeed
        assert_eq!(rate_limiter.tokens, 5); // Remaining tokens should be 5

        assert!(!rate_limiter.consume(10)); // Attempt to consume 10 tokens, should fail
        assert_eq!(rate_limiter.tokens, 5); // Remaining tokens should still be 5

        thread::sleep(Duration::from_secs(3)); // Sleep to allow tokens to be replenished
        //rate_limiter.update_tokens(); // Manually update tokens based on elapsed time

        assert!(rate_limiter.consume(10)); // Attempt to consume 10 tokens after refill, should succeed
        assert_eq!(rate_limiter.tokens, 0); // Remaining tokens should be 0
    }
    
    fn setup_ratelimit_settings() -> RateLimit {
        return RateLimit { capacity: 10, fill_rate: 2 }
    }
}