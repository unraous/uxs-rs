use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MoonshotConfig {
    pub api_key: String,
    pub models: Vec<String>,
    pub chosen_model: String,
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
            chosen_model: String::from("moonshot-v1-8k"),
        }
    }
}