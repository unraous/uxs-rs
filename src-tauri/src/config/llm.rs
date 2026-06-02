pub mod bigmodel;
pub mod deepseek;
pub mod google;
pub mod moonshot;
pub mod openai;
pub mod openrouter;
pub mod ollama;

use crate::core::qa_pipeline::llm::LLM; 

use bigmodel::BigModelConfig;
use deepseek::DeepSeekConfig;
use google::GoogleConfig;
use moonshot::MoonshotConfig;
use openai::OpenAIConfig;
use openrouter::OpenrouterConfig;
use ollama::OllamaConfig;

use parking_lot::Mutex;
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
    Ollama,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LLMConfig {
    pub provider: Mutex::<LLMProvider>,
    pub bigmodel: BigModelConfig,
    pub deepseek: DeepSeekConfig,
    pub google: GoogleConfig,
    pub moonshot: MoonshotConfig,
    pub openai: OpenAIConfig,
    pub openrouter: OpenrouterConfig,
    pub ollama: OllamaConfig,
}

impl LLMConfig {
    pub fn switch_to(&self, provider: LLMProvider) {
        *self.provider.lock() = provider;
    }

    pub fn current(&self) -> &dyn LLM {
        match *self.provider.lock() {
            LLMProvider::BigModel => &self.bigmodel,
            LLMProvider::DeepSeek => &self.deepseek,
            LLMProvider::Google => &self.google,
            LLMProvider::Moonshot => &self.moonshot,
            LLMProvider::OpenAI => &self.openai,
            LLMProvider::Openrouter => &self.openrouter,
            LLMProvider::Ollama => &self.ollama,
        }
    }
}