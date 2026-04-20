use crate::core::script::load_on;

use anyhow;
use tauri::{LogicalPosition, LogicalSize, Webview, WebviewBuilder, WebviewUrl};


pub fn init_on(window: &tauri::Window, label: &str) -> Result<Webview, Box<dyn std::error::Error>> {
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
            WebviewBuilder::new(label, WebviewUrl::External("about:blank".parse()?))
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
                .on_page_load(load_on),
            LogicalPosition::new(logical_size.width * 0.51, logical_size.height * 0.46),
            LogicalSize::new(logical_size.width * 0.48, logical_size.height * 0.48)
        ),
        _ => return Err(anyhow::anyhow!("未知的Webview标签").into()),
    };

    Ok(window.add_child(builder, position, size)?)
}