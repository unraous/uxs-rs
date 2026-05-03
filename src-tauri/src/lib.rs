// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod core;
pub mod commands;
pub mod config;
pub mod ui;
pub mod network;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use ui::window;
    
    core::logger::init().expect("Failed to initialize logger");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(register!())
        .setup(window::init)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}