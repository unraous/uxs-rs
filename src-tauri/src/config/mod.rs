mod path;
mod metadata;

use metadata::MetadataConfig;
use path::PathsConfig;

use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;

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

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::default()
});    
