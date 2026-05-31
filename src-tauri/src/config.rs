mod path;
mod metadata;
mod options;

pub mod llm;

use metadata::MetadataConfig;
use path::PathsConfig;
use options::OptionsConfig;
use llm::LLMConfig;

use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
    pub metadata: MetadataConfig,
    pub paths: PathsConfig,
    pub options: OptionsConfig,
    pub llm: LLMConfig,
}

impl Config {
    // In there the logget has not been initialized, so just ignore the error and use default config when failed to load the config file.
    // Hope there won't be a hidden danger in this.
    fn load() -> Self {
        let config = Self::default();
        match toml::from_str::<Config>(
            std::fs::read_to_string(&config.paths.files["config"])
                .unwrap_or_default().as_str()
        ) {
            Ok(loaded) => loaded,
            Err(_) => config
        } 
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::write(
            &self.paths.files["config"],
            toml::to_string_pretty(&self)?
        )?;
        log::info!("配置已保存到 {}", self.paths.files["config"].display());
        Ok(())
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::load);    
