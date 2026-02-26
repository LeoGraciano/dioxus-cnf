use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TitleStatus {
    Valid,
    Cancelled,
    Renegotiated,
}

impl Default for TitleStatus {
    fn default() -> Self {
        Self::Valid
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    pub id: Uuid,
    pub associate_id: Uuid,
    pub document_number: String,
    pub title_date: Option<chrono::NaiveDate>,
    pub total_value: Option<f64>,
    pub is_valid: bool,
    pub financial_operation: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Title {
    pub fn new(
        associate_id: Uuid,
        document_number: String,
        title_date: Option<chrono::NaiveDate>,
        total_value: Option<f64>,
        financial_operation: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            associate_id,
            document_number,
            title_date,
            total_value,
            is_valid: true,
            financial_operation,
            created_at: now,
            updated_at: now,
        }
    }
}
