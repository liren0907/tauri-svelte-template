use axum::{
    Json,
    extract::rejection::{JsonRejection, PathRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

/// Every error leaves the API as `{ "code": "...", "message": "..." }`, so
/// clients can branch on the stable `code` instead of parsing `message`.
#[derive(Debug)]
pub enum AppError {
    /// 404 — the resource or route does not exist.
    NotFound(String),
    /// 422 — the request was well-formed but its data is invalid.
    Validation(String),
    /// 400 — the request could not be understood (malformed JSON, bad id, ...).
    BadRequest(String),
}

#[derive(Serialize)]
struct ErrorBody<'a> {
    code: &'a str,
    message: &'a str,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::NotFound(message) => (StatusCode::NOT_FOUND, "not_found", message),
            AppError::Validation(message) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "validation_error",
                message,
            ),
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, "bad_request", message),
        };
        (status, Json(ErrorBody { code, message })).into_response()
    }
}

/// Turn Axum's JSON extractor errors into our error shape. A body that parses
/// as JSON but has the wrong fields is a validation error; anything else is a
/// bad request.
impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        match rejection {
            JsonRejection::JsonDataError(e) => AppError::Validation(e.body_text()),
            other => AppError::BadRequest(other.body_text()),
        }
    }
}

impl From<PathRejection> for AppError {
    fn from(rejection: PathRejection) -> Self {
        AppError::BadRequest(rejection.body_text())
    }
}
