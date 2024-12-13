use image::{GenericImageView, ImageBuffer};
use std::{fs, path::Path};
use tauri::{AppHandle, Emitter};
use tauri_plugin_shell::ShellExt;

use crate::{FilesMap, QuiltStatus, QuiltStatusState};

#[tauri::command(async)]
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

#[tauri::command(async)]
pub fn delete_file(folder: &str, filename: String) {
    let path = std::path::Path::new(folder).join(&filename);
    let path = path.to_str().unwrap();
    fs::remove_file(path).unwrap();
}

#[tauri::command(async)]
pub fn move_file(folder: &str, filename: String, destination: &str) {
    let path = std::path::Path::new(folder).join(&filename);
    let path = path.to_str().unwrap();
    let destination = std::path::Path::new(destination).join(&filename);
    let destination = destination.to_str().unwrap();
    fs::rename(path, destination).unwrap();
}

#[tauri::command(async)]
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

#[tauri::command(async)]
pub fn sort_files(frame_folders: Vec<String>, final_folder: String, views: usize) {
    println!("Folders: {:?}", frame_folders);
    let no_of_folders: usize = frame_folders.len();
    for (i, folder) in frame_folders.iter().enumerate() {
        println!("Sorting folder: {}", folder);
        let files = fs::read_dir(folder).unwrap();
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
                let original_file = Path::new(&folder).join(&filename);
                let original_file = original_file.to_str().unwrap();
                let path = Path::new(&final_folder).join(&filename);
                let path = path.to_str().unwrap();
                fs::copy(original_file, path).unwrap();
            }
        }
    }
}

#[tauri::command(async)]
pub fn make_quilt(
    app: AppHandle,
    sorted_folder: String,
    output_folder: String,
    columns: isize,
    rows: isize,
    framerate: isize,
) {
    let quilt_size = columns * rows;

    let mut quilt_layout: Vec<Vec<String>> =
        vec![vec!['-'.to_string(); columns as usize]; rows as usize];
    let mut current_row = 0;
    let mut current_column = 0;

    let mut dir: Vec<_> = fs::read_dir(sorted_folder)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    dir.sort_by(|a, b| a.file_name().unwrap().cmp(b.file_name().unwrap()));
    let no_of_quilts = dir.len() as isize / quilt_size;

    app.emit(
        "quilt_status",
        QuiltStatus {
            amount: no_of_quilts,
            index: 0,
            status: QuiltStatusState::InProgress,
        },
    )
    .unwrap();

    for quilt_i in 0..no_of_quilts {
        for i in 0..quilt_size {
            let filename = dir[(i + quilt_size * quilt_i) as usize]
                .to_str()
                .unwrap()
                .to_string();
            if filename.contains("_v") {
                quilt_layout[current_row as usize][current_column as usize] = filename;
                current_column += 1;
                if current_column == columns {
                    current_column = 0;
                    current_row += 1;
                }
            } else {
                println!("INVALID FILE FOUND '_v': {}", filename);
            }
        }
        current_row = 0;

        let first_img_path = &quilt_layout[0][0];
        let first_img = image::open(first_img_path).unwrap();
        let (img_width, img_height) = first_img.dimensions();
        let quilt_width = img_width * columns as u32;
        let quilt_height = img_height * rows as u32;

        let mut quilt = ImageBuffer::new(quilt_width, quilt_height);
        let mut current_width: u32;
        let mut current_height: u32 = 0;
        for row in 0..rows {
            current_width = 0;
            for column in 0..columns {
                let current_img_path = &quilt_layout[row as usize][column as usize];
                let current_img = image::open(current_img_path).unwrap();
                let current_img_map = current_img.to_rgba8();

                for x in 0..img_width {
                    for y in 0..img_height {
                        let pixel = current_img_map.get_pixel(x, y);
                        quilt.put_pixel(
                            current_width + x,
                            (quilt_height - 1) - (current_height + y),
                            *pixel,
                        );
                    }
                }
                current_width = img_width * <isize as TryInto<u32>>::try_into(column + 1).unwrap();
            }
            current_height = img_height * <isize as TryInto<u32>>::try_into(row + 1).unwrap();
        }

        let output_path =
            std::path::Path::new(&output_folder).join(format!("quilt_{}.png", quilt_i));
        // flip the quilt vertically
        let quilt = image::imageops::flip_vertical(&quilt);
        quilt.save(&output_path).unwrap();
        println!("Saved {:?}", &output_path);
        app.emit(
            "quilt_status",
            QuiltStatus {
                amount: no_of_quilts,
                index: quilt_i + 1,
                status: QuiltStatusState::InProgress,
            },
        )
        .unwrap();
    }
    app.emit(
        "quilt_status",
        QuiltStatus {
            amount: no_of_quilts,
            index: no_of_quilts,
            status: QuiltStatusState::Finished,
        },
    )
    .unwrap();

    if framerate > 0 {
        app.emit(
            "quilt_status",
            QuiltStatus {
                amount: no_of_quilts,
                index: no_of_quilts,
                status: QuiltStatusState::CreatingAnimation,
            },
        )
        .unwrap();
        let framerate: &str = &framerate.to_string();
        let input_path = std::path::Path::new(&output_folder).join("%d.png");
        let output_path = std::path::Path::new(&output_folder).join("animation.mp4");
        let sidebar_command = app.shell().sidecar("ffmpeg").unwrap().args([
            "-framerate",
            framerate,
            "-i",
            input_path.to_str().unwrap(),
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            output_path.to_str().unwrap(),
        ]);
        let _ = sidebar_command.spawn().unwrap();
        app.emit(
            "quilt_status",
            QuiltStatus {
                amount: no_of_quilts,
                index: no_of_quilts,
                status: QuiltStatusState::CreatedAnimation,
            },
        )
        .unwrap();
    }
    // ffmpeg -framerate 24 -i %d.png -c:v libx264 -pix_fmt yuv420p animation.mp4
}
