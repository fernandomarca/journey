use super::domain_event_trait::DomainEventTrait;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripCreatedEvent {
    pub trip_id: String,
    pub participants: Option<Vec<Uuid>>,
}

impl TripCreatedEvent {
    pub fn new(trip_id: String, participants: Option<Vec<Uuid>>) -> Self {
        TripCreatedEvent {
            trip_id,
            participants,
        }
    }
}

impl DomainEventTrait for TripCreatedEvent {
    fn get_subject(&self) -> String {
        "trip_created_event".to_string()
    }
}
