use super::AppJsonResult;
use super::Database;
use crate::libs::trip;
use axum::extract::Path;
use axum::Json;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

pub async fn get_trip_details(db: Database, Path(trip_id): Path<Uuid>) -> AppJsonResult<Value> {
    let trip = db
        .trip()
        .find_unique(trip::id::equals(trip_id.to_string()))
        .select(trip::select!({
            id
            destination
            starts_at
            ends_at
            is_confirmed
        }))
        .exec()
        .await
        .map_err(|e| format!("find error {}", e))?;

    match trip {
        Some(trip) => Ok(Json::from(json!(trip))),
        None => Err("trip not found".to_string()),
    }
}
