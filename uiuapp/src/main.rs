#![allow(non_snake_case)]

use crate::document::*;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use uiuapp::*;

use lazy_static::lazy_static;
use uiua::Primitive as P;
use uiuapp::Either as E;

const UNKNOWN_GLYPH: char = '¡';

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

type ButtonIcon = E<Vec<P>, (&'static str, &'static str)>;
lazy_static! {
    /// The car of each line is the default icon. when pressed, the cdr is the radial menu icons
    static ref button_icons: [Vec<ButtonIcon>; 5 * 4] = [
        vec![E::Left(vec![P::Add])],
        vec![E::Left(vec![P::Round])],
        vec![E::Left(vec![P::Gt])],
        vec![E::Left(vec![P::Shape])],
        vec![E::Right(("🧪", ""))],
        vec![E::Left(vec![P::Transpose])],
        vec![E::Left(vec![P::Sort])],
        vec![E::Left(vec![P::Where])],
        vec![E::Left(vec![P::Under])],
        vec![E::Left(vec![P::Try])],
        vec![E::Left(vec![P::Fork])],
        vec![E::Left(vec![P::Identity])],
        vec![E::Left(vec![P::Tau])],
        vec![E::Right(("[", "stack-function"))],
        vec![E::Right(("@", "string-literal"))],
        vec![E::Left(vec![P::IndexOf])],
        vec![E::Left(vec![P::By])],
        vec![E::Left(vec![P::Fold])],
        vec![E::Right(("0", "constant-value"))],
        vec![E::Left(vec![P::Sub, P::By, P::Neg])],
    ];
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
                           ButtonIcons { input_contents }
                    }
              }
        }
    }
}

#[component]
fn ButtonIcons(input_contents: Signal<String>) -> Element {
    rsx! {
        for button in button_icons.clone() {
            match button[0] {
                E::Left(ref prims) => {
                    let primes = prims.clone();
                    rsx! {
                        button { class: "uiua-char-input",
                                 onclick: move |evt| {
                                     evt.prevent_default();
                                     input_contents.write().push_str(&primes.iter().map(|p|p.glyph().unwrap_or(UNKNOWN_GLYPH)).collect::<String>());
                                 },
                            for p in prims {
                                span { class: css_of_prim(&p), "{p.glyph().unwrap_or(UNKNOWN_GLYPH)}" }
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
