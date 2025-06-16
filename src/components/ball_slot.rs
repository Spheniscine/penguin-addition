use dioxus::prelude::*;

use crate::{components::{Ball, Math}, game::GameState};

#[component]
pub fn BallSlot(game_state: Signal<GameState>, index: usize) -> Element {
    let state = game_state();
    let i = state.permutation[index];
    rsx! {
        div {
            style: "position: relative; width: 35rem;",
            Ball { tex: state.equations[i].answer.clone() }
        }
    }
}