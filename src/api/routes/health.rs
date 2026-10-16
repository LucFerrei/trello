use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};

use serde::{Deserialize, Serialize};

pub fn health_route() -> Router {
    Router::new()
        .route("/", get(check_health))
}

async fn check_health() -> &'static str {
    "Healthy"
}


