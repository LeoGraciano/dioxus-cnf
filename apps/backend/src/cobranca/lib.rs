pub mod application;
pub mod domain;
pub mod infrastructure;

pub use domain::entities::{DistributionLog, Negotiation, Title, Installment, TitleStatus, InstallmentStatus};
pub use domain::ports::{
    TitleRepository, InstallmentRepository, NegotiationRepository, DistributionLogRepository,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_status_default_is_valid() {
        assert_eq!(TitleStatus::default(), TitleStatus::Valid);
    }

    #[test]
    fn installment_status_default_is_open() {
        assert_eq!(InstallmentStatus::default(), InstallmentStatus::Open);
    }
}
