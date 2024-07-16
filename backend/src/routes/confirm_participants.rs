use super::AppResult;
use super::Database;
use crate::libs::participant;
use crate::AppError;
use axum::extract::Path;
use axum::response::Redirect;
use tracing::debug;
use uuid::Uuid;

pub async fn confirm_participants(
    db: Database,
    Path(participant_id): Path<Uuid>,
) -> AppResult<Redirect> {
    let participant = db
        .participant()
        .find_unique(participant::id::equals(participant_id.to_string()))
        .exec()
        .await
        .map_err(|e| {
            debug!("Error query: {:?}", e);
            e
        })?;

    match participant {
        Some(participant) if (participant.is_confirmed) => Ok(Redirect::to(
            format!("http://localhost:3000/trips/{}", participant_id).as_str(),
        )),
        Some(_participant) => {
            db.participant()
                .update(
                    participant::id::equals(participant_id.to_string()),
                    vec![participant::is_confirmed::set(true)],
                )
                .exec()
                .await?;

            Ok(Redirect::to(
                format!("http://localhost:3000/trips/{}", participant_id).as_str(),
            ))
        }
        None => {
            debug!("Participant not found");
            Err(AppError::NotFound)
        }
    }
}
