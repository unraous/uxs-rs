mod deepseek;
mod bigmodel;
mod google;
mod moonshot;
mod openai;
mod openrouter;
mod local_ollama;

use super::html::Question;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

const SYSTEM_PROMPT: &str = include_str!("./system_prompt.txt");

#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerItem {
    #[serde(rename = "题号", alias = "index")]
    pub id: String,
    #[serde(rename = "解析", alias = "explanation")]
    pub explanation: String,
    #[serde(rename = "答案", alias = "content")]
    pub answer: String,
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
    if questions.is_empty() { anyhow::bail!("接收到空的题目列表"); }
    if model.is_empty() { anyhow::bail!("未选择 AI 模型"); }
    Ok(())
}

