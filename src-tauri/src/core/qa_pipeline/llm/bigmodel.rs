use super::{AnswerItem, SYSTEM_PROMPT, LLM};

use crate::core::qa_pipeline::html::Question;
use crate::config::llm::BigModelConfig;

use async_trait::async_trait;

#[async_trait]
impl LLM for BigModelConfig {
    async fn solve(&self, question: Vec<Question>) -> Result<Vec<AnswerItem>, Box<dyn std::error::Error>> {
        // 参数验证
        if self.api_key.is_empty() {
            log::error!("BigModel API key 未配置");
            return Err("BigModel API key is empty".into());
        }
        if question.is_empty() {
            log::warn!("接收到空的题目列表，将返回空答案数组");
            return Ok(Vec::new());
        }

        if self.chosen_model.is_empty() {
            return Err("No BigModel model selected".into());
        }

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

        // 发送 HTTP 请求
        let client = reqwest::Client::new();
        let response = client
            .post("https://open.bigmodel.cn/api/paas/v4/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        // 解析响应
        let data: serde_json::Value = response.json().await?;

        if let Some(content) = data["choices"][0]["message"]["content"].as_str() {
            // 解析 LLM 返回的 JSON 答案数组
            let answers = serde_json::from_str::<Vec<AnswerItem>>(content)?;
            Ok(answers)
        } else {
            Err(format!("No valid content in BigModel response: {}", data).into())
        }
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
        
        // 从环境变量 BIGMODEL_API_KEY 读取，如果不存在则报错
        let api_key = std::env::var("BIGMODEL_API_KEY")
            .expect("请在 .env 文件或环境变量中设置 BIGMODEL_API_KEY");


        let config = BigModelConfig { api_key, ..Default::default() };

        match config.solve(questions.clone()).await {
            Ok(answers) => {
                assert_eq!(answers.len(), questions.len());
                for (i, answer) in answers.iter().enumerate() {
                    println!("问题 {} 的回答: {:?}", i + 1, answer);
                }
            }
            Err(e) => {
                println!("调用失败 (可能是因为 mock-key): {}", e);
            }
        }
    }
}