#![allow(dead_code)]

use super::entity::Entity;
use super::events::domain_event_trait::DomainEvent;
use super::events::trip_confirmed_event::TripConfirmedEvent;
use super::events::trip_created_event::TripCreatedEvent;
use super::participant::Participant;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
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

    #[serde(skip)]
    pub domain_events: Vec<DomainEvent>,
}

impl Trip {
    pub fn new(
        destination: String,
        starts_at: DateTime<FixedOffset>,
        ends_at: DateTime<FixedOffset>,
    ) -> Self {
        Trip {
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
        }
    }

    pub fn on_trip_created(&mut self, participant: &Participant) {
        let trip_created_event =
            TripCreatedEvent::new(self, participant.name().unwrap(), participant.email());
        let event = DomainEvent::new(trip_created_event);
        self.register_event(event);
    }

    pub fn on_trip_confirmed(&mut self) {
        let trip_confirmed_event = TripConfirmedEvent::new(self);
        let event = DomainEvent::new(trip_confirmed_event);
        self.register_event(event);
    }

    pub fn confirm_trip(&mut self) {
        self.is_confirmed = true;
        self.on_trip_confirmed();
    }

    pub fn include_participants(&self, participants: Vec<Uuid>) -> Self {
        Self {
            participants: Some(participants),
            ..self.clone()
        }
    }

    pub fn update(
        &self,
        destination: String,
        starts_at: DateTime<FixedOffset>,
        ends_at: DateTime<FixedOffset>,
    ) -> Self {
        Self {
            destination,
            starts_at,
            ends_at,
            ..self.clone()
        }
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
