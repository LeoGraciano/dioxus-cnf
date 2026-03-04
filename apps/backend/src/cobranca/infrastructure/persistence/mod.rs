use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

use shared::domain::ports::RepositoryError;

use crate::domain::entities::{DistributionLog, Installment, Negotiation, Title};
use crate::domain::ports::{
    DistributionLogRepository, InstallmentRepository, NegotiationRepository, TitleRepository,
};

// ── InMemoryTitleRepository ───────────────────────────────────────────────────

#[derive(Default)]
pub struct InMemoryTitleRepository {
    store: Mutex<HashMap<Uuid, Title>>,
}

#[async_trait]
impl TitleRepository for InMemoryTitleRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Title>, RepositoryError> {
        Ok(self.store.lock().unwrap().get(&id).cloned())
    }

    async fn find_by_associate(&self, associate_id: Uuid) -> Result<Vec<Title>, RepositoryError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .values()
            .filter(|t| t.associate_id == associate_id)
            .cloned()
            .collect())
    }

    async fn find_by_document(
        &self,
        document_number: &str,
    ) -> Result<Option<Title>, RepositoryError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .values()
            .find(|t| t.document_number == document_number)
            .cloned())
    }

    async fn create(&self, title: &Title) -> Result<Title, RepositoryError> {
        self.store.lock().unwrap().insert(title.id, title.clone());
        Ok(title.clone())
    }

    async fn update(&self, title: &Title) -> Result<Title, RepositoryError> {
        let mut store = self.store.lock().unwrap();
        if !store.contains_key(&title.id) {
            return Err(RepositoryError::NotFound(format!(
                "Title not found: {}",
                title.id
            )));
        }
        store.insert(title.id, title.clone());
        Ok(title.clone())
    }
}

// ── InMemoryInstallmentRepository ─────────────────────────────────────────────

#[derive(Default)]
pub struct InMemoryInstallmentRepository {
    store: Mutex<HashMap<Uuid, Installment>>,
}

#[async_trait]
impl InstallmentRepository for InMemoryInstallmentRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Installment>, RepositoryError> {
        Ok(self.store.lock().unwrap().get(&id).cloned())
    }

    async fn find_by_title(&self, title_id: Uuid) -> Result<Vec<Installment>, RepositoryError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .values()
            .filter(|i| i.title_id == title_id)
            .cloned()
            .collect())
    }

    async fn find_by_associate(
        &self,
        associate_id: Uuid,
    ) -> Result<Vec<Installment>, RepositoryError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .values()
            .filter(|i| i.associate_id == associate_id)
            .cloned()
            .collect())
    }

    /// Returns overdue installments; workgroup_id is used for filtering in real DB
    /// In-memory: returns all installments that have days_overdue set (> 0)
    async fn find_overdue(&self, _workgroup_id: i32) -> Result<Vec<Installment>, RepositoryError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .values()
            .filter(|i| i.days_overdue.is_some_and(|d| d > 0))
            .cloned()
            .collect())
    }

    async fn create(&self, installment: &Installment) -> Result<Installment, RepositoryError> {
        self.store
            .lock()
            .unwrap()
            .insert(installment.id, installment.clone());
        Ok(installment.clone())
    }

    async fn update(&self, installment: &Installment) -> Result<Installment, RepositoryError> {
        let mut store = self.store.lock().unwrap();
        if !store.contains_key(&installment.id) {
            return Err(RepositoryError::NotFound(format!(
                "Installment not found: {}",
                installment.id
            )));
        }
        store.insert(installment.id, installment.clone());
        Ok(installment.clone())
    }
}

// ── InMemoryNegotiationRepository ─────────────────────────────────────────────
// Append-only: no update method

#[derive(Default)]
pub struct InMemoryNegotiationRepository {
    store: Mutex<Vec<Negotiation>>,
}

#[async_trait]
impl NegotiationRepository for InMemoryNegotiationRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Negotiation>, RepositoryError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .find(|n| n.id == id)
            .cloned())
    }

    async fn find_by_installment(
        &self,
        installment_id: Uuid,
    ) -> Result<Vec<Negotiation>, RepositoryError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .filter(|n| n.installment_id == installment_id)
            .cloned()
            .collect())
    }

    async fn find_by_associate(
        &self,
        associate_id: Uuid,
    ) -> Result<Vec<Negotiation>, RepositoryError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .filter(|n| n.associate_id == associate_id)
            .cloned()
            .collect())
    }

    async fn create(&self, negotiation: &Negotiation) -> Result<Negotiation, RepositoryError> {
        self.store.lock().unwrap().push(negotiation.clone());
        Ok(negotiation.clone())
    }
}

// ── InMemoryDistributionLogRepository ────────────────────────────────────────
// Append-only: no update method

#[derive(Default)]
pub struct InMemoryDistributionLogRepository {
    store: Mutex<Vec<DistributionLog>>,
}

#[async_trait]
impl DistributionLogRepository for InMemoryDistributionLogRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DistributionLog>, RepositoryError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .find(|d| d.id == id)
            .cloned())
    }

    async fn find_by_workgroup(
        &self,
        workgroup_id: i32,
    ) -> Result<Vec<DistributionLog>, RepositoryError> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .filter(|d| d.workgroup_id == workgroup_id)
            .cloned()
            .collect())
    }

    async fn create(&self, log: &DistributionLog) -> Result<DistributionLog, RepositoryError> {
        self.store.lock().unwrap().push(log.clone());
        Ok(log.clone())
    }
}
