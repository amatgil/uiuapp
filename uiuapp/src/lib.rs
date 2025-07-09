pub mod highlighting;
pub mod multimedia;
pub mod ui;
pub use highlighting::*;
pub use ui::*;

use dioxus::{
    html::geometry::{euclid::Point2D, ScreenSpace},
    prelude::*,
};
use lazy_static::lazy_static;
use std::{f32::consts::PI, time::Duration};
use uiua::{
    ast::Subscript,
    format::{format_str, FormatConfig},
    PrimClass, Primitive as P, SpanKind,
};

/// An icon is either
/// - A Primitive
/// - An Idiom of multiple Primitives
/// - A string (and its associated html class)
/// (Primitives do not store their class themselves, its computed based on their signature)
#[derive(Debug, Clone, PartialEq)]
pub enum Icon {
    Single(P),
    Idiom(Vec<P>),
    Exper((&'static str, &'static str)),
}

pub const TAU: f32 = 2.0 * PI;
pub const MAX_OUTPUT_CHARS: usize = 1000;
pub const UNKNOWN_GLYPH: char = '¡';
pub const EXPERIMENTAL_ICON: &str = "🧪";
const DEADZONE_RADIUS: f32 = 30.;

#[derive(Debug, Clone)]
pub enum ScrollbackItem {
    Input(Result<Vec<UiuappHistorySpan>, String>),
    Output(Vec<ScrollbackOutput>),
}

#[derive(Debug, Clone)]
pub enum ScrollbackOutput {
    Text(String),
    Image(Vec<u8>),
    Gif(Vec<u8>),
    Audio(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub clean_input_on_run: bool,
    pub execution_limit: Duration,         // TODO: make it do something
    pub audio_sample_time: u32,            // TODO: make it do something
    pub autoplay_video: bool,              // TODO: make it do something
    pub autoplay_audio: bool,              // TODO: make it do something
    pub gayness: (),                       // TODO: make it do something
    pub stack_ordering: StackOrdering,     // TODO: make it do something
    pub font_size: f32,                    // TODO: make it do something
    pub stack_preserved_across_runs: bool, // TODO: make it do something
}
#[derive(Debug, Clone, Default)]
pub enum StackOrdering {
    #[default]
    BottomAtTop,
    TopAtTop,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            clean_input_on_run: false,
            execution_limit: Duration::from_secs(5),
            audio_sample_time: 44100,
            autoplay_video: false,
            autoplay_audio: false,
            gayness: (),
            stack_ordering: StackOrdering::default(),
            font_size: 100.0,                  // TODO: implement
            stack_preserved_across_runs: true, // TODO: implement
        }
    }
}

