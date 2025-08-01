use dioxus::prelude::*;

use crate::{components::{Ball, Math}, game::GameState};

#[component]
pub fn Bucket(game_state: Signal<GameState>, index: usize) -> Element {
    let state = game_state();
    let ball = state.assignment[index];
    let onclick = move |_ev: Event<MouseData>| {
        game_state.write().click_bucket(index);
    };
    rsx! {
        div {
            onclick,
            BucketInner {
                tex: state.equations[index].question.as_str(),
                if let Some(index) = ball {
                    div {
                        style: "position: absolute; width: 35rem; top: 23.5rem;",
                        Ball {
                            game_state, index
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn BucketInner(tex: String, children: Element) -> Element {
    rsx! {
        div {
            style: "position: relative; width: 35rem;",
            img { 
                src: asset!("/assets/images/dad-penguin.svg"),
                style: "position: relative; margin: 0 auto; top: 2rem; width: 34rem;",
            }
            Math { 
                tex,
                style: "color: #000; font-size: 4rem; text-align: center; position: absolute; margin: 0 auto; left: 0rem; top: 18.7rem; width: 33.5rem;",
            },
            {children}
        }
    }
}