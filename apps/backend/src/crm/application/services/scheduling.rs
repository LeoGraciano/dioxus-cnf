use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use shared::domain::ports::RepositoryError;

use crate::domain::entities::{CheckIn, Scheduling};
use crate::domain::ports::{CheckInRepository, SchedulingRepository};
use crate::application::ports::{
    RecordCheckInInput, RecordCheckInOutput, RecordCheckInUseCase,
    RecordCheckOutInput, RecordCheckOutOutput, RecordCheckOutUseCase,
    ScheduleAppointmentInput, ScheduleAppointmentOutput, ScheduleAppointmentUseCase,
    UpdateSchedulingStatusInput, UpdateSchedulingStatusOutput, UpdateSchedulingStatusUseCase,
};

pub struct SchedulingService<S, C>
where
    S: SchedulingRepository,
    C: CheckInRepository,
{
    scheduling_repo: S,
    check_in_repo: C,
}

impl<S, C> SchedulingService<S, C>
where
    S: SchedulingRepository,
    C: CheckInRepository,
{
    pub fn new(scheduling_repo: S, check_in_repo: C) -> Self {
        Self { scheduling_repo, check_in_repo }
    }
}

#[async_trait]
impl<S, C> ScheduleAppointmentUseCase for SchedulingService<S, C>
where
    S: SchedulingRepository + Send + Sync,
    C: CheckInRepository + Send + Sync,
{
    async fn execute(&self, input: ScheduleAppointmentInput) -> Result<ScheduleAppointmentOutput, RepositoryError> {
        let scheduling = Scheduling {
            id: Uuid::new_v4(),
            associate_id: input.associate_id,
            scheduled_date: input.scheduled_date,
            scheduling_type: input.scheduling_type,
            status: crate::domain::entities::SchedulingStatus::Scheduled,
            notes: input.notes,
            created_at: Utc::now(),
        };

        let created = self.scheduling_repo.create(&scheduling).await?;
        Ok(ScheduleAppointmentOutput { scheduling: created })
    }
}

#[async_trait]
impl<S, C> UpdateSchedulingStatusUseCase for SchedulingService<S, C>
where
    S: SchedulingRepository + Send + Sync,
    C: CheckInRepository + Send + Sync,
{
    async fn execute(&self, input: UpdateSchedulingStatusInput) -> Result<UpdateSchedulingStatusOutput, RepositoryError> {
        let mut scheduling = self.scheduling_repo.find_by_id(input.scheduling_id).await?
            .ok_or_else(|| RepositoryError::NotFound(format!("Scheduling not found: {}", input.scheduling_id)))?;

        scheduling.status = input.new_status;
        let updated = self.scheduling_repo.update(&scheduling).await?;
        Ok(UpdateSchedulingStatusOutput { scheduling: updated })
    }
}

#[async_trait]
impl<S, C> RecordCheckInUseCase for SchedulingService<S, C>
where
    S: SchedulingRepository + Send + Sync,
    C: CheckInRepository + Send + Sync,
{
    async fn execute(&self, input: RecordCheckInInput) -> Result<RecordCheckInOutput, RepositoryError> {
        let check_in = CheckIn {
            id: Uuid::new_v4(),
            associate_id: input.associate_id,
            check_in_time: Utc::now(),
            check_out_time: None,
            location: input.location,
        };

        let created = self.check_in_repo.create(&check_in).await?;
        Ok(RecordCheckInOutput { check_in: created })
    }
}

#[async_trait]
impl<S, C> RecordCheckOutUseCase for SchedulingService<S, C>
where
    S: SchedulingRepository + Send + Sync,
    C: CheckInRepository + Send + Sync,
{
    async fn execute(&self, input: RecordCheckOutInput) -> Result<RecordCheckOutOutput, RepositoryError> {
        let mut check_in = self.check_in_repo.find_by_id(input.check_in_id).await?
            .ok_or_else(|| RepositoryError::NotFound(format!("CheckIn not found: {}", input.check_in_id)))?;

        check_in.check_out_time = Some(Utc::now());
        let updated = self.check_in_repo.update(&check_in).await?;
        Ok(RecordCheckOutOutput { check_in: updated })
    }
}
