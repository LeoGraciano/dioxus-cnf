use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

use super::entities::{Associate, City, State, User, Workgroup};

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Entity not found: {0}")]
    NotFound(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

#[async_trait]
pub trait AssociateRepository: Send + Sync {
    async fn find_by_id(&self, id_client_esolution: Uuid) -> Result<Option<Associate>, RepositoryError>;
    async fn find_by_cpf(&self, cpf: &str) -> Result<Option<Associate>, RepositoryError>;
    async fn find_all(&self, page: u32, per_page: u32) -> Result<Vec<Associate>, RepositoryError>;
    async fn create(&self, associate: &Associate) -> Result<Associate, RepositoryError>;
    async fn update(&self, associate: &Associate) -> Result<Associate, RepositoryError>;
}

#[async_trait]
pub trait CityRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<Option<City>, RepositoryError>;
    async fn find_by_state(&self, state_id: i32) -> Result<Vec<City>, RepositoryError>;
}

#[async_trait]
pub trait StateRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<Option<State>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<State>, RepositoryError>;
}

#[async_trait]
pub trait WorkgroupRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<Option<Workgroup>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<Workgroup>, RepositoryError>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, RepositoryError>;
    async fn find_active_collectors(&self) -> Result<Vec<User>, RepositoryError>;
}
