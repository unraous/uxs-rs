use super::{AnswerItem, SYSTEM_PROMPT, LLM};

use crate::core::qa_pipeline::html::Question;
use crate::config::llm::google::GoogleConfig;

use async_trait::async_trait;

#[async_trait]
impl LLM for GoogleConfig {
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
        let request_body = serde_json::json!({
            "systemInstruction": {
                "parts": [
                    {
                        "text": SYSTEM_PROMPT,
                    }
                ]
            },
            "contents": [
                {
                    "parts": [
                        {
                            "text": serde_json::to_string(&question)?,
                        }
                    ]
                }
            ],
            "generationConfig": {
                "responseFormat": {
                    "text": {
                        "mimeType": "APPLICATION_JSON",
                        "schema": {
                            "type": "ARRAY",
                            "items": {
                                "type": "OBJECT",
                                "properties": {
                                    "题号": { 
                                        "type": "STRING",
                                        "description": "题目的编号或索引，应与输入的题目列表中的题号对应"
                                    },
                                    "解析": { 
                                        "type": "STRING",
                                        "description": "对题目的解析/分析内容，包含错别字的猜测与修正，以及对题目选项的分析，最后给出正确答案的选择理由"
                                    },
                                    "答案": { 
                                        "type": "STRING",
                                        "description": "题目的答案，通常是选项中的一个，如 A、B、C、D 等，或者是简答题的直接答案文本，具体取决于题目的类型"
                                    }
                                },
                                "required": ["题号", "解析", "答案"]
                            }
                        }
                    }
                }
            }
        });

        // 发送 HTTP 请求
        let client = reqwest::Client::new();
        let response = client
            .post(format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent", self.chosen_model))
            .header("x-goog-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        // 解析响应
        let data: serde_json::Value = response.json().await?;

        if let Some(content) = data["candidates"][0]["content"]["parts"][0]["text"].as_str() {
            // 解析 LLM 返回 of JSON 答案数组
            let answers = serde_json::from_str::<Vec<AnswerItem>>(content)?;
            Ok(answers)
        } else {
            Err(format!("No valid content in Google response: {}", data).into())
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