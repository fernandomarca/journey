use super::AppJsonResult;
use crate::infra::modules::Modules;
use axum::Extension;
use axum::Json;
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;

pub async fn list_trips(modules: Extension<Arc<Modules>>) -> AppJsonResult<Value> {
    let result = modules.trip_service.find_all().await?;
    Ok(Json::from(json!(result)))
}
