use super::webview;

use crate::config::CONFIG;
use crate::network::ws;

use tauri::{image::Image, WindowBuilder};

fn close() {
    log::debug!("执行程序收尾工作，即将关闭应用");
    CONFIG.save().ok();
}

/// Handles window events, specifically preventing the window from closing when a close request is made.
fn listener(event: &tauri::WindowEvent) {
    log::debug!("监听到窗口事件: {:?}", event);
    match event {
        tauri::WindowEvent::CloseRequested { .. } => close(),
        tauri::WindowEvent::Destroyed => {
            log::debug!("主程序窗口已销毁，应用已关闭");
        }
        _ => {}
    }
}

/// 根据分辨率、方向与标识符定位目标调试显示器，默认降级回退主屏。
fn select_monitor_from(monitors: &[tauri::Monitor]) -> &tauri::Monitor {
    if cfg!(debug_assertions) {
        log::debug!("开始检测1080p屏幕");
        monitors
            .iter()
            .find(|m| {
                let size = m.size();
                size.width == 1920 && size.height == 1080
            })
            .unwrap_or(&monitors[0])
    } else {
        &monitors[0]
    }
}

/**
 * Initializes the application by creating the main window and adding two webviews.
 *
 * Doesn't use `anyhow::Result` because `tauri::Builder::setup` strictly expects
 * `std::result::Result<(), Box<dyn std::error::Error>>` to prevent public API signature coupling with third-party error crates.
 */
pub fn init(app: &mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    log::debug!("开始初始化应用窗口");
    let monitors = app.available_monitors()?;

    let target_monitor = select_monitor_from(&monitors);
    let monitor_pos = target_monitor.position();

    let window = WindowBuilder::new(app, "app")
        // maybe later vision would use this
        // .decorations(false)
        // .transparent(true)
        // .inner_size(800.0, 600.0)
        // .shadow(false)
        .fullscreen(true)
        .position(monitor_pos.x as f64, monitor_pos.y as f64)
        .background_color((0, 0, 0).into())
        .icon(Image::from_bytes(include_bytes!("../../icons/icon.ico"))?)?
        .build()?;

    window.on_window_event(listener);

    webview::init_on(&window, "main")?;
    webview::init_on(&window, "chaoxing")?;
    webview::init_on(&window, "mask")?.hide()?;
    ws::setup();

    log::info!("应用窗口初始化成功");

    Ok(())
}
