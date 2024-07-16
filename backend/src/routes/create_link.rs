use super::AppJsonResult;
use super::Database;
use crate::libs::trip;
use crate::AppError;
use axum::extract::Path;
use axum::Json;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;
use validator::Validate;

pub async fn create_link(
    db: Database,
    Path(trip_id): Path<Uuid>,
    Json(input): Json<CreateLinkRequest>,
) -> AppJsonResult<Value> {
    let command = input.self_validate()?;

    let trip = db
        .trip()
        .find_unique(trip::id::equals(trip_id.to_string()))
        .exec()
        .await?;

    match trip {
        Some(_trip) => {
            let link = db
                .link()
                .create(
                    command.title,
                    command.url,
                    trip::id::equals(trip_id.to_string()),
                    vec![],
                )
                .exec()
                .await?;

            Ok(Json::from(json!({ "LinkId": link.id })))
        }
        None => Err(AppError::NotFound),
    }
}

#[derive(Deserialize, Validate, Clone)]
pub struct CreateLinkRequest {
    #[validate(length(min = 4))]
    title: String,

    #[validate(url)]
    url: String,
}

impl CreateLinkRequest {
    fn self_validate(&self) -> Result<CreateLinkCommand, AppError> {
        self.validate()
            .map_err(|e| AppError::ClientError(e.to_string()))?;

        Ok(CreateLinkCommand::new(
            self.title.to_owned(),
            self.url.to_owned(),
        ))
    }
}

#[derive(Debug, Clone)]
struct CreateLinkCommand {
    title: String,
    url: String,
}

impl CreateLinkCommand {
    pub fn new(title: String, url: String) -> Self {
        Self { title, url }
    }
}
