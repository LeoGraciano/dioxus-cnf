use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckIn {
    pub id: Uuid,
    pub associate_id: Uuid,
    pub check_in_time: DateTime<Utc>,
    pub check_out_time: Option<DateTime<Utc>>,
    pub location: Option<String>,
}
