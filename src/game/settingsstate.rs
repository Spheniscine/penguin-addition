use indexmap::IndexMap;
use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize)]
pub struct SettingsState {
    pub difficulty_options: IndexMap<String, String>,
    pub audio_state: i32,
    pub reset_level: bool,
}