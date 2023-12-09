use alloc::{format, string::String, vec::Vec};

use crate::ast::*;

#[derive(Debug, Clone, PartialEq, Copy)]
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
    pub fn is_any(&self, type_: LoTokenType) -> bool {
        return self.type_ == type_;
    }

    pub fn is(&self, type_: LoTokenType, value: &str) -> bool {
        return self.is_any(type_) && self.value == value;
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
            Some(token) if token.is_any(type_) => Ok(self.next().unwrap()),
            other => {
                let unexpected = other.unwrap_or(&self.terminal_token);
                Err(LoError {
                    message: format!("Unexpected token '{}', wanted {type_:?}", unexpected.value),
                    loc: unexpected.loc.clone(),
                })
            }
        }
    }

    pub fn expect(&mut self, type_: LoTokenType, value: &str) -> Result<&LoToken, LoError> {
        match self.peek() {
            Some(token) if token.is(type_, value) => Ok(self.next().unwrap()),
            other => {
                let unexpected = other.unwrap_or(&self.terminal_token);
                Err(LoError {
                    message: format!("Unexpected token '{}', wanted '{value}'", unexpected.value),
                    loc: unexpected.loc.clone(),
                })
            }
        }
    }

    pub fn eat_any(&mut self, type_: LoTokenType) -> Result<Option<&LoToken>, LoError> {
        let was_some = self.peek().is_some();
        match self.expect_any(type_) {
            Ok(t) => Ok(Some(t)),
            Err(_) if was_some => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub fn eat(&mut self, type_: LoTokenType, value: &str) -> Result<Option<&LoToken>, LoError> {
        let was_some = self.peek().is_some();
        match self.expect(type_, value) {
            Ok(t) => Ok(Some(t)),
            Err(_) if was_some => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub fn next_is(&mut self, type_: LoTokenType, value: &str) -> Result<bool, LoError> {
        match self.peek() {
            Some(token) if token.is(type_, value) => Ok(true),
            Some(_) => Ok(false),
            _ => self.err_eof(format!("Unexpected EOF")),
        }
    }

    pub fn peek(&self) -> Option<&LoToken> {
        self.tokens.get(self.index)
    }

    pub fn peek2(&self) -> Option<(&LoToken, &LoToken)> {
        let t1 = self.tokens.get(self.index)?;
        let t2 = self.tokens.get(self.index + 1)?;
        Some((t1, t2))
    }

    pub fn next(&mut self) -> Option<&LoToken> {
        let token = self.tokens.get(self.index);
        self.index += 1;
        token
    }

    fn err_eof<T>(&self, message: String) -> Result<T, LoError> {
        Err(LoError {
            message,
            loc: self.terminal_token.loc.clone(),
        })
    }
}
