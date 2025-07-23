use dioxus::prelude::*;

use crate::game::{GameState, ScreenState};

const FA_QUESTION_SVG: Asset = asset!("/assets/images/fa-question.svg");

#[component]
pub fn HelpIcon(style: String, game_state: Signal<GameState>) -> Element {
    rsx! {
        div {
            style,
            img {
                style: "cursor: pointer;",
                src: FA_QUESTION_SVG,
                onclick: move |_| game_state.write().screen_state = ScreenState::Help
            }
        }
    }
}