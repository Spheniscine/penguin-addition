use dioxus::prelude::*;

use crate::game::{GameState, ScreenState};

const FA_GEAR_SVG: Asset = asset!("/assets/images/fa-gear.svg");

#[component]
pub fn SettingsIcon(style: String, game_state: Signal<GameState>) -> Element {
    rsx! {
        div {
            style,
            img {
                style: "cursor: pointer;",
                src: FA_GEAR_SVG,
                onclick: move |_| game_state.write().screen_state = ScreenState::Settings
            }
        }
    }
}