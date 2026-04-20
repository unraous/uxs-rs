use super::url::{classify, Type};

use log::{debug, error};

/// Returns the JavaScript code to be injected based on the URL type.
pub fn obtain(t: Type) -> Option<&'static str> {
    match t {
        Type::Course => Some(include_str!("../scripts/core.js")),
        Type::MainSpace => Some(include_str!("../scripts/modify-targets.js")),
        Type::Mask => Some(include_str!("../scripts/show-mask.js")),
        Type::Login => Some(include_str!("../scripts/click-auto-login.js")),
        Type::Other => None,
    }
}

/// Evaluates the given JavaScript code in the specified webview.
pub fn evaluate(webview: &tauri::Webview, script: &str) -> Result<(), String> {
    debug!("正在注入脚本到Webview \"{}\"...", webview.label());
    webview
        .eval(script)
        .map_err(|e| format!("脚本注入失败: {}", e))
}

pub fn load_on(
    webview: tauri::Webview,
    payload: tauri::webview::PageLoadPayload
) {
    if let tauri::webview::PageLoadEvent::Finished = payload.event() {
        let url_type = classify(&payload.url());
        debug!("URL \"{}\" 加载完成, 类别: {:?}", payload.url(), url_type);

        if let Some(script) = obtain(url_type) {
            evaluate(&webview, script).unwrap_or_else(|e| {
                error!("注入脚本失败: {}", e);
            });
        } else {
            debug!("当前URL不需要注入脚本, 跳过注入");
        }
    }
}