use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bracelet {
    pub id: Uuid,
    pub associate_id: Uuid,
    pub bracelet_number: String,
    pub issued_at: DateTime<Utc>,
    pub status: BraceletStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BraceletStatus {
    Issued,
    Active,
    Expired,
    Canceled,
}
