use super::confirm_participants;
use super::confirm_trip;
use super::create_activity;
use super::create_invite;
use super::create_link;
use super::create_trip;
use super::get_activities;
use super::get_links;
use super::get_participant;
use super::get_participants;
use super::get_trip_details;
use super::list_trips;
use super::update_trip;
use axum::routing::get;
use axum::routing::post;
use axum::routing::put;
use axum::Router;

pub fn trip_routes() -> Router {
    Router::new()
        .route("/:trip_id/confirm", get(confirm_trip))
        .route("/", get(list_trips))
        .route("/", post(create_trip))
        .route("/:trip_id/activities", post(create_activity))
        .route("/:trip_id/activities", get(get_activities))
        .route("/:trip_id/links", post(create_link))
        .route("/:trip_id/links", get(get_links))
        .route("/:trip_id/participants", get(get_participants))
        .route("/:trip_id/invites", post(create_invite))
        .route("/:trip_id", put(update_trip))
        .route("/:trip_id", get(get_trip_details))
}

pub fn participants_routes() -> Router {
    Router::new()
        .route("/:participant_id/confirm", get(confirm_participants))
        .route("/:participant_id", get(get_participant))
}
