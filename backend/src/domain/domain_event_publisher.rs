use super::events::domain_event_trait::DomainEvent;

pub trait DomainEventPublisher: Send + Sync {
    fn publish_event(&self, event: &DomainEvent);
}
