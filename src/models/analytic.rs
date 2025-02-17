use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Analytic {
    pub language: Option<String>,
    pub os: Option<String>,
    pub ip: Option<String>,
    pub location: Option<String>
}