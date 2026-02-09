use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;

use crate::{handlers, AppState};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/analyze", post(handlers::analyze_transaction))
        .route("/api/report/:id", get(handlers::get_report))
        .with_state(state)
}

async fn health_check() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "service": "aletta-backend",
            "version": env!("CARGO_PKG_VERSION")
        })),
    )
}