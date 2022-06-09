#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::ffi::OsString;
use std::fs::File;
use std::path::PathBuf;
use tauri::{Builder, CustomMenuItem, Menu, MenuItem, Submenu, Manager, Window, Wry};
use tauri::api::dialog::FileDialogBuilder;
use std::os::windows::fs::MetadataExt;
use substring::Substring;

fn main() {
    let open = CustomMenuItem::new("open".to_string(), "Open");
    let file_menu = Submenu::new("File", Menu::new().add_item(open));
    let menu = Menu::new()
        .add_submenu(file_menu);

    tauri::Builder::default()
        .setup(|app| {
            let main = app.get_window("main").unwrap();

            Ok(())
        })
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "open" => {
                FileDialogBuilder::new()
                    .add_filter("Tomodachi Life Save Data", &["txt"])
                    .pick_file(move |path_buf| match path_buf {
                        Some(file_path) => {
                            &event.window().emit("file-chosen", file_path).unwrap();
                        }
                        _ => {}
                    })
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![open_file_chooser, check_file_target])
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
    let mut f = File::open(path).expect(&format!("Could not open file: {}", path)[..]);

    if f.metadata().unwrap().file_size() != (SAVE_DATA_SIZE as u64) {
        return 1;
    }
    if !path.substring(path.len() - 15, path.len()).eq("savedataArc.txt") {
        return 2;
    }
    return 0;
}
