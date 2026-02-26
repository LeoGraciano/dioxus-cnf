use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

use shared::domain::ports::RepositoryError;

use crate::domain::entities::{
    Bracelet, CheckIn, Contract, Lead, LeadAuditEvent, PipelineStage, Sale, Scheduling, Workshop,
};
use crate::domain::ports::{
    BraceletRepository, CheckInRepository, ContractRepository, LeadAuditRepository, LeadRepository,
    PipelineStageRepository, SaleRepository, SchedulingRepository, WorkshopRepository,
};

// ── InMemoryLeadRepository ────────────────────────────────────────────────────

#[derive(Default)]
pub struct InMemoryLeadRepository {
    store: Mutex<HashMap<Uuid, Lead>>,
}

#[async_trait]
impl LeadRepository for InMemoryLeadRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Lead>, RepositoryError> {
        Ok(self.store.lock().unwrap().get(&id).cloned())
    }

    async fn find_by_cpf(&self, cpf: &str) -> Result<Vec<Lead>, RepositoryError> {
        Ok(self.store.lock().unwrap().values()
            .filter(|l| l.cpf.as_deref() == Some(cpf))
            .cloned()
            .collect())
    }

    async fn find_all(&self, page: u32, per_page: u32) -> Result<Vec<Lead>, RepositoryError> {
        let store = self.store.lock().unwrap();
        let skip = (page.saturating_sub(1) * per_page) as usize;
        Ok(store.values().skip(skip).take(per_page as usize).cloned().collect())
    }

    async fn create(&self, lead: &Lead) -> Result<Lead, RepositoryError> {
        self.store.lock().unwrap().insert(lead.id, lead.clone());
        Ok(lead.clone())
    }

    async fn update(&self, lead: &Lead) -> Result<Lead, RepositoryError> {
        let mut store = self.store.lock().unwrap();
        if !store.contains_key(&lead.id) {
            return Err(RepositoryError::NotFound(format!("Lead not found: {}", lead.id)));
        }
        store.insert(lead.id, lead.clone());
        Ok(lead.clone())
    }
}

// ── InMemoryLeadAuditRepository ───────────────────────────────────────────────

#[derive(Default)]
pub struct InMemoryLeadAuditRepository {
    store: Mutex<Vec<LeadAuditEvent>>,
}

#[async_trait]
impl LeadAuditRepository for InMemoryLeadAuditRepository {
    async fn record(&self, event: &LeadAuditEvent) -> Result<LeadAuditEvent, RepositoryError> {
        self.store.lock().unwrap().push(event.clone());
        Ok(event.clone())
    }

    async fn find_by_lead_id(&self, lead_id: Uuid) -> Result<Vec<LeadAuditEvent>, RepositoryError> {
        Ok(self.store.lock().unwrap().iter()
            .filter(|e| e.lead_id == lead_id)
            .cloned()
            .collect())
    }
}

// ── InMemorySaleRepository ────────────────────────────────────────────────────

#[derive(Default)]
pub struct InMemorySaleRepository {
    store: Mutex<HashMap<Uuid, Sale>>,
}

#[async_trait]
impl SaleRepository for InMemorySaleRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Sale>, RepositoryError> {
        Ok(self.store.lock().unwrap().get(&id).cloned())
    }

    async fn find_by_lead_id(&self, lead_id: Uuid) -> Result<Vec<Sale>, RepositoryError> {
        Ok(self.store.lock().unwrap().values()
            .filter(|s| s.lead_id == lead_id)
            .cloned()
            .collect())
    }

    async fn create(&self, sale: &Sale) -> Result<Sale, RepositoryError> {
        self.store.lock().unwrap().insert(sale.id, sale.clone());
        Ok(sale.clone())
    }

    async fn update(&self, sale: &Sale) -> Result<Sale, RepositoryError> {
        let mut store = self.store.lock().unwrap();
        if !store.contains_key(&sale.id) {
            return Err(RepositoryError::NotFound(format!("Sale not found: {}", sale.id)));
        }
        store.insert(sale.id, sale.clone());
        Ok(sale.clone())
    }
}

