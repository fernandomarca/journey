use super::participant::default_participant_gateway::DefaultParticipantGateway;
use super::participant::participant_repository::ParticipantRepository;
use super::services::domain_service::DomainService;
use super::services::in_memory_service::InMemoryService;
use super::trip::default_trip_gateway::DefaultTripGateway;
use super::trip::trip_repository::TripRepository;
use crate::application::participant_service::ParticipantService;
use crate::application::trip_service::TripService;
use crate::domain::event_service_traits::DomainEventServiceTrait;
use crate::domain::event_service_traits::EventServiceTrait;
use crate::domain::handlers::send_confirmation_trip::SendConfirmationTripHandler;
use crate::domain::participant_gateway_trait::ParticipantGatewayTrait;
use crate::domain::trip_gateway_trait::TripGatewayTrait;
use crate::libs::prisma;
use crate::libs::PrismaClient;
use std::sync::Arc;

pub struct Modules {
    pub trip_service: TripService,
    pub participant_service: ParticipantService,
}

impl Modules {
    pub async fn new() -> Self {
        let prisma = prisma().await;
        // domains events
        let event_service = Box::new(InMemoryService::new());
        let domain_service = domain_service();
        // gateways
        let trip_gateway = trip_gateway(prisma.clone(), event_service, domain_service.clone());
        let participant_gateway = participant_gateway(prisma.clone());
        // services
        let trip_service = TripService::new(
            trip_gateway.clone(),
            participant_gateway.clone(),
            domain_service.clone(),
        );
        let participant_service = ParticipantService::new(participant_gateway.clone());

        Self {
            trip_service,
            participant_service,
        }
    }
}

fn domain_service() -> Arc<Box<dyn DomainEventServiceTrait>> {
    let mut domain_service = Box::new(DomainService::new());
    let send_confirmation_trip_handler = SendConfirmationTripHandler::new();
    domain_service.add_listener(Box::new(send_confirmation_trip_handler));
    Arc::new(domain_service)
}
fn trip_gateway(
    prisma: Arc<PrismaClient>,
    event_service: Box<dyn EventServiceTrait>,
    domain_service: Arc<Box<dyn DomainEventServiceTrait>>,
) -> Arc<Box<dyn TripGatewayTrait>> {
    let trip_repository = TripRepository::new(prisma);

    Arc::new(Box::new(DefaultTripGateway::new(
        trip_repository,
        event_service,
        domain_service,
    )))
}

fn participant_gateway(prisma: Arc<PrismaClient>) -> Arc<Box<dyn ParticipantGatewayTrait>> {
    let participant_repository = ParticipantRepository::new(prisma);
    Arc::new(Box::new(DefaultParticipantGateway::new(
        participant_repository,
    )))
}
