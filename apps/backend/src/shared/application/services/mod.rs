use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::Associate;
use crate::domain::ports::AssociateRepository;
use super::ports::{AssociateService, ServiceError};

pub struct DefaultAssociateService<R: AssociateRepository> {
    repository: R,
}

impl<R: AssociateRepository> DefaultAssociateService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: AssociateRepository + Send + Sync> AssociateService for DefaultAssociateService<R> {
    async fn get_associate(&self, id: Uuid) -> Result<Option<Associate>, ServiceError> {
        self.repository.find_by_id(id).await.map_err(ServiceError::from)
    }

    async fn get_associate_by_cpf(&self, cpf: &str) -> Result<Option<Associate>, ServiceError> {
        self.repository.find_by_cpf(cpf).await.map_err(ServiceError::from)
    }

    async fn list_associates(&self, page: u32, per_page: u32) -> Result<Vec<Associate>, ServiceError> {
        self.repository.find_all(page, per_page).await.map_err(ServiceError::from)
    }

    async fn create_associate(&self, mut associate: Associate) -> Result<Associate, ServiceError> {
        if associate.registration.is_empty() {
            return Err(ServiceError::ValidationError("Registration cannot be empty".to_string()));
        }
        
        let has_phone = associate.phone.as_ref().is_some_and(|p| !p.is_empty());
        let has_email = associate.email.as_ref().is_some_and(|e| !e.is_empty());
        if !has_phone && !has_email {
            return Err(ServiceError::ValidationError("Phone or Email is required".to_string()));
        }
        
        associate.created_at = chrono::Utc::now();
        associate.updated_at = chrono::Utc::now();
        
        self.repository.create(&associate).await.map_err(ServiceError::from)
    }

    async fn update_associate(&self, associate: Associate) -> Result<Associate, ServiceError> {
        self.repository.update(&associate).await.map_err(ServiceError::from)
    }
}