// ── InMemoryWorkshopRepository ────────────────────────────────────────────────

#[derive(Default)]
pub struct InMemoryWorkshopRepository {
    store: Mutex<HashMap<Uuid, Workshop>>,
}

#[async_trait]
impl WorkshopRepository for InMemoryWorkshopRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Workshop>, RepositoryError> {
        Ok(self.store.lock().unwrap().get(&id).cloned())
    }

    async fn find_by_associate(&self, associate_id: Uuid) -> Result<Vec<Workshop>, RepositoryError> {
        Ok(self.store.lock().unwrap().values()
            .filter(|w| w.associate_id == associate_id)
            .cloned()
            .collect())
    }

    async fn find_upcoming(&self, date: chrono::DateTime<chrono::Utc>) -> Result<Vec<Workshop>, RepositoryError> {
        Ok(self.store.lock().unwrap().values()
            .filter(|w| w.scheduled_date >= date)
            .cloned()
            .collect())
    }

    async fn create(&self, workshop: &Workshop) -> Result<Workshop, RepositoryError> {
        self.store.lock().unwrap().insert(workshop.id, workshop.clone());
        Ok(workshop.clone())
    }

    async fn update(&self, workshop: &Workshop) -> Result<Workshop, RepositoryError> {
        let mut store = self.store.lock().unwrap();
        if !store.contains_key(&workshop.id) {
            return Err(RepositoryError::NotFound(format!("Workshop not found: {}", workshop.id)));
        }
        store.insert(workshop.id, workshop.clone());
        Ok(workshop.clone())
    }
}

// ── InMemoryContractRepository ────────────────────────────────────────────────

#[derive(Default)]
pub struct InMemoryContractRepository {
    store: Mutex<HashMap<Uuid, Contract>>,
}

#[async_trait]
impl ContractRepository for InMemoryContractRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Contract>, RepositoryError> {
        Ok(self.store.lock().unwrap().get(&id).cloned())
    }

    async fn find_by_associate(&self, associate_id: Uuid) -> Result<Vec<Contract>, RepositoryError> {
        Ok(self.store.lock().unwrap().values()
            .filter(|c| c.associate_id == associate_id)
            .cloned()
            .collect())
    }

    async fn find_by_number(&self, contract_number: &str) -> Result<Option<Contract>, RepositoryError> {
        Ok(self.store.lock().unwrap().values()
            .find(|c| c.contract_number == contract_number)
            .cloned())
    }

    async fn create(&self, contract: &Contract) -> Result<Contract, RepositoryError> {
        self.store.lock().unwrap().insert(contract.id, contract.clone());
        Ok(contract.clone())
    }

    async fn update(&self, contract: &Contract) -> Result<Contract, RepositoryError> {
        let mut store = self.store.lock().unwrap();
        if !store.contains_key(&contract.id) {
            return Err(RepositoryError::NotFound(format!("Contract not found: {}", contract.id)));
        }
        store.insert(contract.id, contract.clone());
        Ok(contract.clone())
    }
}

// ── InMemoryBraceletRepository ────────────────────────────────────────────────

#[derive(Default)]
pub struct InMemoryBraceletRepository {
    store: Mutex<HashMap<Uuid, Bracelet>>,
}

#[async_trait]
impl BraceletRepository for InMemoryBraceletRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Bracelet>, RepositoryError> {
        Ok(self.store.lock().unwrap().get(&id).cloned())
    }

    async fn find_by_associate(&self, associate_id: Uuid) -> Result<Vec<Bracelet>, RepositoryError> {
        Ok(self.store.lock().unwrap().values()
            .filter(|b| b.associate_id == associate_id)
            .cloned()
            .collect())
    }

    async fn find_by_number(&self, bracelet_number: &str) -> Result<Option<Bracelet>, RepositoryError> {
        Ok(self.store.lock().unwrap().values()
            .find(|b| b.bracelet_number == bracelet_number)
            .cloned())
    }

    async fn create(&self, bracelet: &Bracelet) -> Result<Bracelet, RepositoryError> {
        self.store.lock().unwrap().insert(bracelet.id, bracelet.clone());
        Ok(bracelet.clone())
    }
}

