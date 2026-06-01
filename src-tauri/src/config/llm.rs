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

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum LLMProvider {
    #[default]
    BigModel,
    DeepSeek,
    Google,
    Moonshot,
    OpenAI,
    Openrouter,
    LocalOllama,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LLMConfig {
    pub provider: LLMProvider,
    pub bigmodel: BigModelConfig,
    pub deepseek: DeepSeekConfig,
    pub google: GoogleConfig,
    pub moonshot: MoonshotConfig,
    pub openai: OpenAIConfig,
    pub openrouter: OpenrouterConfig,
    pub local_ollama: LocalOllamaConfig,
}
