use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scheduling {
    pub id: Uuid,
    pub associate_id: Uuid,
    pub scheduled_date: DateTime<Utc>,
    pub scheduling_type: SchedulingType,
    pub status: SchedulingStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SchedulingType {
    Appointment,
    CheckIn,
    Meeting,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SchedulingStatus {
    Scheduled,
    Confirmed,
    Completed,
    Canceled,
}
