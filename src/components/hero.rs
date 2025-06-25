
use dioxus::prelude::*;

use crate::{components::{AudioPreloader, Ball, BallSlot, Bucket}, game::{Audio, Feedback, GameState, DEFAULT_FONT, NUM_BUCKETS}};

#[component]
pub fn Hero() -> Element {
    let mut state = use_signal(|| {
        GameState::test_generate()
    });

    let click_check = move |_| {
        if state.write().check() {
            state.read().feedback.play_audio(Audio::Correct);
            *state.write() = GameState::test_generate();
        } else {
            state.read().feedback.play_audio(Audio::Wrong);
        }
    };

    rsx! {
        AudioPreloader {  }
        div {
            id: "hero",
            div {
                style: "position: absolute; margin: 0 auto; display: flex; flex-direction: row; height: 54rem;",
                for index in 0..NUM_BUCKETS {
                    Bucket {
                        game_state: state,
                        index
                    }
                }
            }
            div {
                style: "position: absolute; top: 54rem; margin: 0 auto; display: flex; flex-direction: row; height: 30rem;",
                for index in 0..NUM_BUCKETS {
                    BallSlot {
                        game_state: state,
                        index
                    }
                }
            }
            if state.read().should_show_check_button() {
                div {
                    style: "position: absolute; top: 54rem; margin: 0 auto; display: flex; flex-direction: row; height: 30rem;",
                    button {
                        r#type: "button",
                        onclick: click_check,
                        style: "position: relative; font-size: 5rem; padding: 1.5rem; height: 9rem; font-family: {DEFAULT_FONT}; top: 50%; transform: translateY(-50%);",
                        "Check"
                    }
                }
            }
        }
    }
}