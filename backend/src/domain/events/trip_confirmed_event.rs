use super::domain_event_trait::DomainEventTrait;
use crate::domain::trip::Trip;
use chrono::DateTime;
use chrono::FixedOffset;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripConfirmedEvent {
    pub id: Uuid,
    pub destination: String,
    pub starts_at: DateTime<FixedOffset>,
    pub ends_at: DateTime<FixedOffset>,
    pub is_confirmed: bool,
    pub participants: Option<Vec<Uuid>>,
}

impl TripConfirmedEvent {
    pub fn new(trip: &Trip) -> Self {
        TripConfirmedEvent {
            id: trip.id,
            destination: trip.destination.clone(),
            starts_at: trip.starts_at,
            ends_at: trip.ends_at,
            is_confirmed: trip.is_confirmed,
            participants: trip.participants.clone(),
        }
    }
}

impl DomainEventTrait for TripConfirmedEvent {
    fn get_subject(&self) -> String {
        "trip_confirmed_event".to_string()
    }
}
