use super::confirm_trip;
use super::create_trip;
use super::list_trips;
use axum::routing::get;
use axum::routing::post;
use axum::Router;

pub fn trip_routes() -> Router {
    Router::new()
        .route("/:trip_id/confirm", get(confirm_trip))
        .route("/", get(list_trips))
        .route("/", post(create_trip))
}
