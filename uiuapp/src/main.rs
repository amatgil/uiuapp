#![allow(non_snake_case)]

use crate::document::*;
use dioxus::prelude::Key::Character;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use uiuapp::ScrollbackItem as SBI;
use uiuapp::*;

use lazy_static::lazy_static;
use uiua::Primitive as P;
use uiuapp::Either as E;

const UNKNOWN_GLYPH: char = '¡';
const EXPERIMENTAL_ICON: &str = "🧪";
const KEYPAD_WIDTH: usize = 4; // Grid size
const KEYPAD_HEIGHT: usize = 5; // Grid size

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

// Tiny convenience
fn l(p: P) -> Either<Vec<P>, (&'static str, &'static str)> {
    E::Left(vec![p])
}

lazy_static! {
        //vec![E::Left(vec![P::Add]),
        //     E::Left(vec![P::Sub]),
        //     E::Left(vec![P::Mul]),
        //     E::Left(vec![P::Div]),
        //     E::Left(vec![P::Round]),
        //     E::Left(vec![P::Floor]),
        //     E::Left(vec![P::Ceil]), // Ceils because testing
        //     E::Left(vec![P::Ceil]),
        //     E::Left(vec![P::Ceil]),
        //     E::Left(vec![P::Ceil]),
        //     E::Left(vec![P::Ceil]),
        //     E::Left(vec![P::Ceil]),
        //     E::Left(vec![P::Ceil]),
        //     E::Left(vec![P::Ceil]),
        //     E::Left(vec![P::Ceil]),
        //     E::Left(vec![P::Ceil]),
        //     E::Left(vec![P::Ceil]),
        //     E::Left(vec![P::Ceil]),
        //     E::Left(vec![P::Ceil]),
        //     E::Left(vec![P::Ceil]),
        //     E::Left(vec![P::Ceil]),
        //     E::Left(vec![P::Ceil]),
        //     E::Left(vec![P::Ceil]),
        //],
    /// The car of each line is the default icon. when pressed, the cdr is the radial menu icons
    static ref button_icons: [Vec<ButtonIcon>; KEYPAD_HEIGHT * KEYPAD_WIDTH] = [
        vec![
            l(P::Identity),
            l(P::Slf),
            l(P::Backward),
            l(P::Pop),
            l(P::Dup),
            l(P::Flip),
        ],
        vec![l(P::Dip), l(P::Gap), l(P::On), l(P::By)],
        vec![
            l(P::Fork),
            l(P::Both),
            l(P::Bracket),
            l(P::Switch),
            l(P::Try),
            l(P::Case),
            l(P::Do),
            l(P::Assert),
        ],
        vec![l(P::Un), l(P::Anti), l(P::Under), l(P::Obverse)],
        vec![
            l(P::Neg),
            l(P::Add),
            l(P::Not),
            l(P::Abs),
            l(P::Sqrt),
            l(P::Sin),
            l(P::Floor),
            l(P::Ceil),
            l(P::Round),
        ],
        vec![
            l(P::Eq),
            l(P::Ne),
            l(P::Le),
            l(P::Lt),
            l(P::Gt),
            l(P::Ge),
            l(P::Min),
            l(P::Max),
        ],
        vec![l(P::Eta), l(P::Pi), l(P::Tau), l(P::Infinity)],
        vec![
            l(P::Len),
            l(P::Shape),
            l(P::First),
            l(P::Last),
            l(P::Reverse),
            l(P::Deshape),
            l(P::Fix),
            l(P::Transpose),
        ],
        vec![l(P::Bits), l(P::Where), l(P::Parse)],
        vec![
            l(P::Sort),
            l(P::Rise),
            l(P::Fall),
            l(P::Classify),
            l(P::Deduplicate),
            l(P::Unique),
        ],
        vec![l(P::Box), l(P::Content), l(P::Inventory)],
        vec![
            l(P::Couple),
            l(P::Join),
            l(P::Select),
            l(P::Pick),
            l(P::Reshape),
            l(P::Drop),
            l(P::Take),
            l(P::Rotate),
            l(P::Keep),
            l(P::Orient),
        ],
        vec![
            l(P::Match),
            l(P::Find),
            l(P::Mask),
            l(P::MemberOf),
            l(P::IndexOf),
            l(P::Partition),
            l(P::Group),
        ],
        vec![l(P::Reduce), l(P::Fold), l(P::Scan), l(P::Repeat)],
        vec![l(P::Rows), l(P::Table), l(P::Tuples), l(P::Stencil)],
        //[
        //  {∘"˙˜◌.:"    ⊙"⋅⟜⊸⤙⤚◡"      ⊃"∩⊓⨬⍣⍩⍢⍤"  °"⌝⍜⌅"}
        //  {¯"±¬⌵√∿⌊⌈⁅" +"-×÷◿ⁿₙ∠ℂ⊥"   ="≠<≤>≥↧↥"  ⚂"ηπτ∞"}
        //  {⧻"△⊢⊣⇌♭¤⍉"  ⇡"⋯⊚⋕"         ⍆"⍏⍖⊛◴◰"    □"◇⍚"}
        //  {⊟"⊂⊏⊡↯↙↘↻▽" ≍"⌕⦷∊⊗⊕⊜"      /"∧\\⍥"     ≡"⊞⧅⧈"}
        //]
        vec![E::Right(("Empty", ""))],
        vec![E::Right(("0", "constant-value"))],
        vec![E::Right(("₀", "constant-value"))],
        vec![E::Right((EXPERIMENTAL_ICON, ""))],
        vec![E::Left(vec![P::Sub, P::By, P::Neg])],
    ];
}

#[component]
fn App() -> Element {
    static CSS: Asset = asset!("/assets/uiuapp.css");
    static _UIUA386: Asset = asset!("/assets/Uiua386.ttf");

    // the text that's been input and evaluated
    // populated for testing
    // TODO(release): depopulate
    let buffer_contents = use_signal(|| {
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
        RadialSelector { input_contents, radial_pos }
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
                  style: "display: inline-block; position: absolute; top: {y}px; left: {x}px;"
            }
            for (i, glyph) in glyphs.clone().into_iter().skip(1).enumerate() { {
                let angle = i as f32 * TAU / (glyphs.len()-1) as f32;
                info!("({y},{x})");
                match glyph {
                    E::Left(ref prims) => {
                        let primes = prims.clone();
                        rsx! {
                            button { class: "uiua-char-input uiua-radial-char-input",
                                     position: "absolute",
                                     top: "calc({y}px + 100vw/5/2 + (100vw/5 - 30px)*sin({angle}) + 30px/2)",
                                     left: "calc({x}px + 100vw/5/2 + (100vw/5 - 30px)*cos({angle}) + 30px/2)",  // 30px is the border of the radial-select
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
