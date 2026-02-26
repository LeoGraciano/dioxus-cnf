use async_trait::async_trait;

use shared::domain::ports::RepositoryError;

use crate::domain::entities::SyncStatus;
use crate::domain::ports::SyncEventRepository;
use crate::application::ports::RetryFailedEventsUseCase;

const MAX_RETRY_COUNT: u32 = 3;

pub struct IdempotentRetryService<S>
where
    S: SyncEventRepository,
{
    sync_repo: S,
}

impl<S> IdempotentRetryService<S>
where
    S: SyncEventRepository,
{
    pub fn new(sync_repo: S) -> Self {
        Self { sync_repo }
    }

    pub async fn retry_event(&self, event_id: uuid::Uuid) -> Result<(), RepositoryError> {
        let event = self.sync_repo.find_by_id(event_id).await?
            .ok_or_else(|| RepositoryError::NotFound(format!("Event not found: {}", event_id)))?;

        if event.retry_count >= MAX_RETRY_COUNT {
            return Err(RepositoryError::NotFound(
                format!("Max retry count reached for event {}", event_id)
            ));
        }

        self.sync_repo.update_status(event_id, SyncStatus::Retrying, None).await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn is_idempotent(&self, _entity_type: &str, _entity_id: &str) -> bool {
        true
    }
}

#[async_trait]
impl<S> RetryFailedEventsUseCase for IdempotentRetryService<S>
where
    S: SyncEventRepository + Send + Sync,
{
    async fn execute(&self, max_events: u32) -> Result<u32, RepositoryError> {
        let failed_events = self.sync_repo.find_failed(max_events).await?;
        let mut retried = 0u32;

        for event in failed_events {
            if event.retry_count < MAX_RETRY_COUNT {
                self.retry_event(event.id).await?;
                retried += 1;
            }
        }

        Ok(retried)
    }
}
