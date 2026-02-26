use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use shared::domain::ports::RepositoryError;

use crate::domain::entities::{LeadAuditEvent, LeadAuditEventType, Sale};
use crate::domain::ports::{LeadAuditRepository, LeadRepository, SaleRepository};
use crate::application::ports::{CreateSaleAttemptInput, CreateSaleAttemptOutput, CreateSaleAttemptUseCase};

pub struct SaleService<S, L, A>
where
    S: SaleRepository,
    L: LeadRepository,
    A: LeadAuditRepository,
{
    sale_repo: S,
    lead_repo: L,
    audit_repo: A,
}

impl<S, L, A> SaleService<S, L, A>
where
    S: SaleRepository,
    L: LeadRepository,
    A: LeadAuditRepository,
{
    pub fn new(sale_repo: S, lead_repo: L, audit_repo: A) -> Self {
        Self { sale_repo, lead_repo, audit_repo }
    }
}

#[async_trait]
impl<S, L, A> CreateSaleAttemptUseCase for SaleService<S, L, A>
where
    S: SaleRepository + Send + Sync,
    L: LeadRepository + Send + Sync,
    A: LeadAuditRepository + Send + Sync,
{
    async fn execute(&self, input: CreateSaleAttemptInput) -> Result<CreateSaleAttemptOutput, RepositoryError> {
        // Validate lead exists
        self.lead_repo.find_by_id(input.lead_id).await?
            .ok_or_else(|| RepositoryError::NotFound(format!("Lead not found: {}", input.lead_id)))?;

        let now = Utc::now();
        let sale = Sale {
            id: Uuid::new_v4(),
            lead_id: input.lead_id,
            stage_id: input.stage_id,
            value: input.value,
            description: input.description,
            closed_by_id: input.closed_by_id,
            created_at: now,
            updated_at: now,
            closed_at: None,
        };

        let created = self.sale_repo.create(&sale).await?;

        let audit_event = LeadAuditEvent {
            id: Uuid::new_v4(),
            lead_id: input.lead_id,
            event_type: LeadAuditEventType::SaleAttempted,
            description: format!("Tentativa de venda registrada - valor: {:.2}, stage: {}", created.value, created.stage_id),
            performed_by_id: Some(created.closed_by_id),
            created_at: now,
        };
        self.audit_repo.record(&audit_event).await?;

        Ok(CreateSaleAttemptOutput { sale: created })
    }
}
