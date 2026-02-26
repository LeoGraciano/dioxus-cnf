use async_trait::async_trait;
use uuid::Uuid;

use shared::domain::ports::RepositoryError;
use super::entities::{Lead, Sale, Workshop, Contract, Bracelet, Scheduling, CheckIn, PipelineStage};

#[async_trait]
pub trait LeadRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Lead>, RepositoryError>;
    async fn find_by_cpf(&self, cpf: &str) -> Result<Vec<Lead>, RepositoryError>;
    async fn find_all(&self, page: u32, per_page: u32) -> Result<Vec<Lead>, RepositoryError>;
    async fn create(&self, lead: &Lead) -> Result<Lead, RepositoryError>;
    async fn update(&self, lead: &Lead) -> Result<Lead, RepositoryError>;
}

#[async_trait]
pub trait SaleRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Sale>, RepositoryError>;
    async fn find_by_lead_id(&self, lead_id: Uuid) -> Result<Vec<Sale>, RepositoryError>;
    async fn create(&self, sale: &Sale) -> Result<Sale, RepositoryError>;
    async fn update(&self, sale: &Sale) -> Result<Sale, RepositoryError>;
}

#[async_trait]
pub trait WorkshopRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Workshop>, RepositoryError>;
    async fn find_by_associate(&self, associate_id: Uuid) -> Result<Vec<Workshop>, RepositoryError>;
    async fn find_upcoming(&self, date: chrono::DateTime<chrono::Utc>) -> Result<Vec<Workshop>, RepositoryError>;
    async fn create(&self, workshop: &Workshop) -> Result<Workshop, RepositoryError>;
    async fn update(&self, workshop: &Workshop) -> Result<Workshop, RepositoryError>;
}

#[async_trait]
pub trait ContractRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Contract>, RepositoryError>;
    async fn find_by_associate(&self, associate_id: Uuid) -> Result<Vec<Contract>, RepositoryError>;
    async fn find_by_number(&self, contract_number: &str) -> Result<Option<Contract>, RepositoryError>;
    async fn create(&self, contract: &Contract) -> Result<Contract, RepositoryError>;
    async fn update(&self, contract: &Contract) -> Result<Contract, RepositoryError>;
}

#[async_trait]
pub trait BraceletRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Bracelet>, RepositoryError>;
    async fn find_by_associate(&self, associate_id: Uuid) -> Result<Vec<Bracelet>, RepositoryError>;
    async fn find_by_number(&self, bracelet_number: &str) -> Result<Option<Bracelet>, RepositoryError>;
    async fn create(&self, bracelet: &Bracelet) -> Result<Bracelet, RepositoryError>;
}

#[async_trait]
pub trait SchedulingRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Scheduling>, RepositoryError>;
    async fn find_by_associate(&self, associate_id: Uuid) -> Result<Vec<Scheduling>, RepositoryError>;
    async fn create(&self, scheduling: &Scheduling) -> Result<Scheduling, RepositoryError>;
    async fn update(&self, scheduling: &Scheduling) -> Result<Scheduling, RepositoryError>;
}

#[async_trait]
pub trait CheckInRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CheckIn>, RepositoryError>;
    async fn find_by_associate_today(&self, associate_id: Uuid) -> Result<Option<CheckIn>, RepositoryError>;
    async fn create(&self, check_in: &CheckIn) -> Result<CheckIn, RepositoryError>;
    async fn update(&self, check_in: &CheckIn) -> Result<CheckIn, RepositoryError>;
}

#[async_trait]
pub trait PipelineStageRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<PipelineStage>, RepositoryError>;
}
