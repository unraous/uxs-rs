use super::{AnswerItem, SYSTEM_PROMPT, LLM};

use crate::core::qa_pipeline::html::Question;
use crate::config::llm::deepseek::DeepSeekConfig;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
impl LLM for DeepSeekConfig {
    fn api_key(&self) -> String { self.api_key.lock().clone() }
    fn set_key(&self, key: &str) { *self.api_key.lock() = key.to_string(); }
    fn available_models(&self) -> Vec<String> { self.models.clone() }
    fn current_model(&self) -> String { self.chosen_model.lock().clone() }
    fn switch_model(&self, model: &str) { *self.chosen_model.lock() = model.to_string(); }

    async fn solve(&self, question: Vec<Question>) -> Result<Vec<AnswerItem>> {
        log::debug!("将使用 DeepSeek 模型 {} 进行推理", *self.chosen_model.lock());

        let request_body = serde_json::json!({
            "model": self.chosen_model,
            "messages": [
                {
                    "role": "system",
                    "content": SYSTEM_PROMPT
                },
                {
                    "role": "user",
                    "content": serde_json::to_string(&question)?
                }
            ],
            "temperature": 0.3,
            "top_p": 0.95,
            "response_format": {
                "type": "json_object"
            }
        });

        let response = reqwest::Client::new()
            .post("https://api.deepseek.com/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key.lock()))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        // 解析响应
        let data: serde_json::Value = response.json().await?;
        let content = data.pointer("/choices/0/message/content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("无法从 DeepSeek 响应提取文本。原始响应: {}", data))?;
        Ok(serde_json::from_str(content)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_solve_with_local_json() {
        // 1. 获取测试文件路径 (假设在 src-tauri 目录下运行)
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/assets/course-page/decrypted.json");
        
        // 2. 读取并解析 JSON 文件
        let json_str = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("找不到测试文件: {:?}", path));
        let questions: Vec<Question> = serde_json::from_str(&json_str)
            .expect("JSON 解析到 Question 结构失败");

        println!("成功加载了 {} 道题目", questions.len());

        dotenv::dotenv().ok();
        
        // 从环境变量 DEEPSEEK_API_KEY 读取，如果不存在则报错
        let api_key = std::env::var("DEEPSEEK_API_KEY")
            .expect("请在 .env 文件或环境变量中设置 DEEPSEEK_API_KEY");


        let config = DeepSeekConfig { api_key: parking_lot::Mutex::new(api_key), ..Default::default() };

        match config.solve(questions.clone()).await {
            Ok(answers) => {
                assert_eq!(answers.len(), questions.len());
                println!("收到回答，{:?}", answers);
            }
            Err(e) => {
                println!("调用失败: {}", e);
                panic!("测试失败");
            }
        }
    }
}