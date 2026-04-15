use super::url::UrlType;
use log::{debug, error};

pub fn get_modification_script(url_type: UrlType) -> Option<&'static str> {
    match url_type {
        UrlType::Course => Some(include_str!("../scripts/core.js")),
        UrlType::MainSpace => Some(include_str!("../scripts/modify-targets.js")),
        UrlType::Login => Some(include_str!("../scripts/click-auto-login.js")),
        UrlType::Other => None,
    }
}

/// Injects the given JavaScript code into the specified webview.
pub fn inject_script(webview: &tauri::Webview, script: &str) -> Result<(), String> {
    debug!("准备注入JavaScript脚本");
    webview
        .eval(script)
        .map_err(|e| {
            error!("注入脚本失败: {}", e);
            format!("脚本注入失败: {}", e)
        })
        .map(|_| {
            debug!("脚本注入成功");
        })
}