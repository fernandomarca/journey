use crate::domain::participant::Participant;
use crate::domain::participant_gateway_trait::ParticipantGatewayTrait;
use crate::AppError;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct ParticipantService {
    participant_gateway: Arc<Box<dyn ParticipantGatewayTrait>>,
}

impl ParticipantService {
    pub fn new(participant_gateway: Arc<Box<dyn ParticipantGatewayTrait>>) -> Arc<Self> {
        Arc::new(ParticipantService {
            participant_gateway,
        })
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

    pub async fn find_participants_by_trip_id(
        &self,
        trip_id: &str,
    ) -> Result<Vec<Participant>, AppError> {
        self.participant_gateway
            .find_participants_by_trip_id(trip_id)
            .await
    }

    pub async fn confirm_participant(&self, participant_id: &str) -> Result<(), AppError> {
        let mut participant = self.participant_gateway.find_by_id(participant_id).await?;

        if participant.is_confirmed() {
            return Ok(());
        }

        participant.confirm();
        self.participant_gateway.update(participant).await
    }

    pub async fn find_by_id(&self, participant_id: &str) -> Result<Participant, AppError> {
        self.participant_gateway.find_by_id(participant_id).await
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
