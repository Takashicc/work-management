use axum::{routing::get, Router};

use super::handlers;

pub fn create_router() -> Router {
    Router::new().route("/health", get(handlers::health_check))
}
