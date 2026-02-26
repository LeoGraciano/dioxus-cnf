use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::ports::{
    ScheduleAppointmentInput, ScheduleAppointmentUseCase,
    UpdateSchedulingStatusInput, UpdateSchedulingStatusUseCase,
};
use crate::domain::entities::{Scheduling, SchedulingStatus, SchedulingType};
use shared::domain::ports::RepositoryError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleAppointmentRequest {
    pub associate_id: Uuid,
    pub scheduled_date: DateTime<Utc>,
    pub scheduling_type: SchedulingType,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSchedulingStatusRequest {
    pub new_status: SchedulingStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingResponse {
    pub scheduling: Scheduling,
}

/// POST /crm/scheduling
/// Cria um novo agendamento para o associado.
pub async fn schedule_appointment<U>(
    use_case: &U,
    req: ScheduleAppointmentRequest,
) -> Result<SchedulingResponse, RepositoryError>
where
    U: ScheduleAppointmentUseCase,
{
    let input = ScheduleAppointmentInput {
        associate_id: req.associate_id,
        scheduled_date: req.scheduled_date,
        scheduling_type: req.scheduling_type,
        notes: req.notes,
    };
    let output = use_case.execute(input).await?;
    Ok(SchedulingResponse { scheduling: output.scheduling })
}

/// PUT /crm/scheduling/{id}/status
/// Atualiza o status de um agendamento.
pub async fn update_scheduling_status<U>(
    use_case: &U,
    scheduling_id: Uuid,
    req: UpdateSchedulingStatusRequest,
) -> Result<SchedulingResponse, RepositoryError>
where
    U: UpdateSchedulingStatusUseCase,
{
    let input = UpdateSchedulingStatusInput {
        scheduling_id,
        new_status: req.new_status,
    };
    let output = use_case.execute(input).await?;
    Ok(SchedulingResponse { scheduling: output.scheduling })
}
