#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use tauri::{CustomMenuItem, SystemTrayEvent, Manager};
use tauri::{Menu, MenuItem, Submenu, SystemTray, SystemTrayMenu, SystemTrayMenuItem};
use enigo::{Enigo};
use enigo::MouseControllable;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_cursor_position() -> (i32, i32) {
    let mut enigo = Enigo::new();
    println!("mouse location: {:?}", enigo.mouse_location());
    let result = enigo.mouse_location();
    result
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
    .add_item(quit)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(hide);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
    .system_tray(tray)
    .plugin(tauri_plugin_positioner::init())
    .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a left click");
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a right click");
            }
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a double click");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                _ => {}
            },
            _ => {}
        })
    .invoke_handler(tauri::generate_handler![greet, get_cursor_position])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
    //tauri::Builder::default()
    //    .system_tray(system_tray)
    //    .invoke_handler(tauri::generate_handler![greet])
    //    .run(tauri::generate_context!())
    //    .expect("error while running tauri application");
}
