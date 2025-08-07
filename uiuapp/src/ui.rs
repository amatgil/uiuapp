use std::time;

use crate::*;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use tokio::time::sleep;
// use tokio::{task::JoinHandle, time::sleep};

const RADIAL_DELAY: Duration = Duration::from_millis(500);

#[component]
pub fn RadialSelector(input_contents: Signal<String>, rad_info: Signal<RadialInfo>) -> Element {
    let glyphs = rad_info().glyphs;
    rsx! {
        if rad_info.read().active {
            div { class: "radial-selector",
                  style: rad_info().style,
                  for (i, glyph) in glyphs.clone().into_iter().skip(1).enumerate() { {
                        dbg!(rad_info().current_selection);
                      let font = if i == rad_info().current_selection {40} else {20};
                      let angle = (i as f32) * 360. / (glyphs.len()-1) as f32;
                      match glyph {
                          Icon::Single(prim) => {
                              rsx! {
                                  button { class: "uiua-char-input uiua-radial-char-input",
                                           style: "position: absolute; left: 50%; top: 50%; transform: translate(-50%, -50%) rotate({angle}deg) translateY(-12vw) rotate(-{angle}deg);",
                                           onclick: move |evt| {
                                               evt.prevent_default();
                                               // input_contents.write().push_str(&primes.iter().map(|p|p.glyph().unwrap_or(UNKNOWN_GLYPH)).collect::<String>());
                                               input_contents.write().push(prim.glyph().unwrap());
                                           },
                                        span { class: css_of_prim(&prim), style: "font-size: {font}px", "{prim.glyph().unwrap_or(UNKNOWN_GLYPH)}" }
                                  }
                              }

                          },

                          Icon::Exper((s, c)) => {
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
                          },
                            Icon::Idiom(prims) => {
                              rsx! {
                                  button { class: "uiua-char-input uiua-radial-char-input",
                                           style: "position: absolute; left: 50%; top: 50%; transform: translate(-50%, -50%) rotate({angle}deg) translateY(-12vw) rotate(-{angle}deg);",
                                           onclick: move |evt| {
                                               evt.prevent_default();
                                               input_contents.write().push_str(&prims.iter().map(|p|p.glyph().unwrap_or(UNKNOWN_GLYPH)).collect::<String>());
                                           },
                                  for p in prims.clone() {
                                        span { class: css_of_prim(&p), style: "font-size: {font}px", "{p.glyph().unwrap_or(UNKNOWN_GLYPH)}" }
                                    }
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
pub fn ButtonIcons(input_contents: Signal<String>, rad_info: Signal<RadialInfo>) -> Element {
    let mut timer = use_signal::<Option<Task>>(|| None);
    let mut cancel_timer = move || {
        if let Some(task) = timer.take() {
            task.cancel();
        }
    };
    let thing = spawn(async move {});
    rsx! {
        for button in button_icons.clone() {
            match button.get(0).unwrap().clone() {
                Icon::Single(ref prims) => {
                    let primsP = prims.clone();
                    let btn_down = button.clone();
                    let btn_up = button.clone();
                    rsx! {
                        button { tabindex: 0,
                                class: "uiua-char-input",
                                 style: "touch-action: none; -webkit-touch-callout: none; -webkit-user-select: none; user-select: none;",
                                 onpointerdown: move |evt| {
                                    evt.prevent_default();
                                    cancel_timer();
                                    info!("Pointer down fired");
                                    let coords = evt.data.screen_coordinates().to_f32();
                                    let task = spawn(async move {
                                        sleep(RADIAL_DELAY).await;
                                        rad_info.write().update(coords);
                                        rad_info.write().active = true;
                                    });
                                    timer.set(Some(task));
                                    rad_info.write().start(coords, btn_down.clone());
                                 },
                                 onpointermove: move |evt| {
                                     evt.prevent_default();
                                     info!("Pointer move fired");
                                     rad_info.write().update(evt.data.screen_coordinates().to_f32());
                                 },
                                 onpointerup: move |evt| {
                                    evt.prevent_default();
                                    cancel_timer();
                                    info!("Pointer up fired");
                                    let pr = if rad_info().active {
                                        let current_index = rad_info().current_selection;
                                        let Icon::Single(ref current_prims) = btn_up[current_index + 1] else {panic!()};
                                        current_prims
                                    } else {&primsP};

                                    rad_info.write().reset();
                                    input_contents.write().push(pr.glyph().unwrap());
                                 },
                                span { class: css_of_prim(prims), "{prims.glyph().unwrap_or(UNKNOWN_GLYPH)}" }
                        }
                    }
                },

                Icon::Exper((s, c)) => {
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
                },
                Icon::Idiom(prims) => {
                              rsx! {
                                  button { class: "uiua-char-input uiua-radial-char-input",
                                           onclick: move |evt| {
                                               evt.prevent_default();
                                               input_contents.write().push_str(&prims.iter().map(|p|p.glyph().unwrap_or(UNKNOWN_GLYPH)).collect::<String>());
                                           },
                                  for p in prims.clone() {
                                        span { class: css_of_prim(&p),"{p.glyph().unwrap_or(UNKNOWN_GLYPH)}" }
                                    }
                                  }
                              }

                          }
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RadialInfo {
    pub active: bool,
    pub current_selection: usize,
    pub starting_position: Point2D<f32, ScreenSpace>,
    pub current_position: Point2D<f32, ScreenSpace>,
    pub glyphs: Vec<Icon>,
    pub style: String,
    // pub timer: Option
}

impl RadialInfo {
    pub fn new() -> Self {
        Self {
            // to do: delete this
            style: "background: gray;".to_string(),
            ..Default::default()
        }
    }

    pub fn start(&mut self, coord: Point2D<f32, ScreenSpace>, glyphs: Vec<Icon>) {
        self.starting_position = coord;
        self.current_position = coord;
        self.glyphs = glyphs;
    }

    pub fn update(&mut self, coord: Point2D<f32, ScreenSpace>) {
        self.current_position = coord;
        self.set_selection();
        self.compute_radial();
    }

    pub fn reset(&mut self) {
        self.active = false;
        self.glyphs.clear();
        self.starting_position = Point2D::default();
        self.current_position = Point2D::default();
    }
    fn compute_radial(&mut self) {
        let num_glyphs = self.glyphs.len() - 1;
        if num_glyphs > 0 {
            let chunk_size = 100. / num_glyphs as f64;
            let spot = chunk_size * self.current_selection as f64;
            if self.current_selection == 0 {
                let low_gray = 100. - (chunk_size / 2.);
                let high_gray = chunk_size / 2.;
                self.style = format!(
                    "background: conic-gradient(gray 0% {high_gray}%, darkgray {high_gray}% {low_gray}%, gray {low_gray}% 100%);",
                );
            } else {
                let low_gray = spot - (chunk_size / 2.);
                let high_gray = spot + (chunk_size / 2.);
                self.style = format!(
                    "background: conic-gradient(darkgray 0% {low_gray}%, gray {low_gray}% {high_gray}%, darkgray {high_gray}% 100%);",
                );
            }
        }
    }
    fn set_selection(&mut self) {
        let num_glyphs = self.glyphs.len() - 1;
        if num_glyphs > 0 {
            let chunk_size = 360 / num_glyphs;
            let vec = self.current_position - self.starting_position;
            let angle =
                (vec.angle_from_x_axis().to_degrees() + 450. + (chunk_size / 2) as f32) % 360.;
            dbg!(angle);
            let section = angle as usize / chunk_size;
            self.current_selection = section;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LastTouchContext {
    pub last_touch: (usize, usize),
    pub timestamp: (), // TODO
}
