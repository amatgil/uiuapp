use crate::*;
use dioxus::prelude::*;

#[component]
pub fn RadialSelector(input_contents: Signal<String>, rad_info: Signal<RadialInfo>) -> Element {
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
pub fn ButtonIcons(input_contents: Signal<String>, rad_info: Signal<RadialInfo>) -> Element {
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

#[derive(Debug, Clone, Default)]
pub struct RadialInfo {
    pub is_active: bool,
    pub current_selection: usize,
    pub starting_position: Point2D<f64, ScreenSpace>,
    pub current_position: Point2D<f64, ScreenSpace>,
    pub glyphs: Vec<Either<Vec<P>, (&'static str, &'static str)>>,
    pub style: String,
}

impl RadialInfo {
    pub fn new() -> Self {
        Self {
            // to do: delete this
            style: "background: none".to_string(),
            ..Default::default()
        }
    }

    pub fn start(
        &mut self,
        coord: Point2D<f64, ScreenSpace>,
        glyphs: Vec<Either<Vec<P>, (&'static str, &'static str)>>,
    ) {
        self.starting_position = coord;
        self.current_position = coord;
        self.glyphs = glyphs;
    }

    pub fn update(&mut self, coord: Point2D<f64, ScreenSpace>) {
        self.current_position = coord;
        // let frac = 360. / (self.glyphs.len() - 1) as f64;
        // let angle = self
        //     .starting_position
        //     .to_vector()
        //     .angle_to(self.current_position.to_vector())
        //     .to_degrees();
        // dbg!(frac);
        // dbg!(angle);
        // dbg!(angle % frac);
        if !self.is_active && self.should_activate() {
            self.is_active = true;
        }
    }
    pub fn should_activate(&self) -> bool {
        self.starting_position.distance_to(self.current_position) > DEADZONE_RADIUS
    }
    pub fn reset(&mut self) {
        self.is_active = false;
        self.glyphs.clear();
        self.starting_position = Point2D::default();
        self.current_position = Point2D::default();
    }
    pub fn _compute_radial(&mut self) {
        let len = self.glyphs.len();
        let mut initial = String::from("background: conic-gradient(");
        let incr = if len > 0 { 100. / len as f64 } else { 100. };
        let mut count = 0.;
        let mut gray = true;
        while count < 100. {
            let color = if gray { "gray" } else { "white" };
            let radius = 60.;
            let upper = count + incr;
            initial.push_str(format!("{} {count}% {upper}%,", color).as_str());
            count = upper;
            gray = !gray;
        }
        initial.push_str(");");
        dbg!(&initial);
        self.style = initial;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LastTouchContext {
    pub last_touch: (usize, usize),
    pub timestamp: (), // TODO
}
