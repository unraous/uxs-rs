use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct LocalOllamaConfig {
    pub models: Vec<String>,
    pub chosen_model: String,
}

