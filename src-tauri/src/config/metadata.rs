use log::LevelFilter;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
#[serde(default)]
pub struct MetadataConfig {
    pub author: String,
    pub title: String,
    pub version: String,
    #[specta(type = String)]
    pub log_level: LevelFilter,
}

impl Default for MetadataConfig {
    fn default() -> Self {
        Self {
            author: "unraous".into(),
            title: "uXueScript".into(),
            version: "2.0.0".into(),
            log_level: if cfg!(debug_assertions) {
                LevelFilter::Debug
            } else {
                LevelFilter::Info
            },
        }
    }
}
