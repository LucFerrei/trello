mod routes;

use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};

use serde::{Deserialize, Serialize};

use routes::health::health_route;

pub fn create_app() -> Router {
    Router::new().nest("/health", health_route())
}
