mod bigmodel;
mod deepseek;
mod google;
mod local_ollama;
mod moonshot;
mod openai;
mod openrouter;

use super::html::Question;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

const SYSTEM_PROMPT: &str = include_str!("./system_prompt.txt");

#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerItem {
    #[serde(alias = "题号", alias = "id")]
    pub index: String,
    #[serde(alias = "解析")]
    pub explanation: String,
    #[serde(alias = "答案", alias = "answer")]
    pub content: String,
}

#[async_trait]
pub trait LLM {
    fn api_key(&self) -> String;
    fn set_key(&self, key: &str);
    fn available_models(&self) -> Vec<String>;
    fn current_model(&self) -> String;
    fn switch_model(&self, model: &str);

    async fn solve(&self, question: Vec<Question>) -> Result<Vec<AnswerItem>>;
}

pub fn validate(model: &str, questions: &[Question]) -> Result<()> {
    if questions.is_empty() {
        anyhow::bail!("接收到空的题目列表");
    }
    if model.is_empty() {
        anyhow::bail!("未选择 AI 模型");
    }
    Ok(())
}
