use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStage {
    pub id: i32,
    pub name: String,
    pub order: i32,
    pub is_won: bool,
    pub is_lost: bool,
}
