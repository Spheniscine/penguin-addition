
use dioxus::prelude::*;

use crate::{components::{AudioPreloader, Ball, BallSlot, Bucket, Settings}, game::{Audio, Feedback, GameState, ScreenState, DEFAULT_FONT, NUM_BUCKETS}};

#[component]
pub fn Hero() -> Element {
    let mut state = use_signal(|| {
        GameState::test_generate()
    });

    let click_check = move |_| {
        if state.write().check() {
            state.read().feedback.play_audio(Audio::Correct);
            state.write().is_won = true;
        } else {
            state.read().feedback.play_audio(Audio::Wrong);
        }
    };

    let advance = move |_| {
        *state.write() = GameState::test_generate();
    };

    rsx! {
        AudioPreloader {  }

        div {
            id: "hero",
            if state.read().screen_state == ScreenState::Game {
        
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
                if state.read().is_won {
                    div {
                        style: "position: absolute; top: 54rem; margin: 0 auto; display: flex; flex-direction: row; height: 30rem; text-align: center;",
                        div {
                            style: "position: relative; font-size: 5rem; height: 9rem; top: 30%; transform: translateY(-50%);",

                            p {
                                style: "padding: 1.5rem; height: 7rem;",
                                "Well done!"
                            }
                            button {
                                r#type: "button",
                                onclick: advance,
                                style: "font-family: {DEFAULT_FONT}; font-size: 5rem; padding: 1.5rem; height: 9rem;",
                                "Continue"
                            }
                        }
                        
                    }
                }
            } else if state.read().screen_state == ScreenState::Settings {
                Settings {
                    game_state: state,
                }
            }
        }
    }
}