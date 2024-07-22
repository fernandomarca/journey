mod application;
mod domain;
mod infra;
mod libs;
mod routes;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Router};
use infra::modules::Modules;
use libs::prisma;
use prisma_client_rust::prisma_errors::query_engine::{RecordNotFound, UniqueKeyViolation};
use prisma_client_rust::QueryError;
use routes::routes_config;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::debug;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("debug"))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let prisma_client = prisma().await;
    let modules = Arc::new(Modules::new().await);

    let app = Router::new()
        .nest("/trips", routes_config::trip_routes())
        .nest("/participants", routes_config::participants_routes())
        .layer(Extension(prisma_client))
        .layer(Extension(modules))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await.unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

enum AppError {
    PrismaError(QueryError),
    NotFound,
    ClientError(String),
    InternalServerError,
}

impl From<QueryError> for AppError {
    fn from(error: QueryError) -> Self {
        match error {
            e if e.is_prisma_error::<RecordNotFound>() => AppError::NotFound,
            e => AppError::PrismaError(e),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::PrismaError(error) if error.is_prisma_error::<UniqueKeyViolation>() => {
                StatusCode::CONFLICT
            }
            AppError::PrismaError(e) => {
                debug!("Prisma error: {:?}", e);
                StatusCode::BAD_REQUEST
            }
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::ClientError(m) => {
                debug!("ClientError: {:?}", m);
                StatusCode::BAD_REQUEST
            }
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        status.into_response()
    }
}
