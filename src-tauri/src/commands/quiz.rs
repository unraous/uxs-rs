use super::CommandsResult;

use crate::core::quiz::{llm::AnswerItem, solve_quiz_from};

#[tauri::command]
pub async fn solve_quiz(html: String) -> CommandsResult<Vec<AnswerItem>> {
    Ok(solve_quiz_from(&html).await?)
}
