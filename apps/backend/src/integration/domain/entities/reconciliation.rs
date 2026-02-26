use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciliationReport {
    pub id: Uuid,
    pub batch_id: Uuid,
    pub entity_type: String,
    pub expected_count: u32,
    pub actual_count: u32,
    pub divergences: Vec<Divergence>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Divergence {
    pub entity_id: String,
    pub issue: DivergenceIssue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DivergenceIssue {
    Missing,
    HashMismatch,
    StatusMismatch,
    ExtraRecord,
}

impl ReconciliationReport {
    pub fn new(
        batch_id: Uuid,
        entity_type: String,
        expected_count: u32,
        actual_count: u32,
        divergences: Vec<Divergence>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            batch_id,
            entity_type,
            expected_count,
            actual_count,
            divergences,
            created_at: Utc::now(),
        }
    }

    pub fn has_divergences(&self) -> bool {
        !self.divergences.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciliationRequest {
    pub entity_type: String,
    pub batch_id: Uuid,
    pub expected_count: u32,
    pub sync_start_time: DateTime<Utc>,
}

impl ReconciliationRequest {
    pub fn new(
        entity_type: String,
        batch_id: Uuid,
        expected_count: u32,
        sync_start_time: DateTime<Utc>,
    ) -> Self {
        Self {
            entity_type,
            batch_id,
            expected_count,
            sync_start_time,
        }
    }
}
