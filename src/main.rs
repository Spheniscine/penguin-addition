use dioxus::prelude::*;
use components::{Hero, Math};
use serde::{Deserialize, Serialize};

mod components;
mod game;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Routable, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Route {
    #[route("/")]
    Default,
    #[route("/addition")]
    Addition,
}

#[component]
fn Default() -> Element {
    rsx! {
        Hero { route: Route::Default }
    }
}

#[component]
fn Addition() -> Element {
    rsx! {
        Hero { route: Route::Addition }
    }
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
        
        Router::<Route> {}
    }
}



