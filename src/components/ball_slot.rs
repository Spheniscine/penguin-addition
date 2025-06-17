use dioxus::prelude::*;

use crate::{components::{Ball, Math}, game::GameState};

#[component]
pub fn BallSlot(game_state: Signal<GameState>, index: usize) -> Element {
    let state = game_state();
    let i = state.permutation[index];
    let visible = !state.assignment.contains(&Some(i));
    rsx! {
        div {
            style: "position: relative; width: 35rem;",
            if visible {
                Ball { game_state, index: i }
            }
        }
    }
}