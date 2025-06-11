
use dioxus::prelude::*;

use crate::components::{Ball, Bucket};

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            div {
                style: "position: absolute; margin: 0 auto; display: flex; flex-direction: row; height: 54rem;",
                Bucket {}
                Bucket {}
                Bucket {}
                Bucket {}
                Bucket {}
            }
            div {
                style: "position: absolute; top: 54rem; margin: 0 auto; display: flex; flex-direction: row; height: 30rem;",
                Ball {}
                Ball {}
                Ball {}
                Ball {}
                Ball {}
            }
        }
    }
}