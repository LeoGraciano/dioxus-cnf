use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DistributionType {
    Associate,
    Installment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionLog {
    pub id: Uuid,
    pub installment_id: Uuid,
    pub associate_id: Uuid,
    pub assigned_to_user_id: i32,
    pub workgroup_id: i32,
    pub distribution_type: DistributionType,
    pub distributed_at: DateTime<Utc>,
}

impl DistributionLog {
    pub fn new(
        installment_id: Uuid,
        associate_id: Uuid,
        assigned_to_user_id: i32,
        workgroup_id: i32,
        distribution_type: DistributionType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            installment_id,
            associate_id,
            assigned_to_user_id,
            workgroup_id,
            distribution_type,
            distributed_at: Utc::now(),
        }
    }
}
