use async_trait::async_trait;

use shared::domain::ports::RepositoryError;

use crate::domain::entities::{Divergence, DivergenceIssue, ReconciliationReport, ReconciliationRequest};
use crate::domain::ports::{ReconciliationReporter, SyncEventRepository};
use crate::application::ports::{ReconcileBatchInput, ReconcileBatchOutput, ReconcileBatchUseCase};

#[allow(dead_code)]
pub struct ReconciliationService<R, S>
where
    R: ReconciliationReporter,
    S: SyncEventRepository,
{
    pub(crate) reporter: R,
    pub(crate) sync_repo: S,
}

impl<R, S> ReconciliationService<R, S>
where
    R: ReconciliationReporter,
    S: SyncEventRepository,
{
    pub fn new(reporter: R, sync_repo: S) -> Self {
        Self { reporter, sync_repo }
    }

    pub async fn reconcile_entity(
        &self,
        request: ReconciliationRequest,
        actual_count: u32,
        actual_ids: Vec<String>,
        expected_ids: Vec<String>,
    ) -> ReconciliationReport {
        let mut divergences = Vec::new();

        let expected_set: std::collections::HashSet<_> = expected_ids.iter().collect();
        let actual_set: std::collections::HashSet<_> = actual_ids.iter().collect();

        for id in &expected_ids {
            if !actual_set.contains(id) {
                divergences.push(Divergence {
                    entity_id: id.clone(),
                    issue: DivergenceIssue::Missing,
                });
            }
        }

        for id in &actual_ids {
            if !expected_set.contains(id) {
                divergences.push(Divergence {
                    entity_id: id.clone(),
                    issue: DivergenceIssue::ExtraRecord,
                });
            }
        }

        ReconciliationReport::new(
            request.batch_id,
            request.entity_type,
            request.expected_count,
            actual_count,
            divergences,
        )
    }

    pub fn generate_text_report(&self, report: &ReconciliationReport) -> String {
        let mut output = String::new();
        output.push_str("Reconciliation Report\n");
        output.push_str("====================\n");
        output.push_str(&format!("Batch ID: {}\n", report.batch_id));
        output.push_str(&format!("Entity Type: {}\n", report.entity_type));
        output.push_str(&format!("Expected Count: {}\n", report.expected_count));
        output.push_str(&format!("Actual Count: {}\n", report.actual_count));
        output.push_str(&format!("Divergences: {}\n", report.divergences.len()));
        output.push_str(&format!("Generated At: {}\n", report.created_at));
        
        if !report.divergences.is_empty() {
            output.push_str("\nDivergence Details:\n");
            for div in &report.divergences {
                output.push_str(&format!("  - {}: {:?}\n", div.entity_id, div.issue));
            }
        }
        
        output
    }
}

#[async_trait]
impl<R, S> ReconcileBatchUseCase for ReconciliationService<R, S>
where
    R: ReconciliationReporter + Send + Sync,
    S: SyncEventRepository + Send + Sync,
{
    async fn execute(&self, input: ReconcileBatchInput) -> Result<ReconcileBatchOutput, RepositoryError> {
        let report = self.reporter.report_divergences(input.batch_id, &input.entity_type).await?;

        let needs_retry = report.has_divergences();

        Ok(ReconcileBatchOutput { report, needs_retry })
    }
}
