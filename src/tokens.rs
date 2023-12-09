use alloc::{format, string::String, vec::Vec};

use crate::ast::*;

#[derive(Debug, Clone, PartialEq)]
pub enum LoTokenType {
    StringLiteral,
    IntLiteral,
    Symbol,
    Delim,
    Operator,
}

#[derive(Debug, Clone)]
pub struct LoToken {
    pub type_: LoTokenType,
    pub value: String,
    pub loc: LoLocation,
}

impl LoToken {
    pub fn to_sexpr(self) -> SExpr {
        SExpr::Atom {
            value: self.value,
            kind: if self.type_ == LoTokenType::StringLiteral {
                AtomKind::String
            } else {
                AtomKind::Symbol
            },
            loc: self.loc,
        }
    }
}

#[derive(Clone)]
pub struct LoTokenStream {
    pub tokens: Vec<LoToken>,
    pub index: usize,
    pub terminal_token: LoToken,
}

impl LoTokenStream {
    pub fn new(tokens: Vec<LoToken>, eof_location: LoLocation) -> Self {
        Self {
            tokens,
            index: 0,
            terminal_token: LoToken {
                type_: LoTokenType::Symbol,
                value: "<EOF>".into(),
                loc: eof_location,
            },
        }
    }

    pub fn expect_any(&mut self, type_: LoTokenType) -> Result<&LoToken, LoError> {
        match self.peek() {
            Some(token) if token.type_ == type_ => Ok(self.next().unwrap()),
            Some(token) => Err(LoError {
                message: format!("Unexpected token '{}', wanted {type_:?}", token.value),
                loc: token.loc.clone(),
            }),
            _ => Err(LoError {
                message: format!("Unexpected EOF, wanted {type_:?}"),
                loc: self.terminal_token.loc.clone(),
            }),
        }
    }

    pub fn expect(&mut self, type_: LoTokenType, value: &str) -> Result<&LoToken, LoError> {
        match self.peek() {
            Some(token) if token.type_ == type_ && token.value == value => Ok(self.next().unwrap()),
            Some(token) => Err(LoError {
                message: format!("Unexpected token '{}', wanted '{value}'", token.value),
                loc: token.loc.clone(),
            }),
            _ => Err(LoError {
                message: format!("Unexpected EOF, wanted '{value}'"),
                loc: self.terminal_token.loc.clone(),
            }),
        }
    }

    pub fn eat_any(&mut self, type_: LoTokenType) -> Result<Option<&LoToken>, LoError> {
        match self.peek() {
            Some(token) if token.type_ == type_ => Ok(self.next()),
            Some(_) => Ok(None),
            _ => Err(LoError {
                message: format!("Unexpected EOF"),
                loc: self.terminal_token.loc.clone(),
            }),
        }
    }

    pub fn eat(&mut self, type_: LoTokenType, value: &str) -> Result<Option<&LoToken>, LoError> {
        match self.peek() {
            Some(token) if token.type_ == type_ && token.value == value => Ok(self.next()),
            Some(_) => Ok(None),
            _ => Err(LoError {
                message: format!("Unexpected EOF"),
                loc: self.terminal_token.loc.clone(),
            }),
        }
    }

    pub fn next_is(&mut self, type_: LoTokenType, value: &str) -> Result<bool, LoError> {
        match self.peek() {
            Some(token) if token.type_ == type_ && token.value == value => Ok(true),
            Some(_) => Ok(false),
            _ => Err(LoError {
                message: format!("Unexpected EOF"),
                loc: self.terminal_token.loc.clone(),
            }),
        }
    }

    pub fn peek(&self) -> Option<&LoToken> {
        self.tokens.get(self.index)
    }

    pub fn next(&mut self) -> Option<&LoToken> {
        let token = self.tokens.get(self.index);
        self.index += 1;
        token
    }
}
