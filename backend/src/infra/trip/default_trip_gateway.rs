use super::trip_repository::TripRepository;
use crate::domain::entity::Entity;
use crate::domain::trip::Trip;
use crate::domain::trip_gateway_trait::TripGatewayTrait;
use crate::infra::services::event_service_trait::EventServiceTrait;
use crate::infra::services::in_memory_service::InMemoryService;
use crate::AppError;
use uuid::Uuid;

pub struct DefaultTripGateway {
    repository: TripRepository,
    event_service: Box<dyn EventServiceTrait>,
}

impl DefaultTripGateway {
    pub fn new(repository: TripRepository, event_service: Box<dyn EventServiceTrait>) -> Self {
        DefaultTripGateway {
            repository,
            event_service,
        }
    }
}

impl TripGatewayTrait for DefaultTripGateway {
    fn find_all(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Trip>, String>> + Send>>
    {
        todo!()
    }

    fn insert(
        &self,
        trip: Trip,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, AppError>> + Send + '_>>
    {
        Box::pin(async move {
            let result = self.repository.insert(&trip).await;
            match result {
                Ok(id) => {
                    trip.handle(|event| self.event_service.send_cloud_event(&event.event));
                    Ok(id)
                }
                Err(e) => Err(AppError::from(e)),
            }
        })
    }

    fn update(
        &self,
        trip: Trip,
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
        Box<dyn std::future::Future<Output = Result<Option<Trip>, String>> + Send + '_>,
    > {
        todo!()
    }
}
