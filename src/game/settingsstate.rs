use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::Difficulty;


#[derive(Clone, Serialize, Deserialize)]
pub struct SettingsState {
    pub difficulty_options: IndexMap<String, String>,
    pub audio_state: i32,
    pub reset_level: bool,
}

impl SettingsState {
    pub fn addend_limit(&self) -> i32 {
        if self.difficulty_options[Difficulty::STR_MAX_RESULT].parse::<i32>().unwrap() > 10 {10} else {5}
    }
}