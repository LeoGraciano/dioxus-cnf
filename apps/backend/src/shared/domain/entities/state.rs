use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub id: i32,
    pub name: String,
    pub abbreviation: String,
}
