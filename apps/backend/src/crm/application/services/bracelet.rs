use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use shared::domain::ports::RepositoryError;

use crate::domain::entities::{Bracelet, BraceletStatus, LeadAuditEvent, LeadAuditEventType};
use crate::domain::ports::{BraceletRepository, LeadAuditRepository};
use crate::application::ports::{IssueBraceletInput, IssueBraceletOutput, IssueBraceletUseCase};

pub struct BraceletService<B, A>
where
    B: BraceletRepository,
    A: LeadAuditRepository,
{
    bracelet_repo: B,
    audit_repo: A,
}

impl<B, A> BraceletService<B, A>
where
    B: BraceletRepository,
    A: LeadAuditRepository,
{
    pub fn new(bracelet_repo: B, audit_repo: A) -> Self {
        Self { bracelet_repo, audit_repo }
    }
}

#[async_trait]
impl<B, A> IssueBraceletUseCase for BraceletService<B, A>
where
    B: BraceletRepository + Send + Sync,
    A: LeadAuditRepository + Send + Sync,
{
    async fn execute(&self, input: IssueBraceletInput) -> Result<IssueBraceletOutput, RepositoryError> {
        // Check for duplicate bracelet number
        if self.bracelet_repo.find_by_number(&input.bracelet_number).await?.is_some() {
            return Err(RepositoryError::ValidationError(
                format!("Numero de pulseira ja existe: {}", input.bracelet_number)
            ));
        }

        let now = Utc::now();
        let bracelet = Bracelet {
            id: Uuid::new_v4(),
            associate_id: input.associate_id,
            bracelet_number: input.bracelet_number,
            issued_at: now,
            status: BraceletStatus::Issued,
        };

        let created = self.bracelet_repo.create(&bracelet).await?;

        let audit_event = LeadAuditEvent {
            id: Uuid::new_v4(),
            lead_id: input.associate_id,
            event_type: LeadAuditEventType::BraceletIssued,
            description: format!("Pulseira {} emitida", created.bracelet_number),
            performed_by_id: None,
            created_at: now,
        };
        self.audit_repo.record(&audit_event).await?;

        Ok(IssueBraceletOutput { bracelet: created })
    }
}
