use super::webview;

use crate::network::ws;

use log::info;
use tauri::{WindowBuilder, image::Image};

/// Initializes the application by creating the main window and adding two webviews.
pub fn init(app: &mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let monitors = app.available_monitors()?;

    let target_monitor = if monitors.len() > 1 {
        &monitors[1]
    } else {
        &monitors[0]
    };
    
    let monitor_pos = target_monitor.position();

    let window = WindowBuilder::new(app, "app")
        .fullscreen(true)
        .position(monitor_pos.x as f64, monitor_pos.y as f64)
        .background_color((0, 0, 0).into())
        .icon(Image::from_path("icons/icon.ico")?)?
        .build()?; 
    
    webview::create_in(&window, "main")?;
    webview::create_in(&window, "chaoxing")?;
    webview::create_in(&window, "mask")?.hide()?;
    ws::init();

    info!("应用窗口初始化成功");

    Ok(())
}