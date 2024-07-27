use super::event_listener_trait::EventListener;
use crate::domain::events::domain_event_trait::DomainEvent;
use crate::infra::participant::participant_repository::ParticipantRepository;

pub struct CreateOwnerParticipantHandler {
    participant_repository: ParticipantRepository,
}

impl CreateOwnerParticipantHandler {
    pub fn new(participant_repository: ParticipantRepository) -> Self {
        CreateOwnerParticipantHandler {
            participant_repository,
        }
    }
}

impl EventListener for CreateOwnerParticipantHandler {
    fn on_event(&self, event: &DomainEvent) {
        todo!()
    }

    fn get_subject(&self) -> String {
        "trip_created_event".to_string()
    }
}
