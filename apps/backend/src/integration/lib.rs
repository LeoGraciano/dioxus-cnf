pub mod application;
pub mod domain;
pub mod infrastructure;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sync_status_default_is_pending() {
        assert_eq!(domain::entities::SyncStatus::default(), domain::entities::SyncStatus::Pending);
    }

    #[test]
    fn sync_direction_eq() {
        assert_eq!(
            domain::entities::SyncDirection::InboundFromLegacy,
            domain::entities::SyncDirection::InboundFromLegacy
        );
        assert_ne!(
            domain::entities::SyncDirection::InboundFromLegacy,
            domain::entities::SyncDirection::OutboundToLegacy
        );
    }
}
