use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::ports::{
    CreateContractInput, CreateContractUseCase, UpdateContractStatusInput,
    UpdateContractStatusUseCase,
};
use crate::domain::entities::{Contract, ContractStatus};
use shared::domain::ports::RepositoryError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContractRequest {
    pub associate_id: Uuid,
    pub contract_number: String,
    pub value: f64,
    pub start_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContractStatusRequest {
    pub new_status: ContractStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractResponse {
    pub contract: Contract,
}

/// POST /crm/contracts
/// Cria um contrato para o associado no estágio Draft.
pub async fn create_contract<U>(
    use_case: &U,
    req: CreateContractRequest,
) -> Result<ContractResponse, RepositoryError>
where
    U: CreateContractUseCase,
{
    let input = CreateContractInput {
        associate_id: req.associate_id,
        contract_number: req.contract_number,
        value: req.value,
        start_date: req.start_date,
    };
    let output = use_case.execute(input).await?;
    Ok(ContractResponse { contract: output.contract })
}

/// PUT /crm/contracts/{id}/status
/// Atualiza o status do contrato (Draft → PendingSignature → Active → ...).
pub async fn update_contract_status<U>(
    use_case: &U,
    contract_id: Uuid,
    req: UpdateContractStatusRequest,
) -> Result<ContractResponse, RepositoryError>
where
    U: UpdateContractStatusUseCase,
{
    let input = UpdateContractStatusInput {
        contract_id,
        new_status: req.new_status,
    };
    let output = use_case.execute(input).await?;
    Ok(ContractResponse { contract: output.contract })
}
