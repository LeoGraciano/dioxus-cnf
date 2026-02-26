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

        let lead_repo = InMemoryLeadRepository::default();
        let audit_repo = InMemoryLeadAuditRepository::default();
        let service = LeadService::new(lead_repo, audit_repo);

        let input = CreateLeadInput {
            source: LeadSource::WhatsApp,
            name: "Maria Silva".to_string(),
            cpf: None,
            phone: Some("11999999999".to_string()),
            email: Some("maria@example.com".to_string()),
            observation: None,
            assigned_to_id: None,
        };

        let output = service.execute(input).await.unwrap();
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
        use std::sync::Arc;

        let lead_repo = Arc::new(InMemoryLeadRepository::default());
        let audit_repo = Arc::new(InMemoryLeadAuditRepository::default());
        let service = LeadService::new(Arc::clone(&lead_repo), Arc::clone(&audit_repo));

        // Criar lead
        let created = service.execute(CreateLeadInput {
            source: LeadSource::Phone,
            name: "João Costa".to_string(),
            cpf: None,
            phone: None,
            email: None,
            observation: None,
            assigned_to_id: Some(1),
        }).await.unwrap();

        // Atualizar status para Won
        let update_output = service.execute(UpdateLeadStatusInput {
            lead_id: created.lead.id,
            new_status: LeadStatus::Won,
            performed_by_id: Some(1),
        }).await.unwrap();

        assert_eq!(update_output.lead.status, LeadStatus::Won);
        assert!(update_output.lead.converted_at.is_some());
        assert_eq!(update_output.audit_event.event_type, LeadAuditEventType::Converted);

        // Verificar histórico auditável
        let history = service.execute(GetLeadHistoryInput { lead_id: created.lead.id }).await.unwrap();
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

        let sched_repo = InMemorySchedulingRepository::default();
        let checkin_repo = InMemoryCheckInRepository::default();
        let service = SchedulingService::new(sched_repo, checkin_repo);

        // Agendar
        let assoc_id = uuid::Uuid::new_v4();
        let sched = service.execute(ScheduleAppointmentInput {
            associate_id: assoc_id,
            scheduled_date: chrono::Utc::now(),
            scheduling_type: SchedulingType::Appointment,
            notes: Some("Primeira visita".to_string()),
        }).await.unwrap();

        assert_eq!(sched.scheduling.associate_id, assoc_id);
        assert_eq!(sched.scheduling.status, domain::entities::SchedulingStatus::Scheduled);

        // Check-in
        let ci = service.execute(RecordCheckInInput {
            associate_id: assoc_id,
            location: Some("Recepção".to_string()),
        }).await.unwrap();

        assert!(ci.check_in.check_out_time.is_none());

        // Check-out
        let co = service.execute(RecordCheckOutInput {
            check_in_id: ci.check_in.id,
        }).await.unwrap();

        assert!(co.check_in.check_out_time.is_some());
    }

    #[tokio::test]
    async fn test_bracelet_numero_duplicado_rejeitado() {
        use application::ports::{IssueBraceletInput, IssueBraceletUseCase};
        use application::services::BraceletService;
        use infrastructure::persistence::{InMemoryBraceletRepository, InMemoryLeadAuditRepository};

        let bracelet_repo = InMemoryBraceletRepository::default();
        let audit_repo = InMemoryLeadAuditRepository::default();
        let service = BraceletService::new(bracelet_repo, audit_repo);

        let input = IssueBraceletInput {
            associate_id: uuid::Uuid::new_v4(),
            bracelet_number: "PUL-001".to_string(),
        };

        // Primeira emissão deve funcionar
        let first = service.execute(input.clone()).await;
        assert!(first.is_ok());

        // Segunda com mesmo número deve falhar
        let second = service.execute(input).await;
        assert!(second.is_err());
    }

    #[tokio::test]
    async fn test_pipeline_stages_disponiveis() {
        use application::ports::GetPipelineStagesUseCase;
        use application::services::PipelineService;
        use infrastructure::persistence::InMemoryPipelineStageRepository;

        let repo = InMemoryPipelineStageRepository::with_default_stages();
        let service = PipelineService::new(repo);

        let output = service.execute().await.unwrap();
        assert_eq!(output.stages.len(), 7);
        assert!(output.stages.iter().any(|s| s.is_won));
        assert!(output.stages.iter().any(|s| s.is_lost));
    }
}
