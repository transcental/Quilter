// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![commands::sort_files, commands::check_folder, commands::delete_file, commands::delete_files_in_folder, commands::sort_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
