use tauri::window;
use log::{debug, error};

#[tauri::command]
pub fn close_app(window: window::Window) {
    let webviews = window.webviews();
    for webview in webviews {
        debug!("webview {} 执行渐变透明动画...", webview.label());
        if let Err(e) = webview.eval(include_str!("../scripts/fade-out.js")) {
            error!("注入渐变动画脚本失败: {}", e);
        }
    }
    std::thread::sleep(std::time::Duration::from_millis(750));
    window.close().ok();
}

#[tauri::command]
pub fn minimize_app(window: window::Window) {
    window.minimize().ok();
}