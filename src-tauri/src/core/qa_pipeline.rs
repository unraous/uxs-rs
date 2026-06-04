pub mod html;
pub mod mapper;
pub mod recognizer;
pub mod render;
pub mod llm;

use html::QuestionsRaw;
use mapper::decrypt;
use llm::AnswerItem;

use crate::config::CONFIG;
use anyhow::Result;


/// A completely stateless, asynchronous function that takes raw HTML content,
/// dynamically extracts and decrypts obfuscated questions using the CRNN ONNX model,
/// solves them via the active LLM configured in CONFIG, and returns the solved AnswerItems.
pub async fn execute_qa_workflow(html: &str) -> Result<Vec<AnswerItem>> {
    let decrypted = decrypt(QuestionsRaw::new(html)?);
    CONFIG.llm.current().solve(decrypted).await
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::fs;
    use llm::LLM;
    use crate::config::llm::bigmodel::BigModelConfig;

    #[tokio::test]
    async fn test_solve_html_integration() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("BIGMODEL_API_KEY")
            .expect("请在 .env 文件或环境变量中设置 BIGMODEL_API_KEY");

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/assets/course-page/webpage.html");
        let html_content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to find test webpage.html at {:?}", path));

        let raw = QuestionsRaw::new(&html_content).expect("Failed to parse HTML questions");
        let decrypted = decrypt(raw);
        assert!(!decrypted.is_empty(), "解密后的题目列表不应为空");

        let local_solver = BigModelConfig {
            api_key: parking_lot::Mutex::new(api_key),
            ..Default::default()
        };

        println!("正在使用内存局部配置变量调用 BigModel 求解器...");
        match local_solver.solve(decrypted).await {
            Ok(answers) => {
                assert!(!answers.is_empty(), "返回的答案列表不应为空");
                for (i, item) in answers.iter().enumerate() {
                    println!("[{}] 题号: {}, 答案: {}", i + 1, item.id, item.answer);
                    println!("    解析: {}", item.explanation);
                }
            }
            Err(e) => {
                panic!("无状态局部求解器执行测试失败: {}", e);
            }
        }
    }
}
