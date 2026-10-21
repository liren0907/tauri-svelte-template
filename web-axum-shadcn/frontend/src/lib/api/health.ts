import { request } from './client.js';

/** `true` when the backend answers its health check. */
export async function isBackendHealthy(): Promise<boolean> {
	try {
		const body = await request<{ status: string }>('/healthz');
		return body.status === 'ok';
	} catch {
		return false;
	}
}
