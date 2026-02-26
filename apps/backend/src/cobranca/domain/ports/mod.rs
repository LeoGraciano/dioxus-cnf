use async_trait::async_trait;
use uuid::Uuid;

use shared::domain::ports::RepositoryError;
use super::entities::{Title, Installment, Negotiation, DistributionLog};

#[async_trait]
pub trait TitleRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Title>, RepositoryError>;
    async fn find_by_associate(&self, associate_id: Uuid) -> Result<Vec<Title>, RepositoryError>;
    async fn find_by_document(&self, document_number: &str) -> Result<Option<Title>, RepositoryError>;
    async fn create(&self, title: &Title) -> Result<Title, RepositoryError>;
    async fn update(&self, title: &Title) -> Result<Title, RepositoryError>;
}

#[async_trait]
pub trait InstallmentRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Installment>, RepositoryError>;
    async fn find_by_title(&self, title_id: Uuid) -> Result<Vec<Installment>, RepositoryError>;
    async fn find_by_associate(&self, associate_id: Uuid) -> Result<Vec<Installment>, RepositoryError>;
    async fn find_overdue(&self, workgroup_id: i32) -> Result<Vec<Installment>, RepositoryError>;
    async fn create(&self, installment: &Installment) -> Result<Installment, RepositoryError>;
    async fn update(&self, installment: &Installment) -> Result<Installment, RepositoryError>;
}

#[async_trait]
pub trait NegotiationRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Negotiation>, RepositoryError>;
    async fn find_by_installment(&self, installment_id: Uuid) -> Result<Vec<Negotiation>, RepositoryError>;
    async fn find_by_associate(&self, associate_id: Uuid) -> Result<Vec<Negotiation>, RepositoryError>;
    // Append-only: no update allowed
    async fn create(&self, negotiation: &Negotiation) -> Result<Negotiation, RepositoryError>;
}

#[async_trait]
pub trait DistributionLogRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DistributionLog>, RepositoryError>;
    async fn find_by_workgroup(&self, workgroup_id: i32) -> Result<Vec<DistributionLog>, RepositoryError>;
    // Append-only: no update allowed
    async fn create(&self, log: &DistributionLog) -> Result<DistributionLog, RepositoryError>;
}
