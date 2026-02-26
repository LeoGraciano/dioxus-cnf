use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub id: Uuid,
    pub associate_id: Uuid,
    pub contract_number: String,
    pub value: f64,
    pub status: ContractStatus,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContractStatus {
    Draft,
    PendingSignature,
    Active,
    Canceled,
    Expired,
}
