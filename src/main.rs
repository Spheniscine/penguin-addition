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
pub fn Hero() -> Element {
    use rand::Rng;
    rsx! {
        div {
            id: "hero",
            img { 
                src: asset!("/assets/images/dad-penguin.svg"),
                style: "position: absolute; margin: 0 auto; width: 39rem;",
            }
            img { 
                src: asset!("/assets/images/baby-penguin.svg"),
                style: "position: absolute; margin: 0 auto; top: 28rem; width: 25rem;",
            }
        }
    }
}
