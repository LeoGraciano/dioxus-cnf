use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::ports::{
    CreateLeadInput, CreateLeadUseCase, GetLeadHistoryInput, GetLeadHistoryUseCase,
    UpdateLeadStatusInput, UpdateLeadStatusUseCase,
};
use crate::domain::entities::{Lead, LeadAuditEvent, LeadSource, LeadStatus};
use shared::domain::ports::RepositoryError;

// ── Request/Response DTOs ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLeadRequest {
    pub name: String,
    pub source: LeadSource,
    pub cpf: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub observation: Option<String>,
    pub assigned_to_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLeadStatusRequest {
    pub new_status: LeadStatus,
    pub performed_by_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadResponse {
    pub lead: Lead,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadListResponse {
    pub leads: Vec<Lead>,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadHistoryResponse {
    pub lead_id: Uuid,
    pub events: Vec<LeadAuditEvent>,
    pub total: usize,
}

// ── Handler Functions ────────────────────────────────────────────────────────

/// POST /crm/leads
/// Cria um novo lead e registra evento de auditoria Created.
pub async fn create_lead<U>(
    use_case: &U,
    req: CreateLeadRequest,
) -> Result<LeadResponse, RepositoryError>
where
    U: CreateLeadUseCase,
{
    let input = CreateLeadInput {
        source: req.source,
        name: req.name,
        cpf: req.cpf,
        phone: req.phone,
        email: req.email,
        observation: req.observation,
        assigned_to_id: req.assigned_to_id,
    };
    let output = use_case.execute(input).await?;
    Ok(LeadResponse { lead: output.lead })
}

/// PUT /crm/leads/{id}/status
/// Atualiza o status do lead e registra evento de auditoria StatusChanged/Won/Lost/Canceled.
pub async fn update_lead_status<U>(
    use_case: &U,
    lead_id: Uuid,
    req: UpdateLeadStatusRequest,
) -> Result<LeadResponse, RepositoryError>
where
    U: UpdateLeadStatusUseCase,
{
    let input = UpdateLeadStatusInput {
        lead_id,
        new_status: req.new_status,
        performed_by_id: req.performed_by_id,
    };
    let output = use_case.execute(input).await?;
    Ok(LeadResponse { lead: output.lead })
}

/// GET /crm/leads/{id}/history
/// Retorna o histórico auditável de eventos do lead, ponta a ponta.
pub async fn get_lead_history<U>(
    use_case: &U,
    lead_id: Uuid,
) -> Result<LeadHistoryResponse, RepositoryError>
where
    U: GetLeadHistoryUseCase,
{
    let input = GetLeadHistoryInput { lead_id };
    let output = use_case.execute(input).await?;
    let total = output.events.len();
    Ok(LeadHistoryResponse { lead_id, events: output.events, total })
}
