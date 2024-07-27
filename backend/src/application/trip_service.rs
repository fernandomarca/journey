use crate::domain::participant::Participant;
use crate::domain::participant_gateway_trait::ParticipantGatewayTrait;
use crate::domain::trip::Trip;
use crate::domain::trip_gateway_trait::TripGatewayTrait;
use crate::AppError;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

pub struct TripService<'a> {
    trip_gateway: &'a dyn TripGatewayTrait,
    participant_gateway: Arc<Box<dyn ParticipantGatewayTrait>>,
}

impl<'a> TripService<'a> {
    pub fn new(
        trip_gateway: &'a dyn TripGatewayTrait,
        participant_gateway: Arc<Box<dyn ParticipantGatewayTrait>>,
    ) -> Self {
        TripService {
            trip_gateway,
            participant_gateway,
        }
    }

    pub async fn insert(&self, create_trip_command: CreateTripCommand) -> Result<String, AppError> {
        let trip = Trip::new(
            create_trip_command.destination,
            create_trip_command.starts_at,
            create_trip_command.ends_at,
        );
        // criar trip and participant owner
        let client = self.trip_gateway.get_transaction();
        client
            ._transaction()
            .run(|_tx| async move {
                let trip_id = self.trip_gateway.insert(trip).await?;
                let participant = Participant::with(
                    Uuid::now_v7(),
                    Some(create_trip_command.owner_name),
                    create_trip_command.owner_email,
                    true,
                    true,
                    Uuid::parse_str(&trip_id).unwrap(),
                );
                let _participant_id = self.participant_gateway.insert(participant).await?;
                Ok(trip_id)
            })
            .await
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
