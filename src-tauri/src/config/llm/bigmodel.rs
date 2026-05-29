use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BigModelConfig {
    pub api_key: String,
    pub models: Vec<String>,
    pub chosen_model: String,
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
            chosen_model: String::from("glm-4.7-flash"),
        }
    }
}