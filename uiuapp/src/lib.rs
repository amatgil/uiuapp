pub mod highlighting;
pub mod ui;
pub use highlighting::*;
pub use ui::*;

use dioxus::{
    html::geometry::{euclid::Point2D, ScreenSpace},
    prelude::*,
};
use lazy_static::lazy_static;
use std::{f32::consts::PI, fmt::Display};
use uiua::{
    ast::Subscript,
    format::{format_str, FormatConfig},
    PrimClass, Primitive as P, SpanKind,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}
use Either as E;

/// An icon is either
/// - (A vector of) Primitives
/// - A string (and its associated html class)
/// (Primitives do not store their class themselves, its computed based on their signature)
///
/// Primitives are stored as a vector to support multi-primitive icons, like `wrench` (subbyneg)
pub type ButtonIcon = Either<Vec<P>, (&'static str, &'static str)>;

pub const TAU: f32 = 2.0 * PI;
pub const MAX_OUTPUT_CHARS: usize = 1000;
pub const UNKNOWN_GLYPH: char = '¡';
pub const EXPERIMENTAL_ICON: &str = "🧪";
const DEADZONE_RADIUS: f64 = 30.;

#[derive(Debug, Clone)]
pub enum ScrollbackItem {
    Input(Result<Vec<UiuappHistorySpan>, String>),
    Output(String),
}

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

pub fn handle_running_code(
    mut input_contents: Signal<String>,
    mut buffer_contents: Signal<Vec<ScrollbackItem>>,
) {
    use ScrollbackItem as SBI;
    buffer_contents
        .write()
        .push(SBI::Input(highlight_code(&input_contents.read().clone())));
    match run_uiua(&input_contents()) {
        Ok(v) => {
            for s in v {
                buffer_contents.write().push(SBI::Output(s));
            }
            //*input_contents.write() = String::new(); // This was seen as undesirable, TODO: add as Setting option
        }
        Err(s) => {
            buffer_contents.write().push(SBI::Output(s));
            *input_contents.write() = String::new();
        }
    }
}

// Tiny convenience for single-character glyphs in button_icons
fn l(p: P) -> Either<Vec<P>, (&'static str, &'static str)> {
    E::Left(vec![p])
}

lazy_static! {
    /// The car of each line is the default icon. when pressed, the cdr is the radial menu icons
    /// See [ButtonIcon]'s documentation for an explanation of the type
    pub static ref button_icons: [Vec<ButtonIcon>; 4 * 5] = [
        // ====== ROW ONE ======
        // Id
        vec![
            l(P::Identity), l(P::Slf), l(P::Backward), l(P::Pop),
            l(P::Dup), l(P::Flip), l(P::Stack),
        ],
        // Stack
        vec![
            l(P::Fork), l(P::Both), l(P::Bracket),
            l(P::Dip), l(P::Gap),
            l(P::On), l(P::By),
            l(P::Off), l(P::With),
            l(P::Below),
        ],
        // Inv
        vec![l(P::Un), l(P::Anti), l(P::Under), l(P::Obverse), l(P::Fill)], // TODO: find a home for fill
        // Iter
        vec![
            l(P::Reduce), l(P::Fold), l(P::Scan), l(P::Repeat),
            l(P::Switch), l(P::Do), l(P::Try), l(P::Case), l(P::Assert),
        ],
        // Sub
        vec![
            l(P::Rows), l(P::Table), l(P::Stencil), l(P::Tuples),
            l(P::Partition), l(P::Group)
        ],

        // ====== ROW TWO ======

        // MAr
        vec![
            l(P::Neg), l(P::Sign), l(P::Not), l(P::Abs), l(P::Sqrt),
            l(P::Sin), l(P::Floor), l(P::Ceil), l(P::Round),
        ],
        // MSt
        vec![
            l(P::Len), l(P::Shape), l(P::First), l(P::Last),
            l(P::Reverse), l(P::Deshape), l(P::Fix), l(P::Transpose),
        ],
        // MVl
        vec![l(P::Range), l(P::Bits), l(P::Where), l(P::Parse)],
        // MCmp
        vec![
            l(P::Sort), l(P::Rise), l(P::Fall), l(P::Classify),
            l(P::Deduplicate), l(P::Unique),
        ],
        // Box
        vec![l(P::Box), l(P::Content), l(P::Inventory)],

        // ===== ROW THREE =====

        // DAr
        vec![
            l(P::Add), l(P::Sub), l(P::Mul), l(P::Div),
            l(P::Modulus), l(P::Pow), l(P::Log), l(P::Atan),
            l(P::Complex), l(P::Base),
        ],

        // DSt
        vec![
            l(P::Couple), l(P::Join), l(P::Select), l(P::Pick),
            l(P::Reshape), l(P::Drop), l(P::Take), l(P::Rotate),
            l(P::Keep), l(P::Orient),
        ],

        // Comp
        vec![
            l(P::Eq), l(P::Ne), l(P::Le), l(P::Lt),
            l(P::Gt), l(P::Ge), l(P::Min), l(P::Max),
        ],
        // DCmp
        vec![
            l(P::Match), l(P::Find), l(P::Mask), l(P::MemberOf),
            l(P::IndexOf), l(P::Partition), l(P::Group),
        ],

        // Const
        vec![l(P::Rand), l(P::Eta), l(P::Pi), l(P::Tau), l(P::Infinity)],

        // ===== ROW FOUR ====

        // TBD
        vec![E::Right(("Empty", ""))], // TODO: Figure out what to put here (baby fat for now)
        // Digits
        vec![E::Right(("0", "constant-value")), // TODO: These should bring up a keypad (see issue #9)
             E::Right(("1", "constant-value")),
             E::Right(("2", "constant-value")),
             E::Right(("3", "constant-value")),
             E::Right(("4", "constant-value")),
             E::Right(("5", "constant-value")),
             E::Right(("6", "constant-value")),
             E::Right(("7", "constant-value")),
             E::Right(("8", "constant-value")),
             E::Right(("9", "constant-value"))],
        // Subs
        vec![E::Right(("₀", "constant-value")),
             E::Right(("₁", "constant-value")),
             E::Right(("₂", "constant-value")),
             E::Right(("₃", "constant-value")),
             E::Right(("₄", "constant-value")),
             E::Right(("₅", "constant-value")),
             E::Right(("₆", "constant-value")),
             E::Right(("₇", "constant-value")),
             E::Right(("₈", "constant-value")),
             E::Right(("₉", "constant-value"))],
        // Exp
        vec![E::Right((EXPERIMENTAL_ICON, ""))], // TODO: Should/Must be autopopulated
        // Idioms
        vec![E::Left(vec![P::Sub, P::By, P::Not])],
    ];
}

#[test]
fn keypad_has_all_prims() {
    fn prim_exists_in_keypad(p: P) -> bool {
        for grouping in button_icons.clone() {
            if grouping.contains(&l(p)) {
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
