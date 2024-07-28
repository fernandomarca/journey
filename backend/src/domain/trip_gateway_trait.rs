#![allow(dead_code)]

use super::trip::Trip;
use crate::libs::PrismaClient;
use crate::AppError;
use std::pin::Pin;
use uuid::Uuid;

pub trait TripGatewayTrait: Send + Sync {
    fn find_all(
        &self,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<Vec<Trip>, String>> + Send + '_>>;
    fn find_by_id(
        &self,
        id: Uuid,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<Option<Trip>, String>> + Send + '_>>;
    fn insert<'a>(
        &'a self,
        trip: &'a Trip,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<String, AppError>> + Send + '_>>;
    fn update(
        &self,
        trip: Trip,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), String>> + Send + '_>>;
    fn delete(
        &self,
        id: Uuid,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), String>> + Send + '_>>;

    fn get_transaction(&self) -> &PrismaClient;
}
