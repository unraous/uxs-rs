use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DeepSeekConfig {
    pub api_key: Mutex<String>,
    pub models: Vec<String>,
    pub chosen_model: String,
}

impl Default for DeepSeekConfig {
    fn default() -> Self {
        Self {
            api_key: Mutex::new(String::new()),
            models: vec![
                String::from("deepseek-v4-flash"),
                String::from("deepseek-v4-pro"),
            ],
            chosen_model: String::from("deepseek-v4-flash"),
        }
    }
}