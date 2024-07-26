#![allow(dead_code)]

use super::entity::Entity;
use super::events::domain_event_trait::DomainEvent;
use super::events::trip_created_event::TripCreatedEvent;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Trip {
    pub id: Uuid,
    pub destination: String,
    pub starts_at: DateTime<FixedOffset>,
    pub ends_at: DateTime<FixedOffset>,
    pub is_confirmed: bool,
    pub created_at: DateTime<FixedOffset>,

    pub participants: Option<Vec<Uuid>>,
    pub activities: Option<Vec<Uuid>>,
    pub links: Option<Vec<Uuid>>,

    pub domain_events: Vec<DomainEvent>,
}

impl Trip {
    pub fn new(
        destination: String,
        starts_at: DateTime<FixedOffset>,
        ends_at: DateTime<FixedOffset>,
    ) -> Self {
        let mut trip = Trip {
            id: Uuid::now_v7(),
            destination,
            starts_at,
            ends_at,
            is_confirmed: false,
            created_at: Utc::now().fixed_offset(),
            participants: None,
            activities: None,
            links: None,
            domain_events: Vec::new(),
        };
        trip.on_trip_created();
        trip
    }

    fn on_trip_created(&mut self) {
        // register event send email to participants
        let trip_created_event =
            TripCreatedEvent::new(self.get_id().to_string(), self.participants.to_owned());

        let event = DomainEvent::new(trip_created_event);

        self.register_event(event);
    }
}

impl Entity for Trip {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_domain_events(&self) -> &Vec<DomainEvent> {
        &self.domain_events
    }

    fn register_event(&mut self, event: DomainEvent) {
        self.domain_events.push(event);
    }
}
