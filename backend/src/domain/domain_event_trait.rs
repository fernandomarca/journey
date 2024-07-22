use chrono::Utc;
use cloudevents::Data;
use cloudevents::Event;
use cloudevents::EventBuilder;
use cloudevents::EventBuilderV10;
use serde::Serialize;
use serde_json::de;
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
                "json",
                "http://localhost:3333/schema",
                Data::Json(json!(event)),
            )
            .subject(event.get_subject())
            .time(Utc::now())
            .build()
            .expect("Failed to build event");
        DomainEvent { event }
    }
}
