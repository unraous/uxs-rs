use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(default)]
pub struct OptionsConfig {
    pub force_speed: bool,
    pub speed: u8,
}

impl Default for OptionsConfig {
    fn default() -> Self {
        Self {
            force_speed: false,
            speed: 2,
        }
    }
}
