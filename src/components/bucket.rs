use dioxus::prelude::*;

use crate::components::{Ball, Math};

#[component]
pub fn Bucket() -> Element {
    rsx! {
        div {
            style: "position: relative; width: 35rem;",
            img { 
                src: asset!("/assets/images/dad-penguin.svg"),
                style: "position: relative; margin: 0 auto; top: 2rem; width: 34rem;",
            }
            Math { 
                tex: "26 + 10",
                style: "color: #000; font-size: 4rem; text-align: center; position: absolute; margin: 0 auto; left: 0rem; top: 18.7rem; width: 33.5rem;",
            }
            div {
                style: "position: absolute; width: 35rem; top: 23.5rem;",
                Ball {}
            }
        }
    }
}