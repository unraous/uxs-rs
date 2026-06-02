use crate::config::CONFIG;
use crate::config::llm::LLMProvider;
use crate::core::qa_pipeline::llm::LLM;

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
pub fn switch_provider(provider: String) {
    log::debug!("正在切换 LLM 提供商到 [{}]", provider);
    match provider.as_str() {
        "BigModel" => *CONFIG.llm.provider.lock() = LLMProvider::BigModel,
        "DeepSeek" => *CONFIG.llm.provider.lock() = LLMProvider::DeepSeek,
        "Google" => *CONFIG.llm.provider.lock() = LLMProvider::Google,
        "Moonshot" => *CONFIG.llm.provider.lock() = LLMProvider::Moonshot,
        "Ollama" => *CONFIG.llm.provider.lock() = LLMProvider::Ollama,
        "OpenAI" => *CONFIG.llm.provider.lock() = LLMProvider::OpenAI,
        "Openrouter" => *CONFIG.llm.provider.lock() = LLMProvider::Openrouter,
        _ => log::warn!("未知的 LLM 提供商: [{}]", provider),
    }
    log::debug!("成功切换 LLM 提供商到 [{}]", provider);
}

#[tauri::command]
pub fn models() -> Vec<String> {
    log::debug!("正在获取可用模型列表...");
    let models = match *CONFIG.llm.provider.lock() {
        LLMProvider::BigModel => CONFIG.llm.bigmodel.models.clone(),
        LLMProvider::DeepSeek => CONFIG.llm.deepseek.models.clone(),
        LLMProvider::Google => CONFIG.llm.google.models.clone(),
        LLMProvider::Ollama => CONFIG.llm.ollama.models.clone(),
        LLMProvider::Moonshot => CONFIG.llm.moonshot.models.clone(),
        LLMProvider::OpenAI => CONFIG.llm.openai.models.clone(),
        LLMProvider::Openrouter => CONFIG.llm.openrouter.models.clone(),
    };
    log::debug!("成功获取模型列表: {:?}", models);
    models
}

#[tauri::command]
pub fn set_key(key: String) {
    log::debug!("正在设置 [{:?}] 的 API 密钥...", CONFIG.llm.provider);
    match *CONFIG.llm.provider.lock() {
        LLMProvider::BigModel => CONFIG.llm.bigmodel.set_key(&key),
        LLMProvider::DeepSeek => CONFIG.llm.deepseek.set_key(&key),
        LLMProvider::Google => CONFIG.llm.google.set_key(&key),
        LLMProvider::Ollama => CONFIG.llm.ollama.set_key(&key),
        LLMProvider::Moonshot => CONFIG.llm.moonshot.set_key(&key),
        LLMProvider::OpenAI => CONFIG.llm.openai.set_key(&key),
        LLMProvider::Openrouter => CONFIG.llm.openrouter.set_key(&key),
    }
    log::debug!("成功设置 [{:?}] 的 API 密钥", CONFIG.llm.provider);
}