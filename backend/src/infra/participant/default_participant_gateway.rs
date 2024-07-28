#![allow(dead_code)]
#![allow(unused_variables)]

use super::participant_repository::ParticipantRepository;
use crate::domain::participant::Participant;
use crate::domain::participant_gateway_trait::ParticipantGatewayTrait;
use crate::AppError;
use uuid::Uuid;

pub struct DefaultParticipantGateway {
    repository: ParticipantRepository,
}

impl DefaultParticipantGateway {
    pub fn new(repository: ParticipantRepository) -> Self {
        DefaultParticipantGateway { repository }
    }
}

impl ParticipantGatewayTrait for DefaultParticipantGateway {
    fn find_all(
        &self,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Vec<Participant>, AppError>> + Send>,
    > {
        todo!()
    }

    fn insert(
        &self,
        participant: Participant,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, AppError>> + Send + '_>>
    {
        Box::pin(async move { self.repository.insert(&participant).await })
    }

    fn update(
        &self,
        participant: Participant,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), AppError>> + Send + '_>>
    {
        Box::pin(async move { self.repository.update(&participant).await })
    }

    fn delete(
        &self,
        id: Uuid,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), AppError>> + Send>> {
        todo!()
    }

    fn find_by_id<'a>(
        &'a self,
        id: &'a str,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Participant, AppError>> + Send + '_>,
    > {
        Box::pin(async move { self.repository.find_by_id(id).await })
    }

    fn find_participants_by_trip_id<'a>(
        &'a self,
        trip_id: &'a str,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Vec<Participant>, AppError>> + Send + '_>,
    > {
        Box::pin(async move { self.repository.find_participants_by_trip_id(trip_id).await })
    }
}
