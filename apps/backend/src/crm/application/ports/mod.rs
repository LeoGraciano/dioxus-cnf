use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use shared::domain::ports::RepositoryError;

use crate::domain::entities::{
    Bracelet, CheckIn, Contract, ContractStatus, Lead, LeadAuditEvent,
    LeadSource, LeadStatus, PipelineStage, Sale, Scheduling, SchedulingStatus, SchedulingType,
    Workshop, WorkshopStatus, WorkshopType,
};

// ── Lead use-cases ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CreateLeadInput {
    pub source: LeadSource,
    pub name: String,
    pub cpf: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub observation: Option<String>,
    pub assigned_to_id: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct CreateLeadOutput {
    pub lead: Lead,
}

#[async_trait]
pub trait CreateLeadUseCase: Send + Sync {
    async fn execute(&self, input: CreateLeadInput) -> Result<CreateLeadOutput, RepositoryError>;
}

#[derive(Debug, Clone)]
pub struct UpdateLeadStatusInput {
    pub lead_id: Uuid,
    pub new_status: LeadStatus,
    pub performed_by_id: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct UpdateLeadStatusOutput {
    pub lead: Lead,
    pub audit_event: LeadAuditEvent,
}

#[async_trait]
pub trait UpdateLeadStatusUseCase: Send + Sync {
    async fn execute(&self, input: UpdateLeadStatusInput) -> Result<UpdateLeadStatusOutput, RepositoryError>;
}

#[derive(Debug, Clone)]
pub struct GetLeadHistoryInput {
    pub lead_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct GetLeadHistoryOutput {
    pub events: Vec<LeadAuditEvent>,
}

#[async_trait]
pub trait GetLeadHistoryUseCase: Send + Sync {
    async fn execute(&self, input: GetLeadHistoryInput) -> Result<GetLeadHistoryOutput, RepositoryError>;
}

// ── Pipeline use-cases ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct GetPipelineStagesOutput {
    pub stages: Vec<PipelineStage>,
}

#[async_trait]
pub trait GetPipelineStagesUseCase: Send + Sync {
    async fn execute(&self) -> Result<GetPipelineStagesOutput, RepositoryError>;
}

// ── Scheduling use-cases ──────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ScheduleAppointmentInput {
    pub associate_id: Uuid,
    pub scheduled_date: DateTime<Utc>,
    pub scheduling_type: SchedulingType,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ScheduleAppointmentOutput {
    pub scheduling: Scheduling,
}

#[async_trait]
pub trait ScheduleAppointmentUseCase: Send + Sync {
    async fn execute(&self, input: ScheduleAppointmentInput) -> Result<ScheduleAppointmentOutput, RepositoryError>;
}

#[derive(Debug, Clone)]
pub struct UpdateSchedulingStatusInput {
    pub scheduling_id: Uuid,
    pub new_status: SchedulingStatus,
}

#[derive(Debug, Clone)]
pub struct UpdateSchedulingStatusOutput {
    pub scheduling: Scheduling,
}

#[async_trait]
pub trait UpdateSchedulingStatusUseCase: Send + Sync {
    async fn execute(&self, input: UpdateSchedulingStatusInput) -> Result<UpdateSchedulingStatusOutput, RepositoryError>;
}

// ── Check-in use-cases ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct RecordCheckInInput {
    pub associate_id: Uuid,
    pub location: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RecordCheckInOutput {
    pub check_in: CheckIn,
}

#[async_trait]
pub trait RecordCheckInUseCase: Send + Sync {
    async fn execute(&self, input: RecordCheckInInput) -> Result<RecordCheckInOutput, RepositoryError>;
}

#[derive(Debug, Clone)]
pub struct RecordCheckOutInput {
    pub check_in_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct RecordCheckOutOutput {
    pub check_in: CheckIn,
}

#[async_trait]
pub trait RecordCheckOutUseCase: Send + Sync {
    async fn execute(&self, input: RecordCheckOutInput) -> Result<RecordCheckOutOutput, RepositoryError>;
}

// ── Workshop use-cases ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CreateWorkshopInput {
    pub associate_id: Uuid,
    pub scheduled_date: DateTime<Utc>,
    pub workshop_type: WorkshopType,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreateWorkshopOutput {
    pub workshop: Workshop,
}

#[async_trait]
pub trait CreateWorkshopUseCase: Send + Sync {
    async fn execute(&self, input: CreateWorkshopInput) -> Result<CreateWorkshopOutput, RepositoryError>;
}

#[derive(Debug, Clone)]
pub struct UpdateWorkshopStatusInput {
    pub workshop_id: Uuid,
    pub new_status: WorkshopStatus,
}

#[derive(Debug, Clone)]
pub struct UpdateWorkshopStatusOutput {
    pub workshop: Workshop,
}

#[async_trait]
pub trait UpdateWorkshopStatusUseCase: Send + Sync {
    async fn execute(&self, input: UpdateWorkshopStatusInput) -> Result<UpdateWorkshopStatusOutput, RepositoryError>;
}

// ── Sale attempt use-cases ────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CreateSaleAttemptInput {
    pub lead_id: Uuid,
    pub stage_id: i32,
    pub value: f64,
    pub description: Option<String>,
    pub closed_by_id: i32,
}

#[derive(Debug, Clone)]
pub struct CreateSaleAttemptOutput {
    pub sale: Sale,
}

#[async_trait]
pub trait CreateSaleAttemptUseCase: Send + Sync {
    async fn execute(&self, input: CreateSaleAttemptInput) -> Result<CreateSaleAttemptOutput, RepositoryError>;
}

// ── Contract use-cases ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CreateContractInput {
    pub associate_id: Uuid,
    pub contract_number: String,
    pub value: f64,
    pub start_date: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateContractOutput {
    pub contract: Contract,
}

#[async_trait]
pub trait CreateContractUseCase: Send + Sync {
    async fn execute(&self, input: CreateContractInput) -> Result<CreateContractOutput, RepositoryError>;
}

#[derive(Debug, Clone)]
pub struct UpdateContractStatusInput {
    pub contract_id: Uuid,
    pub new_status: ContractStatus,
}

#[derive(Debug, Clone)]
pub struct UpdateContractStatusOutput {
    pub contract: Contract,
}

#[async_trait]
pub trait UpdateContractStatusUseCase: Send + Sync {
    async fn execute(&self, input: UpdateContractStatusInput) -> Result<UpdateContractStatusOutput, RepositoryError>;
}

// ── Bracelet use-cases ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct IssueBraceletInput {
    pub associate_id: Uuid,
    pub bracelet_number: String,
}

#[derive(Debug, Clone)]
pub struct IssueBraceletOutput {
    pub bracelet: Bracelet,
}

#[async_trait]
pub trait IssueBraceletUseCase: Send + Sync {
    async fn execute(&self, input: IssueBraceletInput) -> Result<IssueBraceletOutput, RepositoryError>;
}
