use super::domain_event_publisher::DomainEventPublisher;
use super::events::domain_event_trait::DomainEvent;
use uuid::Uuid;

pub trait Entity {
    fn get_id(&self) -> Uuid;
    fn get_domain_events(&self) -> &Vec<DomainEvent>;

    fn register_event(&mut self, event: DomainEvent);

    fn publish_domain_events(&self, publisher: impl DomainEventPublisher) {
        for event in self.get_domain_events() {
            publisher.publish_event(event);
        }
    }

    fn handle(&self, handler: impl Fn(&DomainEvent)) {
        for event in self.get_domain_events() {
            handler(event);
        }
    }
}
