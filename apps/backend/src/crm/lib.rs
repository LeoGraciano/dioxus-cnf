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
}
