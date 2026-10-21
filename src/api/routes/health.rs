use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};

use serde::{Deserialize, Serialize};

pub fn health_route() -> Router {
    Router::new().route("/", get(check_health))
}

async fn check_health() -> &'static str {
    "Healthy"
}
