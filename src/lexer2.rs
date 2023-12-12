use crate::{ast::*, tokens::*};
use alloc::{boxed::Box, format, string::String, vec::Vec};

type LexResult = Result<LoToken, LoError>;

#[derive(Clone)]
struct Lexer2 {
    file_name: Box<str>,
    chars: Vec<char>,
    index: usize,
    line: usize,
    col: usize,
}

pub fn lex_all(file_name: &str, script: &str) -> Result<LoTokenStream, LoError> {
    Lexer2::new(file_name, script).lex_all()
}

impl Lexer2 {
    fn new(file_name: &str, script: &str) -> Self {
        Self {
            file_name: file_name.into(),
            chars: script.chars().collect::<Vec<_>>(),
            index: 0,
            line: 1,
            col: 1,
        }
    }

    fn lex_all(&mut self) -> Result<LoTokenStream, LoError> {
        let mut tokens = Vec::new();

        self.skip_space();

        while self.index < self.chars.len() {
            tokens.push(self.lex_token()?);
            self.skip_space();
        }

        Ok(LoTokenStream::new(tokens, self.loc()))
    }

    fn lex_token(&mut self) -> LexResult {
        let char = self.current_char()?;

        if char == '"' {
            return self.lex_string();
        }
        if char.is_numeric() {
            return self.lex_int_literal();
        }
        if is_symbol_char(char) {
            return self.lex_symbol();
        }
        if is_delim_char(char) {
            return self.lex_delim();
        }
        if is_operator_char(char) {
            return self.lex_operator();
        }

        return Err(LoError {
            message: format!("Unexpected char: {char}"),
            loc: self.loc(),
        });
    }

    fn lex_int_literal(&mut self) -> LexResult {
        let mut loc = self.loc();

        while self.current_char()?.is_numeric() {
            self.next_char();
        }

        loc.length = self.index - loc.offset;

        Ok(LoToken {
            type_: LoTokenType::IntLiteral,
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

        Ok(LoToken {
            type_: LoTokenType::Symbol,
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

        Ok(LoToken {
            type_: LoTokenType::StringLiteral,
            value,
            loc,
        })
    }

    fn lex_delim(&mut self) -> LexResult {
        let loc = self.loc();

        self.next_char(); // skip delimiter char

        Ok(LoToken {
            type_: LoTokenType::Delim,
            value: self.chars[loc.offset].into(),
            loc,
        })
    }

    fn lex_operator(&mut self) -> LexResult {
        let mut loc = self.loc();

        while is_operator_char(self.current_char()?) {
            self.next_char();
        }

        loc.length = self.index - loc.offset;

        Ok(LoToken {
            type_: LoTokenType::Operator,
            value: self.chars[loc.offset..self.index].iter().collect(),
            loc,
        })
    }

    fn skip_space(&mut self) {
        while self.current_char().map(is_space_char).unwrap_or(false) {
            self.next_char();
        }

        // skip comment
        if let Ok('/') = self.current_char() {
            let Ok('/') = self.peek_next_char() else {
                return;
            };

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

    fn current_char(&mut self) -> Result<char, LoError> {
        self.chars
            .get(self.index)
            .copied()
            .ok_or_else(|| self.err_unexpected_eof())
    }

    fn peek_next_char(&mut self) -> Result<char, LoError> {
        self.chars
            .get(self.index + 1)
            .copied()
            .ok_or_else(|| self.err_unexpected_eof())
    }

    fn err_unexpected_char(&self) -> LoError {
        LoError {
            message: format!("ParseError: Unexpected character"),
            loc: self.loc(),
        }
    }

    fn err_unexpected_eof(&self) -> LoError {
        LoError {
            message: format!("ParseError: Unexpected EOF"),
            loc: self.loc(),
        }
    }

    fn loc(&self) -> LoLocation {
        LoLocation {
            file_name: self.file_name.clone(),
            offset: self.index,
            length: 1,
            line: self.line,
            col: self.col,
        }
    }
}

fn is_space_char(c: char) -> bool {
    match c {
        ' ' | '\n' | '\t' => true,
        _ => false,
    }
}

fn is_symbol_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn is_delim_char(c: char) -> bool {
    "(){}[],;".contains(c)
}

fn is_operator_char(c: char) -> bool {
    "!#$%&*+-./:<=>?@\\^~|".contains(c)
}
