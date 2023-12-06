use alloc::{boxed::Box, format, string::String, vec::Vec};

use crate::{ast::*, tokens::*};

type LexResult = Result<LoleToken, LoleError>;

#[derive(Clone)]
struct Lexer {
    file_name: Box<str>,
    chars: Vec<char>,
    index: usize,
    line: usize,
    col: usize,
}

pub fn lex(file_name: &str, script: &str) -> Result<LoleTokenStream, LoleError> {
    let mut lexer = Lexer::new(file_name, script);
    let tokens = lexer.lex_all()?;
    Ok(LoleTokenStream::new(tokens, lexer.loc()))
}

impl Lexer {
    fn new(file_name: &str, script: &str) -> Self {
        Self {
            file_name: file_name.into(),
            chars: script.chars().collect::<Vec<_>>(),
            index: 0,
            line: 1,
            col: 1,
        }
    }

    fn lex_all(&mut self) -> Result<Vec<LoleToken>, LoleError> {
        if self.current_char()? == '@' {
            for _ in 0.."@new_syntax\n".len() {
                self.next_char();
            }
        }

        let mut items = Vec::new();

        self.skip_space();

        while self.index < self.chars.len() {
            items.push(self.lex_token()?);
            self.skip_space();
        }

        Ok(items)
    }

    fn lex_token(&mut self) -> LexResult {
        let char = self.current_char()?;

        if char.is_alphabetic() {
            return self.lex_symbol();
        }

        if char.is_numeric() {
            return self.lex_int_literal();
        }

        if char == '"' {
            return self.lex_string();
        }

        return self.lex_punct();
    }

    fn lex_int_literal(&mut self) -> LexResult {
        let mut loc = self.loc();

        while self.current_char()?.is_numeric() {
            self.next_char();
        }

        loc.length = self.index - loc.offset;

        Ok(LoleToken {
            type_: LoleTokenType::IntLiteral,
            value: self.chars[loc.offset..self.index].iter().collect(),
            loc,
        })
    }

    fn lex_symbol(&mut self) -> LexResult {
        let mut loc = self.loc();

        while is_symbol_char(self.current_char()?) {
            self.next_char();
        }

        loc.length = self.index - loc.offset;

        Ok(LoleToken {
            type_: LoleTokenType::Symbol,
            value: self.chars[loc.offset..self.index].iter().collect(),
            loc,
        })
    }

    fn lex_string(&mut self) -> LexResult {
        let mut loc = self.loc();

        self.next_char(); // skip start quote

        let mut value = String::new();

        loop {
            let c = self.current_char()?;
            match c {
                '"' => break,
                '\\' => {
                    self.next_char();
                    match self.current_char()? {
                        'n' => value.push('\n'),
                        't' => value.push('\t'),
                        '\\' => value.push('\\'),
                        '"' => value.push('"'),
                        _ => {
                            return Err(self.err_unexpected_char());
                        }
                    }
                }
                _ => {
                    value.push(c);
                }
            };
            self.next_char();
        }

        self.next_char(); // skip end quote

        loc.length = self.index - loc.offset;

        Ok(LoleToken {
            type_: LoleTokenType::String,
            value,
            loc,
        })
    }

    fn lex_punct(&mut self) -> LexResult {
        let mut loc = self.loc();

        while is_punct_char(self.current_char()?) {
            self.next_char();
        }

        loc.length = self.index - loc.offset;

        Ok(LoleToken {
            type_: LoleTokenType::Punct,
            value: self.chars[loc.offset..self.index].iter().collect(),
            loc,
        })
    }

    fn skip_space(&mut self) {
        while self.current_char().map(is_space).unwrap_or(false) {
            self.next_char();
        }

        // skip comment
        if let Ok('#') = self.current_char() {
            loop {
                self.next_char();

                let Ok(char) = self.current_char() else {
                    return;
                };

                if char == '\n' {
                    self.skip_space();
                    break;
                }
            }
        }
    }

    fn next_char(&mut self) {
        self.index += 1;

        let Ok(char) = self.current_char() else {
            return;
        };

        if char == '\n' {
            self.col = 0;
            self.line += 1;
            return;
        }

        self.col += 1;
    }

    fn current_char(&mut self) -> Result<char, LoleError> {
        self.chars
            .get(self.index)
            .copied()
            .ok_or_else(|| self.err_unexpected_eof())
    }

    fn err_unexpected_char(&self) -> LoleError {
        LoleError {
            message: format!("ParseError: Unexpected character"),
            loc: self.loc(),
        }
    }

    fn err_unexpected_eof(&self) -> LoleError {
        LoleError {
            message: format!("ParseError: Unexpected EOF"),
            loc: self.loc(),
        }
    }

    fn loc(&self) -> LoleLocation {
        LoleLocation {
            file_name: self.file_name.clone(),
            offset: self.index,
            length: 1,
            line: self.line,
            col: self.col,
        }
    }
}

fn is_space(c: char) -> bool {
    match c {
        ' ' | '\n' | '\t' => true,
        _ => false,
    }
}

fn is_symbol_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn is_punct_char(c: char) -> bool {
    "(){}[]!$%&*+,-./:;<=>?@\\^~|".contains(c)
}
