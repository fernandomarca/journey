use super::event_service_trait::DomainEventServiceTrait;
use crate::domain::events::domain_event_trait::DomainEvent;
use crate::domain::handlers::event_listener_trait::EventListener;
use cloudevents::AttributesReader;

pub struct DomainService {
    listeners: Vec<Box<dyn EventListener>>,
}

impl DomainService {
    pub fn new() -> Self {
        DomainService {
            listeners: Vec::new(),
        }
    }

    pub fn add_listener(&mut self, listener: Box<dyn EventListener>) {
        self.listeners.push(listener);
    }

    fn notify_listeners(&self, event: &DomainEvent) {
        for listener in &self.listeners {
            if event.event.subject() == Some(&listener.get_subject()) {
                listener.on_event(event);
            }
        }
    }
}

impl DomainEventServiceTrait for DomainService {
    fn handle(&self, event: &DomainEvent) {
        self.notify_listeners(event);
    }
}
