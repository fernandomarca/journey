use super::AppJsonResult;
use super::Database;
use crate::libs::activity;
use crate::libs::trip;
use crate::libs::trip::activities;
use axum::extract::Path;
use axum::Json;
use chrono::Datelike;
use prisma_client_rust::Direction;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

pub async fn get_activities(db: Database, Path(trip_id): Path<Uuid>) -> AppJsonResult<Value> {
    let trip = db
        .trip()
        .find_unique(trip::id::equals(trip_id.to_string()))
        .with(activities::fetch(vec![]).order_by(activity::occurs_at::order(Direction::Asc)))
        .exec()
        .await
        .map_err(|e| format!("find error {}", e))?;

    match trip {
        Some(trip) => {
            let activities = trip.activities().ok();

            let difference_in_days_between_start_and_end =
                (trip.ends_at - trip.starts_at).num_days() + 1;

            let activities: Vec<Value> = (0..difference_in_days_between_start_and_end)
                .map(|day| {
                    let date = trip.starts_at + chrono::Duration::days(day);
                    json!({
                        "date": date.to_rfc3339(),
                        "activities":activities
                        .map(|activities| {
                            activities
                                .iter()
                                .filter(|activity| activity.occurs_at.day() == date.day())
                                .collect::<Vec<_>>()
                        })
                    })
                })
                .collect();

            Ok(Json::from(json!(activities)))
        }
        None => Err("trip not found".to_string()),
    }
}
