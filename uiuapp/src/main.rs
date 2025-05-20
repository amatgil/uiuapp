#![allow(non_snake_case)]

use crate::document::*;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use uiuapp::*;

use uiua::Primitive as P;
use uiuapp::Either as E;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[component]
fn App() -> Element {
    static CSS: Asset = asset!("/assets/uiuapp.css");
    static _UIUA386: Asset = asset!("/assets/Uiua386.ttf");

    // the text that's been input and evaluated
    // populated for testing
    let buffer_contents = use_signal(|| {
        vec![
            "+ 1 1".to_string(),
            "2".to_string(),
            "˙⊞=⇡3".to_string(),
            "1 0 0\n0 1 0\n0 0 1".to_string(),
            "˙⊞=⇡3".to_string(),
            "1 0 0\n0 1 0\n0 0 1".to_string(),
            "˙⊞=⇡3".to_string(),
            "1 0 0\n0 1 0\n0 0 1".to_string(),
        ]
    });
    // Has been input but not yet evaluated
    let mut input_contents = use_signal(|| String::new());

    rsx! {
        Meta { charset: "UTF-8" }
        Meta {
            content: "width=device-width, initial-scale=1.0",
            name: "viewport",
        }
        Title { "cas/uiuapp" }
        Stylesheet { href: CSS }

        div { class: "wrapper",
              div { class: "code-zone",
                    div { class: "code-display-zone",
                          div { class: "code-scrollbackbuffer",
                                for (i, text) in buffer_contents.read().iter().enumerate() {
                                    if i % 2 == 0 {
                                        p { class: "user-input", "{text}" }
                                    } else {
                                        p { class: "user-result", "{text}" }
                                    }
                                }
                          }
                          div { class: "code-buttons",
                                button { "Settings" }
                          }
                    }
                    div { class: "code-textarea-zone",
                          input { class: "uiua-input", value: input_contents }
                          button { class: "run-button", "Run" },
                    }
              }
              div { class: "input-zone",
                    div { class: "special-buttons",
                          button { "Return" }
                          button { ";" }
                          button { "←" }
                          button { "↓" }
                          button { "↑" }
                          button { "→" }
                          button { "Bksp" }
                    }
                    div { class: "input-grid-buttons",
                          ButtonIcons {}
                    }
              }
        }
    }
}

#[component]
fn ButtonIcons() -> Element {
    let button_icons: [E<Vec<P>, (&'static str, &'static str)>; 20] = [
        E::Left(vec![P::Add]),
        E::Left(vec![P::Round]),
        E::Left(vec![P::Gt]),
        E::Left(vec![P::Shape]),
        E::Right(("🧪", "")),
        E::Left(vec![P::Transpose]),
        E::Left(vec![P::Sort]),
        E::Left(vec![P::Where]),
        E::Left(vec![P::Under]),
        E::Left(vec![P::Try]),
        E::Left(vec![P::Fork]),
        E::Left(vec![P::Identity]),
        E::Left(vec![P::Tau]),
        E::Right(("[", "stack-function")),
        E::Right(("@", "string-literal")),
        E::Left(vec![P::IndexOf]),
        E::Left(vec![P::By]),
        E::Left(vec![P::Fold]),
        E::Right(("0", "constant-value")),
        E::Left(vec![P::Sub, P::By, P::Neg]),
    ];

    // button { class: "uiua-char-input", span { class: "monadic-function", "⊚" } }

    // button { class: "uiua-char-input", // Wrench go brrr
    //          span { class:  "dyadic-function", "-" }
    //          span { class:  "monadic-modifier", "⊸" }
    //          span { class:  "monadic-function", "¬" }
    // }

    rsx! {
        for button in button_icons {
            match button {
                E::Left(prims) => {
                    rsx! {
                        button { class: "uiua-char-input",
                            for p in prims {
                                span { class: css_of_prim(&p), "{p.glyph().map(|t|t.to_string()).unwrap_or(p.name().to_string())}" }
                            }
                        }
                    }
                },

                E::Right((s, c)) => {
                    rsx! {
                        button { class: "{c}", "{s}" }
                    }
                }
            }
        }
    }
}
