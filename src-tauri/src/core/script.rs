use super::url::{classify, Type};

/// 根据 URL 类型获取对应的 JavaScript 脚本内容（并按需改造）。
pub fn obtain(t: Type) -> Option<String> {
    match t {
        Type::Course => Some(include_str!("../scripts/core.js").to_string()),
        Type::MainSpace => Some(include_str!("../scripts/modify-targets.js").to_string()),
        Type::Mask => Some(include_str!("../scripts/show-mask.js").to_string()),
        Type::Login => Some(include_str!("../scripts/click-auto-login.js").to_string()),
        Type::Other => None,
    }
}

/// 向指定的 Webview 注入 JavaScript 代码，并获取可能的错误。
pub fn evaluate(webview: &tauri::Webview, script: &str) -> Result<(), String> {
    log::debug!("正在注入脚本到Webview \"{}\"...", webview.label());
    webview
        .eval(script)
        .map_err(|e| format!("脚本注入失败: {}", e))
}

/// 负责在页面加载完成后根据 URL 类型注入对应的脚本。
pub fn load_on(webview: tauri::Webview, payload: tauri::webview::PageLoadPayload) {
    if let tauri::webview::PageLoadEvent::Finished = payload.event() {
        let url_type = classify(payload.url());
        log::debug!("URL \"{}\" 加载完成, 类别: {:?}", payload.url(), url_type);

        if let Some(script) = obtain(url_type) {
            evaluate(&webview, &script).unwrap_or_else(|e| {
                log::error!("注入脚本失败: {}", e);
            });
        } else {
            log::debug!("当前URL不需要注入脚本, 跳过注入");
        }
    }
}
