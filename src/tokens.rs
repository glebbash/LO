use alloc::{string::String, vec::Vec};

use crate::ast::*;

#[derive(Debug, PartialEq)]
pub enum LoleTokenType {
    String,
    IntLiteral,
    Symbol,
    Punct,
}

#[derive(Debug)]
pub struct LoleToken {
    pub type_: LoleTokenType,
    pub value: String,
    pub loc: LoleLocation,
}

pub struct LoleTokenStream {
    tokens: Vec<LoleToken>,
    index: usize,
    eof_location: LoleLocation,
}

impl LoleTokenStream {
    pub fn new(tokens: Vec<LoleToken>, eof_location: LoleLocation) -> Self {
        Self {
            tokens,
            index: 0,
            eof_location,
        }
    }

    pub fn expect_symbol(&mut self) -> Result<&LoleToken, LoleError> {
        match self.peek() {
            Some(token) => {
                if token.type_ == LoleTokenType::Symbol {
                    Ok(self.next().unwrap())
                } else {
                    Err(LoleError {
                        message: String::from("unexpected token, symbol expected"),
                        loc: token.loc.clone(),
                    })
                }
            }
            _ => Err(LoleError {
                message: String::from("unexpected token, symbol expected"),
                loc: self.eof_location.clone(),
            }),
        }
    }

    pub fn eat_symbol(&mut self, value: &str) -> Option<&LoleToken> {
        match self.peek() {
            Some(token) if token.type_ == LoleTokenType::Symbol && token.value == value => {
                self.next()
            }
            _ => None,
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
