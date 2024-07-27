use crate::domain::participant::Participant;
use crate::domain::participant_gateway_trait::ParticipantGatewayTrait;
use crate::AppError;
use std::sync::Arc;
use uuid::Uuid;

pub struct ParticipantService {
    participant_gateway: Arc<Box<dyn ParticipantGatewayTrait>>,
}

impl ParticipantService {
    pub fn new(participant_gateway: Arc<Box<dyn ParticipantGatewayTrait>>) -> Self {
        ParticipantService {
            participant_gateway,
        }
    }

    pub async fn insert(
        &self,
        create_participant_command: CreateParticipantCommand,
    ) -> Result<String, AppError> {
        let participant = Participant::new(
            create_participant_command.name,
            create_participant_command.email,
            create_participant_command.is_confirmed,
            create_participant_command.is_owner,
            create_participant_command.trip_id,
        );
        // criar participant and participant owner
        self.participant_gateway.insert(participant).await
    }
}

#[derive(Debug, Clone)]
pub struct CreateParticipantCommand {
    name: Option<String>,
    email: String,
    is_confirmed: bool,
    is_owner: bool,
    trip_id: Uuid,
}

impl CreateParticipantCommand {
    pub fn new(
        name: Option<String>,
        email: String,
        is_confirmed: bool,
        is_owner: bool,
        trip_id: Uuid,
    ) -> Self {
        Self {
            name,
            email,
            is_confirmed,
            is_owner,
            trip_id,
        }
    }
}
