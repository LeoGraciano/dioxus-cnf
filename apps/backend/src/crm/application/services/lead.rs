use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use shared::domain::ports::RepositoryError;

use crate::domain::entities::{Lead, LeadAuditEvent, LeadAuditEventType, LeadStatus};
use crate::domain::ports::{LeadAuditRepository, LeadRepository};
use crate::application::ports::{
    CreateLeadInput, CreateLeadOutput, CreateLeadUseCase,
    GetLeadHistoryInput, GetLeadHistoryOutput, GetLeadHistoryUseCase,
    UpdateLeadStatusInput, UpdateLeadStatusOutput, UpdateLeadStatusUseCase,
};

pub struct LeadService<L, A>
where
    L: LeadRepository,
    A: LeadAuditRepository,
{
    lead_repo: L,
    audit_repo: A,
}

impl<L, A> LeadService<L, A>
where
    L: LeadRepository,
    A: LeadAuditRepository,
{
    pub fn new(lead_repo: L, audit_repo: A) -> Self {
        Self { lead_repo, audit_repo }
    }
}

#[async_trait]
impl<L, A> CreateLeadUseCase for LeadService<L, A>
where
    L: LeadRepository + Send + Sync,
    A: LeadAuditRepository + Send + Sync,
{
    async fn execute(&self, input: CreateLeadInput) -> Result<CreateLeadOutput, RepositoryError> {
        let now = Utc::now();
        let lead = Lead {
            id: Uuid::new_v4(),
            id_client_esolution: None,
            source: input.source,
            status: LeadStatus::New,
            name: input.name,
            cpf: input.cpf,
            phone: input.phone,
            email: input.email,
            observation: input.observation,
            assigned_to_id: input.assigned_to_id,
            created_at: now,
            updated_at: now,
            converted_at: None,
        };

        let created = self.lead_repo.create(&lead).await?;

        let audit_event = LeadAuditEvent {
            id: Uuid::new_v4(),
            lead_id: created.id,
            event_type: LeadAuditEventType::Created,
            description: format!("Lead '{}' criado via {:?}", created.name, created.source),
            performed_by_id: created.assigned_to_id,
            created_at: now,
        };
        self.audit_repo.record(&audit_event).await?;

        Ok(CreateLeadOutput { lead: created })
    }
}

#[async_trait]
impl<L, A> UpdateLeadStatusUseCase for LeadService<L, A>
where
    L: LeadRepository + Send + Sync,
    A: LeadAuditRepository + Send + Sync,
{
    async fn execute(&self, input: UpdateLeadStatusInput) -> Result<UpdateLeadStatusOutput, RepositoryError> {
        let mut lead = self.lead_repo.find_by_id(input.lead_id).await?
            .ok_or_else(|| RepositoryError::NotFound(format!("Lead not found: {}", input.lead_id)))?;

        let old_status = lead.status.clone();
        lead.status = input.new_status.clone();
        lead.updated_at = Utc::now();

        if matches!(input.new_status, LeadStatus::Won) {
            lead.converted_at = Some(Utc::now());
        }

        let updated = self.lead_repo.update(&lead).await?;

        let event_type = match &input.new_status {
            LeadStatus::Won => LeadAuditEventType::Converted,
            LeadStatus::Lost => LeadAuditEventType::Lost,
            LeadStatus::Canceled => LeadAuditEventType::Canceled,
            _ => LeadAuditEventType::StatusChanged,
        };

        let audit_event = LeadAuditEvent {
            id: Uuid::new_v4(),
            lead_id: updated.id,
            event_type,
            description: format!("Status alterado de {:?} para {:?}", old_status, input.new_status),
            performed_by_id: input.performed_by_id,
            created_at: Utc::now(),
        };
        let recorded = self.audit_repo.record(&audit_event).await?;

        Ok(UpdateLeadStatusOutput { lead: updated, audit_event: recorded })
    }
}

#[async_trait]
impl<L, A> GetLeadHistoryUseCase for LeadService<L, A>
where
    L: LeadRepository + Send + Sync,
    A: LeadAuditRepository + Send + Sync,
{
    async fn execute(&self, input: GetLeadHistoryInput) -> Result<GetLeadHistoryOutput, RepositoryError> {
        let events = self.audit_repo.find_by_lead_id(input.lead_id).await?;
        Ok(GetLeadHistoryOutput { events })
    }
}
