use core::fmt;

use alloc::{boxed::Box, string::String, vec::Vec};

// TODO: add parser tests

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
pub enum SExpr {
    Atom { value: String, loc: Location },
    List { value: Vec<SExpr>, loc: Location },
}

impl SExpr {
    pub fn loc(&self) -> &Location {
        match self {
            Self::Atom { loc, .. } => loc,
            Self::List { loc, .. } => loc,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedEOF { loc: Location },
    UnexpectedChar { loc: Location },
}

impl ParseError {
    pub fn loc(&self) -> &Location {
        match self {
            Self::UnexpectedEOF { loc, .. } => loc,
            Self::UnexpectedChar { loc, .. } => loc,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedChar { .. } => write!(f, "Unexpected character"),
            ParseError::UnexpectedEOF { .. } => write!(f, "Unexpected EOF"),
        }
    }
}

pub type ParseResult = Result<SExpr, ParseError>;

#[derive(Clone)]
struct Parser {
    file_name: Box<str>,
    chars: Vec<char>,
    index: usize,
    line: usize,
    col: usize,
}

pub fn parse(file_name: &str, script: &str) -> Result<Vec<SExpr>, ParseError> {
    Parser::new(file_name, script).parse_all()
}

impl Parser {
    fn new(file_name: &str, script: &str) -> Self {
        Self {
            file_name: file_name.into(),
            chars: script.chars().collect::<Vec<_>>(),
            index: 0,
            line: 1,
            col: 1,
        }
    }

    fn parse_all(&mut self) -> Result<Vec<SExpr>, ParseError> {
        self.skip_space();

        let mut items = Vec::new();

        while self.index < self.chars.len() {
            if !is_list_start(self.current_char()?) {
                return Err(ParseError::UnexpectedChar { loc: self.loc() });
            }

            let res = self.parse_list()?;
            self.skip_space();
            items.push(res);
        }

        Ok(items)
    }

    fn parse_expr(&mut self) -> ParseResult {
        if is_list_start(self.current_char()?) {
            self.parse_list()
        } else {
            self.parse_atom()
        }
    }

    fn parse_atom(&mut self) -> ParseResult {
        let mut loc = self.loc();

        loop {
            let c = self.current_char()?;
            if is_space(c) || is_list_end(c) || c == ';' {
                break;
            }
            self.next_char();
        }

        loc.length = self.index - loc.offset;

        Ok(SExpr::Atom {
            value: self.chars[loc.offset..self.index].iter().collect(),
            loc,
        })
    }

    fn parse_list(&mut self) -> ParseResult {
        let mut loc = self.loc();

        let list_start_char = self.current_char()?;
        self.next_char(); // eat list start

        self.skip_space();

        let mut items = Vec::new();

        while !is_list_end(self.current_char()?) {
            let res = self.parse_expr()?;
            self.skip_space();
            items.push(res);
        }

        let list_end_char = self.current_char()?;
        self.next_char(); // eat list end

        if !is_valid_list_chars(list_start_char, list_end_char) {
            return Err(ParseError::UnexpectedChar { loc: self.loc() });
        }

        if list_start_char == '{' && items.len() >= 2 {
            items.swap(0, 1);
        }

        loc.length = self.index - loc.offset;

        Ok(SExpr::List { value: items, loc })
    }

    fn skip_space(&mut self) {
        while self.current_char().map(is_space).unwrap_or(false) {
            self.next_char();
        }

        if let Ok(';') = self.current_char() {
            self.next_char();

            while self.current_char().map(|c| c != '\n').unwrap_or(false) {
                self.next_char();
            }

            if let Ok('\n') = self.current_char() {
                self.next_char();
            }

            self.skip_space();
        }
    }

    fn next_char(&mut self) {
        self.index += 1;

        if let Ok(char) = self.current_char() {
            if char == '\n' {
                self.col = 0;
                self.line += 1;
                return;
            }

            self.col += 1;
        }
    }

    fn current_char(&mut self) -> Result<char, ParseError> {
        self.chars
            .get(self.index)
            .copied()
            .ok_or_else(|| ParseError::UnexpectedEOF { loc: self.loc() })
    }

    fn loc(&self) -> Location {
        Location {
            file_name: self.file_name.clone(),
            offset: self.index,
            length: 1,
            line: self.line,
            col: self.col,
        }
    }
}

fn is_list_start(c: char) -> bool {
    match c {
        '(' | '{' | '[' => true,
        _ => false,
    }
}

fn is_list_end(c: char) -> bool {
    match c {
        ')' | '}' | ']' => true,
        _ => false,
    }
}

fn is_valid_list_chars(start: char, end: char) -> bool {
    match (start, end) {
        ('(', ')') => true,
        ('{', '}') => true,
        ('[', ']') => true,
        _ => false,
    }
}

fn is_space(c: char) -> bool {
    match c {
        ' ' | '\n' | '\t' => true,
        _ => false,
    }
}
