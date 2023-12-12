use alloc::{boxed::Box, format, string::String};

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
