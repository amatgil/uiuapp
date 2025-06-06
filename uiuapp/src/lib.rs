use std::f32::consts::PI;

use uiua::Primitive as P;

pub type ButtonIcon = Either<Vec<P>, (&'static str, &'static str)>;

pub const TAU: f32 = 2.0 * PI;
pub const MAX_OUTPUT_CHARS: usize = 1000;

pub fn run_uiua(code: &str) -> Result<Vec<String>, String> {
    let mut runtime = uiua::Uiua::with_safe_sys();
    match runtime.run_str(code) {
        Ok(_compiler) => {
            let mut out = vec![];
            for s in runtime.take_stack() {
                let s = s.show();
                if s.len() > MAX_OUTPUT_CHARS {
                    out.push(s.chars()
                        .take(MAX_OUTPUT_CHARS)
                        .chain(vec!['.', '.', '.'].into_iter())
                        .collect());
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


#[derive(Debug, Clone)]
pub enum ScrollbackItem {
    Input(String),
    Output(String)
}


use dioxus::prelude::*;
pub fn handle_running_code(mut input_contents: Signal<String>, mut buffer_contents: Signal<Vec<ScrollbackItem>>,) {
    use ScrollbackItem as SBI;
    match run_uiua(&input_contents()) {
        Ok(v) =>  {
            // TODO: The pushed Input should be the formatted
            // string instead of the input string
            buffer_contents.write().push(SBI::Input(input_contents.read().clone()));
            for s in v {
                buffer_contents.write().push(SBI::Output(s));
            }
            *input_contents.write() = String::new();
        },
        Err(s) => {
            buffer_contents.write().push(SBI::Input(input_contents.read().clone()));
            buffer_contents.write().push(SBI::Output(s));
            *input_contents.write() = String::new();
        }
    }


}

