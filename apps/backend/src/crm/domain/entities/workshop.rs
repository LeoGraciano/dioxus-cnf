use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workshop {
    pub id: Uuid,
    pub associate_id: Uuid,
    pub scheduled_date: DateTime<Utc>,
    pub workshop_type: WorkshopType,
    pub status: WorkshopStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkshopType {
    Visit,
    TrialClass,
    Presentation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkshopStatus {
    Scheduled,
    Confirmed,
    Completed,
    Canceled,
    NoShow,
}
