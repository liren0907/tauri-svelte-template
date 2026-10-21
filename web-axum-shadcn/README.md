# web-axum-shadcn

A starter template where a **SvelteKit + shadcn-svelte** frontend talks to an **Axum** backend over plain **HTTP**. The same frontend runs in a desktop window (Tauri), in the browser, or in Docker — the backend is an independent service in every case.

The demo app is a small notes manager: list, create, edit and delete notes through a REST API, plus a page showcasing the UI components.

| Part | Stack |
|------|-------|
| Frontend | SvelteKit 3 (SPA, `adapter-static`) · Svelte 5 runes · Tailwind CSS 4 · shadcn-svelte · Lucide icons (`@lucide/svelte`) · mode-watcher (light/dark) |
| Desktop shell | Tauri 2 — a plain window, **no IPC**; the UI uses HTTP like in a browser |
| Backend | Axum 0.8 · Tokio · tower-http (CORS, request logs) · in-memory store |
| Containers | Multi-stage Dockerfiles · nginx serving the SPA and proxying `/api` · Docker Compose |

## Quick Start

Prerequisites: [Node.js](https://nodejs.org/) 22.17+, [Yarn 1](https://classic.yarnpkg.com/), [Rust](https://rustup.rs/) 1.90+, and the [Tauri system dependencies](https://v2.tauri.app/start/prerequisites/) for the desktop window. Docker is optional.

```bash
yarn install        # installs the frontend dependencies
yarn tauri dev      # desktop window + Axum backend + Vite, all in one command
```

Closing the window stops everything. Other ways to run it:

```bash
yarn dev                      # backend + Vite only — open http://localhost:1430 in a browser
yarn docker                   # production-like containers — open http://localhost:3000
```

## How It Fits Together

```
 Tauri window / browser                    Docker
          │                                   │
          ▼                                   ▼
 Vite dev server :1430                 nginx (frontend) :3000
 serves the SPA, proxies ─┐            serves the SPA, proxies ─┐
 /api and /healthz        │            /api and /healthz        │
                          ▼                                     ▼
                 Axum backend :8080                    Axum backend (backend:8080)
```

The browser always calls `/api/...` on the origin that served the page, so no CORS is involved in development or in Docker. CORS only matters when the frontend and API live on different origins — see [Desktop builds](#desktop-production-build).

## API

| Method & path | Description | Success |
|---------------|-------------|---------|
| `GET /healthz` | Liveness probe | `200 {"status":"ok"}` |
| `GET /api/notes` | List notes, newest first | `200 Note[]` |
| `POST /api/notes` | Create a note — body `{ "title", "content"? }` | `201 Note` |
| `GET /api/notes/{id}` | Fetch one note | `200 Note` |
| `PUT /api/notes/{id}` | Replace title and content | `200 Note` |
| `DELETE /api/notes/{id}` | Delete a note | `204` |

A `Note` is `{ id, title, content, createdAt, updatedAt }`. Every error has the body `{ "code": "...", "message": "..." }`:

| Status | `code` | When |
|--------|--------|------|
| 400 | `bad_request` | Malformed JSON or an invalid id |
| 404 | `not_found` | Unknown note or route |
| 422 | `validation_error` | Missing or blank title, or a title over 200 characters |

Notes live in memory and reset when the backend restarts. To persist them, replace the bodies of the methods on `AppState` in `backend/src/state.rs` — handlers never touch the storage directly.

## Configuration

Backend settings come from environment variables (copy `backend/.env.example` to `backend/.env` for local overrides):

| Variable | Default | Purpose |
|----------|---------|---------|
| `HOST` | `127.0.0.1` | Address to bind (`0.0.0.0` in the container) |
| `PORT` | `8080` | Port to listen on |
| `CORS_ALLOWED_ORIGINS` | `http://localhost:1430,tauri://localhost,http://tauri.localhost` | Browser origins allowed to call the API directly |
| `RUST_LOG` | `info` | Log filter, e.g. `debug` |

Frontend build setting:

| Variable | Default | Purpose |
|----------|---------|---------|
| `VITE_API_BASE_URL` | *(empty = same origin)* | Absolute API URL baked into the build, for when the API is on another origin |

## Desktop Production Build

A packaged desktop app has no Vite proxy, so point it at a running backend and make sure that backend allows the Tauri origin (the default `CORS_ALLOWED_ORIGINS` already does):

```bash
VITE_API_BASE_URL=https://api.example.com yarn tauri build
```

Change `identifier` in `backend/src-tauri/tauri.conf.json` before shipping.

## Commands

Run from this folder:

| Command | What it does |
|---------|--------------|
| `yarn install` | Install frontend dependencies |
| `yarn tauri dev` | Desktop window with backend and Vite |
| `yarn dev` | Backend and Vite for browser development |
| `yarn build` | Build the static SPA into `frontend/build` |
| `yarn typecheck` | Run `svelte-check` on the frontend |
| `yarn test` | Run the backend test suite |
| `yarn docker` | `docker compose -f docker/docker-compose.yaml up --build` |

Inside `frontend/`, use `yarn run check` for type checking — plain `yarn check` is Yarn 1's built-in integrity check, not the script.

## Project Structure

```
web-axum-shadcn/
├── package.json             # forwards commands to frontend/ and backend/; installs the Tauri CLI
├── docker/
│   └── docker-compose.yaml  # backend + frontend containers
├── backend/                 # Axum service (standalone Cargo project)
│   ├── src/
│   │   ├── main.rs          # config, logging, server, graceful shutdown
│   │   ├── lib.rs           # router and CORS — shared with the tests
│   │   ├── config.rs        # environment variables
│   │   ├── error.rs         # { code, message } error responses
│   │   ├── state.rs         # Note model and in-memory store
│   │   └── routes/          # health.rs, notes.rs
│   ├── tests/api.rs         # HTTP tests, no network port needed
│   ├── Dockerfile
│   └── src-tauri/           # Tauri desktop shell: its own crate, no commands
└── frontend/                # SvelteKit app
    ├── src/
    │   ├── routes/          # +layout.svelte, +page.svelte (notes), showcase/
    │   └── lib/
    │       ├── api/         # fetch wrapper, typed endpoints, API types
    │       ├── components/  # app sidebar, theme toggle, backend status
    │       │   └── ui/      # shadcn-svelte components (yours to edit)
    │       └── nav.ts       # sidebar entries
    ├── vite.config.ts       # SvelteKit config, dev port 1430, /api proxy
    ├── nginx.conf
    └── Dockerfile
```

## Extending

- **New API endpoint** — add a handler in `backend/src/routes/`, register it in that module's `router()`, and cover it in `backend/tests/api.rs`.
- **Call it from the UI** — add the types to `frontend/src/lib/api/types.ts` and a wrapper next to `notes.ts`.
- **New page** — create `frontend/src/routes/<name>/+page.svelte` and add an entry to `navItems` in `frontend/src/lib/nav.ts`.
- **More components** — `cd frontend && npx shadcn-svelte@latest add <component>`. Imports use the `#lib` alias with a `.js` extension, e.g. `#lib/components/ui/button/index.js`.
