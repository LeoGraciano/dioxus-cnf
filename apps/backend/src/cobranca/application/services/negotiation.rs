use async_trait::async_trait;

use shared::domain::ports::RepositoryError;

use crate::application::ports::{
    GetNegotiationHistoryInput, GetNegotiationHistoryOutput, GetNegotiationHistoryUseCase,
    RecordNegotiationInput, RecordNegotiationOutput, RecordNegotiationUseCase,
};
use crate::domain::entities::{InstallmentStatus, Negotiation, NegotiationResult};
use crate::domain::ports::{InstallmentRepository, NegotiationRepository};

pub struct NegotiationService<I, N>
where
    I: InstallmentRepository,
    N: NegotiationRepository,
{
    installment_repo: I,
    negotiation_repo: N,
}

impl<I, N> NegotiationService<I, N>
where
    I: InstallmentRepository,
    N: NegotiationRepository,
{
    pub fn new(installment_repo: I, negotiation_repo: N) -> Self {
        Self {
            installment_repo,
            negotiation_repo,
        }
    }
}

#[async_trait]
impl<I, N> RecordNegotiationUseCase for NegotiationService<I, N>
where
    I: InstallmentRepository,
    N: NegotiationRepository,
{
    async fn execute(
        &self,
        input: RecordNegotiationInput,
    ) -> Result<RecordNegotiationOutput, RepositoryError> {
        // Validate installment exists
        let mut installment = self
            .installment_repo
            .find_by_id(input.installment_id)
            .await?
            .ok_or_else(|| {
                RepositoryError::NotFound(format!(
                    "Installment not found: {}",
                    input.installment_id
                ))
            })?;

        // Validate promise fields for PromiseToPay result
        if input.result == NegotiationResult::PromiseToPay && input.promise_date.is_none() {
            return Err(RepositoryError::ValidationError(
                "promise_date is required when result is PromiseToPay".to_string(),
            ));
        }

        // Create the negotiation record (append-only, immutable)
        let negotiation = Negotiation::new(
            input.installment_id,
            input.associate_id,
            input.user_id,
            input.result.clone(),
            input.promise_date,
            input.promise_value,
            input.observation,
        );
        let saved_negotiation = self.negotiation_repo.create(&negotiation).await?;

        // Update installment status based on negotiation result
        let new_status = match input.result {
            NegotiationResult::Paid => InstallmentStatus::Paid,
            NegotiationResult::Renegotiated => InstallmentStatus::Renegotiated,
            _ => installment.status.clone(),
        };
        installment.status = new_status;
        let updated_installment = self.installment_repo.update(&installment).await?;

        Ok(RecordNegotiationOutput {
            negotiation: saved_negotiation,
            installment: updated_installment,
        })
    }
}

#[async_trait]
impl<I, N> GetNegotiationHistoryUseCase for NegotiationService<I, N>
where
    I: InstallmentRepository,
    N: NegotiationRepository,
{
    async fn execute(
        &self,
        input: GetNegotiationHistoryInput,
    ) -> Result<GetNegotiationHistoryOutput, RepositoryError> {
        let negotiations = self
            .negotiation_repo
            .find_by_installment(input.installment_id)
            .await?;
        Ok(GetNegotiationHistoryOutput { negotiations })
    }
}
