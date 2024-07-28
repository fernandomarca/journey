use super::domain_event_trait::DomainEventTrait;
use crate::domain::participant::Participant;
use crate::domain::trip::Trip;
use chrono::DateTime;
use chrono::FixedOffset;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripCreatedEvent {
    pub id: Uuid,
    pub destination: String,
    pub starts_at: DateTime<FixedOffset>,
    pub ends_at: DateTime<FixedOffset>,
    pub is_confirmed: bool,
    pub participants: Option<Vec<Uuid>>,
    pub activities: Option<Vec<Uuid>>,
    pub links: Option<Vec<Uuid>>,
    pub owner_name: String,
    pub owner_email: String,
}

impl TripCreatedEvent {
    pub fn new(trip: &Trip, owner_name: &str, owner_email: &str) -> Self {
        TripCreatedEvent {
            id: trip.id,
            destination: trip.destination.clone(),
            starts_at: trip.starts_at,
            ends_at: trip.ends_at,
            is_confirmed: trip.is_confirmed,
            participants: trip.participants.clone(),
            activities: trip.activities.clone(),
            links: trip.links.clone(),
            owner_name: owner_name.to_string(),
            owner_email: owner_email.to_string(),
        }
    }
}

impl DomainEventTrait for TripCreatedEvent {
    fn get_subject(&self) -> String {
        "trip_created_event".to_string()
    }
}
