use super::{AnswerItem, SYSTEM_PROMPT, LLM};

use crate::core::qa_pipeline::html::Question;
use crate::config::llm::google::GoogleConfig;

use async_trait::async_trait;

#[async_trait]
impl LLM for GoogleConfig {
    fn set_key(&mut self, key: &str) {
        self.api_key = key.to_string();
        log::info!("Google API key 已更新");
    }

    async fn solve(&self, question: Vec<Question>) -> Result<Vec<AnswerItem>, Box<dyn std::error::Error>> {
        // 参数验证
        if self.api_key.is_empty() {
            log::error!("Google API key 未配置");
            return Err("Google API key is empty".into());
        }
        if question.is_empty() {
            log::warn!("接收到空的题目列表，将返回空答案数组");
            return Ok(Vec::new());
        }

        if self.chosen_model.is_empty() {
            return Err("No Google model selected".into());
        }

        // see reference: https://ai.google.dev/gemini-api/docs/text-generation?hl=zh-cn#rest
        // the mimeType is not string but enum, so need to use ""APPLICATION_JSON"" instead of "application/json"
        let mut request_body: serde_json::Value = serde_json::from_str(
            include_str!("./google-request.json")
        )?;

        // 2. 动态注入变量 (直接通过路径赋值)
        request_body["systemInstruction"]["parts"][0]["text"] = serde_json::json!(SYSTEM_PROMPT);
        request_body["contents"][0]["parts"][0]["text"] = serde_json::json!(serde_json::to_string(&question)?);        

        // 3. 发送请求
        let client = reqwest::Client::new();
        let response = client
            .post(format!(
                "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
                self.chosen_model
            ))
            .header("x-goog-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        // 解析响应
        let data: serde_json::Value = response.json().await?;

        let content = data["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| format!("No valid content in Google response: {}", data))?;

        let answers = serde_json::from_str::<Vec<AnswerItem>>(content)?;
        Ok(answers)
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
        
        // 从环境变量 GOOGLE_API_KEY 读取，如果不存在则报错
        let api_key = std::env::var("GOOGLE_API_KEY")
            .expect("请在 .env 文件或环境变量中设置 GOOGLE_API_KEY");


        let config = GoogleConfig { api_key, ..Default::default() };

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