# Tauri Svelte Templates

Starter templates for building apps with **Tauri 2 + Svelte 5 + SvelteKit**. Tauri can be used in two ways, and each template shows one:

- **`tauri-ipc-daisyui`** keeps the Rust backend inside the desktop app and calls it over Tauri IPC.
- **`web-axum-shadcn`** uses Tauri only as a desktop window around a web frontend that talks HTTP to a standalone Axum service, so the same code also runs in the browser or in Docker.

Each folder is a complete, self-contained project: copy the one you want and start from there.

## Templates

| Template | Frontend ↔ Backend | Backend | UI | Runs as |
|----------|--------------------|---------|----|---------|
| [`tauri-ipc-daisyui`](./tauri-ipc-daisyui) | Tauri IPC (`invoke`) | Rust inside the Tauri app | Tailwind CSS 3 + DaisyUI 4 | Desktop app |
| [`web-axum-shadcn`](./web-axum-shadcn) | HTTP (REST) | Axum, a standalone service | Tailwind CSS 4 + shadcn-svelte + Lucide | Desktop window (Tauri), browser, or Docker |

## Use a Template

Copy a single template folder into a new project with [degit](https://github.com/Rich-Harris/degit):

```bash
npx degit liren0907/tauri-svelte-template/web-axum-shadcn my-app
# or
npx degit liren0907/tauri-svelte-template/tauri-ipc-daisyui my-app
```

Then follow the README inside that folder. Both start with:

```bash
cd my-app
yarn install
yarn tauri dev
```
