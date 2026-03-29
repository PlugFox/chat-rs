//! Axum router and middleware setup.

use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::routing::get;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use crate::state::AppState;

/// Build the axum router with all routes and middleware.
#[allow(deprecated)] // TimeoutLayer::new — no http crate dep yet
pub fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ws", get(crate::ws::upgrade_handler))
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}
