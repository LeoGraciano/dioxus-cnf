use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::ports::{
    RecordCheckInInput, RecordCheckInUseCase, RecordCheckOutInput, RecordCheckOutUseCase,
};
use crate::domain::entities::CheckIn;
use shared::domain::ports::RepositoryError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckInRequest {
    pub associate_id: Uuid,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckInResponse {
    pub check_in: CheckIn,
}

/// POST /crm/checkin
/// Registra a entrada de um associado.
pub async fn record_check_in<U>(
    use_case: &U,
    req: CheckInRequest,
) -> Result<CheckInResponse, RepositoryError>
where
    U: RecordCheckInUseCase,
{
    let input = RecordCheckInInput {
        associate_id: req.associate_id,
        location: req.location,
    };
    let output = use_case.execute(input).await?;
    Ok(CheckInResponse { check_in: output.check_in })
}

/// POST /crm/checkin/{id}/checkout
/// Registra a saída de um associado (check-out).
pub async fn record_check_out<U>(
    use_case: &U,
    check_in_id: Uuid,
) -> Result<CheckInResponse, RepositoryError>
where
    U: RecordCheckOutUseCase,
{
    let input = RecordCheckOutInput { check_in_id };
    let output = use_case.execute(input).await?;
    Ok(CheckInResponse { check_in: output.check_in })
}
