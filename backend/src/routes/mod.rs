mod confirm_trip;
mod create_trip;
mod list_trips;
mod trip_routes;

pub use confirm_trip::confirm_trip;
pub use create_trip::create_trip;
pub use list_trips::list_trips;

use super::libs::PrismaClient;
use axum::Extension;
use axum::Json;
use std::sync::Arc;
pub use trip_routes::trip_routes;

pub type Database = Extension<Arc<PrismaClient>>;
pub type AppResult<T> = Result<T, String>;
pub type AppJsonResult<T> = AppResult<Json<T>>;
