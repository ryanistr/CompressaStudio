//! Application entry point and Tauri builder configuration.

mod checksum;
mod commands;
mod compression;
mod file_detection;
mod file_info;
mod tool_detection;

/// Initializes and runs the Tauri application.
///
/// Registers plugins, binds IPC command handlers, and starts the event loop.
/// Panics if the Tauri runtime fails to start.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_file_info,
            commands::get_tool_availability,
            commands::compress_file,
            commands::decompress_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
