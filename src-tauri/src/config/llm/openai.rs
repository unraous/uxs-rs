use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct OpenAIConfig {
    pub api_key: Mutex<String>,
    pub models: Vec<String>,
    pub chosen_model: Mutex<String>,
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        Self {
            api_key: Mutex::new(String::new()),
            models: vec![
                String::from("gpt-5-nano"),
                String::from("gpt-5"),
                String::from("gpt-5.4-mini"),
                String::from("gpt-5.4"),
            ],
            chosen_model: Mutex::new(String::from("gpt-5")),
        }
    }
}