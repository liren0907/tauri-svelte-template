/// A plain window around the SvelteKit app.
///
/// There are no Tauri commands and no IPC on purpose: the UI talks to the Axum
/// backend over HTTP exactly like it does in a browser, so the same frontend
/// runs on the web, in Docker, and on the desktop.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
