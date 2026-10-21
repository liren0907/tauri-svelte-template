use std::net::{IpAddr, SocketAddr};

/// Origins allowed by default: the Vite dev server and the Tauri desktop
/// webview (`tauri://localhost` on macOS/Linux, `http://tauri.localhost` on Windows).
const DEFAULT_CORS_ALLOWED_ORIGINS: &str =
    "http://localhost:1430,tauri://localhost,http://tauri.localhost";

/// Runtime settings, read from environment variables so the same binary runs
/// unchanged on a laptop, in a container, or on a cloud platform.
#[derive(Debug, Clone)]
pub struct Config {
    /// `HOST` — address to bind (default `127.0.0.1`; use `0.0.0.0` in containers).
    pub host: IpAddr,
    /// `PORT` — port to listen on (default `8080`).
    pub port: u16,
    /// `CORS_ALLOWED_ORIGINS` — comma-separated list of allowed browser origins.
    pub cors_allowed_origins: Vec<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let host = env_or("HOST", "127.0.0.1")
            .parse()
            .map_err(|e| format!("invalid HOST: {e}"))?;
        let port = env_or("PORT", "8080")
            .parse()
            .map_err(|e| format!("invalid PORT: {e}"))?;
        let cors_allowed_origins = env_or("CORS_ALLOWED_ORIGINS", DEFAULT_CORS_ALLOWED_ORIGINS)
            .split(',')
            .map(str::trim)
            .filter(|origin| !origin.is_empty())
            .map(String::from)
            .collect();

        Ok(Self {
            host,
            port,
            cors_allowed_origins,
        })
    }

    pub fn addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}
