use axum::{
    Json, Router,
    extract::{
        Path, State,
        rejection::{JsonRejection, PathRejection},
    },
    http::StatusCode,
    routing::get,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{AppState, error::AppError, state::Note};

const MAX_TITLE_CHARS: usize = 200;

/// Request body for creating or replacing a note.
#[derive(Debug, Deserialize)]
pub struct NoteInput {
    pub title: String,
    #[serde(default)]
    pub content: String,
}

impl NoteInput {
    /// Trim the title and check it, returning the cleaned `(title, content)`.
    fn validate(self) -> Result<(String, String), AppError> {
        let title = self.title.trim().to_string();
        if title.is_empty() {
            return Err(AppError::Validation("title must not be empty".to_string()));
        }
        if title.chars().count() > MAX_TITLE_CHARS {
            return Err(AppError::Validation(format!(
                "title must be at most {MAX_TITLE_CHARS} characters"
            )));
        }
        Ok((title, self.content))
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_notes).post(create_note))
        .route("/{id}", get(get_note).put(update_note).delete(delete_note))
}

async fn list_notes(State(state): State<AppState>) -> Json<Vec<Note>> {
    Json(state.list_notes().await)
}

async fn create_note(
    State(state): State<AppState>,
    payload: Result<Json<NoteInput>, JsonRejection>,
) -> Result<(StatusCode, Json<Note>), AppError> {
    let Json(input) = payload?;
    let (title, content) = input.validate()?;
    let note = state.create_note(title, content).await;
    Ok((StatusCode::CREATED, Json(note)))
}

async fn get_note(
    State(state): State<AppState>,
    id: Result<Path<Uuid>, PathRejection>,
) -> Result<Json<Note>, AppError> {
    let Path(id) = id?;
    state
        .get_note(id)
        .await
        .map(Json)
        .ok_or_else(|| note_not_found(id))
}

async fn update_note(
    State(state): State<AppState>,
    id: Result<Path<Uuid>, PathRejection>,
    payload: Result<Json<NoteInput>, JsonRejection>,
) -> Result<Json<Note>, AppError> {
    let Path(id) = id?;
    let Json(input) = payload?;
    let (title, content) = input.validate()?;
    state
        .update_note(id, title, content)
        .await
        .map(Json)
        .ok_or_else(|| note_not_found(id))
}

async fn delete_note(
    State(state): State<AppState>,
    id: Result<Path<Uuid>, PathRejection>,
) -> Result<StatusCode, AppError> {
    let Path(id) = id?;
    if state.delete_note(id).await {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(note_not_found(id))
    }
}

fn note_not_found(id: Uuid) -> AppError {
    AppError::NotFound(format!("note {id} not found"))
}
