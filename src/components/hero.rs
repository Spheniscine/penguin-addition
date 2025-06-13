
use dioxus::prelude::*;

use crate::{components::{Ball, BallSlot, Bucket}, game::GameState};

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
                Bucket {}
                Bucket {}
                Bucket {}
                Bucket {}
                Bucket {}
            }
            div {
                style: "position: absolute; top: 54rem; margin: 0 auto; display: flex; flex-direction: row; height: 30rem;",
                BallSlot {}
                BallSlot {}
                BallSlot {}
                BallSlot {}
                BallSlot {}
            }
        }
    }
}