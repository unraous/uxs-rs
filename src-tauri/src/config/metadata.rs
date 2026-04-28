use serde::{Serialize, Deserialize};
use log::LevelFilter;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetadataConfig {
    pub author: String,
    pub title: String,
    pub version: String,
    pub log_level: LevelFilter,
}

impl Default for MetadataConfig {
    fn default() -> Self {
        Self {
            author: "unraous".into(),
            title: "uxuescript".into(),
            version: "2.0.0".into(),
            log_level: match cfg!(debug_assertions) {
                true => LevelFilter::Debug,
                false => LevelFilter::Info,
            },
        }
    }
}