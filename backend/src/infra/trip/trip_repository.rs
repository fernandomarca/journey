use crate::domain::entity::Entity;
use crate::domain::participant::Participant;
use crate::domain::trip::Trip;
use crate::libs::participant;
use crate::libs::prisma::trip;
use crate::libs::prisma::PrismaClient;
use crate::AppError;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct TripRepository {
    pub db: Arc<PrismaClient>,
}

impl TripRepository {
    pub fn new(db: Arc<PrismaClient>) -> Self {
        TripRepository { db }
    }

    pub async fn update(&self, trip: &Trip) -> Result<(), AppError> {
        self.db
            .trip()
            .update(
                trip::id::equals(trip.get_id().to_string()),
                vec![
                    trip::destination::set(trip.destination.to_owned()),
                    trip::starts_at::set(trip.starts_at),
                    trip::ends_at::set(trip.ends_at),
                    trip::is_confirmed::set(trip.is_confirmed),
                ],
            )
            .exec()
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    pub async fn insert(&self, trip: &Trip) -> Result<String, AppError> {
        let result = self
            .db
            .trip()
            .create(
                trip.destination.to_owned(),
                trip.starts_at,
                trip.ends_at,
                vec![trip::id::set(trip.get_id().to_string())],
            )
            .exec()
            .await
            .map_err(AppError::from)?;
        let trip: Trip = result.into();
        Ok(trip.get_id().to_string())
    }

    pub async fn insert_with_participant(
        &self,
        trip: &Trip,
        participant: &Participant,
    ) -> Result<String, AppError> {
        self.db
            ._transaction()
            .run(|tx| async move {
                let trip_data = tx
                    .trip()
                    .create(
                        trip.destination.to_owned(),
                        trip.starts_at,
                        trip.ends_at,
                        vec![trip::id::set(trip.get_id().to_string())],
                    )
                    .exec()
                    .await
                    .map_err(AppError::from)?;

                let participant = Participant::with(
                    Uuid::now_v7(),
                    participant.name().map(|n| n.to_owned()),
                    participant.email(),
                    true,
                    true,
                    Uuid::parse_str(&trip_data.id).unwrap(),
                );
                //
                let _participant_result = tx
                    .participant()
                    .create(
                        participant.email().to_owned(),
                        trip::id::equals(trip_data.id.to_owned()),
                        vec![
                            participant::id::set(participant.id().to_string()),
                            participant::name::set(participant.name().map(|n| n.to_owned())),
                            participant::is_confirmed::set(participant.is_confirmed()),
                            participant::is_owner::set(participant.is_owner()),
                        ],
                    )
                    .exec()
                    .await
                    .map_err(AppError::from)?;

                Ok(trip_data.id)
            })
            .await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Trip, AppError> {
        let result = self
            .db
            .trip()
            .find_unique(trip::id::equals(id.to_string()))
            .exec()
            .await
            .map_err(AppError::from)?;
        match result {
            Some(trip) => Ok(trip.into()),
            None => Err(AppError::NotFound),
        }
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        self.db
            .trip()
            .delete(trip::id::equals(id.to_string()))
            .exec()
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    pub async fn find_all(&self) -> Result<Vec<Trip>, AppError> {
        let result = self
            .db
            .trip()
            .find_many(vec![])
            .exec()
            .await
            .map_err(AppError::from)?;
        Ok(result.into_iter().map(|trip| trip.into()).collect())
    }
}
