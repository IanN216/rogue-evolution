use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    pub name: String,
    pub title: Option<String>,
    pub kingdom_id: u32,
}
