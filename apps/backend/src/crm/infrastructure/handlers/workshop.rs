use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::ports::{
    CreateWorkshopInput, CreateWorkshopUseCase, UpdateWorkshopStatusInput,
    UpdateWorkshopStatusUseCase,
};
use crate::domain::entities::{Workshop, WorkshopStatus, WorkshopType};
use shared::domain::ports::RepositoryError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkshopRequest {
    pub associate_id: Uuid,
    pub scheduled_date: DateTime<Utc>,
    pub workshop_type: WorkshopType,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorkshopStatusRequest {
    pub new_status: WorkshopStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkshopResponse {
    pub workshop: Workshop,
}

/// POST /crm/workshops
/// Agenda um workshop para o associado.
pub async fn create_workshop<U>(
    use_case: &U,
    req: CreateWorkshopRequest,
) -> Result<WorkshopResponse, RepositoryError>
where
    U: CreateWorkshopUseCase,
{
    let input = CreateWorkshopInput {
        associate_id: req.associate_id,
        scheduled_date: req.scheduled_date,
        workshop_type: req.workshop_type,
        notes: req.notes,
    };
    let output = use_case.execute(input).await?;
    Ok(WorkshopResponse { workshop: output.workshop })
}

/// PUT /crm/workshops/{id}/status
/// Atualiza o status de um workshop.
pub async fn update_workshop_status<U>(
    use_case: &U,
    workshop_id: Uuid,
    req: UpdateWorkshopStatusRequest,
) -> Result<WorkshopResponse, RepositoryError>
where
    U: UpdateWorkshopStatusUseCase,
{
    let input = UpdateWorkshopStatusInput {
        workshop_id,
        new_status: req.new_status,
    };
    let output = use_case.execute(input).await?;
    Ok(WorkshopResponse { workshop: output.workshop })
}
