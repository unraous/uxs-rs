use crate::{
    config::CONFIG,
    core::script::{evaluate, obtain},
};

use log::{debug, error};
use tauri::{window, Manager};

/// Close the application window with a fade-out animation.
#[tauri::command]
pub async fn close(window: window::Window) {
    debug!("正在执行关闭动画并关闭窗口");
    if let Some(mask) = window.get_webview("mask") {
        mask.show().ok();
        evaluate(&mask, &obtain(crate::core::url::Type::Mask).unwrap()).ok();
    } else {
        error!("未找到遮罩Webview，无法执行关闭动画");
    }

    let sleep = tokio::time::sleep(std::time::Duration::from_millis(750));
    let cleanup = async {
        CONFIG.save().ok();
        log::debug!("配置成功保存");
    };

    tokio::join!(sleep, cleanup);
    window.close().ok();
}

/// Minimize the application window.
#[tauri::command]
pub fn minimize(window: window::Window) {
    debug!("正在最小化窗口");
    window.minimize().ok();
}
