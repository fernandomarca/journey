use super::confirm_participants;
use super::confirm_trip;
use super::create_activity;
use super::create_link;
use super::create_trip;
use super::get_activities;
use super::get_links;
use super::list_trips;
use axum::routing::get;
use axum::routing::post;
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
}

pub fn participants_routes() -> Router {
    Router::new().route("/:participant_id/confirm", get(confirm_participants))
}
