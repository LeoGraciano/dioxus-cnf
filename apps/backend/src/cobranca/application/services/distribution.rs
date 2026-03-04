use async_trait::async_trait;

use shared::domain::ports::RepositoryError;

use crate::application::ports::{
    DistributeInstallmentsInput, DistributeInstallmentsOutput, DistributeInstallmentsUseCase,
};
use crate::domain::entities::{DistributionLog, DistributionType};
use crate::domain::ports::{DistributionLogRepository, InstallmentRepository};

pub struct DistributionService<I, D>
where
    I: InstallmentRepository,
    D: DistributionLogRepository,
{
    installment_repo: I,
    distribution_log_repo: D,
}

impl<I, D> DistributionService<I, D>
where
    I: InstallmentRepository,
    D: DistributionLogRepository,
{
    pub fn new(installment_repo: I, distribution_log_repo: D) -> Self {
        Self {
            installment_repo,
            distribution_log_repo,
        }
    }
}

#[async_trait]
impl<I, D> DistributeInstallmentsUseCase for DistributionService<I, D>
where
    I: InstallmentRepository,
    D: DistributionLogRepository,
{
    async fn execute(
        &self,
        input: DistributeInstallmentsInput,
    ) -> Result<DistributeInstallmentsOutput, RepositoryError> {
        if input.collector_user_ids.is_empty() {
            return Err(RepositoryError::ValidationError(
                "At least one collector is required for distribution".to_string(),
            ));
        }

        if input.min_days_overdue > input.max_days_overdue {
            return Err(RepositoryError::ValidationError(
                "min_days_overdue must be <= max_days_overdue".to_string(),
            ));
        }

        // Fetch overdue installments for this workgroup, filtered by window
        let all_overdue = self
            .installment_repo
            .find_overdue(input.workgroup_id)
            .await?;

        let eligible: Vec<_> = all_overdue
            .into_iter()
            .filter(|inst| {
                let days = inst.days_overdue.unwrap_or(0);
                days >= input.min_days_overdue && days <= input.max_days_overdue
            })
            .collect();

        if eligible.is_empty() {
            return Ok(DistributeInstallmentsOutput {
                distributed_count: 0,
                logs: vec![],
            });
        }

        let collector_count = input.collector_user_ids.len();
        let mut logs: Vec<DistributionLog> = Vec::new();

        match input.distribution_type {
            DistributionType::Installment => {
                // Distribute each installment round-robin across collectors
                for (i, installment) in eligible.iter().enumerate() {
                    let collector_id = input.collector_user_ids[i % collector_count];
                    let log = DistributionLog::new(
                        installment.id,
                        installment.associate_id,
                        collector_id,
                        input.workgroup_id,
                        DistributionType::Installment,
                    );
                    let saved = self.distribution_log_repo.create(&log).await?;
                    logs.push(saved);
                }
            }
            DistributionType::Associate => {
                // Group installments by associate, assign all of an associate's
                // installments to the same collector (round-robin over associates)
                let mut associate_groups: Vec<uuid::Uuid> = Vec::new();
                for inst in &eligible {
                    if !associate_groups.contains(&inst.associate_id) {
                        associate_groups.push(inst.associate_id);
                    }
                }

                for (i, associate_id) in associate_groups.iter().enumerate() {
                    let collector_id = input.collector_user_ids[i % collector_count];
                    // Create one log entry per installment for this associate
                    for installment in eligible.iter().filter(|inst| &inst.associate_id == associate_id) {
                        let log = DistributionLog::new(
                            installment.id,
                            *associate_id,
                            collector_id,
                            input.workgroup_id,
                            DistributionType::Associate,
                        );
                        let saved = self.distribution_log_repo.create(&log).await?;
                        logs.push(saved);
                    }
                }
            }
        }

        let distributed_count = logs.len();
        Ok(DistributeInstallmentsOutput {
            distributed_count,
            logs,
        })
    }
}
