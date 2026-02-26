use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use shared::domain::ports::RepositoryError;

use crate::domain::entities::{LeadAuditEvent, LeadAuditEventType, Workshop};
use crate::domain::ports::{LeadAuditRepository, WorkshopRepository};
use crate::application::ports::{
    CreateWorkshopInput, CreateWorkshopOutput, CreateWorkshopUseCase,
    UpdateWorkshopStatusInput, UpdateWorkshopStatusOutput, UpdateWorkshopStatusUseCase,
};

pub struct WorkshopService<W, A>
where
    W: WorkshopRepository,
    A: LeadAuditRepository,
{
    workshop_repo: W,
    audit_repo: A,
}

impl<W, A> WorkshopService<W, A>
where
    W: WorkshopRepository,
    A: LeadAuditRepository,
{
    pub fn new(workshop_repo: W, audit_repo: A) -> Self {
        Self { workshop_repo, audit_repo }
    }
}

#[async_trait]
impl<W, A> CreateWorkshopUseCase for WorkshopService<W, A>
where
    W: WorkshopRepository + Send + Sync,
    A: LeadAuditRepository + Send + Sync,
{
    async fn execute(&self, input: CreateWorkshopInput) -> Result<CreateWorkshopOutput, RepositoryError> {
        let now = Utc::now();
        let workshop = Workshop {
            id: Uuid::new_v4(),
            associate_id: input.associate_id,
            scheduled_date: input.scheduled_date,
            workshop_type: input.workshop_type,
            status: crate::domain::entities::WorkshopStatus::Scheduled,
            notes: input.notes,
            created_at: now,
        };

        let created = self.workshop_repo.create(&workshop).await?;

        // Record audit for the associate (workshop is tied to associate, not lead directly)
        let audit_event = LeadAuditEvent {
            id: Uuid::new_v4(),
            lead_id: input.associate_id,
            event_type: LeadAuditEventType::WorkshopScheduled,
            description: format!("Workshop agendado para {:?} em {}", created.workshop_type, created.scheduled_date),
            performed_by_id: None,
            created_at: now,
        };
        self.audit_repo.record(&audit_event).await?;

        Ok(CreateWorkshopOutput { workshop: created })
    }
}

#[async_trait]
impl<W, A> UpdateWorkshopStatusUseCase for WorkshopService<W, A>
where
    W: WorkshopRepository + Send + Sync,
    A: LeadAuditRepository + Send + Sync,
{
    async fn execute(&self, input: UpdateWorkshopStatusInput) -> Result<UpdateWorkshopStatusOutput, RepositoryError> {
        let mut workshop = self.workshop_repo.find_by_id(input.workshop_id).await?
            .ok_or_else(|| RepositoryError::NotFound(format!("Workshop not found: {}", input.workshop_id)))?;

        workshop.status = input.new_status;
        let updated = self.workshop_repo.update(&workshop).await?;
        Ok(UpdateWorkshopStatusOutput { workshop: updated })
    }
}
