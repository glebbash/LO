use alloc::{boxed::Box, format, string::String, vec::Vec};
use core::fmt::Write;

#[derive(PartialEq)]
pub struct LoError {
    pub message: String,
    pub loc: LoLocation,
}

impl LoError {
    pub fn unreachable(file: &str, line: u32) -> LoError {
        LoError {
            message: format!("Unreachable in {}:{}", file, line),
            loc: LoLocation::internal(),
        }
    }
}

impl From<LoError> for String {
    fn from(err: LoError) -> Self {
        let LoLocation {
            file_name,
            line,
            col,
            ..
        } = err.loc;

        format!("{file_name}:{line}:{col} - {msg}", msg = err.message,)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoLocation {
    pub file_name: Box<str>,
    pub offset: usize,
    pub length: usize,
    pub line: usize,
    pub col: usize,
}

impl LoLocation {
    pub fn internal() -> Self {
        LoLocation {
            file_name: "<internal>".into(),
            offset: 0,
            length: 0,
            line: 0,
            col: 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AtomKind {
    Symbol,
    String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SExpr {
    Atom {
        value: String,
        kind: AtomKind,
        loc: LoLocation,
    },
    List {
        value: Vec<SExpr>,
        loc: LoLocation,
    },
}

impl SExpr {
    pub fn loc(&self) -> &LoLocation {
        match self {
            Self::Atom { loc, .. } => loc,
            Self::List { loc, .. } => loc,
        }
    }
}

impl core::fmt::Display for SExpr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Atom {
                value,
                kind: AtomKind::Symbol,
                ..
            } => f.write_str(value),
            Self::Atom {
                value,
                kind: AtomKind::String,
                ..
            } => f.write_fmt(format_args!("\"{value}\"")),
            Self::List { value, .. } => match &value[..] {
                [] => f.write_str("()"),
                [head, tail @ ..] => {
                    f.write_char('(')?;

                    head.fmt(f)?;

                    for expr in tail {
                        f.write_char(' ')?;
                        expr.fmt(f)?;
                    }

                    f.write_char(')')
                }
            },
        }
    }
}
