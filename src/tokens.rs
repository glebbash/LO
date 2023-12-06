use alloc::{format, string::String, vec::Vec};

use crate::ast::*;

#[derive(Debug, Clone, PartialEq)]
pub enum LoleTokenType {
    StringLiteral,
    IntLiteral,
    Symbol,
    Delim,
    Operator,
}

#[derive(Debug, Clone)]
pub struct LoleToken {
    pub type_: LoleTokenType,
    pub value: String,
    pub loc: LoleLocation,
}

impl LoleToken {
    pub fn to_sexpr(self) -> SExpr {
        SExpr::Atom {
            value: self.value,
            kind: if self.type_ == LoleTokenType::StringLiteral {
                AtomKind::String
            } else {
                AtomKind::Symbol
            },
            loc: self.loc,
        }
    }
}

#[derive(Clone)]
pub struct LoleTokenStream {
    pub tokens: Vec<LoleToken>,
    pub index: usize,
    pub terminal_token: LoleToken,
}

impl LoleTokenStream {
    pub fn new(tokens: Vec<LoleToken>, eof_location: LoleLocation) -> Self {
        Self {
            tokens,
            index: 0,
            terminal_token: LoleToken {
                type_: LoleTokenType::Symbol,
                value: "<EOF>".into(),
                loc: eof_location,
            },
        }
    }

    pub fn expect_any(&mut self, type_: LoleTokenType) -> Result<&LoleToken, LoleError> {
        match self.peek() {
            Some(token) if token.type_ == type_ => Ok(self.next().unwrap()),
            Some(token) => Err(LoleError {
                message: format!("unexpected token {:?}, wanted {type_:?}", token.type_),
                loc: token.loc.clone(),
            }),
            _ => Err(LoleError {
                message: format!("unexpected EOF, wanted {type_:?}"),
                loc: self.terminal_token.loc.clone(),
            }),
        }
    }

    pub fn expect(&mut self, type_: LoleTokenType, value: &str) -> Result<&LoleToken, LoleError> {
        match self.peek() {
            Some(token) if token.type_ == type_ && token.value == value => Ok(self.next().unwrap()),
            Some(token) => Err(LoleError {
                message: format!("unexpected token '{}', wanted '{value}'", token.value),
                loc: token.loc.clone(),
            }),
            _ => Err(LoleError {
                message: format!("unexpected EOF, wanted '{value}'"),
                loc: self.terminal_token.loc.clone(),
            }),
        }
    }

    pub fn eat_any(&mut self, type_: LoleTokenType) -> Result<Option<&LoleToken>, LoleError> {
        match self.peek() {
            Some(token) if token.type_ == type_ => Ok(self.next()),
            Some(_) => Ok(None),
            _ => Err(LoleError {
                message: format!("unexpected EOF"),
                loc: self.terminal_token.loc.clone(),
            }),
        }
    }

    pub fn eat(
        &mut self,
        type_: LoleTokenType,
        value: &str,
    ) -> Result<Option<&LoleToken>, LoleError> {
        match self.peek() {
            Some(token) if token.type_ == type_ && token.value == value => Ok(self.next()),
            Some(_) => Ok(None),
            _ => Err(LoleError {
                message: format!("unexpected EOF"),
                loc: self.terminal_token.loc.clone(),
            }),
        }
    }

    pub fn peek(&self) -> Option<&LoleToken> {
        self.tokens.get(self.index)
    }

    pub fn next(&mut self) -> Option<&LoleToken> {
        let token = self.tokens.get(self.index);
        self.index += 1;
        token
    }
}
