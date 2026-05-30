use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct LocalOllamaConfig {
    pub models: Vec<String>,
    pub chosen_model: String,
}

#[derive(Deserialize)]
struct OllamaResponse {
    models: Vec<OllamaModelItem>,
}

#[derive(Deserialize)]
struct OllamaModelItem {
    model: String,
}


impl LocalOllamaConfig {
    async fn load_models(&mut self) {
        let response = reqwest::get("http://localhost:11434/api/tags")
            .await
            .unwrap_or_else(|e| {
                log::error!("无法获取模型列表: {}", e);
                panic!("无法获取模型列表");
            });
        
        match response.json::<OllamaResponse>().await {
            Ok(data) => {
                self.models = data.models.into_iter().map(|item| item.model).collect();
                if !self.models.is_empty() && !self.models.contains(&self.chosen_model) {
                    self.chosen_model = self.models.first().cloned().unwrap_or_default();
                }
            }
            Err(e) => {
                log::error!("解析模型列表失败: {}", e);
                self.models = Vec::new();
                self.chosen_model = String::new();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test] // 1. 使用 tokio::test 支持异步
    async fn test_local_ollama() { // 2. 函数标记为 async
        let mut config = LocalOllamaConfig::default(); // 3. 加上 mut，因为 load_models 需要修改 self
        
        assert_eq!(config.models, Vec::<String>::new());
        assert_eq!(config.chosen_model, String::new());

        // 4. 现在可以正常 await 了
        // 注意：这个测试在运行时需要你的本地 Ollama 服务处于启动状态
        config.load_models().await;
        
        println!("Fetched models: {:?}", config.models);
    }
}
