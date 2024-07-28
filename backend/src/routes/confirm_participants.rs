use super::AppResult;
use crate::infra::modules::Modules;
use axum::extract::Path;
use axum::response::Redirect;
use axum::Extension;
use std::sync::Arc;
use uuid::Uuid;

pub async fn confirm_participants(
    modules: Extension<Arc<Modules>>,
    Path(participant_id): Path<Uuid>,
) -> AppResult<Redirect> {
    modules
        .participant_service
        .confirm_participant(&participant_id.to_string())
        .await
        .map(|_| {
            Ok(Redirect::to(
                format!("http://localhost:3000/trips/{}", participant_id).as_str(),
            ))
        })?
}
