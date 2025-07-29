use std::rc::Rc;

use dioxus::{logger::tracing, prelude::*};
use strum::IntoEnumIterator;

use crate::{components::{ball::BallInner, bucket::BucketInner, Ball}, game::{GameState, ScreenState}};

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
                color: #000; border-radius: 2rem;",
            tabindex: -1,
            onmounted: onmounted,
            onkeydown: onkeydown,

            p {
                style: "line-height: 9rem;",
                "Click on the baby penguin, then click on the adult with the matching problem."
            },

            div {
                style: "display: flex; flex-direction: row;",
                div {
                    style: "position: relative; top: 21rem; height: 30rem;",
                    BallInner {
                        tex: "3", selected: false
                    },
                },

                img {  
                    src: asset!("/assets/images/right-arrow.svg"),
                    style: "position: relative; top: 10rem; padding-right: 3rem; width: 20rem;",
                }

                BucketInner {
                    tex: "1 + 2"
                },
            }


            p { 
                style: "line-height: 12rem;",
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