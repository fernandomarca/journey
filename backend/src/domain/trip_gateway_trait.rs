#![allow(dead_code)]

use super::participant::Participant;
use super::trip::Trip;
use crate::AppError;
use std::pin::Pin;
use uuid::Uuid;

pub trait TripGatewayTrait: Send + Sync {
    fn find_all(
        &self,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<Vec<Trip>, AppError>> + Send + '_>>;
    fn find_by_id(
        &self,
        id: Uuid,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<Trip, AppError>> + Send + '_>>;
    fn insert<'a>(
        &'a self,
        trip: &'a Trip,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<String, AppError>> + Send + '_>>;

    fn insert_with_participant<'a>(
        &'a self,
        trip: &'a Trip,
        participant: &'a Participant,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, AppError>> + Send + '_>>;
    fn update(
        &self,
        trip: Trip,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), AppError>> + Send + '_>>;
    fn delete(
        &self,
        id: Uuid,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), AppError>> + Send + '_>>;
}
