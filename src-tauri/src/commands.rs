use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct FilesMap {
    files: Vec<String>,
    folder: String,
}

#[tauri::command]
pub fn check_folder(path: String) -> FilesMap {
    let files = fs::read_dir(path.clone()).unwrap();
    let mut unwanted_files: Vec<String> = vec![];
    for file in files {
        let file = file.unwrap();
        let file_name = file.file_name();
        let file_name = file_name.to_str().unwrap();
        if !file_name.contains("_v") {
            unwanted_files.push(file_name.to_string());
        }
    }

    let files_map = FilesMap {
        files: unwanted_files,
        folder: path,
    };

    files_map.into()
}

#[tauri::command]
pub fn delete_file(folder: &str, filename: String) {
    let path = std::path::Path::new(folder).join(&filename);
    let path = path.to_str().unwrap();
    fs::remove_file(path).unwrap();
}

#[tauri::command]
pub fn move_file(folder: &str, filename: String, destination: &str) {
    let path = std::path::Path::new(folder).join(&filename);
    let path = path.to_str().unwrap();
    let destination = std::path::Path::new(destination).join(&filename);
    let destination = destination.to_str().unwrap();
    fs::rename(path, destination).unwrap();
}

#[tauri::command]
pub fn delete_move_files_in_folder(folder: String, files: Vec<String>, delete: bool) {
    if !delete {
        let parent_folder = std::path::Path::new(&folder).parent().unwrap();
        let unwanted_folder = parent_folder.join("unwanted_files");
        let unwanted_folder = unwanted_folder.to_str().unwrap();
        fs::create_dir_all(unwanted_folder).unwrap();
        for file in files {
            move_file(&folder, file, unwanted_folder);
        }
    } else {
        for file in files {
            delete_file(&folder, file);
        }
    }
}

#[tauri::command]
pub fn sort_files(frame_folders: Vec<String>, final_folder: String, views: usize) {
    let no_of_folders: usize = frame_folders.len();
    for i in 0..no_of_folders {
        for folder in &frame_folders {
            let files = fs::read_dir(folder.clone()).unwrap();
            for file in files {
                let filename = file.unwrap().file_name();
                let current_view = filename
                    .to_str()
                    .unwrap()
                    .split("_v")
                    .collect::<Vec<&str>>()[1];
                let current_view = current_view.split(".").collect::<Vec<&str>>()[0];
                let current_view = match current_view.parse::<usize>() {
                    Ok(view) => view,
                    Err(_) => {
                        println!("Failed to parse view: {}", current_view);
                        continue;
                    }
                };
                if current_view < (views / no_of_folders) * (i + 1)
                    && current_view >= (views / no_of_folders) * i
                {
                    let original_file = std::path::Path::new(&folder).join(&filename);
                    let original_file = original_file.to_str().unwrap();
                    let path = std::path::Path::new(&final_folder).join(&filename);
                    let path = path.to_str().unwrap();
                    fs::copy(original_file, path).unwrap();
                }
            }
        }
    }
}
