use super::html::Question;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerItem {
    #[serde(rename = "题号", alias = "index")]
    pub id: String,
    #[serde(rename = "解析", alias = "explanation")]
    pub explanation: String,
    #[serde(rename = "答案", alias = "content")]
    pub answer: String,
}

trait LLM {
    fn solve(&self, question: Vec<Question>) -> Vec<AnswerItem>;
}

