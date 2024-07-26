use super::AppJsonResult;
use super::Database;
use crate::application::trip_service::CreateTripCommand;
use crate::infra::modules::Modules;
use crate::AppError;
use axum::Extension;
use axum::Json;
use chrono::DateTime;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;
use validator::Validate;
use validator::ValidateEmail;
use validator::ValidationError;

pub async fn create_trip(
    _db: Database,
    modules: Extension<Arc<Modules>>,
    Json(input): Json<CreateTripRequest>,
) -> AppJsonResult<Value> {
    let command = input
        .self_validate()
        .map_err(|e| AppError::ClientError(e.to_string()))?;

    // let (trip, _participant) = db
    //     ._transaction()
    //     .run(|tx| async move {
    //         let trip = tx
    //             .trip()
    //             .create(
    //                 command.destination,
    //                 command.starts_at,
    //                 command.ends_at,
    //                 vec![],
    //             )
    //             .exec()
    //             .await?;

    //         let mut participants = vec![participant::create_unchecked(
    //             command.owner_email,
    //             trip.id.to_owned(),
    //             vec![
    //                 participant::name::set(Some(command.owner_name)),
    //                 participant::is_confirmed::set(true),
    //                 participant::is_owner::set(true),
    //             ],
    //         )];

    //         command.emails_to_invite.iter().for_each(|email| {
    //             participants.push(participant::create_unchecked(
    //                 email.to_owned(),
    //                 trip.id.to_owned(),
    //                 vec![
    //                     participant::name::set(None),
    //                     participant::is_confirmed::set(false),
    //                     participant::is_owner::set(false),
    //                 ],
    //             ))
    //         });

    //         tx.participant()
    //             .create_many(participants)
    //             .exec()
    //             .await
    //             .map(|participant| (trip, participant))
    //     })
    //     .await?;
    let trip = modules
        .trip_service_config
        .service()
        .insert(command)
        .await?;

    Ok(Json::from(json!({ "tripId": trip })))
}

#[derive(Deserialize, Validate, Clone)]
pub struct CreateTripRequest {
    #[validate(length(min = 4))]
    destination: String,

    #[validate(custom(function = "validate_datetime"))]
    starts_at: String,

    #[validate(custom(function = "validate_datetime"))]
    ends_at: String,

    #[validate(length(min = 4))]
    owner_name: String,

    #[validate(email)]
    owner_email: String,

    #[validate(custom(function = "validate_emails"))]
    emails_to_invite: Vec<String>,
}

impl CreateTripRequest {
    fn self_validate(&self) -> Result<CreateTripCommand, String> {
        self.validate().map_err(|e| e.to_string())?;
        CreateTripCommand::new(
            self.destination.to_owned(),
            DateTime::parse_from_rfc3339(&self.starts_at).unwrap_or_default(),
            DateTime::parse_from_rfc3339(&self.ends_at).unwrap_or_default(),
            self.owner_name.to_owned(),
            self.owner_email.to_owned(),
            self.emails_to_invite.to_owned(),
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

fn validate_emails(emails: &Vec<String>) -> Result<(), ValidationError> {
    for email in emails {
        if !ValidateEmail::validate_email(email) {
            return Err(ValidationError::new("invalid email"));
        }
    }
    Ok(())
}
