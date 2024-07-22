use crate::domain::domain_event_trait::DomainEvent;
use cloudevents::Event;

pub trait EventServiceTrait: Send + Sync {
    fn send_cloud_event(&self, event: &Event);
}

pub trait DomainEventServiceTrait: Send + Sync {
    fn handle(&self, event: &DomainEvent);
}
