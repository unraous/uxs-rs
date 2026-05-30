pub mod html;
pub mod mapper;
pub mod recognizer;
pub mod render;
pub mod llm;

use html::QuestionsRaw;
use mapper::decrypt;
use llm::AnswerItem;
use crate::config::CONFIG;

/// A completely stateless, asynchronous function that takes raw HTML content,
/// dynamically extracts and decrypts obfuscated questions using the CRNN ONNX model,
/// solves them via the active LLM configured in CONFIG, and returns the solved AnswerItems.
pub async fn solve_html(html: &str) -> Result<Vec<AnswerItem>, String> {
    // 1. Parse raw HTML and extract the scrambled TTF font and questions
    let raw = QuestionsRaw::new(html).map_err(|e| e.to_string())?;
    
    // 2. Decrypt dynamic font obfuscation using the preloaded CRNN ONNX model
    let decrypted = decrypt(raw);
    
    // 3. Load the active LLM provider configured in CONFIG and solve the decrypted questions
    let llm_provider = CONFIG.llm.llm();
    let answers = llm_provider.solve(decrypted).await.map_err(|e| e.to_string())?;
    
    Ok(answers)
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
        // 1. 初始化环境变量并仅在内存中加载 API Key
        dotenv::dotenv().ok();
        let api_key = std::env::var("BIGMODEL_API_KEY")
            .expect("请在 .env 文件或环境变量中设置 BIGMODEL_API_KEY");

        // 2. 读取网页样本 html
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/assets/course-page/webpage.html");
        let html_content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to find test webpage.html at {:?}", path));

        // 3. 执行纯无状态的解析与字形 OCR 解密
        let raw = QuestionsRaw::new(&html_content).expect("Failed to parse HTML questions");
        let decrypted = decrypt(raw);
        assert!(!decrypted.is_empty(), "解密后的题目列表不应为空");

        // 4. 直接在内存中初始化局部变量，完全不碰磁盘上的 config.toml
        let local_solver = BigModelConfig {
            api_key,
            chosen_model: "glm-4-flash".to_string(), // 默认使用快速免费模型
            ..Default::default()
        };

        // 5. 局部的无状态求解器调用并验证
        println!("正在使用内存局部配置变量调用 BigModel 求解器...");
        match local_solver.solve(decrypted).await {
            Ok(answers) => {
                assert!(!answers.is_empty(), "返回的答案列表不应为空");
                println!("\n================== 最终答题结果 (Final Answers) ==================");
                for (i, item) in answers.iter().enumerate() {
                    println!("[{}] 题号: {}, 答案: {}", i + 1, item.id, item.answer);
                    println!("    解析: {}", item.explanation);
                }
                println!("=================================================================\n");
            }
            Err(e) => {
                panic!("无状态局部求解器执行测试失败: {}", e);
            }
        }
    }
}
