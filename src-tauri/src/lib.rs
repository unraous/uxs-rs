// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod core;
pub mod commands;
pub mod config;
pub mod app;
pub mod network;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use app::window;
    
    core::logger::init().expect("Failed to initialize logger");

    // disable GPU for WebView2 to reduce memory usage and improve performance (100MB+ memory usage reduction)
    std::env::set_var(
        "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
        "--disable-gpu --disable-gpu-compositing --js-flags=\"--max-old-space-size=64\""
    );
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(register!())
        .setup(window::init)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}