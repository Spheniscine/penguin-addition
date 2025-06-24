use dioxus::{document, prelude::*};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use strum_macros::{EnumCount, EnumIter};
use web_sys::HtmlAudioElement;

#[derive(Clone, Copy, Debug, EnumCount, EnumIter, Hash, Eq, PartialEq)]
pub enum Audio {
    Correct, Wrong
}

impl Audio {
    pub fn asset(self) -> Asset {
        match self {
            Audio::Correct => asset!("/assets/audio/correct.mp3"),
            Audio::Wrong => asset!("/assets/audio/wrong.mp3"),
        }
    }
}

// for feedback with a time duration, e.g. audio, animation

pub trait Feedback {
    fn play_audio(&self, audio: Audio);
    fn get_audio_state(&self) -> f64;
    fn set_audio_state(&mut self, value: f64);
    fn toggle_audio(&mut self);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedbackImpl {
    pub audio_state: f64,

    pub prev_audio_state: f64,
}

impl Feedback for FeedbackImpl {
    fn play_audio(&self, audio: Audio) {
        if self.audio_state > 0. {
            if let Ok(e) = HtmlAudioElement::new_with_src(&audio.asset().to_string()) {
                e.set_volume(self.audio_state);
                e.play();
            }
        }
    }
    
    fn get_audio_state(&self) -> f64 {
        self.audio_state
    }
    
    fn set_audio_state(&mut self, value: f64) {
        if value == 0. && self.audio_state > 0. { self.prev_audio_state = self.audio_state; }
        self.audio_state = value;
    }

    fn toggle_audio(&mut self) {
        if self.audio_state == 0. {
            self.audio_state = self.prev_audio_state;
        } else {
            self.prev_audio_state = self.audio_state;
            self.audio_state = 0.;
        }
    }
}