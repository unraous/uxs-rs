use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GoogleConfig {
    pub api_key: Mutex<String>,
    pub models: Vec<String>,
    pub chosen_model: String,
}

impl Default for GoogleConfig {
    fn default() -> Self {
        Self {
            api_key: Mutex::new(String::new()),
            models: vec![
                String::from("gemini-3.1-flash-lite"),
                String::from("gemini-3.5-flash"),
                String::from("gemini-3.1-pro"),
                String::from("gemini-flash-lite-latest"),
                String::from("gemini-flash-latest"),
                String::from("gemini-pro-latest"),
                String::from("gemma-4-31b-it")
            ],
            chosen_model: String::from("gemini-3.1-flash-lite"),
        }
    }
}

