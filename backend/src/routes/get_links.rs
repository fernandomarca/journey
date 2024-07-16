use super::AppJsonResult;
use super::Database;
use crate::libs::trip;
use crate::libs::trip::links;
use crate::AppError;
use axum::extract::Path;
use axum::Json;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

pub async fn get_links(db: Database, Path(trip_id): Path<Uuid>) -> AppJsonResult<Value> {
    let trip = db
        .trip()
        .find_unique(trip::id::equals(trip_id.to_string()))
        .with(links::fetch(vec![]))
        .exec()
        .await?;

    match trip {
        Some(trip) => Ok(Json::from(json!({"links": trip.links().ok()}))),
        None => Err(AppError::NotFound),
    }
}
