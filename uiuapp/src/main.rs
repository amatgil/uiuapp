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
            SBI::Input(highlight_code("+ 1 1")),
            SBI::Output("2".to_string()),
            SBI::Input(highlight_code("˙⊞=⇡3")),
            SBI::Output("1 0 0\n0 1 0\n0 0 1".to_string()),
            SBI::Input(highlight_code("˙⊞=⇡3")),
            SBI::Output("1 0 0\n0 1 0\n0 0 1".to_string()),
            SBI::Input(highlight_code("˙⊞=⇡3")),
            SBI::Output("1 0 0\n0 1 0\n0 0 1".to_string()),
        ]
    });
    // Has been input but not yet evaluated
    let mut input_contents = use_signal(String::new);
    let touch_info: Signal<Option<LastTouchContext>> = use_signal(|| None);
    let rad_info: Signal<RadialInfo> = use_signal(RadialInfo::new);

    rsx! {
        Meta { charset: "UTF-8" }
        Meta {
            content: "width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover",
            name: "viewport",
        }
        Title { "cas/uiuapp" }
        Stylesheet { href: CSS }

        div { class: "app",
            div { class: "top-bar",
                button {
                    onclick: move |e| {
                        info!("Settings button pressed (unimplemented as of yet)");
                    },
                    "Settings"
                }
            }
            div { class: "code-view-zone",
                for item in buffer_contents.read().clone() {
                    {
                        match item {
                            SBI::Input(input) => {
                                rsx! {
                                    p { class: "user-input",
                                    onclick: move |_e| {
                                        if input_contents().is_empty() {
                                            *input_contents.write() = match input {
                                                Ok(ref v) => v.iter().map(|uhs| match uhs {
                                                    // Bunch of cloning, this should be benchmarked
                                                    UiuappHistorySpan::UnstyledCode { text } => text.clone(),
                                                    UiuappHistorySpan::StyledCode { text, .. } => text.clone(),
                                                    UiuappHistorySpan::Whitspace(text) => text.clone(),
                                                }).collect::<Vec<String>>().join(""),
                                                Err(ref s) => s.to_string()
                                            };
                                        }
                                    },
                                        match input {
                                            Ok(ref v) => {
                                                rsx! {
                                                    for uhs in v {
                                                        match uhs {
                                                            UiuappHistorySpan::UnstyledCode { text } => rsx! { span { "{text}" } },
                                                            UiuappHistorySpan::StyledCode { class: c, text } => rsx! { span { class: "{c}", "{text}"} },
                                                            UiuappHistorySpan::Whitspace(text) => rsx! { span { "{text}" } },
                                                        }
                                                    }
                                                }
                                            }
                                            Err(ref s) => rsx! { span { "{s}" } }
                                        }

                                    }
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
              div { class: "input-zone",
                    RadialSelector { input_contents, rad_info }
                    div { class: "input-bar",
                    // This textarea should bring up the native keyboard for
                    // ascii-and-related typing
                          textarea { class: "text-box", rows: 1,
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
                    div { class: "special-buttons",
                          button { class: "special-button", onclick: move |_| {input_contents.write().push('\n');}, "Ret" }
                          button { class: "special-button", onclick: move |_| {*buffer_contents.write() = vec![];}, "Clear Past" }
                          button { class: "special-button", onclick: move |_| {*input_contents.write() = "".to_string();}, "Clear Curr" }
                          button { class: "special-button", onclick: move |_| {input_contents.write().push(';');}, ";" }
                          button { class: "special-button", "←" } // TODO: position cursor
                          button { class: "special-button", "↓" }
                          button { class: "special-button", "↑" }
                          button { class: "special-button", "→" }
                          button { class: "special-button", onclick: move |_| {input_contents.write().pop();}, "Bksp" }
                    }
                    div { class: "input-grid-buttons",
                           ButtonIcons { input_contents, rad_info }
                    }
              }
        }
    }
}

#[component]
fn RadialSelector(input_contents: Signal<String>, rad_info: Signal<RadialInfo>) -> Element {
    let glyphs = rad_info().glyphs;
    rsx! {
        if rad_info.read().is_active {
            div { class: "radial-selector",
                  for (i, glyph) in glyphs.clone().into_iter().skip(1).enumerate() { {
                      let angle = (i as f32) * 360. / (glyphs.len()-1) as f32;
                      //TODO: make computed
                      let radius = 60.;
                      match glyph {
                          E::Left(ref prims) => {
                              let primes = prims.clone();
                              rsx! {
                                  button { class: "uiua-char-input uiua-radial-char-input",
                                           style: "position: absolute; left: 50%; top: 50%; transform: translate(-50%, -50%) rotate({angle}deg) translateY(-{radius}px) rotate(-{angle}deg);",
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
fn ButtonIcons(input_contents: Signal<String>, rad_info: Signal<RadialInfo>) -> Element {
    rsx! {
        for button in button_icons.clone() {
            match button[0] {
                E::Left(ref prims) => {
                    let primsP = prims.clone();
                    let btn = button.clone();
                    rsx! {
                        button { class: "uiua-char-input",
                                 onpointerdown: move |evt| {
                                     rad_info.write().start(evt.data.screen_coordinates(), btn.clone());
                                 },
                                 onpointermove: move |evt| {
                                     rad_info.write().update(evt.data.screen_coordinates());
                                 },
                                 onpointerup: move |evt| {
                                     evt.prevent_default();
                                     rad_info.write().reset();
                                     input_contents.write().push_str(&primsP.iter().map(|p|p.glyph().unwrap_or(UNKNOWN_GLYPH)).collect::<String>());
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
