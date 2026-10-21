import { request } from './client.js';
import type { Note, NoteInput } from './types.js';

/** Typed wrappers around the `/api/notes` endpoints. */
export const notesApi = {
	list: () => request<Note[]>('/api/notes'),

	get: (id: string) => request<Note>(`/api/notes/${id}`),

	create: (input: NoteInput) =>
		request<Note>('/api/notes', { method: 'POST', body: JSON.stringify(input) }),

	update: (id: string, input: NoteInput) =>
		request<Note>(`/api/notes/${id}`, { method: 'PUT', body: JSON.stringify(input) }),

	remove: (id: string) => request<void>(`/api/notes/${id}`, { method: 'DELETE' })
};
