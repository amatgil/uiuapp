use std::f32::consts::PI;

use lazy_static::lazy_static;
use uiua::Primitive as P;

pub type ButtonIcon = Either<Vec<P>, (&'static str, &'static str)>;

pub const TAU: f32 = 2.0 * PI;
pub const MAX_OUTPUT_CHARS: usize = 1000;
pub const UNKNOWN_GLYPH: char = '¡';
pub const EXPERIMENTAL_ICON: &str = "🧪";
const DEADZONE_RADIUS: f64 = 30.;

pub fn run_uiua(code: &str) -> Result<Vec<String>, String> {
    let mut runtime = uiua::Uiua::with_safe_sys();
    match runtime.run_str(code) {
        Ok(_compiler) => {
            let mut out = vec![];
            for s in runtime.take_stack() {
                let s = s.show();
                if s.len() > MAX_OUTPUT_CHARS {
                    out.push(
                        s.chars()
                            .take(MAX_OUTPUT_CHARS)
                            .chain(vec!['.', '.', '.'].into_iter())
                            .collect(),
                    );
                } else {
                    out.push(s);
                }
            }

            return Ok(out);
        }
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}
use Either as E;

pub fn css_of_prim(p: &P) -> &'static str {
    let special_cased = [
        (P::Transpose, "uiua-trans"),
        (P::Identity, "stack-function"),
    ];
    if let Some((_, s)) = special_cased.iter().find(|l| l.0 == *p) {
        s
    } else if let Some(args) = p.args() {
        match args {
            0 => "noadic-function",
            1 => "monadic-function",
            2 => "dyadic-function",
            _ => "",
        }
    } else if let Some(args) = p.modifier_args() {
        match args {
            1 => "monadic-modifier",
            2 => "dyadic-modifier",
            _ => "",
        }
    } else {
        ""
    }
}

#[derive(Debug, Clone, Default)]
pub struct RadialInfo {
    pub is_active: bool,
    pub current_selection: usize,
    pub starting_position: Point2D<f64, ScreenSpace>,
    pub current_position: Point2D<f64, ScreenSpace>,
    pub glyphs: Vec<P>,
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

    pub fn start(&mut self, coord: Point2D<f64, ScreenSpace>, glyphs: Vec<P>) {
        self.starting_position = coord;
        self.current_position = coord;
        self.glyphs = glyphs;
    }

