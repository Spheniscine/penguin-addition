use std::rc::Rc;

use dioxus::{logger::tracing, prelude::*};
use strum::IntoEnumIterator;

use crate::game::{Difficulty, Feedback, GameState, Operator, SettingsState};

#[component]
pub fn RadioButton(state: Signal<SettingsState>, game_state: Signal<GameState>, name: String, value: String, children: Element) -> Element {
    let checked = state.read().difficulty_options.get(&name) == Some(&value);

    let name_ref = name.clone(); let value_ref = value.clone();
    let onchange = move |_| {
        state.write().difficulty_options.insert(name_ref.to_string(), value_ref.to_string());
        let addend_limit = state.read().addend_limit();
        let max_addend = state.read().difficulty_options[Difficulty::STR_ADDEND_RANGE].rsplit(',').next().unwrap().parse::<i32>().unwrap();
        if max_addend > addend_limit { 
            state.write().difficulty_options.insert(Difficulty::STR_ADDEND_RANGE.into(), "1,1".into());
        }
        if state.read().difficulty_options != game_state.read().difficulty.to_map() {
            state.write().reset_level = true;
        }
    };
    rsx! {
        label {
            input {
                r#type: "radio",
                onchange,
                name: {name.to_string()},
                value: {value.to_string()},
                checked,
            },
            span { 
                {children}
            },
        },
    }
}

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

    let mut state = use_signal(|| game_state.read().get_settings_state());
    let mut ok = move |_| {
        // game_state.write().apply_settings(&state.read());
        // game_state.write().screen_state = 
    };
    let mut cancel = move |_| {
        // game_state.write().show_settings = false;
    };

    let reset_level_changed = move |evt: Event<FormData>| {
        state.write().reset_level = evt.checked();
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

    let is_plus = state.read().difficulty_options[Difficulty::STR_OPERATOR] == "Plus";
    let op_sign = if is_plus {"+"} else {"−"};
    let op_verb = if is_plus {"Add"} else {"Subtract"};
    let addend_limit = state.read().addend_limit();

    rsx! {
        style {
            "#settingsDialog:focus {{ outline: none; }}"
        }
        div {
            id: "settingsDialog",
            style: "margin: 1.5%; padding: 5rem; width: 91.5%; height: 85%; background-color: #ccc; font-size: 5rem;
                line-height: 9rem;
                color: #000; border-radius: 2rem;",
            tabindex: -1,
            onmounted: onmounted,
            onkeydown: onkeydown,

            p {
                class: "radio-buttons",
                "Operation:",
                for op in Operator::iter() {
                    " ",
                    RadioButton {  
                        state, game_state,
                        name: Difficulty::STR_OPERATOR,
                        value: "{op}",
                        "{op}",
                    },
                }
            },

            p {
                class: "radio-buttons",
                "Up to:",
                for &mx in Difficulty::RESULT_MAXES {
                    " ",
                    RadioButton {  
                        state, game_state,
                        name: Difficulty::STR_MAX_RESULT,
                        value: "{mx}",
                        "{mx}",
                    },
                }
            }

            p {
                class: "radio-buttons",
                "{op_verb} how many:",
                br {},
                for x in 1..=addend_limit {
                    " ",
                    RadioButton {  
                        state, game_state,
                        name: Difficulty::STR_ADDEND_RANGE,
                        value: "{x},{x}",
                        "{op_sign}{x}",
                    },
                }

                br {},

                " ",
                label {
                    RadioButton {  
                        state, game_state,
                        name: Difficulty::STR_ADDEND_RANGE,
                        value: "1,5",
                        "{op_verb} 1 to 5",
                    },
                },

                if addend_limit >= 10 {
                    " ",
                    label {
                        RadioButton {  
                            state, game_state,
                            name: Difficulty::STR_ADDEND_RANGE,
                            value: "1,10",
                            "{op_verb} 1 to 10",
                        },
                    },
                }
            }
            

            p { 
                "Generate new problems: ",
                input {
                    r#type: "checkbox",
                    style: "width: 4rem; height: 4rem;",
                    checked: state.read().reset_level,
                    disabled: state.read().difficulty_options != game_state.read().difficulty.to_map(),
                    onchange: reset_level_changed
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