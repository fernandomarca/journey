use super::AppResult;
use crate::application::trip_service::UpdateTripCommand;
use crate::infra::modules::Modules;
use crate::AppError;
use axum::extract::Path;
use axum::Extension;
use axum::Json;
use chrono::DateTime;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use validator::ValidationError;

pub async fn update_trip(
    modules: Extension<Arc<Modules>>,
    Path(trip_id): Path<Uuid>,
    Json(input): Json<UpdateTripRequest>,
) -> AppResult<()> {
    let command = input.self_validate()?;
    modules.trip_service.update(trip_id, command).await
    // Ok(Json::from(json!({ "tripId": trip.id })))
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

fn validate_datetime(datetime: &str) -> Result<(), ValidationError> {
    let result = DateTime::parse_from_rfc3339(datetime);
    match result {
        Ok(_) => Ok(()),
        Err(_e) => Err(ValidationError::new("datetime parse error")),
    }
}
