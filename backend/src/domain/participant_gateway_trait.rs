#![allow(dead_code)]

use super::participant::Participant;
use crate::AppError;
use std::pin::Pin;
use uuid::Uuid;

pub trait ParticipantGatewayTrait: Send + Sync {
    fn find_all(
        &self,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<Vec<Participant>, AppError>> + Send + '_>>;
    fn find_by_id<'a>(
        &'a self,
        id: &'a str,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<Participant, AppError>> + Send + '_>>;
    fn insert(
        &self,
        participant: Participant,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<String, AppError>> + Send + '_>>;
    fn update(
        &self,
        participant: Participant,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), AppError>> + Send + '_>>;
    fn delete(
        &self,
        id: Uuid,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), AppError>> + Send + '_>>;

    fn find_participants_by_trip_id<'a>(
        &'a self,
        trip_id: &'a str,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Vec<Participant>, AppError>> + Send + '_>,
    >;
}
