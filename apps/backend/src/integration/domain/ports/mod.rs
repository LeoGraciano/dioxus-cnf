use async_trait::async_trait;
use uuid::Uuid;

use shared::domain::ports::RepositoryError;
use super::entities::{ReconciliationReport, SyncEvent, SyncStatus};

/// Port for persisting and querying sync events
#[async_trait]
pub trait SyncEventRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<SyncEvent>, RepositoryError>;
    async fn find_pending(&self, limit: u32) -> Result<Vec<SyncEvent>, RepositoryError>;
    async fn find_failed(&self, limit: u32) -> Result<Vec<SyncEvent>, RepositoryError>;
    async fn create(&self, event: &SyncEvent) -> Result<SyncEvent, RepositoryError>;
    async fn update_status(
        &self,
        id: Uuid,
        status: SyncStatus,
        error_message: Option<String>,
    ) -> Result<(), RepositoryError>;
}

/// Port for reconciliation reporting
#[async_trait]
pub trait ReconciliationReporter: Send + Sync {
    async fn report_divergences(
        &self,
        batch_id: Uuid,
        entity_type: &str,
    ) -> Result<ReconciliationReport, RepositoryError>;
}
