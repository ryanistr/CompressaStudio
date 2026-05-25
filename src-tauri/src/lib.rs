mod checksum;
mod commands;
mod compression;
mod file_detection;
mod file_info;
mod tool_detection;

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
