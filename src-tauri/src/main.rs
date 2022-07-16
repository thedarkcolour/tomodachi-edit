#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs::File;
use serde::Deserialize;
use tauri::{Window, Wry};
use tauri::api::dialog::FileDialogBuilder;
use std::os::windows::fs::MetadataExt;

mod tomodachi;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_file_chooser, check_file_target, load_island, get_food_registry, unlock_all_clothes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn open_file_chooser(window: Window<Wry>) {
    FileDialogBuilder::new()
        .add_filter("Tomodachi Life Save Data", &["txt"])
        .pick_file(move |path_buf| {
            if let Some(path) = path_buf {
                window.emit("file-chosen", path).unwrap();
            }
        });
}

const SAVE_DATA_SIZE: usize = 1985688;

#[tauri::command]
fn check_file_target(path: &str) -> u32 {
    let f = File::open(path)/*.expect(&format!("Could not open file: {}", path)[..])*/;

    return match f {
        Ok(f) => {
            return if f.metadata().unwrap().file_size() != (SAVE_DATA_SIZE as u64) {
                1
            } else if !path[path.len() - 15 .. path.len()].eq("savedataArc.txt") {
                2
            } else {
                0
            }
        },
        Err(_) => {
            3
        }
    }
}

#[tauri::command]
fn load_island(path: &str) -> tomodachi::Island {
    let save_data_u8 = std::fs::read(path).expect(format!("Could not open/read file at path {}", path).as_str());

    return tomodachi::Island::parse(&save_data_u8);
}

#[tauri::command]
fn get_food_registry() -> &'static Vec<tomodachi::Food> {
    return &*tomodachi::FOOD_REGISTRY;
}

#[tauri::command]
fn save_mii_changes(save_file_path: &str, mii_index: usize, changes: Vec<&str>) {
    changes.iter().map(|string| serde_json::from_str::<FieldChanged>(*string).expect("Invalid changed field")).into_iter().for_each(|change| {
        if check_file_target(save_file_path) == 0 {

        } else {
            eprintln!("Invalid path is open?");
        }
    });
}

#[tauri::command]
fn unlock_all_clothes(save_file_path: &str) -> bool {
    tomodachi::unlock_all_clothes(save_file_path);
    true
}

#[derive(Deserialize)]
struct FieldChanged {
    field_name: String,
    new_value: String,
}