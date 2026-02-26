use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use shared::domain::ports::RepositoryError;

use crate::domain::entities::{Contract, ContractStatus, LeadAuditEvent, LeadAuditEventType};
use crate::domain::ports::{ContractRepository, LeadAuditRepository};
use crate::application::ports::{
    CreateContractInput, CreateContractOutput, CreateContractUseCase,
    UpdateContractStatusInput, UpdateContractStatusOutput, UpdateContractStatusUseCase,
};

pub struct ContractService<C, A>
where
    C: ContractRepository,
    A: LeadAuditRepository,
{
    contract_repo: C,
    audit_repo: A,
}

impl<C, A> ContractService<C, A>
where
    C: ContractRepository,
    A: LeadAuditRepository,
{
    pub fn new(contract_repo: C, audit_repo: A) -> Self {
        Self { contract_repo, audit_repo }
    }
}

#[async_trait]
impl<C, A> CreateContractUseCase for ContractService<C, A>
where
    C: ContractRepository + Send + Sync,
    A: LeadAuditRepository + Send + Sync,
{
    async fn execute(&self, input: CreateContractInput) -> Result<CreateContractOutput, RepositoryError> {
        // Check for duplicate contract number
        if self.contract_repo.find_by_number(&input.contract_number).await?.is_some() {
            return Err(RepositoryError::ValidationError(
                format!("Numero de contrato ja existe: {}", input.contract_number)
            ));
        }

        let now = Utc::now();
        let contract = Contract {
            id: Uuid::new_v4(),
            associate_id: input.associate_id,
            contract_number: input.contract_number,
            value: input.value,
            status: ContractStatus::Draft,
            start_date: input.start_date,
            end_date: None,
            created_at: now,
            updated_at: now,
        };

        let created = self.contract_repo.create(&contract).await?;

        let audit_event = LeadAuditEvent {
            id: Uuid::new_v4(),
            lead_id: input.associate_id,
            event_type: LeadAuditEventType::ContractCreated,
            description: format!("Contrato {} criado - valor: {:.2}", created.contract_number, created.value),
            performed_by_id: None,
            created_at: now,
        };
        self.audit_repo.record(&audit_event).await?;

        Ok(CreateContractOutput { contract: created })
    }
}

#[async_trait]
impl<C, A> UpdateContractStatusUseCase for ContractService<C, A>
where
    C: ContractRepository + Send + Sync,
    A: LeadAuditRepository + Send + Sync,
{
    async fn execute(&self, input: UpdateContractStatusInput) -> Result<UpdateContractStatusOutput, RepositoryError> {
        let mut contract = self.contract_repo.find_by_id(input.contract_id).await?
            .ok_or_else(|| RepositoryError::NotFound(format!("Contract not found: {}", input.contract_id)))?;

        contract.status = input.new_status;
        contract.updated_at = Utc::now();
        let updated = self.contract_repo.update(&contract).await?;
        Ok(UpdateContractStatusOutput { contract: updated })
    }
}
