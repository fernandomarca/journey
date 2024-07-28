use crate::domain::participant::Participant;
use crate::domain::trip::Trip;
use crate::domain::trip_gateway_trait::TripGatewayTrait;
use crate::AppError;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Utc;
use std::sync::Arc;

pub struct TripService {
    trip_gateway: Arc<Box<dyn TripGatewayTrait>>,
}

impl TripService {
    pub fn new(trip_gateway: Arc<Box<dyn TripGatewayTrait>>) -> Self {
        TripService { trip_gateway }
    }
    pub async fn insert(&self, create_trip_command: CreateTripCommand) -> Result<String, AppError> {
        let trip = Trip::new(
            create_trip_command.destination,
            create_trip_command.starts_at,
            create_trip_command.ends_at,
        );
        let trip_id = self.trip_gateway.insert(&trip).await?;
        Ok(trip_id)
    }

    pub async fn insert_with_participant(
        &self,
        create_trip_command: CreateTripCommand,
    ) -> Result<String, AppError> {
        let mut trip = Trip::new(
            create_trip_command.destination,
            create_trip_command.starts_at,
            create_trip_command.ends_at,
        );
        let participant = Participant::new(
            Some(create_trip_command.owner_name),
            create_trip_command.owner_email,
            true,
            true,
            trip.id,
        );
        //
        trip.on_trip_created(&participant);
        //
        let trip_id = self
            .trip_gateway
            .insert_with_participant(&trip, &participant)
            .await?;
        Ok(trip_id)
    }
}

#[derive(Debug, Clone)]
pub struct CreateTripCommand {
    pub destination: String,
    pub starts_at: DateTime<FixedOffset>,
    pub ends_at: DateTime<FixedOffset>,
    pub owner_name: String,
    pub owner_email: String,
    pub emails_to_invite: Vec<String>,
}

impl CreateTripCommand {
    pub fn new(
        destination: String,
        starts_at: DateTime<FixedOffset>,
        ends_at: DateTime<FixedOffset>,
        owner_name: String,
        owner_email: String,
        emails_to_invite: Vec<String>,
    ) -> Result<Self, String> {
        let command = Self {
            destination,
            starts_at,
            ends_at,
            owner_name,
            owner_email,
            emails_to_invite,
        };
        if command.starts_at < Utc::now() {
            return Err("invalid trip start date.".to_string());
        }
        if command.ends_at < command.starts_at {
            return Err("invalid trip end date.".to_string());
        }
        Ok(command)
    }
}
