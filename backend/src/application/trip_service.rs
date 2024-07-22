use crate::domain::trip::Trip;
use crate::domain::trip_gateway_trait::TripGatewayTrait;
use crate::AppError;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Utc;

pub struct TripService<'a> {
    trip_gateway: &'a dyn TripGatewayTrait,
}

impl<'a> TripService<'a> {
    pub fn new(trip_gateway: &'a dyn TripGatewayTrait) -> Self {
        TripService { trip_gateway }
    }

    pub async fn insert(&self, create_trip_command: CreateTripCommand) -> Result<String, AppError> {
        let trip = Trip::new(
            create_trip_command.destination,
            create_trip_command.starts_at,
            create_trip_command.ends_at,
        );
        self.trip_gateway.insert(trip).await
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
