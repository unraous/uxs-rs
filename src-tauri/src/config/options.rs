use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OptionsConfig {
    pub launch_option: u8,
    pub force_speed: bool,
    pub speed: u8,
}

impl Default for OptionsConfig {
    fn default() -> Self {
        Self {
            launch_option: 1,
            force_speed: false,
            speed: 2,
        }
    }
}