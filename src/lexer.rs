use crate::utils::*;
use alloc::{format, rc::Rc, string::String, vec::Vec};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum LoTokenType {
    StringLiteral,
    CharLiteral,
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
        self.type_ == type_
    }

    pub fn is(&self, type_: LoTokenType, value: &str) -> bool {
        self.is_any(type_) && self.value == value
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

    pub fn next_is_any(&mut self, type_: LoTokenType) -> Result<bool, LoError> {
        match self.peek() {
            Some(token) if token.is_any(type_) => Ok(true),
            Some(_) => Ok(false),
            _ => self.err_eof(format!("Unexpected EOF")),
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

    pub fn loc(&self) -> &LoLocation {
        if let Some(token) = self.tokens.get(self.index) {
            &token.loc
        } else {
            &self.terminal_token.loc
        }
    }

    fn err_eof<T>(&self, message: String) -> Result<T, LoError> {
        Err(LoError {
            message,
            loc: self.terminal_token.loc.clone(),
        })
    }
}

type LexResult = Result<LoToken, LoError>;

#[derive(Clone)]
struct Lexer {
    file_name: Rc<str>,
    chars: Vec<char>,
    index: usize,
    line: usize,
    col: usize,
    was_newline: bool,
}

pub fn lex_all(file_name: &str, script: &str) -> Result<LoTokenStream, LoError> {
    Lexer::new(file_name, script).lex_all()
}

impl Lexer {
    fn new(file_name: &str, script: &str) -> Self {
        Self {
            file_name: file_name.into(),
            chars: script.chars().collect::<Vec<_>>(),
            index: 0,
            line: 1,
            col: 1,
            was_newline: false,
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

        if char == '\'' {
            return self.lex_char();
        }
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
        if is_operator_start_char(char) {
            return self.lex_operator();
        }

        Err(LoError {
            message: format!("Unexpected char: {char}"),
            loc: self.loc(),
        })
    }

    fn lex_int_literal(&mut self) -> LexResult {
        let mut loc = self.loc();
        let mut value = String::new();

        let hex = match (self.current_char(), self.peek_next_char()) {
            (Ok('0'), Ok('x')) => {
                value.push('0');
                self.next_char();
                value.push('x');
                self.next_char();
                true
            }
            _ => false,
        };

        loop {
            match self.current_char() {
                Ok('_') => {
                    self.next_char();
                }
                Ok(c @ '0'..='9') => {
                    value.push(c);
                    self.next_char();
                }
                Ok(c @ 'A'..='F') if hex => {
                    value.push(c);
                    self.next_char();
                }
                _ => break,
            }
        }

        loc.end_pos = self.pos();

        Ok(LoToken {
            type_: LoTokenType::IntLiteral,
            value,
            loc,
        })
    }

    fn lex_symbol(&mut self) -> LexResult {
        let mut loc = self.loc();

        while is_symbol_char(self.current_char()?) {
            self.next_char();
        }

        loc.end_pos = self.pos();

        Ok(LoToken {
            type_: LoTokenType::Symbol,
            value: self.chars[loc.pos.offset..self.index].iter().collect(),
            loc,
        })
    }

    fn lex_char(&mut self) -> LexResult {
        let mut loc = self.loc();

        self.next_char(); // skip start quote

        let mut value = String::new();

        let c = self.current_char()?;
        if c == '\\' {
            self.next_char();
            match self.current_char()? {
                'n' => value.push('\n'),
                'r' => value.push('\r'),
                't' => value.push('\t'),
                '0' => value.push('\0'),
                '\\' => value.push('\\'),
                '\'' => value.push('\''),
                c => {
                    return Err(LoError {
                        message: format!("ParseError: Invalid escape sequence: \\{c}"),
                        loc: self.loc(),
                    });
                }
            }
        } else {
            value.push(c);
        }

        self.next_char(); // skip actual character

        if self.current_char()? != '\'' {
            return Err(LoError {
                message: format!("ParseError: Unexpected character `{c}`, expected `'`"),
                loc: self.loc(),
            });
        }

        self.next_char(); // skip end quote

        loc.end_pos = self.pos();

        Ok(LoToken {
            type_: LoTokenType::CharLiteral,
            value,
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
                        'r' => value.push('\r'),
                        't' => value.push('\t'),
                        '0' => value.push('\0'),
                        '\\' => value.push('\\'),
                        '"' => value.push('"'),
                        c => {
                            return Err(LoError {
                                message: format!("ParseError: Invalid escape sequence: \\{c}"),
                                loc: self.loc(),
                            });
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

        loc.end_pos = self.pos();

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
            value: self.chars[loc.pos.offset].into(),
            loc,
        })
    }

    fn lex_operator(&mut self) -> LexResult {
        let mut loc = self.loc();
        let mut value = String::new();

        'adding_chars: loop {
            value.push(self.current_char()?);
            for operator in OPERATORS {
                if operator.starts_with(&value) {
                    self.next_char();
                    continue 'adding_chars;
                }
            }
            value.pop();
            break;
        }

        loc.end_pos = self.pos();

        Ok(LoToken {
            type_: LoTokenType::Operator,
            value,
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

        self.col += 1;

        if char == '\n' {
            // NOTE(edge case): when first character is encountered
            //  was_newline is not true but rather undefined,
            //  thus we don't bump the line count
            if self.index == 0 || self.was_newline {
                self.line += 1;
            }
            self.was_newline = true;
            return;
        }

        if self.was_newline {
            self.line += 1;
            self.col = 1;
            self.was_newline = false;
            return;
        }
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

    fn err_unexpected_eof(&self) -> LoError {
        LoError {
            message: format!("ParseError: Unexpected EOF"),
            loc: self.loc(),
        }
    }

    fn loc(&self) -> LoLocation {
        LoLocation {
            file_name: self.file_name.clone(),

            pos: self.pos(),
            end_pos: self.pos(),
        }
    }

    fn pos(&self) -> LoPosition {
        LoPosition {
            offset: self.index,
            line: self.line,
            col: self.col,
        }
    }
}

fn is_space_char(c: char) -> bool {
    match c {
        ' ' | '\n' | '\t' | '\r' => true,
        _ => false,
    }
}

fn is_symbol_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn is_delim_char(c: char) -> bool {
    "(){}[],;".contains(c)
}

static OPERATORS: &[&str] = &[
    "=",   // Assignment
    "==",  // Equality comparison
    "!=",  // Nonequality comparison
    "!",   // Logical NOT
    "&&",  // Short-circuiting logical AND
    "||",  // Short-circuiting logical OR
    "<",   // Less than comparison
    "<=",  // Less than or equal to comparison
    ">",   // Greater than comparison
    ">=",  // Greater than or equal to comparison
    "+",   // Arithmetic addition
    "+=",  // Arithmetic addition and assignment
    "-",   // Arithmetic subtraction
    "-=",  // Arithmetic subtraction and assignment
    "*",   // Arithmetic multiplication
    "*=",  // Arithmetic multiplication and assignment
    "/",   // Arithmetic division
    "/=",  // Arithmetic division and assignment
    "%",   // Arithmetic remainder
    "%=",  // Arithmetic remainder and assignment
    "&",   // Bitwise AND / Pointer to one
    "&*",  // Pointer to any amount
    "&=",  // Bitwise AND and assignment
    "<<",  // Left-shift
    "<<=", // Left-shift and assignment
    "=>",  // Part of match arm syntax
    ">>",  // Right-shift
    ">>=", // Right-shift and assignment
    "^",   // Bitwise exclusive OR
    "^=",  // Bitwise exclusive OR and assignment
    "|",   // Bitwise OR
    "|=",  // Bitwise OR and assignment
    ".",   // Member access
    "..",  // Range operator
    ":",   // Type separator
    "::",  // Path separator
    "@",   // Memory index separator, defer label prefix
];

fn is_operator_start_char(c: char) -> bool {
    for operator in OPERATORS {
        if operator.starts_with(c) {
            return true;
        }
    }
    return false;
}

pub enum InfixOpTag {
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    BitAnd,
    Or,
    BitOr,
    ShiftLeft,
    ShiftRight,

    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    BitAndAssign,
    BitOrAssign,
    ShiftLeftAssign,
    ShiftRightAssign,

    Cast,
    FieldAccess,
    Catch,
}

pub struct InfixOp {
    pub tag: InfixOpTag,
    pub info: OpInfo,
    pub token: LoToken,
}

impl InfixOp {
    pub fn parse(token: LoToken) -> Option<Self> {
        use InfixOpTag::*;
        use OpAssoc::*;
        let (tag, info) = match token.value.as_str() {
            "catch" => (Catch, OpInfo { bp: 12, assoc: L }),

            "." => (FieldAccess, OpInfo { bp: 11, assoc: L }),

            "as" => (Cast, OpInfo { bp: 10, assoc: L }),

            "%" => (Mod, OpInfo { bp: 9, assoc: L }),
            "/" => (Div, OpInfo { bp: 9, assoc: L }),
            "*" => (Mul, OpInfo { bp: 9, assoc: L }),

            "-" => (Sub, OpInfo { bp: 8, assoc: L }),
            "+" => (Add, OpInfo { bp: 8, assoc: L }),

            ">>" => (ShiftRight, OpInfo { bp: 7, assoc: L }),
            "<<" => (ShiftLeft, OpInfo { bp: 7, assoc: L }),

            "&" => (BitAnd, OpInfo { bp: 6, assoc: L }),

            "|" => (BitOr, OpInfo { bp: 5, assoc: L }),

            ">=" => (GreaterEqual, OpInfo { bp: 4, assoc: L }),
            ">" => (Greater, OpInfo { bp: 4, assoc: L }),
            "<=" => (LessEqual, OpInfo { bp: 4, assoc: L }),
            "<" => (Less, OpInfo { bp: 4, assoc: L }),
            "!=" => (NotEqual, OpInfo { bp: 4, assoc: None }),
            "==" => (Equal, OpInfo { bp: 4, assoc: None }),

            "&&" => (And, OpInfo { bp: 3, assoc: L }),
            "||" => (Or, OpInfo { bp: 2, assoc: L }),

            "=" => (Assign, OpInfo { bp: 1, assoc: None }),
            "+=" => (AddAssign, OpInfo { bp: 1, assoc: None }),
            "-=" => (SubAssign, OpInfo { bp: 1, assoc: None }),
            "*=" => (MulAssign, OpInfo { bp: 1, assoc: None }),
            "/=" => (DivAssign, OpInfo { bp: 1, assoc: None }),
            "%=" => (ModAssign, OpInfo { bp: 1, assoc: None }),
            "&=" => (BitAndAssign, OpInfo { bp: 1, assoc: None }),
            "|=" => (BitOrAssign, OpInfo { bp: 1, assoc: None }),
            "<<=" => (ShiftLeftAssign, OpInfo { bp: 1, assoc: None }),
            ">>=" => (ShiftRightAssign, OpInfo { bp: 1, assoc: None }),
            _ => return Option::None,
        };
        Some(Self { tag, info, token })
    }
}

pub enum PrefixOpTag {
    Not,
    Dereference,
}

pub struct PrefixOp {
    pub tag: PrefixOpTag,
    pub info: OpInfo,
    pub token: LoToken,
}

impl PrefixOp {
    pub fn parse(token: LoToken) -> Option<Self> {
        use OpAssoc::*;
        use PrefixOpTag::*;
        let (tag, info) = match token.value.as_str() {
            "!" => (Not, OpInfo { bp: 8, assoc: L }),
            "*" => (Dereference, OpInfo { bp: 8, assoc: L }),
            _ => return Option::None,
        };
        Some(Self { tag, info, token })
    }
}

#[derive(PartialEq)]
pub enum OpAssoc {
    L,
    R,
    None,
}

pub struct OpInfo {
    pub bp: u32,
    assoc: OpAssoc,
}

impl OpInfo {
    pub fn get_min_bp_for_next(&self) -> u32 {
        if self.assoc == OpAssoc::R {
            self.bp - 1
        } else {
            self.bp
        }
    }
}
