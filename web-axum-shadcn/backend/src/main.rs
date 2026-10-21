use backend::{AppState, Config, app, cors_layer};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Load `backend/.env` if present (see `.env.example`); real environment
    // variables always win.
    dotenvy::dotenv().ok();

    // `RUST_LOG` overrides the default log level, e.g. `RUST_LOG=debug`.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let config = Config::from_env().unwrap_or_else(|e| {
        tracing::error!("configuration error: {e}");
        std::process::exit(1);
    });

    let app = app(AppState::with_sample_notes()).layer(cors_layer(&config.cors_allowed_origins));

    let listener = TcpListener::bind(config.addr()).await.unwrap_or_else(|e| {
        tracing::error!("failed to bind {}: {e}", config.addr());
        std::process::exit(1);
    });
    tracing::info!("listening on http://{}", config.addr());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
}

/// Resolve on Ctrl+C or SIGTERM (what Docker and Kubernetes send on stop), so
/// in-flight requests can finish before the process exits.
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen for Ctrl+C");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to listen for SIGTERM")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    tracing::info!("shutdown signal received, finishing in-flight requests");
}
