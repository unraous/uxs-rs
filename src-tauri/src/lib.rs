// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod core;
pub mod config;
pub mod ui;
pub mod network;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use ui::{window, commands};
    
    core::logger::init().expect("Failed to initialize logger");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::close, commands::minimize, commands::metadata
        ])
        .setup(window::init)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}