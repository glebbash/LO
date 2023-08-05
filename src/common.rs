use alloc::{boxed::Box, string::String};

pub struct CompileError {
    pub message: String,
    pub loc: Location,
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
