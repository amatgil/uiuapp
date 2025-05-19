#![allow(non_snake_case)]

use crate::document::*;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[component]
fn App() -> Element {
    static CSS: Asset = asset!("/assets/uiuapp.css");
    static UIUA386: Asset = asset!("/assets/Uiua386.ttf");

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
                                p { class: "user-input", "+ 1 1" }
                                p { class: "user-result", "2" }
                                p { class: "user-input", "˙⊞=⇡3" }
                                p { class: "user-result",
                                    "1 0 0"
                                    br {}
                                    "0 1 0"
                                    br {}
                                    "0 0 1"
                                }
                                p { class: "user-input", "˙⊞=⇡3" }
                                p { class: "user-result",
                                    "1 0 0"
                                    br {}
                                    "0 1 0"
                                    br {}
                                    "0 0 1"
                                }
                                p { class: "user-input", "˙⊞=⇡3" }
                                p { class: "user-result",
                                    "1 0 0"
                                    br {}
                                    "0 1 0"
                                    br {}
                                    "0 0 1"
                                }
                          }
                          div { class: "code-buttons",
                                button { "Settings" }
                          }
                    }
                    div { class: "code-textarea-zone",
                          input { class: "uiua-input" }
                          button { class: "run-button", "Run" }
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
                          button { class: "uiua-char-input", span { class: "dyadic-function",  "+" } }
                          button { class: "uiua-char-input", span { class: "monadic-function", "⁅" } }
                          button { class: "uiua-char-input", span { class: "dyadic-function",  ">" } }
                          button { class: "uiua-char-input", span { class: "monadic-function", "△" } }
                          button { class: "uiua-char-input", "🧪" }
                          button { class: "uiua-char-input", span { class: "uiua-trans", "⍉" } }
                          button { class: "uiua-char-input", span { class: "monadic-function", "⍆" } }
                          button { class: "uiua-char-input", span { class: "monadic-function", "⊚" } }
                          button { class: "uiua-char-input", span { class: "dyadic-modifier",  "⍜" } }
                          button { class: "uiua-char-input", span { class: "dyadic-modifier",  "⍣" } }
                          button { class: "uiua-char-input", span { class: "dyadic-modifier",  "⊃" } }
                          button { class: "uiua-char-input", span { class: "stack-function",   "∘" } }
                          button { class: "uiua-char-input", span { class: "constant-value",   "τ" } }
                          button { class: "uiua-char-input", span { class: "stack-function",   "[" } }
                          button { class: "uiua-char-input", span { class: "string-literal",   "@" } }
                          button { class: "uiua-char-input", span { class: "dyadic-function",  "⊗" } }
                          button { class: "uiua-char-input", span { class: "monadic-modifier", "⊸" } }
                          button { class: "uiua-char-input", span { class: "stack-function",   "←" } }
                          button { class: "uiua-char-input", span { class: "constant-value",   "0" } }
                          button { class: "uiua-char-input",
                                   span { class:  "dyadic-function", "-" }
                                   span { class:  "monadic-modifier", "⊸" }
                                   span { class:  "monadic-function", "¬" }
                          }
                    }
              }
        }
    }
}

#[component]
fn TextZone() -> Element {
    rsx! {
        input {
        }
    }
}
