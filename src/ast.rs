use alloc::{boxed::Box, format, string::String, vec::Vec};
use core::fmt::Write;

#[derive(PartialEq)]
pub struct LoleError {
    pub message: String,
    pub loc: LoleLocation,
}

impl LoleError {
    pub fn unreachable(file: &str, line: u32) -> LoleError {
        LoleError {
            message: format!("Unreachable in {}:{}", file, line),
            loc: LoleLocation::internal(),
        }
    }
}

impl From<LoleError> for String {
    fn from(err: LoleError) -> Self {
        let LoleLocation {
            file_name,
            line,
            col,
            ..
        } = err.loc;

        format!("{file_name}:{line}:{col} - {msg}", msg = err.message,)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoleLocation {
    pub file_name: Box<str>,
    pub offset: usize,
    pub length: usize,
    pub line: usize,
    pub col: usize,
}

impl LoleLocation {
    pub fn internal() -> Self {
        LoleLocation {
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
        loc: LoleLocation,
    },
    List {
        value: Vec<SExpr>,
        loc: LoleLocation,
    },
}

impl SExpr {
    pub fn loc(&self) -> &LoleLocation {
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
