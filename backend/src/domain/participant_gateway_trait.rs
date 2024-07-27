#![allow(dead_code)]

use super::participant::Participant;
use crate::AppError;
use std::pin::Pin;
use uuid::Uuid;

pub trait ParticipantGatewayTrait: Send + Sync {
    fn find_all(
        &self,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<Vec<Participant>, String>> + Send + '_>>;
    fn find_by_id(
        &self,
        id: Uuid,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<Option<Participant>, String>> + Send + '_>>;
    fn insert(
        &self,
        participant: Participant,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<String, AppError>> + Send + '_>>;
    fn update(
        &self,
        participant: Participant,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), String>> + Send + '_>>;
    fn delete(
        &self,
        id: Uuid,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), String>> + Send + '_>>;
}
