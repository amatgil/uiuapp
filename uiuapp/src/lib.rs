use std::f32::consts::PI;

use uiua::Primitive as P;

pub type ButtonIcon = Either<Vec<P>, (&'static str, &'static str)>;

pub const TAU: f32 = 2.0 * PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub fn css_of_prim(p: &P) -> &'static str {
    let special_cased = [(P::Transpose, "uiua-trans")];
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

#[derive(Debug, Clone)]
pub struct RadialInfo {
    pub last_pos: (usize, usize),
    pub glyphs: Vec<ButtonIcon>,
}

#[derive(Debug, Clone, Copy)]
pub struct LastTouchContext {
    pub last_touch: (usize, usize),
    pub timestamp: (), // TODO
}
