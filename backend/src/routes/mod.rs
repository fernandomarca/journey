mod confirm_participants;
mod confirm_trip;
mod create_activity;
mod create_invite;
mod create_link;
mod create_trip;
mod get_activities;
mod get_links;
mod get_participant;
mod get_participants;
mod get_trip_details;
mod list_trips;
pub mod routes_config;
mod update_trip;

pub use confirm_participants::confirm_participants;
pub use confirm_trip::confirm_trip;
pub use create_activity::create_activity;
pub use create_invite::create_invite;
pub use create_link::create_link;
pub use create_trip::create_trip;
pub use get_activities::get_activities;
pub use get_links::get_links;
pub use get_participant::get_participant;
pub use get_participants::get_participants;
pub use get_trip_details::get_trip_details;
pub use list_trips::list_trips;
pub use update_trip::update_trip;

use super::libs::PrismaClient;
use axum::Extension;
use axum::Json;
use std::sync::Arc;

pub type Database = Extension<Arc<PrismaClient>>;
pub type AppResult<T> = Result<T, String>;
pub type AppJsonResult<T> = AppResult<Json<T>>;
