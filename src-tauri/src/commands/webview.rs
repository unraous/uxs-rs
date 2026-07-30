/// Specific Some Webview Control Commands only for [Chaoxing] Webview,only call in [main] webview.
use super::CommandsResult;

use crate::app::webview::UrlStack;

use anyhow::anyhow;
use tauri::{webview::Webview, window, Manager};

fn chaoxing_webview(window: &window::Window) -> CommandsResult<Webview> {
    Ok(window
        .get_webview("chaoxing")
        .ok_or_else(|| anyhow!("未找到webview [chaoxing]"))?)
}

#[tauri::command]
#[specta::specta]
pub fn set_zoom(window: window::Window, scale: f64) -> CommandsResult<()> {
    chaoxing_webview(&window)?.set_zoom(scale)?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn go_back(
    window: window::Window,
    url_stack: tauri::State<'_, UrlStack>,
) -> CommandsResult<()> {
    if let Some(url) = url_stack.back() {
        chaoxing_webview(&window)?.navigate(url)?;
        Ok(())
    } else {
        log::warn!("[chaoxing] 无法后退：已无历史页面");
        Err(anyhow!("无法后退：没有更早的历史页面").into())
    }
}

#[tauri::command]
#[specta::specta]
pub fn go_forward(
    window: window::Window,
    url_stack: tauri::State<'_, UrlStack>,
) -> CommandsResult<()> {
    if let Some(url) = url_stack.forward() {
        log::debug!("[chaoxing] 前进至: {}", url);
        chaoxing_webview(&window)?.navigate(url)?;
        Ok(())
    } else {
        log::warn!("[chaoxing] 无法前进：已是最新页面");
        Err(anyhow!("无法前进：没有更新的历史页面").into())
    }
}
