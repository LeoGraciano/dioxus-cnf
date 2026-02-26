use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct City {
    pub id: i32,
    pub name: String,
    pub state_id: i32,
}
