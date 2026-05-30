use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoogleConfig {
    pub api_key: String,
    pub models: Vec<String>,
    pub chosen_model: String,
}

impl Default for GoogleConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            models: vec![
                String::from("gemini-3.1-flash-lite"),
                String::from("gemini-3.5-flash"),
                String::from("gemini-3.1-pro"),
                String::from("gemini-flash-lite-latest"),
                String::from("gemini-flash-latest"),
                String::from("gemini-pro-latest"),
                String::from("gemma-4-31b-it")
            ],
            chosen_model: String::from("gemini-3.1-flash-lite"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let config = GoogleConfig::default();

        // 1. 验证 chosen_model 是否包含在可选列表 models 中
        assert!(
            config.models.contains(&config.chosen_model),
            "默认的 chosen_model [{}] 不在模型列表 {:?} 中",
            config.chosen_model,
            config.models
        );

        // 2. 验证序列化与反序列化是否保持一致
        // 需要确保项目依赖中有 serde_json
        let serialized = serde_json::to_string(&config).expect("序列化失败");
        let deserialized: GoogleConfig = serde_json::from_str(&serialized).expect("反序列化失败");

        assert_eq!(deserialized.chosen_model, config.chosen_model);
        assert_eq!(deserialized.models, config.models);
        assert_eq!(deserialized.api_key, config.api_key);
    }

}