// ── InMemorySchedulingRepository ─────────────────────────────────────────────

#[derive(Default)]
pub struct InMemorySchedulingRepository {
    store: Mutex<HashMap<Uuid, Scheduling>>,
}

#[async_trait]
impl SchedulingRepository for InMemorySchedulingRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Scheduling>, RepositoryError> {
        Ok(self.store.lock().unwrap().get(&id).cloned())
    }

    async fn find_by_associate(&self, associate_id: Uuid) -> Result<Vec<Scheduling>, RepositoryError> {
        Ok(self.store.lock().unwrap().values()
            .filter(|s| s.associate_id == associate_id)
            .cloned()
            .collect())
    }

    async fn create(&self, scheduling: &Scheduling) -> Result<Scheduling, RepositoryError> {
        self.store.lock().unwrap().insert(scheduling.id, scheduling.clone());
        Ok(scheduling.clone())
    }

    async fn update(&self, scheduling: &Scheduling) -> Result<Scheduling, RepositoryError> {
        let mut store = self.store.lock().unwrap();
        if !store.contains_key(&scheduling.id) {
            return Err(RepositoryError::NotFound(format!("Scheduling not found: {}", scheduling.id)));
        }
        store.insert(scheduling.id, scheduling.clone());
        Ok(scheduling.clone())
    }
}

// ── InMemoryCheckInRepository ─────────────────────────────────────────────────

#[derive(Default)]
pub struct InMemoryCheckInRepository {
    store: Mutex<HashMap<Uuid, CheckIn>>,
}

#[async_trait]
impl CheckInRepository for InMemoryCheckInRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CheckIn>, RepositoryError> {
        Ok(self.store.lock().unwrap().get(&id).cloned())
    }

    async fn find_by_associate_today(&self, associate_id: Uuid) -> Result<Option<CheckIn>, RepositoryError> {
        let today = chrono::Utc::now().date_naive();
        Ok(self.store.lock().unwrap().values()
            .find(|c| c.associate_id == associate_id && c.check_in_time.date_naive() == today)
            .cloned())
    }

    async fn create(&self, check_in: &CheckIn) -> Result<CheckIn, RepositoryError> {
        self.store.lock().unwrap().insert(check_in.id, check_in.clone());
        Ok(check_in.clone())
    }

    async fn update(&self, check_in: &CheckIn) -> Result<CheckIn, RepositoryError> {
        let mut store = self.store.lock().unwrap();
        if !store.contains_key(&check_in.id) {
            return Err(RepositoryError::NotFound(format!("CheckIn not found: {}", check_in.id)));
        }
        store.insert(check_in.id, check_in.clone());
        Ok(check_in.clone())
    }
}

// ── InMemoryPipelineStageRepository ──────────────────────────────────────────

pub struct InMemoryPipelineStageRepository {
    stages: Vec<PipelineStage>,
}

impl InMemoryPipelineStageRepository {
    pub fn with_default_stages() -> Self {
        Self {
            stages: vec![
                PipelineStage { id: 1, name: "Novo Lead".to_string(), order: 1, is_won: false, is_lost: false },
                PipelineStage { id: 2, name: "Contato Realizado".to_string(), order: 2, is_won: false, is_lost: false },
                PipelineStage { id: 3, name: "Qualificado".to_string(), order: 3, is_won: false, is_lost: false },
                PipelineStage { id: 4, name: "Proposta Enviada".to_string(), order: 4, is_won: false, is_lost: false },
                PipelineStage { id: 5, name: "Negociacao".to_string(), order: 5, is_won: false, is_lost: false },
                PipelineStage { id: 6, name: "Fechado - Ganho".to_string(), order: 6, is_won: true, is_lost: false },
                PipelineStage { id: 7, name: "Fechado - Perdido".to_string(), order: 7, is_won: false, is_lost: true },
            ],
        }
    }
}

#[async_trait]
impl PipelineStageRepository for InMemoryPipelineStageRepository {
    async fn find_all(&self) -> Result<Vec<PipelineStage>, RepositoryError> {
        Ok(self.stages.clone())
    }
}
