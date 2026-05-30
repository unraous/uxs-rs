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

use crate::core::qa_pipeline::llm::LLM;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct LLMConfig {
    pub provider: LLMProvider,
    bigmodel: BigModelConfig,
    deepseek: DeepSeekConfig,
    google: GoogleConfig,
    moonshot: MoonshotConfig,
    openai: OpenAIConfig,
    openrouter: OpenrouterConfig,
    local_ollama: LocalOllamaConfig,
}



impl LLMConfig {
    pub fn llm(&self) -> &dyn LLM {
        match self.provider {
            LLMProvider::BigModel => &self.bigmodel,
            LLMProvider::DeepSeek => &self.deepseek,
            LLMProvider::Google => &self.google,
            LLMProvider::Moonshot => &self.moonshot,
            LLMProvider::OpenAI => &self.openai,
            LLMProvider::Openrouter => &self.openrouter,
            LLMProvider::LocalOllama => &self.local_ollama,
        }
    }
}