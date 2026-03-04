use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

use shared::domain::ports::RepositoryError;

use crate::domain::entities::{DistributionLog, DistributionType, Installment, Negotiation, NegotiationResult};

// ── Distribution use-cases ────────────────────────────────────────────────────

/// Input for distributing overdue installments among collectors in a workgroup.
/// Respects the min/max attendance window defined by the workgroup.
#[derive(Debug, Clone)]
pub struct DistributeInstallmentsInput {
    pub workgroup_id: i32,
    /// Ordered list of collector (employee) user IDs in this workgroup
    pub collector_user_ids: Vec<i32>,
    /// Minimum days overdue before an installment qualifies for distribution
    pub min_days_overdue: i32,
    /// Maximum days overdue; installments beyond this are excluded
    pub max_days_overdue: i32,
    /// If true, distributes at associate level (all installments of an associate
    /// go to the same collector). If false, distributes per installment.
    pub distribution_type: DistributionType,
}

#[derive(Debug)]
pub struct DistributeInstallmentsOutput {
    /// Number of distribution log entries created
    pub distributed_count: usize,
    pub logs: Vec<DistributionLog>,
}

#[async_trait]
pub trait DistributeInstallmentsUseCase: Send + Sync {
    async fn execute(
        &self,
        input: DistributeInstallmentsInput,
    ) -> Result<DistributeInstallmentsOutput, RepositoryError>;
}

// ── Negotiation use-cases ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct RecordNegotiationInput {
    pub installment_id: Uuid,
    pub associate_id: Uuid,
    pub user_id: i32,
    pub result: NegotiationResult,
    pub promise_date: Option<NaiveDate>,
    pub promise_value: Option<f64>,
    pub observation: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RecordNegotiationOutput {
    pub negotiation: Negotiation,
    pub installment: Installment,
}

#[async_trait]
pub trait RecordNegotiationUseCase: Send + Sync {
    async fn execute(
        &self,
        input: RecordNegotiationInput,
    ) -> Result<RecordNegotiationOutput, RepositoryError>;
}

#[derive(Debug, Clone)]
pub struct GetNegotiationHistoryInput {
    pub installment_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct GetNegotiationHistoryOutput {
    /// Ordered chronologically; this list is immutable (append-only)
    pub negotiations: Vec<Negotiation>,
}

#[async_trait]
pub trait GetNegotiationHistoryUseCase: Send + Sync {
    async fn execute(
        &self,
        input: GetNegotiationHistoryInput,
    ) -> Result<GetNegotiationHistoryOutput, RepositoryError>;
}
