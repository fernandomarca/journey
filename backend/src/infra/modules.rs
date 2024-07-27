use super::participant;
use super::participant::default_participant_gateway::DefaultParticipantGateway;
use super::participant::participant_repository::ParticipantRepository;
use super::services::domain_service::DomainService;
use super::services::in_memory_service::InMemoryService;
use super::trip::default_trip_gateway::DefaultTripGateway;
use super::trip::trip_repository::TripRepository;
use crate::application::participant_service::ParticipantService;
use crate::application::trip_service::TripService;
use crate::domain::handlers::invite_trip_participants::InviteTripParticipantsHandler;
use crate::domain::participant_gateway_trait::ParticipantGatewayTrait;
use crate::domain::trip_gateway_trait::TripGatewayTrait;
use crate::libs::prisma;
use crate::libs::PrismaClient;
use std::sync::Arc;

pub struct Modules {
    pub trip_service_config: TripServiceConfig,
    pub participant_service_config: ParticipantServiceConfig,
}

impl Modules {
    pub async fn new() -> Self {
        let prisma = prisma().await;
        //
        let participant_gateway = Arc::new(participant_gateway(prisma.clone()));

        let participant_service_config = ParticipantServiceConfig::new(participant_gateway.clone());
        //
        let trip_gateway = trip_gateway(prisma.clone());
        let trip_service_config = TripServiceConfig::new(trip_gateway, participant_gateway.clone());

        Self {
            trip_service_config,
            participant_service_config,
        }
    }
}

pub struct TripServiceConfig {
    trip_gateway: Box<dyn TripGatewayTrait>,
    participant_gateway: Arc<Box<dyn ParticipantGatewayTrait>>,
}

impl TripServiceConfig {
    pub fn new(
        trip_gateway: Box<dyn TripGatewayTrait>,
        participant_gateway: Arc<Box<dyn ParticipantGatewayTrait>>,
    ) -> Self {
        TripServiceConfig {
            trip_gateway,
            participant_gateway,
        }
    }

    pub fn service(&self) -> TripService {
        TripService::new(self.trip_gateway.as_ref(), self.participant_gateway.clone())
    }
}

#[derive(Clone)]
pub struct ParticipantServiceConfig {
    participant_gateway: Arc<Box<dyn ParticipantGatewayTrait>>,
}

impl ParticipantServiceConfig {
    pub fn new(participant_gateway: Arc<Box<dyn ParticipantGatewayTrait>>) -> Self {
        ParticipantServiceConfig {
            participant_gateway,
        }
    }

    pub fn service(&self) -> ParticipantService {
        ParticipantService::new(self.participant_gateway.clone())
    }
}

fn trip_gateway(prisma: Arc<PrismaClient>) -> Box<dyn TripGatewayTrait> {
    let trip_repository = TripRepository::new(prisma);

    let event_service = Box::new(InMemoryService::new());
    let mut domain_service = Box::new(DomainService::new());

    let invite_trip_participants_handler =
        InviteTripParticipantsHandler::new(trip_repository.clone());

    domain_service.add_listener(Box::new(invite_trip_participants_handler));

    Box::new(DefaultTripGateway::new(
        trip_repository,
        event_service,
        domain_service,
    ))
}

fn participant_gateway(prisma: Arc<PrismaClient>) -> Box<dyn ParticipantGatewayTrait> {
    let participant_repository = ParticipantRepository::new(prisma);
    Box::new(DefaultParticipantGateway::new(participant_repository))
}
