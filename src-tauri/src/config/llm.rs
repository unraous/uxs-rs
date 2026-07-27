pub mod bigmodel;
pub mod deepseek;
pub mod google;
pub mod moonshot;
pub mod ollama;
pub mod openai;
pub mod openrouter;

use crate::core::quiz::llm::LLM;

use bigmodel::BigModelConfig;
use deepseek::DeepSeekConfig;
use google::GoogleConfig;
use moonshot::MoonshotConfig;
use ollama::OllamaConfig;
use openai::OpenAIConfig;
use openrouter::OpenrouterConfig;

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, EnumString};

#[derive(Serialize, Deserialize, Debug, Default, EnumIter, EnumString, AsRefStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
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
#[serde(default)]
pub struct LLMConfig {
    pub provider: Mutex<LLMProvider>,
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
