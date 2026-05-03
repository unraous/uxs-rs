use crate::config::CONFIG;

use log::debug;


// Get application metadata such as version and author information.
#[tauri::command]
pub fn metadata(key: String) -> String {
    debug!("正在获取元数据 [{}]", key);
    let res = match key.as_str() {
        "version" => CONFIG.metadata.version.clone(),
        "author" => CONFIG.metadata.author.clone(),
        _ => "unknown".to_string(),
    };
    debug!("成功获取元数据 [{}]: {}", key, res);
    res
}