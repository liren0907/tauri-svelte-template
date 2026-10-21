use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/healthz", get(healthz))
}

/// Liveness probe for load balancers, container orchestrators and the UI's
/// connection indicator.
async fn healthz() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
