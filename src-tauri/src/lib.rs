use std::{f32::consts::E, fs::{self, ReadDir}, path::Path};

use serde::Serialize;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    Emitter
};
use tauri_plugin_dialog::{DialogExt, FilePath};
use walkdir::WalkDir;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Clone, Serialize)]
#[serde(rename_all="camelCase")]
enum FileType {
    Main,
    File,
    Folder
}

#[derive(Clone, Serialize)]
#[serde(rename_all="camelCase")]
struct ProgramFile {
    file_type: FileType,
    name: String
}

fn get_all_files(directory: FilePath) -> Vec<ProgramFile>{
    let mut paths: Vec<ProgramFile> = Vec::new();
    for entry in WalkDir::new(directory.to_string()).into_iter().filter_map(Result::ok) {
        let the_file_type: FileType;
        if entry.file_type().is_file() {
            the_file_type = FileType::File;
        } else {
            the_file_type = FileType::Folder;
        }
        paths.push(ProgramFile{
            file_type: the_file_type,
            name: entry 
                .path()
                .display()
                .to_string()
        }); 
    }
    paths.push(ProgramFile{
        file_type: FileType::Main,
        name: directory.to_string(),
    });
    paths
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // my custom settings menu item
            let open = MenuItemBuilder::new("open")
                .id("open")
                .accelerator("CmdOrCtrl+,")
                .build(app)?;

            // my custom app submenu
            let app_submenu = SubmenuBuilder::new(app, "App")
                .item(&open)
                .services()
                .hide()
                .hide_others()
                .quit()
                .build()?;

            // ... any other submenus

            let menu = MenuBuilder::new(app)
                .items(&[
                    &app_submenu,
                    // ... include references to any other submenus
                ])
                .build()?;

            // set the menu
            app.set_menu(menu)?;

            // listen for menu item click events
            app.on_menu_event(move |app, event| {
                if event.id() == open.id() {
                    // emit a window event to the frontend
                    let app_clone = app.clone();
                    let _event = app.emit(
                        "open",
                        app.dialog().file().pick_folder(move |folder| {
                                    if folder.is_some() {
                                        app_clone.emit_to("okee", "open_files", 
                                        get_all_files(folder.unwrap())).unwrap();
                                    }
                                }
                            )
                    );
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
