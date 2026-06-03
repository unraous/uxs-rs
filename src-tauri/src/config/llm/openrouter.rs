use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenrouterConfig {
    pub api_key: Mutex<String>,
    pub models: Vec<String>,
    pub chosen_model: Mutex<String>,
}

impl Default for OpenrouterConfig {
    fn default() -> Self {
        Self {
            api_key: Mutex::new(String::new()),
            models: vec![
                String::from("moonshotai/kimi-k2.6:free"),
                String::from("deepseek/deepseek-v4-flash:free"),
                String::from("nvidia/nemotron-3-nano-omni-30b-a3b-reasoning:free"),
                String::from("google/gemma-4-31b-it:free"),
            ],
            chosen_model: Mutex::new(String::from("moonshotai/kimi-k2.6:free")),
        }
    }
}
