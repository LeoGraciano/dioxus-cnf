use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::ports::{CreateSaleAttemptInput, CreateSaleAttemptUseCase};
use crate::domain::entities::Sale;
use shared::domain::ports::RepositoryError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSaleAttemptRequest {
    pub lead_id: Uuid,
    pub stage_id: i32,
    pub value: f64,
    pub description: Option<String>,
    pub closed_by_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleResponse {
    pub sale: Sale,
}

/// POST /crm/sales
/// Registra uma tentativa de venda vinculada a um lead.
pub async fn create_sale_attempt<U>(
    use_case: &U,
    req: CreateSaleAttemptRequest,
) -> Result<SaleResponse, RepositoryError>
where
    U: CreateSaleAttemptUseCase,
{
    let input = CreateSaleAttemptInput {
        lead_id: req.lead_id,
        stage_id: req.stage_id,
        value: req.value,
        description: req.description,
        closed_by_id: req.closed_by_id,
    };
    let output = use_case.execute(input).await?;
    Ok(SaleResponse { sale: output.sale })
}
