use crate::*;

#[derive(Debug, Clone)]
pub enum UiuappHistorySpan {
    UnstyledCode {
        text: String,
    },
    StyledCode {
        class: String, // HTML/css class
        text: String,  // the actual code
    },
    Whitspace(String),
}

/// Returns tuples of (span, text)
pub fn highlight_code(code: &str) -> Result<Vec<UiuappHistorySpan>, String> {
    let config = FormatConfig::default();
    let code = match format_str(code, &config) {
        Ok(s) => s.output,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let mut output: Vec<UiuappHistorySpan> = vec![];

    for s in uiua::lsp::Spans::from_input(&code).spans {
        let text = &code[s.span.start.byte_pos as usize..s.span.end.byte_pos as usize].trim_end();

        let whitespace = (code[0..s.span.start.byte_pos as usize])
            .chars()
            .rev()
            .take_while(|c| c.is_whitespace())
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();

        output.push(UiuappHistorySpan::Whitspace(whitespace));
        output.push(match html_class_of(&s.value) {
            Some(class) => UiuappHistorySpan::StyledCode {
                class: class.to_string(),
                text: text.to_string(),
            },
            None => UiuappHistorySpan::UnstyledCode {
                text: text.to_string(),
            },
        });
    }

    Ok(output)
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

fn html_class_of(s: &SpanKind) -> Option<&'static str> {
    match s {
        SpanKind::Primitive(p, sub) => html_class_of_prim_sub(Some(*p), *sub),
        SpanKind::String => Some("string-literal"),
        SpanKind::Number => Some("constant-value"),
        SpanKind::Comment => Some("comment"),
        SpanKind::OutputComment => Some("comment"),
        SpanKind::Strand => Some("strand"),
        SpanKind::Obverse(_) => Some("monadic-modifier"),
        SpanKind::FuncDelim(s, _) => {
            if s.args() == 1 {
                Some("monadic-function")
            } else if s.args() == 2 {
                Some("dyadic-function")
            } else {
                None
            }
        }
        SpanKind::MacroDelim(s) => {
            if *s == 1 {
                Some("monadic-modifier")
            } else if *s == 2 {
                Some("dyadic-modifier")
            } else {
                None
            }
        }
        SpanKind::Subscript(prim, Some(sub)) => html_class_of_prim_sub(*prim, Some(*sub)),
        SpanKind::Ident { .. }
        | SpanKind::Label
        | SpanKind::Signature
        | SpanKind::Whitespace
        | SpanKind::Placeholder(_)
        | SpanKind::Delimiter
        | SpanKind::LexOrder
        | SpanKind::ImportSrc(_)
        | SpanKind::Subscript(_, None) => None,
    }
}
fn html_class_of_prim(prim: P, args: Option<usize>) -> Option<&'static str> {
    if let Some(args) = prim.modifier_args() {
        return if args == 1 {
            Some("monadic-modifier")
        } else {
            Some("dyadic-modifier")
        };
    }

    if matches!(prim.class(), PrimClass::Stack | PrimClass::Debug) || prim == P::Identity {
        Some("stack-function")
    } else if prim == P::Transpose {
        Some("uiua-trans")
    } else {
        args.or(prim.sig().map(|sig| sig.args()))
            .and_then(|args| match args {
                0 => Some("noadic-function"),
                1 => Some("monadic-function"),
                2 => Some("dyadic-function"),
                _ => None,
            })
    }
}

fn html_class_of_prim_sub(prim: Option<P>, sub: Option<Subscript>) -> Option<&'static str> {
    let args = prim
        .and_then(|prim| prim.subscript_sig(sub))
        .map(|sig| sig.args());
    prim.map(|prim| html_class_of_prim(prim, args))
        .unwrap_or_default()
}

impl Display for UiuappHistorySpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UiuappHistorySpan::UnstyledCode { text } => text.clone(),
                UiuappHistorySpan::StyledCode { class, text } => {
                    format!("<span class={class}>{text}</span>")
                }
                UiuappHistorySpan::Whitspace(text) => text.clone(),
            }
        )
    }
}
