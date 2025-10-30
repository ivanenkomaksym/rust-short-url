use serde::{Serialize, Deserialize};
use redis_macros::{FromRedisValue, ToRedisArgs};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, FromRedisValue, ToRedisArgs)]
pub struct Analytic {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub language: Option<String>,
    pub os: Option<String>,
    pub ip: Option<String>,
    pub location: Option<String>,
    pub referrer: Option<String>,
    pub device_type: Option<String>,
    pub browser: Option<String>,
    pub user_agent: Option<String>,
}