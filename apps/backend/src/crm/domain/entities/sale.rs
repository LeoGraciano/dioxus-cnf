use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sale {
    pub id: Uuid,
    pub lead_id: Uuid,
    pub stage_id: i32,
    pub value: f64,
    pub description: Option<String>,
    pub closed_by_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}
