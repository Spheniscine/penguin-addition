use dioxus::{logger::tracing, prelude::*};
use strum::IntoEnumIterator;

use crate::game::{Difficulty, Feedback, GameState, Operator};

#[component]
pub fn Settings(game_state: Signal<GameState>) -> Element {
    // let mut state = use_signal(|| {
    //     game_state.read().new_settings_state()
    // });

    // let mut difficulty_changed = move |evt: Event<FormData>| {
    //     state.write().difficulty = evt.value().parse().unwrap_or(Difficulty::Easy);
    //     if state.read().difficulty != game_state.read().difficulty {
    //         state.write().reset_level = true;
    //     }
    // };

    // let reset_level_changed = move |evt: Event<FormData>| {
    //     state.write().reset_level = evt.checked();
    // };

    // let audio_settings_changed = move |evt: Event<FormData>| {
    //     state.write().audio_state = evt.value().parse().unwrap_or(100);
    // };

    let mut ok = move |_| {
        // game_state.write().apply_settings(&state.read());
        // game_state.write().screen_state = 
    };
    let mut cancel = move |_| {
        // game_state.write().show_settings = false;
    };

    let mut onmounted = async move |e: Event<MountedData>| {
        e.set_focus(true).await;
    };
    let mut onkeydown = move |e: Event<KeyboardData>| {
        let key = e.key();
        match key {
            Key::Enter => {
                // game_state.write().apply_settings(&state.read());
                // game_state.write().show_settings = false;
            }
            Key::Escape => {
                // game_state.write().show_settings = false;
            }
            _ => {}
        }
    };

    rsx! {
        style {
            "#settingsDialog:focus {{ outline: none; }}"
        }
        div {
            id: "settingsDialog",
            style: "margin: 1.5%; padding: 5rem; width: 91.5%; height: 85%; background-color: #ccc; font-size: 5rem; line-height: 10rem;
                color: #000; border-radius: 2rem;",
            tabindex: -1,
            onmounted: onmounted,
            onkeydown: onkeydown,

            p {
                "Operation:",
                for op in Operator::iter() {
                    " ",
                    input {
                        r#type: "radio",
                        style: "width: 4rem; height: 4rem;",
                        id: "operator_{op}",
                        name: "operator",
                        value: "{op}",
                    },
                    label {
                        r#for: "operator_{op}",
                        " {op}",
                    },
                }
            },

            p {
                "Up to:",
                for &mx in Difficulty::RESULT_MAXES {
                    " ",
                    input {
                        r#type: "radio",
                        style: "width: 4rem; height: 4rem;",
                        id: "max_result_{mx}",
                        name: "max_result",
                        value: "{mx}",
                    },
                    label {
                        r#for: "max_result_{mx}",
                        " {mx}",
                    },
                }
            }

            p {
                "Add how many:",
                br {},
                for x in 1..=10 {
                    " ",
                    input {
                        r#type: "radio",
                        style: "width: 4rem; height: 4rem;",
                        id: "addend_range_{x}",
                        name: "addend_range",
                        value: "{x},{x}",
                    },
                    label {
                        r#for: "addend_range_{x}",
                        " +{x}",
                    },
                }

                br {},

                " ",
                input {
                    r#type: "radio",
                    style: "width: 4rem; height: 4rem;",
                    id: "addend_range_1_5",
                    name: "addend_range",
                    value: "1,5",
                },
                label {
                    r#for: "addend_range_1_5",
                    " Add 1 to 5",
                },

                " ",
                input {
                    r#type: "radio",
                    style: "width: 4rem; height: 4rem;",
                    id: "addend_range_1_10",
                    name: "addend_range",
                    value: "1,10",
                },
                label {
                    r#for: "addend_range_1_10",
                    " Add 1 to 10",
                },
            }
            

            p { 
                "Generate new problems: ",
                input {
                    r#type: "checkbox",
                    style: "width: 4rem; height: 4rem;",
                    // checked: state.read().reset_level,
                    // disabled: state.read().difficulty != game_state.read().difficulty,
                    // onchange: reset_level_changed
                }
            },

            p { 
                "Audio: ",
                input {
                    r#type: "range",
                    style: "width: 50rem; height: 4rem;",
                    min: 0, max: 100, step: 5, 
                    // value: state.read().audio_state,
                    // oninput: audio_settings_changed
                },
                // " {state.read().audio_state}",
            },

            p { 
                button {
                    r#type: "button",
                    style: "width: 20rem; font-size: 5rem; font-family: 'Trebuchet MS', 'Lucida Sans Unicode', 'Lucida Grande', 'Lucida Sans', Arial, sans-serif;",
                    onclick: ok,
                    "OK"
                },
                " ",
                button {
                    r#type: "button",
                    style: "width: 20rem; font-size: 5rem; font-family: 'Trebuchet MS', 'Lucida Sans Unicode', 'Lucida Grande', 'Lucida Sans', Arial, sans-serif;",
                    onclick: cancel,
                    "Cancel"
                },
            },

            p {
                style: "position: absolute; bottom: 1.5rem; font-size: 3rem;",
                "© OnlineMathLearning.com"
            },
        }
    }
}