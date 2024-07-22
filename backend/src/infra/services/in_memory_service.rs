use super::event_service_trait::EventServiceTrait;
use crate::domain::domain_event_trait::DomainEvent;
use cloudevents::Event;
use tracing::debug;

pub struct InMemoryService {}

impl InMemoryService {
    pub fn new() -> Self {
        InMemoryService {}
    }
}

impl EventServiceTrait for InMemoryService {
    fn send_cloud_event(&self, event: &Event) {
        debug!("send_cloud_event: {:?}", event);
    }
}
