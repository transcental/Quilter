// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
use serde::Serialize;

#[derive(Serialize)]
pub struct FilesMap {
    files: Vec<String>,
    folder: String,
}

#[derive(Clone, Serialize)]
enum QuiltStatusState {
    InProgress,
    Finished,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct QuiltStatus {
    amount: isize,
    index: isize,
    status: QuiltStatusState,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::sort_files,
            commands::check_folder,
            commands::delete_file,
            commands::move_file,
            commands::delete_move_files_in_folder,
            commands::sort_files,
            commands::make_quilt
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
