use super::AppJsonResult;
use super::Database;
use crate::libs::trip;
use axum::Json;

pub async fn list_trips(db: Database) -> AppJsonResult<Vec<trip::Data>> {
    let result = db
        .trip()
        .find_many(vec![])
        .exec()
        .await
        .map_err(|e| format!("list error {}", e))?;
    Ok(Json::from(result))
}
