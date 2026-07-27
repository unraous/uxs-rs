use crate::config::{llm::LLMProvider, options::OptionsConfig, CONFIG};
use strum::IntoEnumIterator;

// Get application metadata such as version and author information.
#[tauri::command]
pub fn metadata(key: String) -> String {
    log::debug!("正在获取元数据 [{}]", key);
    let res = match key.as_str() {
        "version" => CONFIG.metadata.version.clone(),
        "author" => CONFIG.metadata.author.clone(),
        _ => "unknown".to_string(),
    };
    log::info!("成功获取元数据 [{}]: {}", key, res);
    res
}

#[tauri::command]
pub fn options() -> OptionsConfig {
    log::debug!("正在获取配置信息...");
    let options = *CONFIG.options.lock();
    log::info!("成功获取配置信息: {:?}", options);
    options
}

#[tauri::command]
pub fn set_options(options: OptionsConfig) {
    log::debug!("正在设置配置信息...");
    *CONFIG.options.lock() = options;
    save_config();
    log::info!("成功设置配置信息: {:?}", options);
}

// Get the list of available LLM providers.
#[tauri::command]
pub fn providers() -> Vec<String> {
    log::debug!("正在获取可用 AI Provider 列表...");
    let providers: Vec<String> = LLMProvider::iter()
        .map(|p| p.as_ref().to_string())
        .collect();
    log::info!("成功获取 AI Provider 列表: {:?}", providers);
    providers
}

// Get the current LLM provider.
#[tauri::command]
pub fn current_provider() -> String {
    let provider = CONFIG.llm.provider.lock();
    log::debug!("正在获取当前 AI Provider: {}", provider.as_ref());
    provider.as_ref().to_string()
}

// Switch the current LLM provider to the specified one, returning true if successful, false otherwise.
#[tauri::command]
pub fn switch_provider(provider: String) -> bool {
    log::debug!("正在切换 AI Provider 到 [{}]", provider);
    if let Ok(llmp) = provider.parse::<LLMProvider>() {
        CONFIG.llm.switch_to(llmp);
        log::info!("成功切换 AI Provider 到 [{}]", provider);
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
    log::info!("成功获取模型列表: {:?}", models);
    models
}

// Get the current model for the current LLM provider.
#[tauri::command]
pub fn current_model() -> String {
    log::debug!(
        "当正在获取当前模型: {}",
        CONFIG.llm.current().current_model()
    );
    CONFIG.llm.current().current_model()
}

// Switch the current model for the current LLM provider to the specified one.
#[tauri::command]
pub fn switch_model(model: String) {
    log::debug!("正在切换模型到 [{}]...", model);
    CONFIG.llm.current().switch_model(&model);
    log::info!("成功切换模型到 [{}]", model);
}

#[tauri::command]
pub fn api_key() -> String {
    log::debug!("正在获取当前 API Key...");
    let key = CONFIG.llm.current().api_key();
    log::debug!("成功获取当前 API Key: {}...", key);
    key
}

// Set the API key for the current LLM provider.
#[tauri::command]
pub fn set_key(key: String) {
    log::debug!("正在设置 [{:?}] 的 API 密钥...", CONFIG.llm.provider);
    CONFIG.llm.current().set_key(&key);
    log::info!("成功设置 [{:?}] 的 API 密钥", CONFIG.llm.provider);
    save_config();
}

#[tauri::command]
pub fn save_config() {
    log::debug!("正在保存配置文件...");
    if let Err(e) = CONFIG.save() {
        log::error!("保存配置文件失败: {}", e);
    } else {
        log::info!("成功保存配置文件");
    }
}
