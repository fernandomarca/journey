mod confirm_participants;
mod confirm_trip;
mod create_activity;
mod create_link;
mod create_trip;
mod get_activities;
mod get_links;
mod list_trips;
pub mod routes_config;

pub use confirm_participants::confirm_participants;
pub use confirm_trip::confirm_trip;
pub use create_activity::create_activity;
pub use create_link::create_link;
pub use create_trip::create_trip;
pub use get_activities::get_activities;
pub use get_links::get_links;
pub use list_trips::list_trips;

use super::libs::PrismaClient;
use axum::Extension;
use axum::Json;
use std::sync::Arc;

pub type Database = Extension<Arc<PrismaClient>>;
pub type AppResult<T> = Result<T, String>;
pub type AppJsonResult<T> = AppResult<Json<T>>;
