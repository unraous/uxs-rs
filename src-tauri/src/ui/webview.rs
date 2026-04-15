use crate::core::{classify, obtain_through, inject};

use anyhow;
use log::{debug, error};
use tauri::{LogicalPosition, LogicalSize, Webview, WebviewBuilder, WebviewUrl};

fn load_script(
    webview: tauri::Webview,
    payload: tauri::webview::PageLoadPayload
) {
    if let tauri::webview::PageLoadEvent::Finished = payload.event() {
        let url_type = classify(&payload.url());
        debug!("URL \"{}\" 加载完成, 类别: {:?}", payload.url(), url_type);

        if let Some(script) = obtain_through(url_type) {
            debug!("准备注入JavaScript脚本");
            inject(&webview, script).unwrap_or_else(|e| {
                error!("注入脚本失败: {}", e);
            });
        } else {
            debug!("当前URL不需要注入脚本, 跳过注入");
        }
    }
}


pub fn create_in(window: &tauri::Window, label: &str) -> Result<Webview, Box<dyn std::error::Error>> {
    let logical_size: LogicalSize<f64> = tauri::LogicalSize::from_physical(
        window.inner_size()?, window.scale_factor()?
    );

    let (builder, position, size) = match label {
        "main" => (
            WebviewBuilder::new(label, WebviewUrl::default())
                .background_color((0, 0, 0, 0).into()),
            LogicalPosition::new(0.0, 0.0),
            LogicalSize::new(logical_size.width, logical_size.height)
        ),
        "mask" => (
            WebviewBuilder::new(label, WebviewUrl::default())
                .background_color((0, 0, 0, 0).into()),
            LogicalPosition::new(0.0, 0.0),
            LogicalSize::new(logical_size.width, logical_size.height)
        ),
        "chaoxing" => (
            WebviewBuilder::new(
                label,
                WebviewUrl::External("https://i.chaoxing.com/".parse()?)
            )
                .background_color((0, 0, 0, 0).into())
                .on_page_load(load_script),
            LogicalPosition::new(logical_size.width * 0.51, logical_size.height * 0.46),
            LogicalSize::new(logical_size.width * 0.48, logical_size.height * 0.48)
        ),
        _ => return Err(anyhow::anyhow!("未知的Webview标签").into()),
    };

    Ok(window.add_child(builder, position, size)?)
}