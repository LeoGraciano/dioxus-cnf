use serde::{Deserialize, Serialize};

use crate::application::ports::{GetPipelineStagesOutput, GetPipelineStagesUseCase};
use crate::domain::entities::PipelineStage;
use shared::domain::ports::RepositoryError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStagesResponse {
    pub stages: Vec<PipelineStage>,
}

/// GET /crm/pipeline
/// Retorna todos os estágios do pipeline de vendas.
pub async fn get_pipeline_stages<U>(
    use_case: &U,
) -> Result<PipelineStagesResponse, RepositoryError>
where
    U: GetPipelineStagesUseCase,
{
    let output: GetPipelineStagesOutput = use_case.execute().await?;
    Ok(PipelineStagesResponse { stages: output.stages })
}
