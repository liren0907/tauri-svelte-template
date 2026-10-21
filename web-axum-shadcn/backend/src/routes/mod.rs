pub mod health;
pub mod notes;

use crate::error::AppError;

/// Fallback for unknown routes, so even a 404 uses the JSON error shape.
pub async fn not_found() -> AppError {
    AppError::NotFound("route not found".to_string())
}
