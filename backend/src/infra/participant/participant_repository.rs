use crate::domain::participant::Participant;
use crate::libs::prisma::participant;
use crate::libs::prisma::PrismaClient;
use crate::libs::trip;
use crate::AppError;
use std::sync::Arc;

#[derive(Clone)]
pub struct ParticipantRepository {
    db: Arc<PrismaClient>,
}

impl ParticipantRepository {
    pub fn new(db: Arc<PrismaClient>) -> Self {
        ParticipantRepository { db }
    }

    pub async fn insert(&self, participant: &Participant) -> Result<String, AppError> {
        let result = self
            .db
            .participant()
            .create(
                participant.email().to_owned(),
                trip::id::equals(participant.trip_id().to_string()),
                vec![
                    participant::id::set(participant.id().to_string()),
                    participant::name::set(participant.name().map(|name| name.to_owned())),
                    participant::is_owner::set(participant.is_owner()),
                    participant::is_confirmed::set(participant.is_confirmed()),
                ],
            )
            .exec()
            .await
            .map_err(AppError::from)?;
        let participant: Participant = result.into();
        Ok(participant.id().to_string())
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Participant, AppError> {
        let result = self
            .db
            .participant()
            .find_unique(participant::id::equals(id.to_string()))
            .exec()
            .await
            .map_err(AppError::from)?;
        match result {
            Some(participant) => Ok(participant.into()),
            None => Err(AppError::NotFound),
        }
    }

    pub async fn find_participants_by_trip_id(
        &self,
        trip_id: &str,
    ) -> Result<Vec<Participant>, AppError> {
        let result = self
            .db
            .participant()
            .find_many(vec![
                participant::trip_id::equals(trip_id.to_string()),
                participant::is_owner::equals(false),
            ])
            .exec()
            .await
            .map_err(AppError::from)?;
        Ok(result
            .into_iter()
            .map(|participant| participant.into())
            .collect())
    }
}
