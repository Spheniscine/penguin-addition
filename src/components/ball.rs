use dioxus::prelude::*;

use crate::{components::Math, game::GameState};

#[component]
pub fn Ball(game_state: Signal<GameState>, index: usize) -> Element {
    let state = game_state();
    let tex = state.equations[index].answer.clone();
    rsx! {
        div {
            style: "position: relative; width: 35rem;",
            img { 
                src: asset!("/assets/images/baby-penguin.svg"),
                style: "position: relative; margin: 0 auto; top: 1rem; left: 4.5rem; width: 25rem;",
            }
            Math { 
                tex,
                style: "color: #000; font-size: 4rem; text-align: center; position: absolute; margin: 0 auto; left: 0rem; top: 16.6rem; width: 34rem",
            }
        }
    }
}