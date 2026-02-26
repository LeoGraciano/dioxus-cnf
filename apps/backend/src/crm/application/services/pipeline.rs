use async_trait::async_trait;

use shared::domain::ports::RepositoryError;

use crate::domain::ports::PipelineStageRepository;
use crate::application::ports::{GetPipelineStagesOutput, GetPipelineStagesUseCase};

pub struct PipelineService<P>
where
    P: PipelineStageRepository,
{
    pipeline_repo: P,
}

impl<P> PipelineService<P>
where
    P: PipelineStageRepository,
{
    pub fn new(pipeline_repo: P) -> Self {
        Self { pipeline_repo }
    }
}

#[async_trait]
impl<P> GetPipelineStagesUseCase for PipelineService<P>
where
    P: PipelineStageRepository + Send + Sync,
{
    async fn execute(&self) -> Result<GetPipelineStagesOutput, RepositoryError> {
        let stages = self.pipeline_repo.find_all().await?;
        Ok(GetPipelineStagesOutput { stages })
    }
}
