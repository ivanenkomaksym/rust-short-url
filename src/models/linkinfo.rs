use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LinkInfo {
    pub long_url: String,
    pub clicks: u32
}