pub fn run_uiua(code: &str) -> Result<Vec<ScrollbackOutput>, String> {
    let mut runtime = uiua::Uiua::with_safe_sys();
    match runtime.compile_run(|comp| comp.experimental(true).load_str(code)) {
        Ok(_compiler) => Ok(runtime
            .take_stack()
            .into_iter()
            .map(|v| ScrollbackOutput::from_uiuavalue(v, ()))
            .collect()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn handle_running_code(
    mut input_contents: Signal<String>,
    mut buffer_contents: Signal<Vec<ScrollbackItem>>,
    settings: Signal<Settings>,
) {
    use ScrollbackItem as SBI;
    buffer_contents
        .write()
        .push(SBI::Input(highlight_code(&input_contents.read().clone())));
    match run_uiua(&input_contents()) {
        Ok(sbo) => {
            let s = sbo
                .into_iter()
                .map(|s| match s {
                    ScrollbackOutput::Text(ref text) if text.len() > MAX_OUTPUT_CHARS => {
                        ScrollbackOutput::Text(
                            text.chars()
                                .take(MAX_OUTPUT_CHARS)
                                .chain(vec!['.', '.', '.'].into_iter())
                                .collect(),
                        )
                    }
                    x => x,
                })
                .collect();

            buffer_contents.write().push(SBI::Output(s));
            if settings.read().clean_input_on_run {
                *input_contents.write() = String::new();
            }
        }
        Err(s) => {
            buffer_contents
                .write()
                .push(SBI::Output(vec![ScrollbackOutput::Text(s)]));
            *input_contents.write() = String::new();
        }
    }
}

// Tiny convenience for single-character glyphs in button_icons
fn s(p: P) -> Icon {
    Icon::Single(p)
}

lazy_static! {
    /// The car of each line is the default icon. when pressed, the cdr is the radial menu icons
    /// See [ButtonIcon]'s documentation for an explanation of the type
    pub static ref button_icons: [Vec<Icon>; 4 * 5] = [
        // ====== ROW ONE ======
        // Id
        vec![
            s(P::Identity), s(P::Slf), s(P::Backward), s(P::Pop),
            s(P::Dup), s(P::Flip), s(P::Stack),
        ],
        // Stack
        vec![
            s(P::Fork), s(P::Both), s(P::Bracket),
            s(P::Dip), s(P::Gap),
            s(P::On), s(P::By),
            s(P::Off), s(P::With),
            s(P::Below),
        ],
        // Inv
        vec![s(P::Un), s(P::Anti), s(P::Under), s(P::Obverse), s(P::Fill)], // TODO: find a home for fill
        // Iter
        vec![
            s(P::Reduce), s(P::Fold), s(P::Scan), s(P::Repeat),
            s(P::Switch), s(P::Do), s(P::Try), s(P::Case), s(P::Assert),
        ],
        // Sub
        vec![
            s(P::Rows), s(P::Table), s(P::Stencil), s(P::Tuples),
            s(P::Partition), s(P::Group)
        ],

        // ====== ROW TWO ======

        // MAr
        vec![
            s(P::Neg), s(P::Sign), s(P::Not), s(P::Abs), s(P::Sqrt),
            s(P::Sin), s(P::Floor), s(P::Ceil), s(P::Round),
        ],
        // MSt
        vec![
            s(P::Len), s(P::Shape), s(P::First), s(P::Last),
            s(P::Reverse), s(P::Deshape), s(P::Fix), s(P::Transpose),
        ],
        // MVl
        vec![s(P::Range), s(P::Bits), s(P::Where), s(P::Parse)],
        // MCmp
        vec![
            s(P::Sort), s(P::Rise), s(P::Fall), s(P::Classify),
            s(P::Deduplicate), s(P::Unique),
        ],
        // Box
        vec![s(P::Box), s(P::Content), s(P::Inventory)],

        // ===== ROW THREE =====

        // DAr
        vec![
            s(P::Add), s(P::Sub), s(P::Mul), s(P::Div),
            s(P::Modulus), s(P::Pow), s(P::Log), s(P::Atan),
            s(P::Complex), s(P::Base),
        ],

        // DSt
        vec![
            s(P::Couple), s(P::Join), s(P::Select), s(P::Pick),
            s(P::Reshape), s(P::Drop), s(P::Take), s(P::Rotate),
            s(P::Keep), s(P::Orient),
        ],

        // Comp
        vec![
            s(P::Eq), s(P::Ne), s(P::Le), s(P::Lt),
            s(P::Gt), s(P::Ge), s(P::Min), s(P::Max),
        ],
        // DCmp
        vec![
            s(P::Match), s(P::Find), s(P::Mask), s(P::MemberOf),
            s(P::IndexOf), s(P::Partition), s(P::Group),
        ],

        // Const
        vec![s(P::Rand), s(P::Eta), s(P::Pi), s(P::Tau), s(P::Infinity)],

        // ===== ROW FOUR ====

        // TBD
        vec![Icon::Exper(("Empty", ""))], // TODO: Figure out what to put here (baby fat for now)
        // Digits
        vec![Icon::Exper(("0", "constant-value")), // TODO: These should bring up a keypad (see issue #9)
             Icon::Exper(("1", "constant-value")),
             Icon::Exper(("2", "constant-value")),
             Icon::Exper(("3", "constant-value")),
             Icon::Exper(("4", "constant-value")),
             Icon::Exper(("5", "constant-value")),
             Icon::Exper(("6", "constant-value")),
             Icon::Exper(("7", "constant-value")),
             Icon::Exper(("8", "constant-value")),
             Icon::Exper(("9", "constant-value"))],
        // Subs
        vec![Icon::Exper(("₀", "constant-value")),
             Icon::Exper(("₁", "constant-value")),
             Icon::Exper(("₂", "constant-value")),
             Icon::Exper(("₃", "constant-value")),
             Icon::Exper(("₄", "constant-value")),
             Icon::Exper(("₅", "constant-value")),
             Icon::Exper(("₆", "constant-value")),
             Icon::Exper(("₇", "constant-value")),
             Icon::Exper(("₈", "constant-value")),
             Icon::Exper(("₉", "constant-value"))],
        // Exp
        vec![Icon::Exper((EXPERIMENTAL_ICON, ""))], // TODO: Should/Must be autopopulated
        // Idioms
        vec![Icon::Idiom(vec![P::Sub, P::By, P::Not])],
    ];
}

#[test]
fn keypad_has_all_prims() {
    fn prim_exists_in_keypad(p: P) -> bool {
        for grouping in button_icons.clone() {
            if grouping.contains(&s(p)) {
                return true;
            }
        }
        return false;
    }
    for prim in uiua::Primitive::non_deprecated() {
        if prim.glyph().is_none() || prim.is_experimental() {
            continue;
        }
        if !prim_exists_in_keypad(prim) {
            panic!(
                "Glyph is not typable: '{}' ({})",
                prim.glyph().unwrap(),
                prim.name()
            );
        }
    }
}
