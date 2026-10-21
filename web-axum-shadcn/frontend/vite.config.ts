import tailwindcss from '@tailwindcss/vite';
import adapter from '@sveltejs/adapter-static';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

// Where the Axum backend listens during development (see ../backend).
const API_TARGET = 'http://localhost:8080';

export default defineConfig({
	plugins: [
		tailwindcss(),
		sveltekit({
			compilerOptions: {
				// Force runes mode for the project, except for libraries. Can be removed in svelte 6.
				runes: ({ filename }) =>
					filename.split(/[/\\]/).includes('node_modules') ? undefined : true
			},
			// Pure SPA: no server rendering, unknown paths fall back to index.html.
			// The same static output is served by nginx (Docker) and by Tauri (desktop).
			adapter: adapter({ fallback: 'index.html' })
		})
	],

	// Keep Rust compiler output visible when running under `tauri dev`.
	clearScreen: false,
	server: {
		// Tauri's devUrl points at this port, so it must not move.
		port: 1430,
		strictPort: true,
		// The browser (or Tauri webview) calls /api on the Vite server, which
		// forwards to the backend. Requests stay same-origin, so no CORS in dev.
		proxy: {
			'/api': API_TARGET,
			'/healthz': API_TARGET
		}
	}
});
