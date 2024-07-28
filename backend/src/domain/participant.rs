use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct Participant {
    id: Uuid,
    name: Option<String>,
    email: String,
    is_confirmed: bool,
    is_owner: bool,
    trip_id: Uuid,
}

impl Participant {
    pub fn new(
        name: Option<String>,
        email: String,
        is_confirmed: bool,
        is_owner: bool,
        trip_id: Uuid,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            name,
            email,
            is_confirmed,
            is_owner,
            trip_id,
        }
    }

    pub fn with(
        id: Uuid,
        name: Option<String>,
        email: &str,
        is_confirmed: bool,
        is_owner: bool,
        trip_id: Uuid,
    ) -> Self {
        Self {
            id,
            name: name.map(|n| n.to_owned()),
            email: email.to_owned(),
            is_confirmed,
            is_owner,
            trip_id,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn is_confirmed(&self) -> bool {
        self.is_confirmed
    }

    pub fn is_owner(&self) -> bool {
        self.is_owner
    }

    pub fn trip_id(&self) -> Uuid {
        self.trip_id
    }

    pub fn confirm(&mut self) {
        self.is_confirmed = true;
    }
}
