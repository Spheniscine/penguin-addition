use dioxus::prelude::*;
use components::Math;

mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/katex@0.16.21/dist/katex.min.css",
            integrity: "sha384-zh0CIslj+VczCZtlzBcjt5ppRcsAmDnRem7ESsYwWwg3m/OaJ2l4x7YBZl9Kxxib",
            crossorigin: "anonymous"
        }

        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } 
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        
        Hero {}

    }
}

#[component]
pub fn Penguin() -> Element {
    rsx! {
        div {
            style: "width: 35rem;",
            img { 
                src: asset!("/assets/images/dad-penguin.svg"),
                style: "position: relative; margin: 0 auto; top: 2rem; width: 34rem;",
            }
            img { 
                src: asset!("/assets/images/baby-penguin.svg"),
                style: "position: relative; margin: 0 auto; left: 4.5rem; top: -25rem; width: 25rem;",
            }
        }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            div {
                style: "position: absolute; margin: 0 auto; display: flex; flex-direction: row;",
                Penguin {}
                Penguin {}
                Penguin {}
                Penguin {}
                Penguin {}
            }
        }
    }
}
