use crate::core::{classify_url, get_modification_script, inject_script};
use crate::network::start_ws_server;
use log::{debug, info};
use tauri::{LogicalPosition, LogicalSize, WebviewBuilder, WebviewUrl, WindowBuilder, image::Image};

/// Handles the page load event for the webview. If the page load is finished.
fn on_page_load_finished(
    webview: tauri::Webview,
    payload: tauri::webview::PageLoadPayload
) {
    if let tauri::webview::PageLoadEvent::Finished = payload.event() {
        let url_type = classify_url(&payload.url());
        debug!("URL \"{}\" 加载完成, 类别: {:?}", payload.url(), url_type);

        if let Some(script) = get_modification_script(url_type) {
            let _ = inject_script(&webview, script);
        } else {
            debug!("当前URL不需要注入脚本, 跳过注入");
        }
    }
}

/// Handle window events for debugging purposes
fn on_window_event(event: &tauri::WindowEvent) {
    debug!("窗口事件: {:?}", event);
}

/// Initializes the application by creating the main window and adding two webviews.
pub fn init_app(app: &mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let monitors = app.available_monitors()?;

    let target_monitor = if monitors.len() > 1 {
        &monitors[1]
    } else {
        &monitors[0]
    };
    
    let monitor_pos = target_monitor.position();

    let window = WindowBuilder::new(app, "app")
        .fullscreen(true)
        .position(monitor_pos.x as f64, monitor_pos.y as f64)
        .background_color((0, 0, 0).into())
        .icon(Image::from_path("icons/icon.ico")?)?
        .build()?; 
    
    let logical_size: LogicalSize<f64> = tauri::LogicalSize::from_physical(
        window.inner_size()?, window.scale_factor()?
    );

    let _main_webview = window.add_child(
        WebviewBuilder::new("main", WebviewUrl::default())
            .background_color((0, 0, 0, 0).into()),
        LogicalPosition::new(0.0, 0.0),
        LogicalSize::new(logical_size.width, logical_size.height)
    )?;  
    
    let _chaoxing_webview = window.add_child(
        WebviewBuilder::new(
            "chaoxing",
            WebviewUrl::External("https://i.chaoxing.com".parse()?),
        )
        .zoom_hotkeys_enabled(true) 
        .background_color((0, 0, 0, 0).into())
        .on_page_load(on_page_load_finished),
        LogicalPosition::new(logical_size.width * 0.51, logical_size.height * 0.46),
        LogicalSize::new(logical_size.width * 0.48, logical_size.height * 0.48)
    )?;

    window.on_window_event(on_window_event);

    info!("应用窗口初始化成功");
    start_ws_server();

    Ok(())
}