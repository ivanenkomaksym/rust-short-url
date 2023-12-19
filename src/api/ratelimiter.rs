use std::time::Instant;

use crate::{configuration::settings::RateLimit, constants::{DEFAULT_CAPACITY, DEFAULT_FILL_RATE}};

#[derive(Clone)]
pub struct RateLimiter {
    pub capacity: usize,
    pub tokens: usize,
    pub fill_rate: usize,
    pub last_update: Instant,
}

impl RateLimiter {
    pub fn new(rate_limit_options: Option<RateLimit>) -> Self {
        match rate_limit_options {
            Some(rate_limit) => {        
                RateLimiter {
                    capacity: rate_limit.capacity,
                    tokens: rate_limit.capacity,
                    fill_rate: rate_limit.fill_rate,
                    last_update: Instant::now()
                }
            }
            None => {
                RateLimiter {
                    tokens: DEFAULT_CAPACITY,
                    capacity: DEFAULT_CAPACITY,
                    fill_rate: DEFAULT_FILL_RATE,
                    last_update: Instant::now()
                }
            }
        }
    }

    pub fn consume(&mut self, tokens: usize) -> bool {
        self.update_tokens();

        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
        } else {
            false
        }
    }

    pub fn update_tokens(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);
        let seconds = elapsed.as_secs();
        let new_tokens = (self.fill_rate as u64 * seconds) as usize;

        if new_tokens > 0 {
            self.tokens = usize::min(self.capacity, self.tokens + new_tokens);
            self.last_update = now;
        }
    }
}