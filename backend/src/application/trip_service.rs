use crate::domain::participant::Participant;
use crate::domain::trip::Trip;
use crate::domain::trip_gateway_trait::TripGatewayTrait;
use crate::AppError;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

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
        // register event
        trip.on_trip_created(&participant);
        //
        let trip_id = self
            .trip_gateway
            .insert_with_participant(&trip, &participant)
            .await?;
        Ok(trip_id)
    }

    pub async fn confirm_trip(&self, trip_id: Uuid) -> Result<(), AppError> {
        let mut trip = self.trip_gateway.find_by_id(trip_id).await?;

        if trip.is_confirmed {
            return Ok(());
        }
        trip.confirm_trip();
        self.trip_gateway.update(trip).await
    }

    pub async fn find_all(&self) -> Result<Vec<Trip>, AppError> {
        self.trip_gateway.find_all().await
    }

    pub async fn update(
        &self,
        trip_id: Uuid,
        update_trip_command: UpdateTripCommand,
    ) -> Result<(), AppError> {
        let trip = self.trip_gateway.find_by_id(trip_id).await?;
        let updated_trip = trip.update(
            update_trip_command.destination,
            update_trip_command.starts_at,
            update_trip_command.ends_at,
        );
        self.trip_gateway.update(updated_trip).await
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

#[derive(Debug, Clone)]
pub struct UpdateTripCommand {
    pub destination: String,
    pub starts_at: DateTime<FixedOffset>,
    pub ends_at: DateTime<FixedOffset>,
}

impl UpdateTripCommand {
    pub fn new(
        destination: String,
        starts_at: DateTime<FixedOffset>,
        ends_at: DateTime<FixedOffset>,
    ) -> Result<Self, AppError> {
        let command = Self {
            destination,
            starts_at,
            ends_at,
        };
        if command.starts_at < Utc::now() {
            return Err(AppError::ClientError(
                "invalid trip start date.".to_string(),
            ));
        }
        if command.ends_at < command.starts_at {
            return Err(AppError::ClientError("invalid trip end date.".to_string()));
        }
        Ok(command)
    }
}
