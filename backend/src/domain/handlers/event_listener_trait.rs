use crate::domain::events::domain_event_trait::DomainEvent;

pub trait EventListener: Send + Sync {
    fn on_event(&self, event: &DomainEvent);
    fn get_subject(&self) -> String;
}
