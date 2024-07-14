use super::AppJsonResult;
use axum::extract::Path;
use axum::Json;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

pub async fn confirm_trip(Path(trip_id): Path<Uuid>) -> AppJsonResult<Value> {
    Ok(Json::from(json!({ "tripId": trip_id })))
}
