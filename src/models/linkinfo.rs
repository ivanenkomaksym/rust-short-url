use serde::{Serialize, Deserialize};
use redis_macros::{FromRedisValue, ToRedisArgs};

use super::analytic::Analytic;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, FromRedisValue, ToRedisArgs)]
pub struct LinkInfo {
    pub short_url: String,
    pub long_url: String,
    pub analytics: Option<Vec<Analytic>>,
}