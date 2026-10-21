pub mod commands;
pub mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::file_ops::read_file_content,
            commands::file_ops::open_file_dialog,
            commands::file_ops::path_exists,
            commands::file_ops::copy_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
