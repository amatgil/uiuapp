#![allow(non_snake_case)]

use crate::document::*;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use uiuapp::ScrollbackItem as SBI;
use uiuapp::*;

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
    // TODO(release): depopulate
    let mut buffer_contents = use_signal(|| {
        vec![
            SBI::Input("+ 1 1".to_string()),
            SBI::Output("2".to_string()),
            SBI::Input("˙⊞=⇡3".to_string()),
            SBI::Output("1 0 0\n0 1 0\n0 0 1".to_string()),
            SBI::Input("˙⊞=⇡3".to_string()),
            SBI::Output("1 0 0\n0 1 0\n0 0 1".to_string()),
            SBI::Input("˙⊞=⇡3".to_string()),
            SBI::Output("1 0 0\n0 1 0\n0 0 1".to_string()),
        ]
    });
    // Has been input but not yet evaluated
    let mut input_contents = use_signal(|| String::new());
    //let mut radial_pos: Signal<Option<RadialInfo>> = use_signal(|| None);
    let mut touch_info: Signal<Option<LastTouchContext>> = use_signal(|| None);

    // TODO(release): depopulate
    let mut radial_pos: Signal<Option<RadialInfo>> = use_signal(|| {
        None
        //Some(RadialInfo {
        //    last_pos: (300, 200),
        //    glyphs: button_icons[0].clone(),
        //})
    });

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
                                for (i, item) in buffer_contents.read().iter().enumerate() {
                                    {
                                        match item {
                                            SBI::Input(text) => {
                                                let t = text.clone();
                                                rsx! {
                                                    p { class: "user-input",
                                                    onclick: move |_e| {
                                                        if input_contents().is_empty() {
                                                            *input_contents.write() = t.clone();
                                                        }
                                                    },
                                                    "{text}" }
                                                }
                                            },
                                            SBI::Output(text) => {
                                                rsx! {
                                                    p { class: "user-result", "{text}" }
                                                }
                                            }
                                        }
                                    }
                                }
                          }
                          div { class: "code-buttons",
                              button {
                                  onclick: move |e| {
                                      info!("Settings button pressed (unimplemented as of yet)");
                                  },
                                  "Settings"
                              }
                          }
                    }
                    div { class: "code-textarea-zone",
                    // This textarea should bring up the native keyboard for
                    // ascii-and-related typing
                          textarea { class: "uiua-input", rows: 2,
                                     onkeydown: move |e| {
                                         if let Key::Enter = e.key() {
                                             info!("Return gotten");
                                             if e.modifiers().contains(Modifiers::CONTROL) {
                                                 e.prevent_default();
                                                 info!("Running from shortcut");
                                                 handle_running_code(input_contents, buffer_contents);
                                             }
                                         }
                                     },
                                     onchange: move |e| {
                                         *input_contents.write() = e.value();
                                     },
                                     value: input_contents }
                          button { class: "run-button",
                                   onclick: move |e| {
                                       handle_running_code(input_contents, buffer_contents);
                                   },
                                   "Run" },
                    }
              }
              div { class: "input-zone",
                    RadialSelector { input_contents, radial_pos }
                    div { class: "special-buttons",
                          button { onclick: move |_| {input_contents.write().push('\n');}, "Return" }
                          button { onclick: move |_| {
                              *buffer_contents.write() = vec![];
                          }, "Clear" }
                          button { onclick: move |_| {input_contents.write().push(';');}, ";" }
                          button { "←" } // TODO: position cursor
                          button { "↓" }
                          button { "↑" }
                          button { "→" }
                          button { onclick: move |_| {input_contents.write().pop();}, "Bksp" }
                    }
                    div { class: "input-grid-buttons",
                           ButtonIcons { input_contents, radial_pos }
                    }
              }
        }
    }
}

#[component]
fn RadialSelector(
    input_contents: Signal<String>,
    radial_pos: Signal<Option<RadialInfo>>,
) -> Element {
    rsx! {
        if let Some(RadialInfo { last_pos: (y, x), glyphs }) = radial_pos() {
            div { class: "radial-selector",
                  for (i, glyph) in glyphs.clone().into_iter().skip(1).enumerate() {
                      {
                          let angle = i as f32 * glyphs.len() as f32 / 360.0;
                          let radius = 100;
                          match glyph {
                              E::Left(ref prims) => {
                                  let primes = prims.clone();
                                  rsx! {
                                      button { class: "uiua-char-input uiua-radial-char-input",
                                          style: "transform: rotate({angle}deg) translate({radius}px) rotate(-{angle}deg)",
                                          position: "absolute",
                                          top: "{y}px",
                                          left: "{y}px",
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
                                      button {
                                          onclick: move |e| {
                                              e.prevent_default();
                                              if &s != &EXPERIMENTAL_ICON {
                                                  input_contents.write().push_str(s);
                                              }
                                          },
                                          class: "{c}", "{s}"
                                      }
                                  }
                              }
                          }
                      }
                  }
            }
        } else {
            div { class: "radial-selector",
                  style: "display:none;"
            }
        }
    }
}

#[component]
fn ButtonIcons(input_contents: Signal<String>, radial_pos: Signal<Option<RadialInfo>>) -> Element {
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
                        button {
                            onclick: move |e| {
                                e.prevent_default();
                                if s != EXPERIMENTAL_ICON {
                                    input_contents.write().push_str(s);
                                }
                            },
                            class: "{c}", "{s}"
                        }
                    }
                }
            }
        }
    }
}
