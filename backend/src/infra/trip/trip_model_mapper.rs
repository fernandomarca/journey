use crate::domain::trip::Trip;
use crate::libs::prisma;
use uuid::Uuid;

impl From<prisma::trip::Data> for Trip {
    fn from(prisma_trip: prisma::trip::Data) -> Self {
        Trip {
            id: Uuid::parse_str(&prisma_trip.id).unwrap(),
            destination: prisma_trip.destination,
            starts_at: prisma_trip.starts_at,
            ends_at: prisma_trip.ends_at,
            is_confirmed: prisma_trip.is_confirmed,
            created_at: prisma_trip.created_at,
            participants: prisma_trip
                .participants
                .map(|p| p.iter().map(|x| Uuid::parse_str(&x.id).unwrap()).collect()),
            activities: prisma_trip
                .activities
                .map(|a| a.iter().map(|x| Uuid::parse_str(&x.id).unwrap()).collect()),
            links: prisma_trip
                .links
                .map(|l| l.iter().map(|x| Uuid::parse_str(&x.id).unwrap()).collect()),
            domain_events: Vec::new(),
        }
    }
}
