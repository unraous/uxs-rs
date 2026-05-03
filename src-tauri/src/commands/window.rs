use crate::core::script::{obtain, evaluate};

use tauri::{Manager, window};
use log::{debug, error};

/// Close the application window with a fade-out animation.
#[tauri::command]
pub fn close(window: window::Window) {
    debug!("正在执行关闭动画并关闭窗口");
    if let Some(mask) = window.get_webview("mask") {
        mask.show().ok();
        evaluate(&mask, obtain(crate::core::url::Type::Mask).unwrap_or("")).ok();
        std::thread::sleep(std::time::Duration::from_millis(750));
    } else {
        error!("未找到遮罩Webview，无法执行关闭动画");
    }
    window.close().ok();
}

/// Minimize the application window.
#[tauri::command]
pub fn minimize(window: window::Window) {
    debug!("正在最小化窗口");
    window.minimize().ok();
}