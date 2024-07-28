use super::AppResult;
use crate::infra::modules::Modules;
use crate::libs::mail::get_client_mail;
use crate::libs::participant;
use crate::libs::prisma::trip;
use crate::AppError;
use axum::extract::Path;
use axum::response::Redirect;
use axum::Extension;
use chrono::format::StrftimeItems;
use chrono::Locale;
use futures::future::join_all;
use lettre::message::Mailbox;
use lettre::message::MultiPart;
use lettre::message::SinglePart;
use lettre::Message;
use lettre::Transport;
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
