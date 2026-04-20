use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use log::LevelFilter;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetadataConfig {
    pub name: String,
    pub version: String,
    pub log_level: LevelFilter,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PathsConfig {
    pub cwd: PathBuf,
    pub scripts: ScriptsConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScriptsConfig {
    pub directory: PathBuf,
    
    pub course: String,
    pub login: String,
    pub main_page: String,
    pub mask: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub metadata: MetadataConfig,
    pub paths: PathsConfig,
}

fn cwd() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

impl Default for MetadataConfig {
    fn default() -> Self {
        Self {
            name: "uxuescript".into(),
            version: "2.0.0".into(),
            log_level: match cfg!(debug_assertions) {
                true => LevelFilter::Debug,
                false => LevelFilter::Info,
            },
        }
    }
}

impl Default for PathsConfig {
    fn default() -> Self {
        Self {
            cwd: cwd(),
            scripts: ScriptsConfig::default(),
        }
    }
}

impl Default for ScriptsConfig {
    fn default() -> Self {
        Self {
            directory: cwd().join("scripts"),
            course: "core.js".into(),
            login: "click-auto-login.js".into(),
            main_page: "core.js".into(),
            mask: "show-mask.js".into(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            metadata: MetadataConfig::default(),
            paths: PathsConfig::default(),
        }
    }
}
