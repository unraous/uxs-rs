use crate::{config::CONFIG, core::script::obtain};

use tauri::{window, Manager};

/// Close the application window with a fade-out animation.
#[tauri::command]
#[specta::specta]
pub async fn close(window: window::Window) {
    log::debug!("正在执行关闭动画并关闭窗口");
    if let Some(mask) = window.get_webview("mask") {
        mask.show().ok();
        if let Err(e) = mask.eval(obtain(crate::core::url::Type::Mask).unwrap()) {
            log::error!("执行脚本失败 {}", e);
        }
    } else {
        log::error!("未找到遮罩Webview，无法执行关闭动画");
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
#[specta::specta]
pub fn minimize(window: window::Window) {
    log::debug!("正在最小化窗口");
    window.minimize().ok();
}
