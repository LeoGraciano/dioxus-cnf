use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::Associate;
use crate::domain::ports::{AssociateRepository, RepositoryError};

#[async_trait]
pub trait AssociateService: Send + Sync {
    async fn get_associate(&self, id: Uuid) -> Result<Option<Associate>, ServiceError>;
    async fn get_associate_by_cpf(&self, cpf: &str) -> Result<Option<Associate>, ServiceError>;
    async fn list_associates(&self, page: u32, per_page: u32) -> Result<Vec<Associate>, ServiceError>;
    async fn create_associate(&self, associate: Associate) -> Result<Associate, ServiceError>;
    async fn update_associate(&self, associate: Associate) -> Result<Associate, ServiceError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Repository error: {0}")]
    RepositoryError(#[from] RepositoryError),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Business rule violation: {0}")]
    BusinessRuleViolation(String),
}
