use super::AppJsonResult;
use super::Database;
use crate::libs::prisma::participant;
use axum::extract::Path;
use axum::Json;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

pub async fn get_participant(
    db: Database,
    Path(participant_id): Path<Uuid>,
) -> AppJsonResult<Value> {
    let participant = db
        .participant()
        .find_unique(participant::id::equals(participant_id.to_string()))
        .select(participant::select!({
            id
            name
            email
            is_confirmed
        }))
        .exec()
        .await
        .map_err(|e| format!("find error {}", e))?;

    match participant {
        Some(participant) => Ok(Json::from(json!(participant))),
        None => Err("participant not found".to_string()),
    }
}
