use super::services::domain_service::DomainService;
use super::services::in_memory_service::InMemoryService;
use super::trip::default_trip_gateway::DefaultTripGateway;
use super::trip::trip_repository;
use super::trip::trip_repository::TripRepository;
use crate::application::trip_service::TripService;
use crate::domain::trip_gateway_trait::TripGatewayTrait;
use crate::libs::prisma;
use serde::de;

pub struct Modules {
    pub trip_service_config: TripServiceConfig,
}

impl Modules {
    pub async fn new() -> Self {
        let prisma = prisma().await;

        let trip_repository = TripRepository::new(prisma);

        let event_service = Box::new(InMemoryService::new());
        let domain_service = Box::new(DomainService::new());

        let trip_gateway = Box::new(DefaultTripGateway::new(
            trip_repository,
            event_service,
            domain_service,
        ));

        let trip_service_config = TripServiceConfig::new(trip_gateway);

        Self {
            trip_service_config,
        }
    }
}

pub struct TripServiceConfig {
    trip_gateway: Box<dyn TripGatewayTrait>,
}

impl TripServiceConfig {
    pub fn new(trip_gateway: Box<dyn TripGatewayTrait>) -> Self {
        TripServiceConfig { trip_gateway }
    }

    pub fn service(&self) -> TripService {
        TripService::new(self.trip_gateway.as_ref())
    }
}
