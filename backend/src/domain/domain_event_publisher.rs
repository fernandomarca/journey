use super::domain_event_trait::DomainEvent;

pub trait DomainEventPublisher {
    fn publish_event(&self, event: &DomainEvent);
}
