pub mod bigmodel;
pub mod deepseek;
pub mod google;
pub mod moonshot;
pub mod openai;
pub mod openrouter;
pub mod local_ollama;

use bigmodel::BigModelConfig;
use deepseek::DeepSeekConfig;
use google::GoogleConfig;
use moonshot::MoonshotConfig;
use openai::OpenAIConfig;
use openrouter::OpenrouterConfig;
use local_ollama::LocalOllamaConfig;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CustomLLMConfig {
    pub api_key: String,
    pub model: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct LLMConfig {
    pub deepseek: DeepSeekConfig,
    pub google: GoogleConfig,
    pub moonshot: MoonshotConfig,
    pub bigmodel: BigModelConfig,
    pub openai: OpenAIConfig,
    pub openrouter: OpenrouterConfig,
    pub local_ollama: LocalOllamaConfig,
    pub custom: CustomLLMConfig,
}
