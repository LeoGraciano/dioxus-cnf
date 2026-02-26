use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lead {
    pub id: Uuid,
    pub id_client_esolution: Option<Uuid>,
    pub source: LeadSource,
    pub status: LeadStatus,
    pub name: String,
    pub cpf: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub observation: Option<String>,
    pub assigned_to_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub converted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LeadSource {
    Phone,
    Website,
    WhatsApp,
    Partner,
    Event,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LeadStatus {
    New,
    Contacted,
    Qualified,
    Proposal,
    Negotiation,
    Won,
    Lost,
    Canceled,
}

impl Default for LeadStatus {
    fn default() -> Self {
        Self::New
    }
}
