use super::AppJsonResult;
use super::Database;
use crate::libs::prisma::trip;
use crate::AppError;
use axum::extract::Path;
use axum::Json;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Utc;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;
use validator::Validate;
use validator::ValidationError;

pub async fn update_trip(
    db: Database,
    Path(trip_id): Path<Uuid>,
    Json(input): Json<UpdateTripRequest>,
) -> AppJsonResult<Value> {
    let command = input.self_validate()?;

    let trip = db
        .trip()
        .find_unique(trip::id::equals(trip_id.to_string()))
        .exec()
        .await?;

    match trip {
        Some(trip) => {
            let _updated_trip = db
                .trip()
                .update(
                    trip::id::equals(trip.id.clone()),
                    vec![
                        trip::destination::set(command.destination),
                        trip::starts_at::set(command.starts_at),
                        trip::ends_at::set(command.ends_at),
                    ],
                )
                .exec()
                .await?;

            Ok(Json::from(json!({ "tripId": trip.id })))
        }
        None => Err(AppError::NotFound),
    }
}

#[derive(Deserialize, Validate, Clone)]
pub struct UpdateTripRequest {
    #[validate(length(min = 4))]
    destination: String,

    #[validate(custom(function = "validate_datetime"))]
    starts_at: String,

    #[validate(custom(function = "validate_datetime"))]
    ends_at: String,
}

impl UpdateTripRequest {
    fn self_validate(&self) -> Result<UpdateTripCommand, AppError> {
        self.validate()
            .map_err(|e| AppError::ClientError(e.to_string()))?;
        UpdateTripCommand::new(
            self.destination.to_owned(),
            DateTime::parse_from_rfc3339(&self.starts_at).unwrap_or_default(),
            DateTime::parse_from_rfc3339(&self.ends_at).unwrap_or_default(),
        )
    }
}

#[derive(Debug, Clone)]
struct UpdateTripCommand {
    destination: String,
    starts_at: DateTime<FixedOffset>,
    ends_at: DateTime<FixedOffset>,
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

fn validate_datetime(datetime: &str) -> Result<(), ValidationError> {
    let result = DateTime::parse_from_rfc3339(datetime);
    match result {
        Ok(_) => Ok(()),
        Err(_e) => Err(ValidationError::new("datetime parse error")),
    }
}
