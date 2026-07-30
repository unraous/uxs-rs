use super::CommandsResult;

use crate::core::quiz::{llm::AnswerItem, solve};

#[tauri::command]
pub async fn solve_quiz(html: String) -> CommandsResult<Vec<AnswerItem>> {
    match solve(&html).await {
        Ok(answers) => {
            log::info!("solve_quiz 答题成功，获得答案数量: {}", answers.len());
            Ok(answers)
        }
        Err(e) => {
            log::error!("solve_quiz 答题失败，详细原因: {:?}", e);
            Err(e.into())
        }
    }
}
