// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod core;
pub mod ui;
pub mod network;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use ui::{window, commands};
    use core::config;

    use log::info;

    let config = config::Config::default();

    
    
    
    env_logger::Builder::from_default_env()
    .filter_level(log::LevelFilter::Debug)
    .init();

    info!("Starting {} v{}, working directory: {}", config.metadata.name, config.metadata.version, config.paths.cwd.display());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::close, commands::minimize
        ])
        .setup(window::init)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}