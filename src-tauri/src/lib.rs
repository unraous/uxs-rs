// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod app;
pub mod commands;
pub mod url;
pub mod ws;

pub use app::init_app;
pub use commands::{close_app, minimize_app};
pub use url::{UrlType, classify_url, get_modification_script};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            close_app, minimize_app
        ])
        .setup(init_app)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
