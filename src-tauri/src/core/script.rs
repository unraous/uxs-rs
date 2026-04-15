use super::url::Type;

/// Returns the JavaScript code to be injected based on the URL type.
pub fn obtain_through(t: Type) -> Option<&'static str> {
    match t {
        Type::Course => Some(include_str!("../scripts/core.js")),
        Type::MainSpace => Some(include_str!("../scripts/modify-targets.js")),
        Type::Login => Some(include_str!("../scripts/click-auto-login.js")),
        Type::Other => None,
    }
}

/// Injects the given JavaScript code into the specified webview.
pub fn inject(webview: &tauri::Webview, script: &str) -> Result<(), String> {
    webview
        .eval(script)
        .map_err(|e| format!("脚本注入失败: {}", e))
}