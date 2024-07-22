use super::event_service_trait::DomainEventServiceTrait;
use super::event_service_trait::EventServiceTrait;
use crate::domain::domain_event_trait::DomainEvent;
use cloudevents::AttributesReader;
use cloudevents::Event;
use tracing::debug;

pub struct DomainService {}

impl DomainService {
    pub fn new() -> Self {
        DomainService {}
    }
}

impl DomainEventServiceTrait for DomainService {
    fn handle(&self, event: &DomainEvent) {
        let subject = event.event.subject().unwrap_or_default();
        match subject {
            "trip_created_event" => {
                debug!("TripCreatedEvent: {:?}", event);
            }
            _ => {
                debug!("Unknown event: {:?}", event);
            }
        }
    }
}
