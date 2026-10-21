use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Shared application state, cloned into every request handler.
///
/// Notes are kept in memory and disappear when the process stops. To use a
/// real database, replace the body of the methods below — handlers only talk
/// to these methods, never to the map directly.
#[derive(Clone, Default)]
pub struct AppState {
    notes: Arc<RwLock<HashMap<Uuid, Note>>>,
}

impl AppState {
    /// State pre-filled with a few notes so a fresh install has something to show.
    pub fn with_sample_notes() -> Self {
        let now = Utc::now();
        let samples = [
            (
                "Welcome",
                "This note lives in the Axum backend. Edit or delete it from the UI.",
            ),
            (
                "How it works",
                "The SvelteKit frontend talks to the API over plain HTTP: GET, POST, PUT and DELETE on /api/notes.",
            ),
            (
                "Data is in memory",
                "Restarting the backend resets the list. Swap AppState for a database to keep notes.",
            ),
        ];

        let notes = samples
            .into_iter()
            .enumerate()
            .map(|(i, (title, content))| {
                let created_at = now - TimeDelta::minutes(i as i64);
                let note = Note {
                    id: Uuid::new_v4(),
                    title: title.to_string(),
                    content: content.to_string(),
                    created_at,
                    updated_at: created_at,
                };
                (note.id, note)
            })
            .collect();

        Self {
            notes: Arc::new(RwLock::new(notes)),
        }
    }

    /// All notes, newest first.
    pub async fn list_notes(&self) -> Vec<Note> {
        let mut notes: Vec<Note> = self.notes.read().await.values().cloned().collect();
        notes.sort_by_key(|note| std::cmp::Reverse(note.created_at));
        notes
    }

    pub async fn get_note(&self, id: Uuid) -> Option<Note> {
        self.notes.read().await.get(&id).cloned()
    }

    pub async fn create_note(&self, title: String, content: String) -> Note {
        let now = Utc::now();
        let note = Note {
            id: Uuid::new_v4(),
            title,
            content,
            created_at: now,
            updated_at: now,
        };
        self.notes.write().await.insert(note.id, note.clone());
        note
    }

    pub async fn update_note(&self, id: Uuid, title: String, content: String) -> Option<Note> {
        let mut notes = self.notes.write().await;
        let note = notes.get_mut(&id)?;
        note.title = title;
        note.content = content;
        note.updated_at = Utc::now();
        Some(note.clone())
    }

    /// Returns `true` if a note was removed.
    pub async fn delete_note(&self, id: Uuid) -> bool {
        self.notes.write().await.remove(&id).is_some()
    }
}
