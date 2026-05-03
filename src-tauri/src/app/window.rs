use super::webview;

use crate::network::ws;
use crate::config::CONFIG;

use log::{debug, info};
use tauri::{WindowBuilder, image::Image};

fn close() {
    debug!("执行程序收尾工作，即将关闭应用");
    CONFIG.save().ok();
}

/// Handles window events, specifically preventing the window from closing when a close request is made.
fn listener(event: &tauri::WindowEvent) {
    debug!("监听到窗口事件: {:?}", event);
    match event {
        tauri::WindowEvent::CloseRequested {..} => close(),
        tauri::WindowEvent::Destroyed => { 
            debug!("主程序窗口已销毁，应用已关闭");
        },
        _ => {}
    }
}

/// Initializes the application by creating the main window and adding two webviews.
pub fn init(app: &mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    debug!("开始初始化应用窗口");
    // let monitors = app.available_monitors()?;

    // let target_monitor = if monitors.len() > 1 {
    //     &monitors[1]
    // } else {
    //     &monitors[0]
    // };
    
    // let monitor_pos = target_monitor.position();

    let window = WindowBuilder::new(app, "app")
        .fullscreen(true)
        // .position(monitor_pos.x as f64, monitor_pos.y as f64)
        .background_color((0, 0, 0).into())
        .icon(Image::from_path("icons/icon.ico")?)?
        .build()?; 

    window.on_window_event(listener);
    
    webview::init_on(&window, "main")?;
    webview::init_on(&window, "chaoxing")?;
    webview::init_on(&window, "mask")?.hide()?;
    ws::setup();

    info!("应用窗口初始化成功");

    Ok(())
}