use crate::domain::domain_event_trait::DomainEvent;
use cloudevents::Event;

pub trait EventServiceTrait: Send + Sync {
    fn send(&self, event: &DomainEvent);
    fn send_cloud_event(&self, event: &Event);
}
