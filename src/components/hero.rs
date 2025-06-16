
use dioxus::prelude::*;

use crate::{components::{Ball, BallSlot, Bucket}, game::{GameState, NUM_BUCKETS}};

#[component]
pub fn Hero() -> Element {
    let mut state = use_signal(|| {
        GameState::test_generate()
    });

    rsx! {
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
        }
    }
}