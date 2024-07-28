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
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Participant>, String>> + Send>>
    {
        todo!()
    }

    fn insert(
        &self,
        participant: Participant,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, AppError>> + Send + '_>>
    {
        Box::pin(async move {
            let result = self.repository.insert(&participant).await;
            match result {
                Ok(id) => Ok(id),
                Err(e) => Err(e),
            }
        })
    }

    fn update(
        &self,
        participant: Participant,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), String>> + Send>> {
        todo!()
    }

    fn delete(
        &self,
        id: Uuid,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), String>> + Send>> {
        todo!()
    }

    fn find_by_id(
        &self,
        id: Uuid,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Option<Participant>, String>> + Send + '_>,
    > {
        todo!()
    }
}