    pub fn update(&mut self, coord: Point2D<f64, ScreenSpace>) {
        self.current_position = coord;
        let frac = 360. / (self.glyphs.len() - 1) as f64;
        let angle = self
            .starting_position
            .to_vector()
            .angle_to(self.current_position.to_vector())
            .to_degrees();
        dbg!(frac);
        dbg!(angle);
        dbg!(angle % frac);
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
    pub fn compute_radial(&mut self) {
        let len = self.glyphs.len();
        let mut initial = String::from("background: conic-gradient(");
        let incr = if len > 0 { 100. / len as f64 } else { 100. };
        let mut count = 0.;
        let mut gray = true;
        while count < 100. {
            let color = if gray { "gray" } else { "white" };
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

#[derive(Debug, Clone)]
pub enum ScrollbackItem {
    Input(String),
    Output(String),
}

use dioxus::{
    html::geometry::{
        euclid::{default, Point2D},
        Coordinates, ScreenSpace,
    },
    prelude::*,
};
pub fn handle_running_code(
    mut input_contents: Signal<String>,
    mut buffer_contents: Signal<Vec<ScrollbackItem>>,
) {
    use ScrollbackItem as SBI;
    match run_uiua(&input_contents()) {
        Ok(v) => {
            // TODO: The pushed Input should be the formatted
            // string instead of the input string
            buffer_contents
                .write()
                .push(SBI::Input(input_contents.read().clone()));
            for s in v {
                buffer_contents.write().push(SBI::Output(s));
            }
            *input_contents.write() = String::new();
        }
        Err(s) => {
            buffer_contents
                .write()
                .push(SBI::Input(input_contents.read().clone()));
            buffer_contents.write().push(SBI::Output(s));
            *input_contents.write() = String::new();
        }
    }
}

// Tiny convenience for single-character glyphs
fn l(p: Vec<P>) -> Either<Vec<P>, (&'static str, &'static str)> {
    E::Left(p)
}

lazy_static! {
    /// The car of each line is the default icon. when pressed, the cdr is the radial menu icons
    /// See [this pad link](https://www.uiua.org/pad?src=0_17_0-dev_1__SWQgICAgIOKGkCBtYXBA4oiY4pahIsuZy5zil4wuOiIKU3RhY2sgIOKGkCBtYXBA4oqD4pahIuKIqeKKk-KKmeKLheKfnOKKuOKkmeKkmuKXoSIKSW52ICAgIOKGkCBtYXBAwrDilqEi4oyd4o2c4oyFIgpJdGVyICAg4oaQIG1hcEAv4pahIuKIp1xc4o2l4o2j4o2p4o2i4o2kIgpTdWIgICAg4oaQIG1hcEDiiaHilqEi4oqe4qeF4qeI4oqV4oqcIgpNQXIgICAg4oaQIG1hcEDCr-KWoSLCscKs4oy14oia4oi_4oyK4oyI4oGFIgpNU3QgICAg4oaQIG1hcEDip7vilqEi4paz4oqi4oqj4oeM4pmtwqTijYkiCk1WbCAgICDihpAgbWFwQOKHoeKWoSLii6_iiprii5UiCk1DbXAgICDihpAgbWFwQOKNhuKWoSLijY_ijZbiipvil7Til7AiCkJveCAgICDihpAgbWFwQOKWoeKWoSLil4fijZoiCkRBciAgICDihpAgbWFwQCvilqEiLcOXw7fil7_igb_igpniiKDihILiiqUiCkRTdCAgICDihpAgbWFwQOKKn-KWoSLiioLiio_iiqHihq_ihpnihpjihrvilr0iCkNvbXAgICDihpAgbWFwQD3ilqEi4omgPOKJpD7iiaXihqfihqUiCkRDbXAgICDihpAgbWFwQOKJjeKWoSLijJXiprfiiIriipciCkNvbnN0ICDihpAgbWFwQOKaguKWoSLOt8-Az4TiiJ4iCk51bXMgICDihpAgbWFwQDDilqEiMTIzNDU2Nzg5IgpTdWJzICAg4oaQIG1hcEDigoDilqEi4oKB4oKC4oKD4oKE4oKF4oKG4oKH4oKI4oKJIgpFeHAgICAg4oaQIG1hcEB44pahIuKIqOKnhuKni_CdhJDil6DiqZziiILiiKsiCklkaW9tcyDihpAgIi3iirjCrCIKWwogIHtJZCBTdGFjayBJbnYgSXRlciBTdWJ9CiAge01BciBNU3QgTVZsIE1DbXAgQm94fQogIHtEQXIgRFN0IENvbXAgRENtcCBDb25zdH0KICB7IkVtcHR5IiBOdW1zIFN1YnMgRXhwIElkaW9tc30KXQo=) for the origin
    pub static ref button_icons: [ButtonIcon; 4 * 5] = [
        // ====== ROW ONE ======
        // Id
            l(vec![P::Identity, P::Slf,P::Backward,  P::Pop, P::Dup, P::Flip, P::Stack]),
        // Stack
            l(vec![P::Fork, P::Both, P::Bracket, P::Dip, P::Gap, P::On, P::By,P::Off, P::With, P::Below]),
        // Inv
        l(vec![P::Un, P::Anti, P::Under, P::Obverse, P::Fill]), // TODO: find a home for fill
        // Iter
        l(vec![
            P::Reduce,
            P::Fold,
            P::Scan,
            P::Repeat,
            P::Switch,
            P::Do,
            P::Try,
            P::Case,
            P::Assert,
        ]),
        // Sub
        l(vec![
            P::Rows,
            P::Table,
            P::Stencil,
            P::Tuples,
            P::Partition,
            P::Group
        ]),

        // ====== ROW TWO ======

        // MAr
        l(vec![
            P::Neg,
            P::Sign,
            P::Not,
            P::Abs,
            P::Sqrt,
            P::Sin,
            P::Floor,
            P::Ceil,
            P::Round,
        ]),
        // MSt
        l(vec![
            P::Len,
            P::Shape,
            P::First,
            P::Last,
            P::Reverse,
            P::Deshape,
            P::Fix,
            P::Transpose,
        ]),
        // MVl
        l(vec![P::Range, P::Bits, P::Where, P::Parse]),
        // MCmp
        l(vec![
            P::Sort,
            P::Rise,
            P::Fall,
            P::Classify,
            P::Deduplicate,
            P::Unique,
        ]),
        // Box
        l(vec![P::Box, P::Content, P::Inventory]),

        // ===== ROW THREE =====

        // DAr
        l(vec![
            P::Add,
            P::Sub,
            P::Mul,
            P::Div,
            P::Modulus,
            P::Pow,
            P::Log,
            P::Atan,
            P::Complex,
            P::Base,
        ]),

        // DSt
        l(vec![
            P::Couple,
            P::Join,
            P::Select,
            P::Pick,
            P::Reshape,
            P::Drop,
            P::Take,
            P::Rotate,
            P::Keep,
            P::Orient,
        ]),

        // Comp
        l(vec![
            P::Eq,
            P::Ne,
            P::Le,
            P::Lt,
            P::Gt,
            P::Ge,
            P::Min,
            P::Max,
        ]),
        // DCmp
        l(vec![
            P::Match,
            P::Find,
            P::Mask,
            P::MemberOf,
            P::IndexOf,
            P::Partition,
            P::Group,
        ]),

        // Const
        l(vec![P::Rand, P::Eta, P::Pi, P::Tau, P::Infinity]),

        // ===== ROW FOUR ====

        // TBD
        E::Right(("Empty", "")),
        // Digits
        E::Right(("0", "constant-value")),
        // Subs
        E::Right(("₀", "constant-value")),
        // Exp
        E::Right((EXPERIMENTAL_ICON, "")), // TODO: Should/Must be autopopulated
        // Idioms
        l(vec![P::Sub, P::By, P::Neg]),
    ];
}

// #[test]
// fn keypad_has_all_prims() {
//     // fn prim_exists_in_keypad(p: P) -> bool {
//     //     for grouping in button_icons.clone() {
//     //         if grouping.contains(&l(p)) {
//     //             return true;
//     //         }
//     //     }
//     //     return false;
//     // }
//     for prim in uiua::Primitive::non_deprecated() {
//         if prim.glyph().is_none() || prim.is_experimental() {
//             continue;
//         }
//         if !prim_exists_in_keypad(prim) {
//             panic!(
//                 "Glyph is not typable: '{}' ({})",
//                 prim.glyph().unwrap(),
//                 prim.name()
//             );
//         }
//     }
// }
