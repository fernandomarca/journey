use crate::domain::entity::Entity;
use crate::domain::trip::Trip;
use crate::libs::prisma;
use crate::libs::prisma::trip;
use crate::libs::prisma::PrismaClient;
use crate::AppError;
use std::sync::Arc;

pub struct TripRepository {
    db: Arc<PrismaClient>,
}

impl TripRepository {
    pub fn new(db: Arc<PrismaClient>) -> Self {
        TripRepository { db }
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
}
