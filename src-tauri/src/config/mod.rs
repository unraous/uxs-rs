mod path;
mod metadata;

use metadata::MetadataConfig;
use path::PathsConfig;

use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use std::fs;
use toml;
use log::{info, warn};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub metadata: MetadataConfig,
    pub paths: PathsConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            metadata: MetadataConfig::default(),
            paths: PathsConfig::default(),
        }
    }
}

impl Config {
    fn load() -> Self {
        let config = Self::default();
        match toml::from_str::<Config>(
            fs::read_to_string(&config.paths.file("config"))
                .unwrap_or_default().as_str()
        ) {
            Ok(loaded) => loaded,
            Err(e) => {
                warn!("加载配置失败，使用默认配置: {}", e);
                config
            }
        }
        
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(
            &self.paths.file("config"),
            toml::to_string_pretty(&self)?
        )?;
        info!("配置已保存到 TOML 文件");
        Ok(())
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::load);    
