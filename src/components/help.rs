use std::rc::Rc;

use dioxus::{logger::tracing, prelude::*};
use strum::IntoEnumIterator;

use crate::game::{GameState, ScreenState};

#[component]
pub fn Help(game_state: Signal<GameState>) -> Element {
    
    let mut ok = move |_| {
        game_state.write().screen_state = ScreenState::Game;
    };

    let mut onmounted = async move |e: Event<MountedData>| {
        e.set_focus(true).await;
    };
    let mut onkeydown = move |e: Event<KeyboardData>| {
        let key = e.key();
        match key {
            Key::Enter | Key::Escape => {
                game_state.write().screen_state = ScreenState::Game;
            }
            _ => {}
        }
    };

    rsx! {
        style {
            "#helpDialog:focus {{ outline: none; }}"
        }
        div {
            id: "helpDialog",
            style: "margin: 1.5%; padding: 5rem; width: 91.5%; height: 85%; background-color: #ccc; font-size: 5rem;
                line-height: 9rem;
                color: #000; border-radius: 2rem;",
            tabindex: -1,
            onmounted: onmounted,
            onkeydown: onkeydown,

            p {
                "TEST"
            },


            p { 
                button {
                    r#type: "button",
                    style: "width: 20rem; font-size: 5rem; font-family: 'Trebuchet MS', 'Lucida Sans Unicode', 'Lucida Grande', 'Lucida Sans', Arial, sans-serif;",
                    onclick: ok,
                    "OK"
                },
            },
        }
    }
}