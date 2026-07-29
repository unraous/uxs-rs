use super::url::{classify, Type};

/// 根据 URL 类型获取对应的 JavaScript 静态脚本内容。
pub fn obtain(t: Type) -> Option<&'static str> {
    match t {
        Type::Course => Some(include_str!("../scripts/core.js")),
        Type::MainSpace => Some(include_str!("../scripts/modify-targets.js")),
        Type::Mask => Some(include_str!("../scripts/show-mask.js")),
        Type::Login => Some(include_str!("../scripts/click-auto-login.js")),
        _ => None,
    }
}

/// 负责在页面加载完成后根据 URL 类型注入对应的脚本。
pub fn load_on(webview: tauri::Webview, payload: tauri::webview::PageLoadPayload) {
    if let tauri::webview::PageLoadEvent::Finished = payload.event() {
        let url_type = classify(payload.url());
        log::debug!("URL \"{}\" 加载完成, 类别: {:?}", payload.url(), url_type);

        if let Some(script) = obtain(url_type) {
            log::debug!("正在注入脚本到Webview \"{}\"...", webview.label());
            if let Err(e) = webview.eval(script) {
                log::error!("Webview \"{}\" 脚本注入失败: {}", webview.label(), e);
            } else {
                log::info!("Webview \"{}\" 脚本注入成功", webview.label())
            }
        } else {
            log::debug!("当前URL不需要注入脚本, 跳过注入");
        }
    }
}
