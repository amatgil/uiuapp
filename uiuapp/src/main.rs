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
                           ButtonIcons { input_contents, rad_info }
                    }
              }
        }
    }
}

#[component]
fn RadialSelector(input_contents: Signal<String>, rad_info: Signal<RadialInfo>) -> Element {
    let glyphs = rad_info().glyphs.clone().into_iter().skip(1).enumerate();
    rsx! {
            if rad_info.read().is_active {
            div { class: "radial-selector",
                for (i, p) in glyphs {
                      {
                        // THIS NEEDS TO BE CALCULATED
                          let radius = 60;
                            let primes = rad_info().glyphs.clone();
                                let angle = (i as f32) * 360.0 / (rad_info().glyphs.len() - 1) as f32;
                                  rsx! {
                                      button { class: "uiua-char-input uiua-radial-char-input",
                                          style: "position: absolute; left: 50%; top: 50%; transform: translate(-50%, -50%) rotate({angle}deg) translateY(-{radius}px) rotate(-{angle}deg);",
                                          onclick: move |evt| {
                                              evt.prevent_default();
                                              input_contents.write().push_str(&primes.iter().map(|p|p.glyph().unwrap_or(UNKNOWN_GLYPH)).collect::<String>());
                                          },
                                        span { class: css_of_prim(&p), "{p.glyph().unwrap_or(UNKNOWN_GLYPH)}" }
                                      }
                                  }


                                  // rsx! {
                                  //     button {
                                  //         class: "{c}",
                                  //         style: "position: absolute; left: 50%; top: 50%; transform: translate(-50%, -50%) rotate({angle}deg) translateY(-{radius}px) rotate(-{angle}deg);",
                                  //         onclick: move |e| {
                                  //             e.prevent_default();
                                  //             if s != EXPERIMENTAL_ICON {
                                  //                 input_contents.write().push_str(s);
                                  //             }
                                  //         },
                                  //         "{s}"
                                  //     }
                                  // }
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
            match button {
                E::Left(prims) => {
                    let primes = prims.clone();
                    let primes2 = prims.clone();
                    rsx! {
                        button { class: "uiua-char-input",
                            onpointerdown: move |evt| {
                            rad_info.write().start(evt.data.screen_coordinates(), primes.clone());
                            },
                            onpointermove: move |evt| {
                            rad_info.write().update(evt.data.screen_coordinates());
                            },
                            onpointerup: move |evt| {
                            evt.prevent_default();
                            rad_info.write().reset();
                            input_contents.write().push_str(&primes2.iter().map(|p|p.glyph().unwrap_or(UNKNOWN_GLYPH)).collect::<String>());
                            },
                                span { class: css_of_prim(&prims[0]), "{&prims[0].glyph().unwrap_or(UNKNOWN_GLYPH)}" }
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
