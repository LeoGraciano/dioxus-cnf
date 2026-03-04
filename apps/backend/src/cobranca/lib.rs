pub mod application;
pub mod domain;
pub mod infrastructure;

pub use domain::entities::{
    DistributionLog, DistributionType, Installment, InstallmentStatus, Negotiation,
    NegotiationResult, Title, TitleStatus,
};
pub use domain::ports::{
    DistributionLogRepository, InstallmentRepository, NegotiationRepository, TitleRepository,
};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use uuid::Uuid;

    use application::ports::{
        DistributeInstallmentsInput, DistributeInstallmentsUseCase, GetNegotiationHistoryInput,
        GetNegotiationHistoryUseCase, RecordNegotiationInput, RecordNegotiationUseCase,
    };
    use application::services::{DistributionService, NegotiationService};
    use infrastructure::persistence::{
        InMemoryDistributionLogRepository, InMemoryInstallmentRepository,
        InMemoryNegotiationRepository,
    };

    // ── Basic entity tests ────────────────────────────────────────────────────

    #[test]
    fn title_status_default_is_valid() {
        assert_eq!(TitleStatus::default(), TitleStatus::Valid);
    }

    #[test]
    fn installment_status_default_is_open() {
        assert_eq!(InstallmentStatus::default(), InstallmentStatus::Open);
    }

    // ── Regression tests: Cobranca domain ────────────────────────────────────

    /// Helper: create an installment with days_overdue set
    fn make_installment(associate_id: Uuid, days_overdue: i32) -> Installment {
        let mut inst = Installment::new(
            Uuid::new_v4(),
            associate_id,
            1,
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            500.0,
        );
        inst.days_overdue = Some(days_overdue);
        inst
    }

    #[tokio::test]
    async fn test_distribution_regras_workgroup_janela_minima_maxima() {
        // Tests that installments outside the min/max window are excluded
        let inst_repo = InMemoryInstallmentRepository::default();
        let log_repo = InMemoryDistributionLogRepository::default();

        let associate_a = Uuid::new_v4();
        let associate_b = Uuid::new_v4();

        // 5 days overdue - inside window [3, 10]
        let inst_inside = make_installment(associate_a, 5);
        // 2 days overdue - below min window
        let inst_below_min = make_installment(associate_b, 2);
        // 15 days overdue - above max window
        let inst_above_max = make_installment(associate_b, 15);

        InstallmentRepository::create(&inst_repo, &inst_inside).await.unwrap();
        InstallmentRepository::create(&inst_repo, &inst_below_min).await.unwrap();
        InstallmentRepository::create(&inst_repo, &inst_above_max).await.unwrap();

        let service = DistributionService::new(inst_repo, log_repo);
        let output = DistributeInstallmentsUseCase::execute(
            &service,
            DistributeInstallmentsInput {
                workgroup_id: 1,
                collector_user_ids: vec![101],
                min_days_overdue: 3,
                max_days_overdue: 10,
                distribution_type: DistributionType::Installment,
            },
        )
        .await
        .unwrap();

        // Only the installment within the [3, 10] window should be distributed
        assert_eq!(output.distributed_count, 1);
        assert_eq!(output.logs[0].assigned_to_user_id, 101);
    }

    #[tokio::test]
    async fn test_distribution_por_parcela_round_robin() {
        // Tests round-robin distribution across collectors per installment
        let inst_repo = InMemoryInstallmentRepository::default();
        let log_repo = InMemoryDistributionLogRepository::default();

        let associate = Uuid::new_v4();
        for _ in 0..4 {
            let inst = make_installment(associate, 7);
            InstallmentRepository::create(&inst_repo, &inst).await.unwrap();
        }

        let service = DistributionService::new(inst_repo, log_repo);
        let output = DistributeInstallmentsUseCase::execute(
            &service,
            DistributeInstallmentsInput {
                workgroup_id: 2,
                collector_user_ids: vec![201, 202],
                min_days_overdue: 1,
                max_days_overdue: 30,
                distribution_type: DistributionType::Installment,
            },
        )
        .await
        .unwrap();

        assert_eq!(output.distributed_count, 4);
        // Round-robin: collectors 201, 202, 201, 202
        let collectors: Vec<i32> = output.logs.iter().map(|l| l.assigned_to_user_id).collect();
        assert_eq!(collectors.iter().filter(|&&c| c == 201).count(), 2);
        assert_eq!(collectors.iter().filter(|&&c| c == 202).count(), 2);
    }

    #[tokio::test]
    async fn test_distribution_por_associado_mesmo_cobrador() {
        // Tests that all installments of the same associate go to the same collector
        let inst_repo = InMemoryInstallmentRepository::default();
        let log_repo = InMemoryDistributionLogRepository::default();

        let associate_a = Uuid::new_v4();
        let associate_b = Uuid::new_v4();

        // Associate A: 2 installments
        for _ in 0..2 {
            let inst = make_installment(associate_a, 8);
            InstallmentRepository::create(&inst_repo, &inst).await.unwrap();
        }
        // Associate B: 3 installments
        for _ in 0..3 {
            let inst = make_installment(associate_b, 8);
            InstallmentRepository::create(&inst_repo, &inst).await.unwrap();
        }

        let service = DistributionService::new(inst_repo, log_repo);
        let output = DistributeInstallmentsUseCase::execute(
            &service,
            DistributeInstallmentsInput {
                workgroup_id: 3,
                collector_user_ids: vec![301, 302],
                min_days_overdue: 1,
                max_days_overdue: 30,
                distribution_type: DistributionType::Associate,
            },
        )
        .await
        .unwrap();

        // 5 total installments distributed
        assert_eq!(output.distributed_count, 5);

        // All installments from associate_a must go to the same collector
        let collector_a: Vec<i32> = output
            .logs
            .iter()
            .filter(|l| l.associate_id == associate_a)
            .map(|l| l.assigned_to_user_id)
            .collect();
        assert!(collector_a.windows(2).all(|w| w[0] == w[1]));

        // All installments from associate_b must go to the same collector
        let collector_b: Vec<i32> = output
            .logs
            .iter()
            .filter(|l| l.associate_id == associate_b)
            .map(|l| l.assigned_to_user_id)
            .collect();
        assert!(collector_b.windows(2).all(|w| w[0] == w[1]));
    }

    #[tokio::test]
    async fn test_distribution_sem_cobradores_retorna_erro() {
        let inst_repo = InMemoryInstallmentRepository::default();
        let log_repo = InMemoryDistributionLogRepository::default();

        let service = DistributionService::new(inst_repo, log_repo);
        let result = DistributeInstallmentsUseCase::execute(
            &service,
            DistributeInstallmentsInput {
                workgroup_id: 1,
                collector_user_ids: vec![],
                min_days_overdue: 1,
                max_days_overdue: 30,
                distribution_type: DistributionType::Installment,
            },
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_negociacao_registro_e_historico_imutavel() {
        // Tests that negotiation records are created and retrievable as immutable history
        let inst_repo = InMemoryInstallmentRepository::default();
        let neg_repo = InMemoryNegotiationRepository::default();

        let associate_id = Uuid::new_v4();
        let mut installment = make_installment(associate_id, 10);
        installment.days_overdue = Some(10);
        let saved_inst = InstallmentRepository::create(&inst_repo, &installment).await.unwrap();
        let installment_id = saved_inst.id;

        let service = NegotiationService::new(inst_repo, neg_repo);

        // Record first negotiation: NoContact
        let out1 = RecordNegotiationUseCase::execute(
            &service,
            RecordNegotiationInput {
                installment_id,
                associate_id,
                user_id: 10,
                result: NegotiationResult::NoContact,
                promise_date: None,
                promise_value: None,
                observation: Some("Nao atendeu".to_string()),
            },
        )
        .await
        .unwrap();
        assert_eq!(out1.negotiation.result, NegotiationResult::NoContact);

        // Record second negotiation: PromiseToPay
        let promise_date = NaiveDate::from_ymd_opt(2026, 4, 1).unwrap();
        let out2 = RecordNegotiationUseCase::execute(
            &service,
            RecordNegotiationInput {
                installment_id,
                associate_id,
                user_id: 10,
                result: NegotiationResult::PromiseToPay,
                promise_date: Some(promise_date),
                promise_value: Some(500.0),
                observation: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(out2.negotiation.result, NegotiationResult::PromiseToPay);
        assert_eq!(out2.negotiation.promise_date, Some(promise_date));

        // Get history: must contain both negotiations in order (immutable)
        let history = GetNegotiationHistoryUseCase::execute(
            &service,
            GetNegotiationHistoryInput { installment_id },
        )
        .await
        .unwrap();
        assert_eq!(history.negotiations.len(), 2);
        assert_eq!(history.negotiations[0].result, NegotiationResult::NoContact);
        assert_eq!(history.negotiations[1].result, NegotiationResult::PromiseToPay);
    }

    #[tokio::test]
    async fn test_negociacao_promise_to_pay_sem_data_retorna_erro() {
        let inst_repo = InMemoryInstallmentRepository::default();
        let neg_repo = InMemoryNegotiationRepository::default();

        let associate_id = Uuid::new_v4();
        let installment = make_installment(associate_id, 5);
        let saved = InstallmentRepository::create(&inst_repo, &installment).await.unwrap();

        let service = NegotiationService::new(inst_repo, neg_repo);
        let result = RecordNegotiationUseCase::execute(
            &service,
            RecordNegotiationInput {
                installment_id: saved.id,
                associate_id,
                user_id: 10,
                result: NegotiationResult::PromiseToPay,
                promise_date: None, // missing required field
                promise_value: None,
                observation: None,
            },
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_negociacao_pago_atualiza_status_parcela() {
        // Tests that a Paid negotiation updates the installment status
        let inst_repo = InMemoryInstallmentRepository::default();
        let neg_repo = InMemoryNegotiationRepository::default();

        let associate_id = Uuid::new_v4();
        let installment = make_installment(associate_id, 20);
        let saved = InstallmentRepository::create(&inst_repo, &installment).await.unwrap();

        let service = NegotiationService::new(inst_repo, neg_repo);
        let output = RecordNegotiationUseCase::execute(
            &service,
            RecordNegotiationInput {
                installment_id: saved.id,
                associate_id,
                user_id: 5,
                result: NegotiationResult::Paid,
                promise_date: None,
                promise_value: None,
                observation: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(output.installment.status, InstallmentStatus::Paid);
    }

    #[tokio::test]
    async fn test_fluxo_completo_cobranca_ponta_a_ponta() {
        // Full flow: create installment -> distribute -> negotiate -> verify history
        let inst_repo = InMemoryInstallmentRepository::default();
        let dist_log_repo = InMemoryDistributionLogRepository::default();
        let neg_repo = InMemoryNegotiationRepository::default();

        let associate_id = Uuid::new_v4();

        // Create 2 overdue installments for the associate
        let inst1 = make_installment(associate_id, 15);
        let inst2 = make_installment(associate_id, 20);
        let saved1 = InstallmentRepository::create(&inst_repo, &inst1).await.unwrap();
        let saved2 = InstallmentRepository::create(&inst_repo, &inst2).await.unwrap();

        // Step 1: Distribute by associate (both go to same collector)
        let dist_service = DistributionService::new(inst_repo, dist_log_repo);
        let dist_output = DistributeInstallmentsUseCase::execute(
            &dist_service,
            DistributeInstallmentsInput {
                workgroup_id: 10,
                collector_user_ids: vec![500],
                min_days_overdue: 10,
                max_days_overdue: 30,
                distribution_type: DistributionType::Associate,
            },
        )
        .await
        .unwrap();
        assert_eq!(dist_output.distributed_count, 2);
        assert!(dist_output.logs.iter().all(|l| l.assigned_to_user_id == 500));

        // Step 2: Negotiate first installment - PromiseToPay
        // Need fresh inst_repo reference - use the internal store from DistributionService
        // Since we moved inst_repo into dist_service, we need to reconstruct
        let inst_repo2 = InMemoryInstallmentRepository::default();
        InstallmentRepository::create(&inst_repo2, &saved1).await.unwrap();
        InstallmentRepository::create(&inst_repo2, &saved2).await.unwrap();

        let neg_service = NegotiationService::new(inst_repo2, neg_repo);
        let promise = NaiveDate::from_ymd_opt(2026, 5, 1).unwrap();
        let neg_output = RecordNegotiationUseCase::execute(
            &neg_service,
            RecordNegotiationInput {
                installment_id: saved1.id,
                associate_id,
                user_id: 500,
                result: NegotiationResult::PromiseToPay,
                promise_date: Some(promise),
                promise_value: Some(saved1.value),
                observation: Some("Promessa firme".to_string()),
            },
        )
        .await
        .unwrap();
        assert_eq!(neg_output.negotiation.result, NegotiationResult::PromiseToPay);

        // Step 3: Negotiate second installment - Paid
        let neg_paid = RecordNegotiationUseCase::execute(
            &neg_service,
            RecordNegotiationInput {
                installment_id: saved2.id,
                associate_id,
                user_id: 500,
                result: NegotiationResult::Paid,
                promise_date: None,
                promise_value: None,
                observation: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(neg_paid.installment.status, InstallmentStatus::Paid);

        // Step 4: Verify immutable history for first installment
        let history = GetNegotiationHistoryUseCase::execute(
            &neg_service,
            GetNegotiationHistoryInput { installment_id: saved1.id },
        )
        .await
        .unwrap();
        assert_eq!(history.negotiations.len(), 1);
        assert_eq!(history.negotiations[0].result, NegotiationResult::PromiseToPay);
    }
}
