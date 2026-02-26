use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadAuditEvent {
    pub id: Uuid,
    pub lead_id: Uuid,
    pub event_type: LeadAuditEventType,
    pub description: String,
    pub performed_by_id: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LeadAuditEventType {
    Created,
    StatusChanged,
    Contacted,
    Scheduled,
    CheckedIn,
    WorkshopScheduled,
    SaleAttempted,
    ContractCreated,
    BraceletIssued,
    Converted,
    Lost,
    Canceled,
}
