use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct LinkInfo {
    pub short_url: String,
    pub long_url: String,
    pub clicks: u32
}