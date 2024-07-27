use super::event_service_traits::EventServiceTrait;
use super::events::domain_event_trait::DomainEvent;
use uuid::Uuid;

pub trait Entity {
    fn get_id(&self) -> Uuid;
    fn get_domain_events(&self) -> &Vec<DomainEvent>;

    fn register_event(&mut self, event: DomainEvent);

    fn publish_to_cloud_event(&self, publisher: impl EventServiceTrait) {
        for event in self.get_domain_events() {
            publisher.send_cloud_event(&event.event);
        }
    }

    fn handle(&self, handler: impl Fn(&DomainEvent)) {
        for event in self.get_domain_events() {
            handler(event);
        }
    }
}
