use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::ports::{IssueBraceletInput, IssueBraceletUseCase};
use crate::domain::entities::Bracelet;
use shared::domain::ports::RepositoryError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueBraceletRequest {
    pub associate_id: Uuid,
    pub bracelet_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BraceletResponse {
    pub bracelet: Bracelet,
}

/// POST /crm/bracelets
/// Emite uma pulseira para o associado. Valida unicidade do número.
pub async fn issue_bracelet<U>(
    use_case: &U,
    req: IssueBraceletRequest,
) -> Result<BraceletResponse, RepositoryError>
where
    U: IssueBraceletUseCase,
{
    let input = IssueBraceletInput {
        associate_id: req.associate_id,
        bracelet_number: req.bracelet_number,
    };
    let output = use_case.execute(input).await?;
    Ok(BraceletResponse { bracelet: output.bracelet })
}
