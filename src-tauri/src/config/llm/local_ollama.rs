use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
struct OllamaModelItem {
    model: String,
}

#[derive(Deserialize)]
struct OllamaResponse {
    models: Vec<OllamaModelItem>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LocalOllamaConfig {
    pub models: Vec<String>,
    pub chosen_model: String,
}

impl LocalOllamaConfig {
    async fn fetch_models_from(&mut self, url: &str) -> Result<(), reqwest::Error> {
        let data = reqwest::get(url).await?.json::<OllamaResponse>().await?;

        self.models = data.models.into_iter().map(|item| item.model).collect();
        if let Some(first_model) = self.models.first() {
            if !self.models.contains(&self.chosen_model) {
                self.chosen_model = first_model.clone();
            }
        }
        
        Ok(())
    }

    async fn fetch_from(&mut self, url: &str) {
        if let Err(e) = self.fetch_models_from(url).await {
            log::warn!("Ollama 获取失败 (可能未启动或解析错误): {}", e);
            self.models.clear();
            self.chosen_model.clear();
        }
    }

    pub async fn load_models(&mut self) {
        self.fetch_from("http://localhost:11434/api/tags").await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_local_ollama_normal() {
        let mut config = LocalOllamaConfig::default();
        config.load_models().await;
        println!("Fetched models: {:?}", config.models);
    }

    #[tokio::test]
    async fn test_local_ollama_offline() {
        let mut config = LocalOllamaConfig {
            models: vec!["dummy-model".to_string()],
            chosen_model: "dummy-model".to_string(),
        };
        // 模拟 Ollama 挂掉 / 端口不通的情况
        config.fetch_from("http://localhost").await;
        
        assert!(config.models.is_empty(), "网络不通时 models 应该被清空");
        assert!(config.chosen_model.is_empty(), "网络不通时 chosen_model 应该被置空");
    }
}
