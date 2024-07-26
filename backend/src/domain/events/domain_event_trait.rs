use chrono::Utc;
use cloudevents::Data;
use cloudevents::Event;
use cloudevents::EventBuilder;
use cloudevents::EventBuilderV10;
use serde::de::DeserializeOwned;
use serde::de::Error;
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

pub trait DomainEventTrait: Serialize + Send + Sync {
    fn get_subject(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct DomainEvent {
    pub event: Event,
}

impl DomainEvent {
    pub fn new(event: impl DomainEventTrait) -> Self {
        let event = EventBuilderV10::new()
            .id(Uuid::now_v7())
            .source("http://localhost:3333")
            .ty("example.demo")
            .data_with_schema(
                "binary",
                "http://localhost:3333/schema",
                // Data::Binary(serde_json::to_vec(&event).unwrap()),
                Data::Json(json!(event)),
            )
            .subject(event.get_subject())
            .time(Utc::now())
            .build()
            .expect("Failed to build event");
        DomainEvent { event }
    }

    pub fn to_struct<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        if let Some(data) = self.event.data() {
            match data {
                Data::Json(json_value) => {
                    return serde_json::from_value::<T>(json_value.clone());
                }
                Data::Binary(json_value) => {
                    return serde_json::from_slice::<T>(json_value);
                }
                Data::String(json_value) => {
                    return serde_json::from_str::<T>(json_value);
                }
            }
        }
        Err(serde_json::Error::custom("Invalid event data"))
    }
}
