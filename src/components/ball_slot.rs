use dioxus::prelude::*;

use crate::components::{Ball, Math};

#[component]
pub fn BallSlot() -> Element {
    rsx! {
        div {
            style: "position: relative; width: 35rem;",
            Ball {}
        }
    }
}