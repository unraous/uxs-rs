use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(default, rename_all = "camelCase")] // Maps Rust `snake_case` to TypeScript `camelCase`
pub struct OptionsConfig {
    pub persist_session: bool,
    pub mute_webview: bool,
    pub speed_lock: bool,
    pub speed_value: f32,
}

impl Default for OptionsConfig {
    fn default() -> Self {
        Self {
            persist_session: true,
            mute_webview: true,
            speed_lock: false,
            speed_value: 2.0,
        }
    }
}
