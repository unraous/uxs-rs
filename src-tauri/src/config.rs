mod path;

pub mod llm;
pub mod metadata;
pub mod options;

use llm::LLMConfig;
use metadata::MetadataConfig;
use options::OptionsConfig;
use path::PathsConfig;

use anyhow::Result;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Config {
    pub metadata: MetadataConfig,
    pub paths: PathsConfig,
    pub options: Mutex<OptionsConfig>,
    pub llm: LLMConfig,
}

impl Config {
    // In there the logget has not been initialized, so just ignore the error and use default config when failed to load the config file.
    // Hope there won't be a hidden danger in this.
    fn load() -> Self {
        let config = Self::default();
        match toml::from_str::<Config>(
            std::fs::read_to_string(&config.paths.files["config"])
                .unwrap_or_default()
                .as_str(),
        ) {
            Ok(loaded) => loaded,
            Err(_) => config,
        }
    }

    pub fn save(&self) -> Result<()> {
        std::fs::write(&self.paths.files["config"], toml::to_string_pretty(&self)?)?;
        log::info!("配置已保存到 {}", self.paths.files["config"].display());
        Ok(())
    }
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::load);
