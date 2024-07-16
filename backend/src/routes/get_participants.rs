use super::AppJsonResult;
use super::Database;
use crate::libs::trip;
use crate::AppError;
use axum::extract::Path;
use axum::Json;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

pub async fn get_participants(db: Database, Path(trip_id): Path<Uuid>) -> AppJsonResult<Value> {
    let trip = db
        .trip()
        .find_unique(trip::id::equals(trip_id.to_string()))
        // .include(trip::include!({
        //     participants: select {
        //         id
        //         name
        //         email
        //         is_confirmed
        //     }
        // }))
        .select(trip::select!({
            participants(vec![]): select {
                id
                name
                email
                is_confirmed
            }
        }))
        .exec()
        .await?;

    match trip {
        Some(trip) => {
            let participants = trip.participants;
            Ok(Json::from(json!({"participants": participants})))
        }
        None => Err(AppError::NotFound),
    }
}
