use super::AppResult;
use crate::infra::modules::Modules;
use axum::extract::Path;
use axum::response::Redirect;
use axum::Extension;
use std::sync::Arc;
use uuid::Uuid;

pub async fn confirm_trip(
    modules: Extension<Arc<Modules>>,
    Path(trip_id): Path<Uuid>,
) -> AppResult<Redirect> {
    modules.trip_service.confirm_trip(trip_id).await.map(|_| {
        Ok(Redirect::to(
            format!("http://localhost:3000/trips/{}", trip_id).as_str(),
        ))
    })?
}
