use crate::domain::participant::Participant;
use crate::libs::prisma;
use uuid::Uuid;

impl From<prisma::participant::Data> for Participant {
    fn from(prisma_participant: prisma::participant::Data) -> Self {
        Participant::with(
            Uuid::parse_str(&prisma_participant.id).unwrap(),
            prisma_participant.name,
            prisma_participant.email,
            prisma_participant.is_confirmed,
            prisma_participant.is_owner,
            Uuid::parse_str(&prisma_participant.trip_id).unwrap(),
        )
    }
}
