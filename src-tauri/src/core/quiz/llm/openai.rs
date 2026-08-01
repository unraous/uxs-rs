use super::{AnswerItem, LLM, SYSTEM_PROMPT};

use crate::config::llm::OpenAIConfig;
use crate::core::quiz::html::Question;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
impl LLM for OpenAIConfig {
    fn api_key(&self) -> String {
        self.api_key.lock().clone()
    }
    fn set_key(&self, key: &str) {
        *self.api_key.lock() = key.to_string();
    }
    fn available_models(&self) -> Vec<String> {
        self.models.clone()
    }
    fn current_model(&self) -> String {
        self.chosen_model.lock().clone()
    }
    fn switch_model(&self, model: &str) {
        *self.chosen_model.lock() = model.to_string();
    }

    async fn solve(&self, question: Vec<Question>) -> Result<Vec<AnswerItem>> {
        log::debug!("将使用 OpenAI 模型 {} 进行推理", *self.chosen_model.lock());

        let mut request_body: serde_json::Value =
            serde_json::from_str(include_str!("./openai-request.json"))?;
        request_body["model"] = serde_json::json!(*self.chosen_model.lock());
        request_body["instructions"] = serde_json::json!(SYSTEM_PROMPT);
        request_body["input"] = serde_json::json!(serde_json::to_string(&question)?);

        let response = reqwest::Client::new()
            .post("https://api.openai.com/v1/responses")
            .header("Authorization", format!("Bearer {}", self.api_key.lock()))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let data: serde_json::Value = response.json().await?;

        let content = data
            .pointer("/output/1/content/0/text")
            .and_then(|v| v.as_str())
            .or_else(|| {
                data["output"]
                    .as_array()?
                    .iter()
                    .find(|o| o["type"] == "message")?
                    .pointer("/content/0/text")?
                    .as_str()
            })
            .ok_or_else(|| anyhow::anyhow!("无法从 OpenAI 响应提取文本。原始响应: {}", data))?;
        Ok(serde_json::from_str::<Vec<AnswerItem>>(content)?)
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
        let json_str =
            fs::read_to_string(&path).unwrap_or_else(|_| panic!("找不到测试文件: {:?}", path));
        let questions: Vec<Question> =
            serde_json::from_str(&json_str).expect("JSON 解析到 Question 结构失败");

        println!("成功加载了 {} 道题目", questions.len());

        dotenv::dotenv().ok();

        // 从环境变量 OPENAI_API_KEY 读取，如果不存在则报错
        let api_key =
            std::env::var("OPENAI_API_KEY").expect("请在 .env 文件或环境变量中设置 OPENAI_API_KEY");

        let config = OpenAIConfig {
            api_key: parking_lot::Mutex::new(api_key),
            ..Default::default()
        };

        match config.solve(questions.clone()).await {
            Ok(answers) => {
                assert_eq!(answers.len(), questions.len());
                for (i, answer) in answers.iter().enumerate() {
                    println!("问题 {} 的回答: {:?}", i + 1, answer);
                }
            }
            Err(e) => {
                println!("调用失败: {}", e);
                panic!("测试失败");
            }
        }
    }
}
