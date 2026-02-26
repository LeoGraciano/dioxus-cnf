pub mod domain;
pub mod application;
pub mod infrastructure;

pub use domain::{entities, value_objects, ports};
pub use application::{ports as app_ports, services};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn associate_status_default_is_active() {
        assert_eq!(entities::AssociateStatus::default(), entities::AssociateStatus::Active);
    }

    #[test]
    fn pagination_offset_calculation() {
        let p = value_objects::Pagination { page: 2, per_page: 20 };
        assert_eq!(p.offset(), 20);
    }

    #[test]
    fn repository_error_display() {
        let err = ports::RepositoryError::NotFound("test".to_string());
        assert!(err.to_string().contains("test"));
    }
}
