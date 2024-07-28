#![allow(dead_code)]

use crate::domain::events::domain_event_trait::DomainEvent;
use cloudevents::Event;

// it's publishers
pub trait EventServiceTrait: Send + Sync {
    fn send_cloud_event(&self, event: &Event);
}

pub trait DomainEventServiceTrait: Send + Sync {
    fn handle(&self, event: &DomainEvent);
}
