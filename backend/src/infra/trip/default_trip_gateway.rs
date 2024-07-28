#![allow(dead_code)]
#![allow(unused_variables)]

use super::trip_repository::TripRepository;
use crate::domain::entity::Entity;
use crate::domain::event_service_traits::DomainEventServiceTrait;
use crate::domain::event_service_traits::EventServiceTrait;
use crate::domain::participant::Participant;
use crate::domain::trip::Trip;
use crate::domain::trip_gateway_trait::TripGatewayTrait;
use crate::AppError;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use uuid::Uuid;

pub struct DefaultTripGateway {
    repository: TripRepository,
    event_service: Box<dyn EventServiceTrait>,
    domain_service: Arc<Box<dyn DomainEventServiceTrait>>,
}

impl DefaultTripGateway {
    pub fn new(
        repository: TripRepository,
        event_service: Box<dyn EventServiceTrait>,
        domain_service: Arc<Box<dyn DomainEventServiceTrait>>,
    ) -> Self {
        DefaultTripGateway {
            repository,
            event_service,
            domain_service,
        }
    }
}

impl TripGatewayTrait for DefaultTripGateway {
    fn find_all(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Trip>, AppError>> + Send + '_>>
    {
        Box::pin(async move { self.repository.find_all().await })
    }

    fn insert<'a>(
        &'a self,
        trip: &'a Trip,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, AppError>> + Send + '_>>
    {
        Box::pin(async move {
            let result = self.repository.insert(trip).await;
            match result {
                Ok(id) => {
                    // trip.handle(|event| self.event_service.send_cloud_event(&event.event));
                    trip.handle(|event| self.domain_service.handle(event));
                    Ok(id)
                }
                Err(e) => Err(e),
            }
        })
    }

    fn insert_with_participant<'a>(
        &'a self,
        trip: &'a Trip,
        participant: &'a Participant,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, AppError>> + Send + '_>>
    {
        Box::pin(async move {
            let result = self
                .repository
                .insert_with_participant(trip, participant)
                .await;
            match result {
                Ok(id) => {
                    trip.handle(|event| self.domain_service.handle(event));
                    Ok(id)
                }
                Err(e) => Err(e),
            }
        })
    }
    fn update(
        &self,
        trip: Trip,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), AppError>> + Send + '_>>
    {
        Box::pin(async move {
            let result = self.repository.update(&trip).await;
            match result {
                Ok(_) => {
                    trip.handle(|event| self.domain_service.handle(event));
                    Ok(())
                }
                Err(e) => Err(e),
            }
        })
    }

    fn delete(
        &self,
        id: Uuid,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), AppError>> + Send + '_>>
    {
        Box::pin(async move { self.repository.delete(&id.to_string()).await })
    }

    fn find_by_id(&self, id: Uuid) -> Ret {
        Box::pin(async move { self.repository.find_by_id(&id.to_string()).await })
    }
}

type Ret<'a> = Pin<Box<dyn Future<Output = Result<Trip, AppError>> + Send + 'a>>;
