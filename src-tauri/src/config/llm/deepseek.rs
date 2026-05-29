use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeepSeekConfig {
    pub api_key: String,
    pub models: Vec<String>,
    pub chosen_model: String,
}

impl Default for DeepSeekConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            models: vec![
                String::from("deepseek-v4-flash"),
                String::from("deepseek-v4-pro"),
            ],
            chosen_model: String::from("deepseek-v4-flash"),
        }
    }
}