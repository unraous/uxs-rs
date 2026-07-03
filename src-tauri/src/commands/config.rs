use crate::config::CONFIG;
use crate::config::llm::LLMProvider;

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

// Switch the current LLM provider to the specified one, returning true if successful, false otherwise.
#[tauri::command]
pub fn switch_provider(provider: String) -> bool {
    log::debug!("正在切换 AI Provider 到 [{}]", provider);
    if let Ok(llmp) = provider.parse::<LLMProvider>() {
        CONFIG.llm.switch_to(llmp);
        log::debug!("成功切换 AI Provider 到 [{}]", provider);
        true
    } else {
        log::error!("无效的 AI Provider [{}]", provider);
        false
    }
}

// Get the list of available models for the current LLM provider.
#[tauri::command]
pub fn models() -> Vec<String> {
    log::debug!("正在获取可用模型列表...");
    let models = CONFIG.llm.current().available_models();
    log::debug!("成功获取模型列表: {:?}", models);
    models
}

// Set the API key for the current LLM provider. 
#[tauri::command]
pub fn set_key(key: String) {
    log::debug!("正在设置 [{:?}] 的 API 密钥...", CONFIG.llm.provider);
    CONFIG.llm.current().set_key(&key);
    log::debug!("成功设置 [{:?}] 的 API 密钥", CONFIG.llm.provider);
}

// Switch the current model for the current LLM provider to the specified one.
#[tauri::command]
pub fn switch_model(model: String) {
    log::debug!("正在切换模型到 [{}]...", model);
    CONFIG.llm.current().switch_model(&model);
    log::debug!("成功切换模型到 [{}]", model);
}
