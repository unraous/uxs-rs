
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoogleConfig {
    pub api_key: String,
    pub models: Vec<String>,
}

impl Default for GoogleConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            models: vec![
                String::from("gemini-3.1-flash-lite"),
                String::from("gemini-3.5-flash"),
                String::from("gemini-3.1-pro"),
                String::from("gemini-flash-lite-latest"),
                String::from("gemini-flash-latest"),
                String::from("gemini-pro-latest"),
                String::from("gemma-4-31b-it")
            ],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MoonshotConfig {
    pub api_key: String,
    pub models: Vec<String>,
}

impl Default for MoonshotConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            models: vec![
                String::from("moonshot-v1-8k"),
                String::from("kimi-k2.6"),
                String::from("kimi-k2.5"),
            ],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BigModelConfig {
    pub api_key: String,
    pub models: Vec<String>,
}

impl Default for BigModelConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            models: vec![
                String::from("glm-4.7-flash"),
                String::from("glm-4.7"),
                String::from("glm-5"),
                String::from("glm-5-turbo"),
                String::from("glm-5.1"),
            ],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAIConfig {
    pub api_key: String,
    pub models: Vec<String>,
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            models: vec![
                String::from("gpt-5-nano"),
                String::from("gpt-5"),
                String::from("gpt-5.4-mini"),
                String::from("gpt-5.4"),
            ],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenrouterConfig {
    pub api_key: String,
    pub models: Vec<String>,
}

impl Default for OpenrouterConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            models: vec![
                String::from("deepseek/deepseek-v4-flash:free"),
                String::from("nvidia/nemotron-3-nano-omni-30b-a3b-reasoning:free"),
                String::from("google/gemma-4-31b-it:free"),
            ],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct LocalOllamaConfig {
    pub models: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CustomLLMConfig {
    pub api_key: String,
    pub model: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct LLMConfig {
    pub google: GoogleConfig,
    pub moonshot: MoonshotConfig,
    pub bigmodel: BigModelConfig,
    pub openai: OpenAIConfig,
    pub openrouter: OpenrouterConfig,
    pub local_ollama: LocalOllamaConfig,
    pub custom: CustomLLMConfig,
}
