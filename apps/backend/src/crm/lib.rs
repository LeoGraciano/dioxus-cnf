pub mod application;
pub mod domain;
pub mod infrastructure;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lead_status_default_is_new() {
        assert_eq!(domain::entities::LeadStatus::default(), domain::entities::LeadStatus::New);
    }

    #[test]
    fn workshop_status_eq() {
        assert_eq!(domain::entities::WorkshopStatus::Scheduled, domain::entities::WorkshopStatus::Scheduled);
        assert_ne!(domain::entities::WorkshopStatus::Scheduled, domain::entities::WorkshopStatus::Completed);
    }

    // ── Regression tests: CRM domain fluxos completos ─────────────────────────

    #[tokio::test]
    async fn test_lead_criacao_e_auditoria() {
        use application::ports::{CreateLeadInput, CreateLeadUseCase};
        use application::services::LeadService;
        use domain::entities::LeadSource;
        use infrastructure::persistence::{InMemoryLeadAuditRepository, InMemoryLeadRepository};

        let service = LeadService::new(InMemoryLeadRepository::default(), InMemoryLeadAuditRepository::default());

        let input = CreateLeadInput {
            source: LeadSource::WhatsApp,
            name: "Maria Silva".to_string(),
            cpf: None,
            phone: Some("11999999999".to_string()),
            email: Some("maria@example.com".to_string()),
            observation: None,
            assigned_to_id: None,
        };

        let output = CreateLeadUseCase::execute(&service, input).await.unwrap();
        assert_eq!(output.lead.name, "Maria Silva");
        assert_eq!(output.lead.status, domain::entities::LeadStatus::New);
    }

    #[tokio::test]
    async fn test_lead_status_change_gera_auditoria() {
        use application::ports::{
            CreateLeadInput, CreateLeadUseCase, GetLeadHistoryInput, GetLeadHistoryUseCase,
            UpdateLeadStatusInput, UpdateLeadStatusUseCase,
        };
        use application::services::LeadService;
        use domain::entities::{LeadAuditEventType, LeadSource, LeadStatus};
        use infrastructure::persistence::{InMemoryLeadAuditRepository, InMemoryLeadRepository};

        let service = LeadService::new(InMemoryLeadRepository::default(), InMemoryLeadAuditRepository::default());

        // Criar lead
        let created = CreateLeadUseCase::execute(&service, CreateLeadInput {
            source: LeadSource::Phone,
            name: "João Costa".to_string(),
            cpf: None,
            phone: None,
            email: None,
            observation: None,
            assigned_to_id: Some(1),
        }).await.unwrap();

        // Atualizar status para Won
        let update_output = UpdateLeadStatusUseCase::execute(&service, UpdateLeadStatusInput {
            lead_id: created.lead.id,
            new_status: LeadStatus::Won,
            performed_by_id: Some(1),
        }).await.unwrap();

        assert_eq!(update_output.lead.status, LeadStatus::Won);
        assert!(update_output.lead.converted_at.is_some());
        assert_eq!(update_output.audit_event.event_type, LeadAuditEventType::Converted);

        // Verificar histórico auditável
        let history = GetLeadHistoryUseCase::execute(&service, GetLeadHistoryInput { lead_id: created.lead.id }).await.unwrap();
        // deve ter: Created + StatusChanged(Won)
        assert_eq!(history.events.len(), 2);
        assert_eq!(history.events[0].event_type, LeadAuditEventType::Created);
        assert_eq!(history.events[1].event_type, LeadAuditEventType::Converted);
    }

    #[tokio::test]
    async fn test_scheduling_agendamento_e_checkin() {
        use application::ports::{
            RecordCheckInInput, RecordCheckInUseCase, RecordCheckOutUseCase,
            RecordCheckOutInput, ScheduleAppointmentInput, ScheduleAppointmentUseCase,
        };
        use application::services::SchedulingService;
        use domain::entities::SchedulingType;
        use infrastructure::persistence::{InMemoryCheckInRepository, InMemorySchedulingRepository};

        let service = SchedulingService::new(InMemorySchedulingRepository::default(), InMemoryCheckInRepository::default());

        // Agendar
        let assoc_id = uuid::Uuid::new_v4();
        let sched = ScheduleAppointmentUseCase::execute(&service, ScheduleAppointmentInput {
            associate_id: assoc_id,
            scheduled_date: chrono::Utc::now(),
            scheduling_type: SchedulingType::Appointment,
            notes: Some("Primeira visita".to_string()),
        }).await.unwrap();

        assert_eq!(sched.scheduling.associate_id, assoc_id);
        assert_eq!(sched.scheduling.status, domain::entities::SchedulingStatus::Scheduled);

        // Check-in
        let ci = RecordCheckInUseCase::execute(&service, RecordCheckInInput {
            associate_id: assoc_id,
            location: Some("Recepção".to_string()),
        }).await.unwrap();

        assert!(ci.check_in.check_out_time.is_none());

        // Check-out
        let co = RecordCheckOutUseCase::execute(&service, RecordCheckOutInput {
            check_in_id: ci.check_in.id,
        }).await.unwrap();

        assert!(co.check_in.check_out_time.is_some());
    }

    #[tokio::test]
    async fn test_bracelet_numero_duplicado_rejeitado() {
        use application::ports::{IssueBraceletInput, IssueBraceletUseCase};
        use application::services::BraceletService;
        use infrastructure::persistence::{InMemoryBraceletRepository, InMemoryLeadAuditRepository};

        let service = BraceletService::new(InMemoryBraceletRepository::default(), InMemoryLeadAuditRepository::default());

        let input = IssueBraceletInput {
            associate_id: uuid::Uuid::new_v4(),
            bracelet_number: "PUL-001".to_string(),
        };

        // Primeira emissão deve funcionar
        let first = IssueBraceletUseCase::execute(&service, input.clone()).await;
        assert!(first.is_ok());

        // Segunda com mesmo número deve falhar
        let second = IssueBraceletUseCase::execute(&service, input).await;
        assert!(second.is_err());
    }

    #[tokio::test]
    async fn test_pipeline_stages_disponiveis() {
        use application::ports::GetPipelineStagesUseCase;
        use application::services::PipelineService;
        use infrastructure::persistence::InMemoryPipelineStageRepository;

        let service = PipelineService::new(InMemoryPipelineStageRepository::with_default_stages());

        let output = GetPipelineStagesUseCase::execute(&service).await.unwrap();
        assert_eq!(output.stages.len(), 7);
        assert!(output.stages.iter().any(|s| s.is_won));
        assert!(output.stages.iter().any(|s| s.is_lost));
    }

    // ── Regression tests: CRM fluxo ponta a ponta ─────────────────────────────

    /// Testa o fluxo completo: Lead → Agendamento → Workshop → Venda → Contrato → Pulseira
    /// Garante que histórico auditável está disponível ponta a ponta.
    #[tokio::test]
    async fn test_fluxo_completo_crm_ponta_a_ponta() {
        use application::ports::{
            CreateContractInput, CreateContractUseCase,
            CreateLeadInput, CreateLeadUseCase,
            CreateSaleAttemptInput, CreateSaleAttemptUseCase,
            CreateWorkshopInput, CreateWorkshopUseCase,
            GetLeadHistoryInput, GetLeadHistoryUseCase,
            IssueBraceletInput, IssueBraceletUseCase,
            RecordCheckInInput, RecordCheckInUseCase,
            ScheduleAppointmentInput, ScheduleAppointmentUseCase,
            UpdateLeadStatusInput, UpdateLeadStatusUseCase,
        };
        use application::services::{
            BraceletService, ContractService, LeadService, SaleService, SchedulingService, WorkshopService,
        };
        use domain::entities::{LeadAuditEventType, LeadSource, LeadStatus, SchedulingType, WorkshopType};
        use domain::ports::LeadRepository;
        use infrastructure::persistence::{
            InMemoryBraceletRepository, InMemoryCheckInRepository, InMemoryContractRepository,
            InMemoryLeadAuditRepository, InMemoryLeadRepository,
            InMemorySaleRepository, InMemorySchedulingRepository, InMemoryWorkshopRepository,
        };

        // Instanciar serviço de lead
        let lead_service = LeadService::new(
            InMemoryLeadRepository::default(),
            InMemoryLeadAuditRepository::default(),
        );

        // 1. Criar lead
        let lead_out = CreateLeadUseCase::execute(&lead_service, CreateLeadInput {
            source: LeadSource::Website,
            name: "Ana Luiza".to_string(),
            cpf: Some("987.654.321-00".to_string()),
            phone: Some("11977776666".to_string()),
            email: Some("ana@example.com".to_string()),
            observation: None,
            assigned_to_id: Some(3),
        }).await.unwrap();

        assert_eq!(lead_out.lead.status, LeadStatus::New);
        let lead_id = lead_out.lead.id;
        let associate_id = uuid::Uuid::new_v4();

        // 2. Agendar visita (pipeline: Contacted)
        UpdateLeadStatusUseCase::execute(&lead_service, UpdateLeadStatusInput {
            lead_id,
            new_status: LeadStatus::Contacted,
            performed_by_id: Some(3),
        }).await.unwrap();

        let scheduling_service = SchedulingService::new(
            InMemorySchedulingRepository::default(),
            InMemoryCheckInRepository::default(),
        );
        let sched_out = ScheduleAppointmentUseCase::execute(&scheduling_service, ScheduleAppointmentInput {
            associate_id,
            scheduled_date: chrono::Utc::now() + chrono::Duration::days(1),
            scheduling_type: SchedulingType::Appointment,
            notes: Some("Visita de apresentação".to_string()),
        }).await.unwrap();

        assert_eq!(sched_out.scheduling.associate_id, associate_id);

        // 3. Check-in na visita
        let ci_out = RecordCheckInUseCase::execute(&scheduling_service, RecordCheckInInput {
            associate_id,
            location: Some("Sede CNF".to_string()),
        }).await.unwrap();

        assert!(ci_out.check_in.check_out_time.is_none());

        // 4. Agendar workshop (pipeline: Qualified)
        UpdateLeadStatusUseCase::execute(&lead_service, UpdateLeadStatusInput {
            lead_id,
            new_status: LeadStatus::Qualified,
            performed_by_id: Some(3),
        }).await.unwrap();

        let workshop_service = WorkshopService::new(
            InMemoryWorkshopRepository::default(),
            InMemoryLeadAuditRepository::default(),
        );
        let workshop_out = CreateWorkshopUseCase::execute(&workshop_service, CreateWorkshopInput {
            associate_id,
            scheduled_date: chrono::Utc::now() + chrono::Duration::days(7),
            workshop_type: WorkshopType::TrialClass,
            notes: Some("Aula experimental de natação".to_string()),
        }).await.unwrap();

        assert_eq!(workshop_out.workshop.associate_id, associate_id);

        // 5. Tentativa de venda (pipeline: Proposal)
        UpdateLeadStatusUseCase::execute(&lead_service, UpdateLeadStatusInput {
            lead_id,
            new_status: LeadStatus::Proposal,
            performed_by_id: Some(3),
        }).await.unwrap();

        // Pre-populate lead_repo for SaleService (SaleService validates lead exists)
        let sale_lead_repo = InMemoryLeadRepository::default();
        LeadRepository::create(&sale_lead_repo, &lead_out.lead).await.unwrap();

        let sale_service = SaleService::new(
            InMemorySaleRepository::default(),
            sale_lead_repo,
            InMemoryLeadAuditRepository::default(),
        );
        let sale_out = CreateSaleAttemptUseCase::execute(&sale_service, CreateSaleAttemptInput {
            lead_id,
            stage_id: 4,
            value: 2400.00,
            description: Some("Plano anual premium".to_string()),
            closed_by_id: 3,
        }).await.unwrap();

        assert_eq!(sale_out.sale.lead_id, lead_id);

        // 6. Fechar venda (Won)
        let won_out = UpdateLeadStatusUseCase::execute(&lead_service, UpdateLeadStatusInput {
            lead_id,
            new_status: LeadStatus::Won,
            performed_by_id: Some(3),
        }).await.unwrap();

        assert_eq!(won_out.lead.status, LeadStatus::Won);
        assert!(won_out.lead.converted_at.is_some());
        assert_eq!(won_out.audit_event.event_type, LeadAuditEventType::Converted);

        // 7. Criar contrato
        let contract_service = ContractService::new(
            InMemoryContractRepository::default(),
            InMemoryLeadAuditRepository::default(),
        );
        let contract_out = CreateContractUseCase::execute(&contract_service, CreateContractInput {
            associate_id,
            contract_number: "CNT-2025-002".to_string(),
            value: 2400.00,
            start_date: chrono::Utc::now(),
        }).await.unwrap();

        assert_eq!(contract_out.contract.associate_id, associate_id);

        // 8. Emitir pulseira
        let bracelet_service = BraceletService::new(
            InMemoryBraceletRepository::default(),
            InMemoryLeadAuditRepository::default(),
        );
        let bracelet_out = IssueBraceletUseCase::execute(&bracelet_service, IssueBraceletInput {
            associate_id,
            bracelet_number: "PUL-2025-0042".to_string(),
        }).await.unwrap();

        assert_eq!(bracelet_out.bracelet.associate_id, associate_id);
        assert_eq!(bracelet_out.bracelet.bracelet_number, "PUL-2025-0042");

        // 9. Verificar histórico auditável ponta a ponta
        // Lead passou por: Created → Contacted → Qualified → Proposal → Won
        let history = GetLeadHistoryUseCase::execute(&lead_service, GetLeadHistoryInput { lead_id }).await.unwrap();
        assert_eq!(history.events.len(), 5);
        assert_eq!(history.events[0].event_type, LeadAuditEventType::Created);
        assert_eq!(history.events[4].event_type, LeadAuditEventType::Converted);

        // Garantir que todos os artefatos foram criados com sucesso
        assert!(!workshop_out.workshop.id.is_nil());
        assert!(!ci_out.check_in.id.is_nil());
        assert!(!contract_out.contract.id.is_nil());
        assert!(!bracelet_out.bracelet.id.is_nil());
    }
}
