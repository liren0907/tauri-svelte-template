// Shapes of the JSON exchanged with the Axum backend.
// Keep in sync with backend/src/state.rs and backend/src/routes/notes.rs.

export interface Note {
	id: string;
	title: string;
	content: string;
	/** RFC 3339 timestamp */
	createdAt: string;
	/** RFC 3339 timestamp */
	updatedAt: string;
}

export interface NoteInput {
	title: string;
	content: string;
}

/** Every API error has this body (see backend/src/error.rs). */
export interface ApiErrorBody {
	code: string;
	message: string;
}
