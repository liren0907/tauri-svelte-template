import type { ApiErrorBody } from './types.js';

/**
 * Base URL of the Axum API.
 *
 * Empty (the default) means "same origin": the Vite dev server and the nginx
 * container both forward `/api` to the backend. Set `VITE_API_BASE_URL` at
 * build time when the API lives elsewhere, e.g. for a desktop build that talks
 * to a deployed backend.
 */
const API_BASE_URL = (import.meta.env.VITE_API_BASE_URL ?? '').replace(/\/$/, '');

/** An API call that failed. `code` comes from the backend error body. */
export class ApiError extends Error {
	readonly status: number;
	readonly code: string;

	constructor(status: number, code: string, message: string) {
		super(message);
		this.name = 'ApiError';
		this.status = status;
		this.code = code;
	}
}

/** Send a request to the API and decode the JSON response. */
export async function request<T>(path: string, init: RequestInit = {}): Promise<T> {
	const headers = new Headers(init.headers);
	if (init.body !== undefined) headers.set('content-type', 'application/json');

	let response: Response;
	try {
		response = await fetch(`${API_BASE_URL}${path}`, { ...init, headers });
	} catch {
		throw new ApiError(0, 'network_error', 'Cannot reach the backend. Is it running?');
	}

	if (!response.ok) {
		const body = (await response.json().catch(() => null)) as ApiErrorBody | null;
		throw new ApiError(
			response.status,
			body?.code ?? 'http_error',
			body?.message ?? `${response.status} ${response.statusText}`
		);
	}

	if (response.status === 204) return undefined as T;
	return (await response.json()) as T;
}
