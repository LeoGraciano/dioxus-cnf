use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InstallmentStatus {
    Open,
    Paid,
    Renegotiated,
    Cancelled,
    Overdue,
}

impl Default for InstallmentStatus {
    fn default() -> Self {
        Self::Open
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Installment {
    pub id: Uuid,
    pub title_id: Uuid,
    pub associate_id: Uuid,
    pub installment_number: i32,
    pub due_date: NaiveDate,
    pub value: f64,
    pub status: InstallmentStatus,
    pub days_overdue: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Installment {
    pub fn new(
        title_id: Uuid,
        associate_id: Uuid,
        installment_number: i32,
        due_date: NaiveDate,
        value: f64,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title_id,
            associate_id,
            installment_number,
            due_date,
            value,
            status: InstallmentStatus::default(),
            days_overdue: None,
            created_at: now,
            updated_at: now,
        }
    }
}
