# Tauri Svelte Template

A minimal starter template for building desktop applications with **Tauri v2 + SvelteKit + Svelte 5 + Tailwind CSS 3 + DaisyUI 4**.

## What's Included

- **Tauri v2** — Rust backend with native file dialog (image/video filter)
- **SvelteKit** — Static adapter (SPA) with pre-configured routing
- **Svelte 5** — Using runes (`$state`, `$props`, `$derived`)
- **Tailwind CSS 3 + DaisyUI 4** — Utility-first styling with component library
- **Theme switching** — Light / Dark toggle in sidebar
- **Media viewer** — Open and display images and videos via Tauri asset protocol
- **Toast notifications** — Built-in toast system
- **UI component library** — 11 generic DaisyUI wrapper components (Button, Card, Badge, Alert, Modal, Toast, etc.)
- **Sidebar drawer layout** — Resizable sidebar with responsive breakpoints
- **Google Material Symbols** — Icon font pre-configured

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) & [Yarn](https://yarnpkg.com/)
- [Rust toolchain](https://rustup.rs/) (Edition 2024; Rust 1.88 or newer, latest stable recommended)
- [Tauri system dependencies](https://v2.tauri.app/start/prerequisites/) for your platform

### Install and Run

```bash
yarn install
yarn tauri dev
```

This starts the SvelteKit dev server on `http://localhost:1420` and launches the Tauri desktop shell.

### Build for Production

```bash
yarn tauri build
```

### Type Checking

```bash
yarn run check          # One-shot
yarn run check:watch    # Watch mode
```

Use `yarn run check`, not `yarn check`: the latter is Yarn 1's built-in integrity check, not this script.

## Project Structure

```
src/
├── routes/
│   ├── +layout.svelte          # Drawer layout with sidebar
│   ├── +page.svelte            # Media viewer (open image/video)
│   ├── Sidebar.svelte          # Navigation + theme toggle
│   └── showcase/+page.svelte   # UI component showcase
├── lib/
│   ├── components/ui/          # 11 generic DaisyUI components
│   ├── stores/                 # Toast notification store
│   └── utils/                  # Tauri utilities
├── app.html                    # HTML shell
└── app.css                     # Tailwind + global styles

src-tauri/
├── src/
│   ├── main.rs                 # Tauri app entry
│   ├── lib.rs                  # Command registration
│   ├── commands/file_ops.rs    # File operations (read, dialog, exists, copy)
│   └── core/dialog_handler.rs  # Native file dialog (image/video filter)
├── Cargo.toml
└── tauri.conf.json
```

## Adding New Features

### New Route

Create `src/routes/my-page/+page.svelte` and add a nav entry in the `navItems` array in `src/routes/Sidebar.svelte`.

### New Tauri Command

1. Add the command in `src-tauri/src/commands/file_ops.rs` (or create a new module)
2. Register it in `src-tauri/src/lib.rs` inside `invoke_handler`
3. Call it from the frontend: `await invoke('my_command', { param: value })`

## Tech Stack

| Layer | Technology | Version |
|-------|-----------|---------|
| Desktop | Tauri | v2 |
| Frontend | SvelteKit | v2 |
| UI Framework | Svelte | v5 |
| Styling | Tailwind CSS | v3 |
| Components | DaisyUI | v4 |
| Backend | Rust | Edition 2024 |
