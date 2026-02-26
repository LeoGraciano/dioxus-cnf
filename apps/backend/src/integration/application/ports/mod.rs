use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::ReconciliationReport;
use shared::domain::ports::RepositoryError;

#[derive(Debug, Clone)]
pub struct ReconcileBatchInput {
    pub entity_type: String,
    pub batch_id: Uuid,
    pub expected_count: u32,
}

#[derive(Debug, Clone)]
pub struct ReconcileBatchOutput {
    pub report: ReconciliationReport,
    pub needs_retry: bool,
}

#[async_trait]
pub trait ReconcileBatchUseCase: Send + Sync {
    async fn execute(&self, input: ReconcileBatchInput) -> Result<ReconcileBatchOutput, RepositoryError>;
}

#[async_trait]
pub trait ProcessSyncEventUseCase: Send + Sync {
    async fn execute(&self, event_id: Uuid) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait RetryFailedEventsUseCase: Send + Sync {
    async fn execute(&self, max_events: u32) -> Result<u32, RepositoryError>;
}
