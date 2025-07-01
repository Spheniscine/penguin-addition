use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize)]
pub struct SettingsState {
    pub audio_state: i32,
    pub reset_level: bool,
}