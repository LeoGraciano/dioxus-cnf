use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::Associate;
use crate::domain::ports::{AssociateRepository, RepositoryError};

pub struct PostgresAssociateRepository {
    connection_string: String,
}

impl PostgresAssociateRepository {
    pub fn new(connection_string: &str) -> Self {
        Self {
            connection_string: connection_string.to_string(),
        }
    }
}

#[async_trait]
impl AssociateRepository for PostgresAssociateRepository {
    async fn find_by_id(&self, _id_client_esolution: Uuid) -> Result<Option<Associate>, RepositoryError> {
        todo!("Implement with sqlx")
    }

    async fn find_by_cpf(&self, _cpf: &str) -> Result<Option<Associate>, RepositoryError> {
        todo!("Implement with sqlx")
    }

    async fn find_all(&self, _page: u32, _per_page: u32) -> Result<Vec<Associate>, RepositoryError> {
        todo!("Implement with sqlx")
    }

    async fn create(&self, _associate: &Associate) -> Result<Associate, RepositoryError> {
        todo!("Implement with sqlx")
    }

    async fn update(&self, _associate: &Associate) -> Result<Associate, RepositoryError> {
        todo!("Implement with sqlx")
    }
}
