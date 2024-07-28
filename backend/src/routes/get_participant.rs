use super::AppJsonResult;
use crate::infra::modules::Modules;
use axum::extract::Path;
use axum::Extension;
use axum::Json;
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

pub async fn get_participant(
    modules: Extension<Arc<Modules>>,
    Path(participant_id): Path<Uuid>,
) -> AppJsonResult<Value> {
    let participant = modules
        .participant_service
        .find_by_id(&participant_id.to_string())
        .await;

    match participant {
        Ok(participant) => Ok(Json::from(json!(participant))),
        Err(e) => Err(e),
    }
}
