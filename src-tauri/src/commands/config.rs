use crate::config::CONFIG;
use crate::config::llm::LLMProvider;

use anyhow::Result;

// Get application metadata such as version and author information.
#[tauri::command]
pub fn metadata(key: String) -> String {
    log::debug!("正在获取元数据 [{}]", key);
    let res = match key.as_str() {
        "version" => CONFIG.metadata.version.clone(),
        "author" => CONFIG.metadata.author.clone(),
        _ => "unknown".to_string(),
    };
    log::debug!("成功获取元数据 [{}]: {}", key, res);
    res
}

#[tauri::command]
pub fn switch_provider(provider: String) -> Result<()> {
    log::debug!("正在切换 AI Provider 到 [{}]", provider);
    CONFIG.llm.switch_to(provider.parse::<LLMProvider>()?);
    log::debug!("成功切换 AI Provider 到 [{}]", provider);
    Ok(())
}

#[tauri::command]
pub fn models() -> Vec<String> {
    log::debug!("正在获取可用模型列表...");
    let models = CONFIG.llm.current().available_models();
    log::debug!("成功获取模型列表: {:?}", models);
    models
}

#[tauri::command]
pub fn set_key(key: String) {
    log::debug!("正在设置 [{:?}] 的 API 密钥...", CONFIG.llm.provider);
    CONFIG.llm.current().set_key(&key);
    log::debug!("成功设置 [{:?}] 的 API 密钥", CONFIG.llm.provider);
}

#[tauri::command]
pub fn switch_model(model: String) {
    log::debug!("正在切换模型到 [{}]...", model);
    CONFIG.llm.current().switch_model(&model);
    log::debug!("成功切换模型到 [{}]", model);
}