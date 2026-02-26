use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Associate {
    pub id_client_esolution: Uuid,
    pub registration: String,
    pub name: String,
    pub cpf: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: AssociateStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssociateStatus {
    Active,
    Inactive,
    Suspended,
    Canceled,
}

impl Default for AssociateStatus {
    fn default() -> Self {
        Self::Active
    }
}
