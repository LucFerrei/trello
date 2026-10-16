mod routes;

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};

use serde::{Deserialize, Serialize};

use routes::health::health_route;

pub fn create_app() -> Router {
    Router::new()
        .nest("/health", health_route())
}
