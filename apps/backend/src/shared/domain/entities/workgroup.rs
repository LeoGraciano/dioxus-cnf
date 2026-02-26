use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workgroup {
    pub id: i32,
    pub name: String,
    pub min_days_window: i32,
    pub max_days_window: i32,
    pub is_active: bool,
}
