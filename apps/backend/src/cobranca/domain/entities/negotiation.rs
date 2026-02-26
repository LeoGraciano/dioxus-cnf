use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NegotiationResult {
    Paid,
    PromiseToPay,
    Renegotiated,
    Refused,
    NoContact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Negotiation {
    pub id: Uuid,
    pub installment_id: Uuid,
    pub associate_id: Uuid,
    pub user_id: i32,
    pub result: NegotiationResult,
    pub promise_date: Option<NaiveDate>,
    pub promise_value: Option<f64>,
    pub observation: Option<String>,
    pub negotiated_at: DateTime<Utc>,
}

impl Negotiation {
    pub fn new(
        installment_id: Uuid,
        associate_id: Uuid,
        user_id: i32,
        result: NegotiationResult,
        promise_date: Option<NaiveDate>,
        promise_value: Option<f64>,
        observation: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            installment_id,
            associate_id,
            user_id,
            result,
            promise_date,
            promise_value,
            observation,
            negotiated_at: Utc::now(),
        }
    }
}
