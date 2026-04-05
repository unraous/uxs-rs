/// This module defines the main application logic for the Tauri application, including window creation and webview management.
use log::{debug, info, error};
use tauri::{LogicalPosition, LogicalSize, WebviewBuilder, WebviewUrl, WindowBuilder};
use crate::url::{classify_url, get_modification_script};

/// Handles the page load event for the webview. If the page load is finished, 
/// it checks the URL and injects the appropriate modification script if needed.
fn page_load_manager(
    webview: tauri::Webview,
    payload: tauri::webview::PageLoadPayload
) {
    if let tauri::webview::PageLoadEvent::Finished = payload.event() {
        let url_type = classify_url(&payload.url());
        debug!("URL \"{}\" 加载完成, 类别: {:?}", payload.url(), url_type);

        if let Some(script) = get_modification_script(url_type) {
            debug!("准备注入JavaScript脚本");
            if let Err(e) = webview.eval(script) {
                error!("注入脚本失败: {}", e);
            } else {
                debug!("脚本注入成功");
            }
        } else {
            debug!("当前URL不需要注入脚本, 跳过注入");
        }
    }
}

fn window_events_manager(event: &tauri::WindowEvent) {
    debug!("窗口事件: {:?}", event);
}

/// Initializes the application by creating the main window and adding two webviews:
/// one for the main space and one for the Chaoxing website.
pub fn init_app(app: &mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let monitors = app.available_monitors()?;

    let target_monitor = if monitors.len() > 1 {
        &monitors[0]
    } else {
        &monitors[0]
    };
    
    let monitor_pos = target_monitor.position();

    let window = WindowBuilder::new(app, "app")
        .fullscreen(true)
        .position(monitor_pos.x as f64, monitor_pos.y as f64)
        .background_color((0, 0, 0).into())
        .build()?; 
    
    let logical_size: LogicalSize<f64> = tauri::LogicalSize::from_physical(
        window.inner_size()?, window.scale_factor()?
    );

    let main_webview = window.add_child(
        WebviewBuilder::new("main", WebviewUrl::default())
        .background_color((0, 0, 0, 0).into()),
        LogicalPosition::new(0.0, 0.0),
        LogicalSize::new(logical_size.width, logical_size.height)
    )?;  
    let chaoxing_webview = window.add_child(
        WebviewBuilder::new(
            "chaoxing",
            WebviewUrl::External("https://i.chaoxing.com".parse()?),
        )
        .zoom_hotkeys_enabled(true) 
        .background_color((0, 0, 0, 0).into())
        .on_page_load(page_load_manager),
        LogicalPosition::new(logical_size.width * 0.51, logical_size.height * 0.46),
        LogicalSize::new(logical_size.width * 0.48, logical_size.height * 0.48)
    )?;

    for webview in [&main_webview, &chaoxing_webview] {
        debug!(
            "Webview \"{}\" 创建成功, position: {:?}, size: {:?}", 
            webview.label(), webview.position(), webview.size()
        );
    }

    window.on_window_event(window_events_manager);

    info!("应用窗口初始化成功") ;
    Ok(())
}