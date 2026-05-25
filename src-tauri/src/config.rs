mod path;
mod metadata;
mod llm;

use metadata::MetadataConfig;
use path::PathsConfig;
use llm::LLMConfig;

use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use std::fs;
use toml;
use log::info;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
    pub metadata: MetadataConfig,
    pub paths: PathsConfig,
    pub llm: LLMConfig,
}
impl Config {
    // In there the logget has not been initialized, so just ignore the error and use default config when failed to load the config file.
    // Hope there won't be a hidden danger in this.
    fn load() -> Self {
        let config = Self::default();
        match toml::from_str::<Config>(
            fs::read_to_string(&config.paths.files["config"])
                .unwrap_or_default().as_str()
        ) {
            Ok(loaded) => loaded,
            Err(_) => config
        } 
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(
            &self.paths.files["config"],
            toml::to_string_pretty(&self)?
        )?;
        info!("配置已保存到 {}", self.paths.files["config"].display());
        Ok(())
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::load);    
