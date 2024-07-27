use crate::domain::event_service_traits::EventServiceTrait;
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
