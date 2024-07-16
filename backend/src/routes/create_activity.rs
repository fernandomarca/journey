use super::AppJsonResult;
use super::Database;
use crate::libs::trip;
use crate::AppError;
use axum::extract::Path;
use axum::Json;
use chrono::DateTime;
use chrono::FixedOffset;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;
use validator::Validate;
use validator::ValidationError;

pub async fn create_activity(
    db: Database,
    Path(trip_id): Path<Uuid>,
    Json(input): Json<CreateActivityRequest>,
) -> AppJsonResult<Value> {
    let command = input.self_validate()?;

    let trip = db
        .trip()
        .find_unique(trip::id::equals(trip_id.to_string()))
        .exec()
        .await?;

    match trip {
        Some(trip) => {
            if command.occurs_at < trip.starts_at {
                return Err(AppError::ClientError(
                    "activity occurs before trip starts".to_string(),
                ));
            }

            if command.occurs_at > trip.ends_at {
                return Err(AppError::ClientError(
                    "activity occurs after trip ends".to_string(),
                ));
            }

            let activity = db
                .activity()
                .create(
                    command.title,
                    command.occurs_at,
                    trip::id::equals(trip_id.to_string()),
                    vec![],
                )
                .exec()
                .await?;

            Ok(Json::from(json!({ "activityId": activity.id })))
        }
        None => Err(AppError::NotFound),
    }
}

#[derive(Deserialize, Validate, Clone)]
pub struct CreateActivityRequest {
    #[validate(length(min = 4))]
    title: String,

    #[validate(custom(function = "validate_datetime"))]
    occurs_at: String,
}

impl CreateActivityRequest {
    fn self_validate(&self) -> Result<CreateActivityCommand, AppError> {
        self.validate()
            .map_err(|e| AppError::ClientError(e.to_string()))?;
        Ok(CreateActivityCommand::new(
            self.title.to_owned(),
            DateTime::parse_from_rfc3339(&self.occurs_at).unwrap_or_default(),
        ))
    }
}

#[derive(Debug, Clone)]
struct CreateActivityCommand {
    title: String,
    occurs_at: DateTime<FixedOffset>,
}

impl CreateActivityCommand {
    pub fn new(title: String, occurs_at: DateTime<FixedOffset>) -> Self {
        Self { title, occurs_at }
    }
}

fn validate_datetime(datetime: &str) -> Result<(), ValidationError> {
    let result = DateTime::parse_from_rfc3339(datetime);
    match result {
        Ok(_) => Ok(()),
        Err(_e) => Err(ValidationError::new("datetime parse error")),
    }
}
