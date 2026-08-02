// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod app;
pub mod commands;
pub mod config;
pub mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use app::webview::UrlStack;
    use app::window;

    core::logger::init().expect("Failed to initialize logger");

    // disable GPU for WebView2 to reduce memory usage and improve performance (100MB+ memory usage reduction)
    std::env::set_var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS", "--disable-gpu");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(UrlStack::default())
        .invoke_handler(commands_collector::register!())
        .setup(window::init)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
