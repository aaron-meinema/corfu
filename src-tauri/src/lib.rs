use tauri::{menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder}, Emitter};
use tauri_plugin_dialog::{FileDialogBuilder};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle();

            FileDialogBuilder::new()
                .set_title("Choose a file") 
                .pick_file(|file_path| 
                    match file_path { 
                        Some(path) => println!("Selected file path: {:?}", path),
                        None => println!("No file selected"),
                    });
            
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
                    let _event = app.emit("custom-event", "/settings");
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
