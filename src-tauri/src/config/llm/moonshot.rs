use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MoonshotConfig {
    pub api_key: Mutex<String>,
    pub models: Vec<String>,
    pub chosen_model: Mutex<String>,
}

impl Default for MoonshotConfig {
    fn default() -> Self {
        Self {
            api_key: Mutex::new(String::new()),
            models: vec![
                String::from("moonshot-v1-8k"),
                String::from("kimi-k2.6"),
                String::from("kimi-k2.5"),
            ],
            chosen_model: Mutex::new(String::from("moonshot-v1-8k")),
        }
    }
}