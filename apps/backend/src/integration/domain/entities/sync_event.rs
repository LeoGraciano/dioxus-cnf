use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Direction of sync between legacy systems and new platform
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncDirection {
    /// Data coming from legacy (CRM2/COB2) -> new platform
    InboundFromLegacy,
    /// Data going from new platform -> legacy (for backwards compatibility)
    OutboundToLegacy,
}

/// Status of a sync event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Retrying,
}

impl Default for SyncStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Represents a synchronization event between legacy and new platform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncEvent {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: String,
    pub direction: SyncDirection,
    pub status: SyncStatus,
    pub payload: serde_json::Value,
    pub error_message: Option<String>,
    pub retry_count: u32,
    pub created_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
}

impl SyncEvent {
    pub fn new(
        entity_type: String,
        entity_id: String,
        direction: SyncDirection,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            entity_type,
            entity_id,
            direction,
            status: SyncStatus::Pending,
            payload,
            error_message: None,
            retry_count: 0,
            created_at: Utc::now(),
            processed_at: None,
        }
    }
}
