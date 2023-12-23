use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QueryParams {
    pub top: Option<usize>,
    pub skip: Option<usize>
}