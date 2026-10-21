//! Notes API — the Axum service behind the `web-axum-shadcn` template.
//!
//! `main.rs` only reads configuration and starts the server. Everything else
//! lives in this library so integration tests can drive the exact same router
//! without opening a network port.

pub mod config;
pub mod error;
pub mod routes;
pub mod state;

use axum::{
    Router,
    http::{HeaderValue, Method, header},
};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

pub use config::Config;
pub use state::AppState;

/// Build the application router.
///
/// CORS is applied separately with [`cors_layer`] because the allowed origins
/// come from runtime configuration.
pub fn app(state: AppState) -> Router {
    Router::new()
        .nest("/api/notes", routes::notes::router())
        .fallback(routes::not_found)
        .layer(
            // One INFO line per request: method, path, status and latency.
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        // Merged after the trace layer so frequent health probes don't flood the logs.
        .merge(routes::health::router())
        .with_state(state)
}

/// Allow browser clients served from `allowed_origins` to call the API.
///
/// Only needed when the frontend is served from a different origin than the
/// API (for example a desktop build, or a frontend hosted on a CDN). During
/// development the Vite dev server proxies `/api`, so requests are same-origin.
pub fn cors_layer(allowed_origins: &[String]) -> CorsLayer {
    let origins: Vec<HeaderValue> = allowed_origins
        .iter()
        .filter_map(|origin| match origin.parse() {
            Ok(value) => Some(value),
            Err(_) => {
                tracing::warn!(%origin, "ignoring invalid CORS origin");
                None
            }
        })
        .collect();

    CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE])
}
