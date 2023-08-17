use core::fmt::Write;

use alloc::{boxed::Box, format, string::String, vec::Vec};

pub struct CompileError {
    pub message: String,
    pub loc: Location,
}

impl CompileError {
    pub fn unreachable(file: &str, line: u32) -> CompileError {
        CompileError {
            message: format!("Unreachable in {}, {}", file, line),
            loc: Location::internal(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub file_name: Box<str>,
    pub offset: usize,
    pub length: usize,
    pub line: usize,
    pub col: usize,
}

impl Location {
    pub fn internal() -> Self {
        Location {
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
        loc: Location,
    },
    List {
        value: Vec<SExpr>,
        loc: Location,
    },
}

impl SExpr {
    pub fn loc(&self) -> &Location {
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
