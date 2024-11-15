use serde::Serialize;
use std::fs;

#[tauri::command]
pub fn sort_files() {
    println!("Hello world")
}

#[derive(Serialize)]
pub struct FilesMap {
    files: Vec<String>,
    folder: String
}

#[tauri::command]
pub fn check_folder(path: String) -> FilesMap {
    println!("Checking folder: {}", path);
    // for file in folder, check if _v is in the name
    let files = fs::read_dir(path.clone()).unwrap();
    let mut unwanted_files: Vec<String> = vec![];
    for file in files {
        let file = file.unwrap();
        let file_name = file.file_name();
        let file_name = file_name.to_str().unwrap();
        if !file_name.contains("_v") {
            // println!("Found file: {}", file_name);
            unwanted_files.push(file_name.to_string());
        }
    }

    let files_map = FilesMap {
        files: unwanted_files,
        folder: path
    };
    
    files_map.into()
}

#[tauri::command]
pub fn delete_file(folder: &str, filename: String) {
    let path = std::path::Path::new(folder).join(&filename);
    let path = path.to_str().unwrap();
    println!("Deleting file: {}", path);
    fs::remove_file(path).unwrap();
}

#[tauri::command]
pub fn delete_files_in_folder(folder: String, files: Vec<String>) {
    for file in files {
        delete_file(&folder, file);
    }
}