use crate::core::*;
use alloc::{format, string::String, vec::Vec};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum LoTokenType {
    StringLiteral,
    CharLiteral,
    IntLiteral,
    Symbol,
    Delim,
    Operator,
    Terminal,
}

#[derive(Clone)]
pub struct LoToken {
    pub type_: LoTokenType,
    pub loc: LoLocation,
}

impl LoToken {
    pub fn get_value(&self, source: &'static [u8]) -> &str {
        if self.type_ == LoTokenType::Terminal {
            return "<EOF>";
        }

        return self.loc.read_span(source);
    }

    pub fn is_any(&self, type_: LoTokenType) -> bool {
        self.type_ == type_
    }

    pub fn is(&self, type_: LoTokenType, value: &str, source: &'static [u8]) -> bool {
        self.is_any(type_) && self.get_value(source) == value
    }
}

pub struct Lexer {
    // context
    pub file_index: usize,
    pub source: &'static [u8],

    // state
    pub source_pos: LoPosition,
    pub was_newline: bool,

    // output
    pub tokens: Vec<LoToken>,
    pub comments: Vec<LoLocation>,
    pub backslashes: Vec<LoLocation>,
    pub double_backslashes: Vec<LoLocation>,
}

impl Lexer {
    pub fn new(source: &'static [u8], file_index: usize) -> Self {
        Self {
            file_index,
            source,

            source_pos: LoPosition {
                offset: 0,
                line: 1,
                col: 1,
            },
            was_newline: false,

            tokens: Vec::new(),
            comments: Vec::new(),
            backslashes: Vec::new(),
            double_backslashes: Vec::new(),
        }
    }

    pub fn lex_file(&mut self) -> Result<(), LoError> {
        self.skip_space();

        while self.source_pos.offset < self.source.len() {
            let token = self.lex_token()?;
            self.tokens.push(token);

            self.skip_space();
        }

        self.tokens.push(LoToken {
            type_: LoTokenType::Terminal,
            loc: self.loc(),
        });

        Ok(())
    }

    fn lex_token(&mut self) -> Result<LoToken, LoError> {
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
        // NOTE: must be after int because is_symbol_char matches digits
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
            message: format!("Unexpected char: {}", char),
            loc: self.loc(),
        })
    }

    fn lex_symbol(&mut self) -> Result<LoToken, LoError> {
        let mut loc = self.loc();

        while is_symbol_char(self.current_char()?) {
            self.next_char();
        }

        loc.end_pos = self.source_pos;

        Ok(LoToken {
            type_: LoTokenType::Symbol,
            loc,
        })
    }

    fn lex_char(&mut self) -> Result<LoToken, LoError> {
        let mut loc = self.loc();

        self.next_char(); // skip start quote

        if self.current_char()? == '\\' {
            self.next_char(); // skip `\`
            match self.current_char()? {
                'n' | 'r' | 't' | '0' | '\\' | '\'' => {
                    self.next_char(); // skip escaped character
                }
                c => {
                    return Err(LoError {
                        message: format!("ParseError: Invalid escape sequence: \\{c}"),
                        loc: self.loc(),
                    });
                }
            }
        } else {
            self.next_char(); // skip actual character
        }

        let end_quote = self.current_char()?;
        if end_quote != '\'' {
            return Err(LoError {
                message: format!("ParseError: Unexpected character `{end_quote}`, expected `'`",),
                loc: self.loc(),
            });
        }
        self.next_char(); // skip end quote

        loc.end_pos = self.source_pos;

        Ok(LoToken {
            type_: LoTokenType::CharLiteral,
            loc,
        })
    }

    pub fn parse_char_literal_value(char_literal: &str) -> u32 {
        match char_literal {
            "'\\n'" => '\n' as u32,
            "'\\r'" => '\r' as u32,
            "'\\t'" => '\t' as u32,
            "'\\0'" => '\0' as u32,
            "'\\''" => '\'' as u32,
            c => c.chars().nth(1).unwrap() as u32,
        }
    }

    fn lex_int_literal(&mut self) -> Result<LoToken, LoError> {
        let mut loc = self.loc();

        let mut hex = false;
        if let Ok('0') = self.current_char() {
            if let Ok('x') = self.peek_char(1) {
                self.next_char();
                self.next_char();
                hex = true;
            }
        }

        loop {
            match self.current_char() {
                Ok('_') | Ok('0'..='9') => {}
                Ok('A'..='F') if hex => {}
                _ => break,
            }
            self.next_char();
        }

        loc.end_pos = self.source_pos;

        Ok(LoToken {
            type_: LoTokenType::IntLiteral,
            loc,
        })
    }

    pub fn parse_int_literal_value(int_literal: &str) -> u64 {
        let int_literal = int_literal.replace("_", "");

        if int_literal.starts_with("0x") {
            return u64::from_str_radix(&int_literal[2..], 16).unwrap();
        }

        int_literal.parse().unwrap()
    }

    fn lex_string(&mut self) -> Result<LoToken, LoError> {
        let mut loc = self.loc();

        self.next_char(); // skip start quote

        loop {
            match self.current_char()? {
                '"' => break,
                '\\' => {
                    self.next_char();
                    match self.current_char()? {
                        'n' | 'r' | 't' | '0' | '\\' | '"' => {}
                        c => {
                            return Err(LoError {
                                message: format!("ParseError: Invalid escape sequence: \\{c}"),
                                loc: self.loc(),
                            });
                        }
                    }
                }
                _ => {}
            };
            self.next_char();
        }

        self.next_char(); // skip end quote

        loc.end_pos = self.source_pos;

        Ok(LoToken {
            type_: LoTokenType::StringLiteral,
            loc,
        })
    }

    fn lex_delim(&mut self) -> Result<LoToken, LoError> {
        let mut loc = self.loc();

        self.next_char(); // skip delimiter char

        loc.end_pos = self.source_pos;

        Ok(LoToken {
            type_: LoTokenType::Delim,
            loc,
        })
    }

    fn lex_operator(&mut self) -> Result<LoToken, LoError> {
        let mut loc = self.loc();

        loop {
            let current_op = unsafe {
                str::from_utf8_unchecked(&self.source[loc.pos.offset..self.source_pos.offset + 1])
            };

            let mut is_start_of_operator = false;
            for operator in OPERATORS {
                if operator.starts_with(current_op) {
                    is_start_of_operator = true;
                    break;
                }
            }

            if !is_start_of_operator {
                break;
            }

            self.next_char();
            self.current_char()?; // EOF check
        }

        let op = unsafe {
            str::from_utf8_unchecked(&self.source[loc.pos.offset..self.source_pos.offset])
        };

        let mut matched_fully = false;
        for operator in OPERATORS {
            if *operator == op {
                matched_fully = true;
                break;
            }
        }

        if !matched_fully {
            return Err(LoError {
                message: format!("Unexpected char: '{}'", self.current_char()?),
                loc: self.loc(),
            });
        }

        loc.end_pos = self.source_pos;

        Ok(LoToken {
            type_: LoTokenType::Operator,
            loc,
        })
    }

    fn skip_space(&mut self) {
        while self.current_char().map(is_space_char).unwrap_or(false) {
            self.next_char();
        }

        if self.current_char() == Ok('/') && self.peek_char(1) == Ok('/') {
            let comment = self.lex_comment();
            self.comments.push(comment);
            self.skip_space();
        }

        if self.current_char() == Ok('\\') {
            let loc = self.loc();

            self.next_char();

            if self.current_char() == Ok('\\') {
                self.next_char();
                self.double_backslashes.push(loc)
            } else {
                self.backslashes.push(loc);
            }

            self.skip_space();
        }
    }

    fn lex_comment(&mut self) -> LoLocation {
        let mut loc = self.loc();

        self.next_char(); // `/`
        self.next_char(); // `/`

        loop {
            let Ok(char) = self.current_char() else {
                break;
            };

            if char == '\n' {
                break;
            }

            self.next_char();
        }

        loc.end_pos = self.source_pos;

        loc
    }

    fn next_char(&mut self) {
        let _ = next_utf8_char(&self.source, &mut self.source_pos.offset);

        let Ok(char) = self.current_char() else {
            return;
        };

        self.source_pos.col += 1;

        if char == '\n' {
            // NOTE(edge case): when first character is encountered
            //  was_newline is not true but rather undefined,
            //  thus we don't bump the line count
            if self.source_pos.offset == 0 || self.was_newline {
                self.source_pos.line += 1;
            }
            self.was_newline = true;
            return;
        }

        if self.was_newline {
            self.source_pos.line += 1;
            self.source_pos.col = 1;
            self.was_newline = false;
        }
    }

    fn current_char(&mut self) -> Result<char, LoError> {
        self.peek_char(0)
    }

    fn peek_char(&mut self, skip_chars: usize) -> Result<char, LoError> {
        let mut char_offset = self.source_pos.offset;
        for _ in 0..skip_chars {
            let _ = next_utf8_char(&self.source, &mut char_offset);
        }
        next_utf8_char(&self.source, &mut char_offset).map_err(|err| match err {
            NextCharError::InvalidUtf8 => LoError {
                message: format!("ParseError: Invalid UTF-8 sequence"),
                loc: self.loc(),
            },
            NextCharError::EndOfSource => LoError {
                message: format!("ParseError: Unexpected EOF"),
                loc: self.loc(),
            },
        })
    }

    fn loc(&self) -> LoLocation {
        let pos = self.source_pos;

        let mut end_pos = pos;
        end_pos.col += 1;

        LoLocation {
            file_index: self.file_index,
            pos,
            end_pos,
        }
    }
}

#[derive(Clone)]
pub struct EscapedString(pub LoLocation);

impl EscapedString {
    pub fn get_raw(&self, source: &'static [u8]) -> &str {
        return self.0.read_span(source);
    }

    pub fn unescape(&self, source: &'static [u8]) -> String {
        let escaped = self.get_raw(source);
        let mut unescaped = String::new();

        let mut chars = escaped.chars();

        chars.next().unwrap(); // skip start quote

        loop {
            let char = chars.next().unwrap();
            match char {
                '"' => break,
                '\\' => {
                    let next_char = chars.next().unwrap();
                    match next_char {
                        'n' => unescaped.push('\n'),
                        'r' => unescaped.push('\r'),
                        't' => unescaped.push('\t'),
                        '0' => unescaped.push('\0'),
                        '\\' => unescaped.push('\\'),
                        '"' => unescaped.push('"'),
                        _ => unreachable!(),
                    }
                }
                _ => {
                    unescaped.push(char);
                }
            }
        }

        unescaped
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
    "(){}[],\\".contains(c)
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
    "*&",  // Pointer to any amount
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
    "?",   // Error propagation
];

fn is_operator_start_char(c: char) -> bool {
    for operator in OPERATORS {
        if operator.starts_with(c) {
            return true;
        }
    }
    return false;
}

#[derive(PartialEq)]
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

    ErrorPropagation,
}

pub struct InfixOp {
    pub tag: InfixOpTag,
    pub info: OpInfo,
    pub token: LoToken,
}

impl InfixOp {
    pub fn parse(token: LoToken, source: &'static [u8]) -> Option<Self> {
        use InfixOpTag::*;
        use OpAssoc::*;

        fn op(token: LoToken, tag: InfixOpTag, bp: u32, assoc: OpAssoc) -> Option<InfixOp> {
            let info = OpInfo { bp, assoc };
            return Some(InfixOp { tag, info, token });
        }

        match token.get_value(source) {
            "catch" => op(token, Catch, 13, L),

            "." => op(token, FieldAccess, 12, L),

            "?" => op(token, ErrorPropagation, 11, None),

            "as" => op(token, Cast, 10, L),

            "%" => op(token, Mod, 9, L),
            "/" => op(token, Div, 9, L),
            "*" => op(token, Mul, 9, L),

            "-" => op(token, Sub, 8, L),
            "+" => op(token, Add, 8, L),

            ">>" => op(token, ShiftRight, 7, L),
            "<<" => op(token, ShiftLeft, 7, L),

            "&" => op(token, BitAnd, 6, L),

            "|" => op(token, BitOr, 5, L),

            ">=" => op(token, GreaterEqual, 4, L),
            ">" => op(token, Greater, 4, L),
            "<=" => op(token, LessEqual, 4, L),
            "<" => op(token, Less, 4, L),
            "!=" => op(token, NotEqual, 4, None),
            "==" => op(token, Equal, 4, None),

            "&&" => op(token, And, 3, L),
            "||" => op(token, Or, 2, L),

            "=" => op(token, Assign, 1, None),
            "+=" => op(token, AddAssign, 1, None),
            "-=" => op(token, SubAssign, 1, None),
            "*=" => op(token, MulAssign, 1, None),
            "/=" => op(token, DivAssign, 1, None),
            "%=" => op(token, ModAssign, 1, None),
            "&=" => op(token, BitAndAssign, 1, None),
            "|=" => op(token, BitOrAssign, 1, None),
            "<<=" => op(token, ShiftLeftAssign, 1, None),
            ">>=" => op(token, ShiftRightAssign, 1, None),
            _ => Option::None,
        }
    }
}

pub enum PrefixOpTag {
    Not,
    Dereference,
    Positive,
    Negative,
}

pub struct PrefixOp {
    pub tag: PrefixOpTag,
    pub info: OpInfo,
    pub token: LoToken,
}

impl PrefixOp {
    pub fn parse(token: LoToken, source: &'static [u8]) -> Option<Self> {
        use OpAssoc::*;
        use PrefixOpTag::*;

        let (tag, info) = match token.get_value(source) {
            "!" => (Not, OpInfo { bp: 8, assoc: L }),
            "*" => (Dereference, OpInfo { bp: 8, assoc: L }),
            "+" => (Positive, OpInfo { bp: 9, assoc: L }),
            "-" => (Negative, OpInfo { bp: 9, assoc: L }),
            _ => return Option::None,
        };
        Some(Self { tag, info, token })
    }
}

#[derive(PartialEq)]
pub enum OpAssoc {
    L,
    None,
}

pub struct OpInfo {
    pub bp: u32,
    assoc: OpAssoc,
}

impl OpInfo {
    pub fn get_min_bp_for_next(&self) -> u32 {
        if self.assoc == OpAssoc::L {
            self.bp + 1
        } else {
            self.bp
        }
    }
}

pub enum NextCharError {
    EndOfSource,
    InvalidUtf8,
}

pub fn next_utf8_char(source: &[u8], offset: &mut usize) -> Result<char, NextCharError> {
    if *offset >= source.len() {
        return Err(NextCharError::EndOfSource);
    }

    let first = source[*offset];
    let width: usize;
    let min: u32;
    let mut code: u32;

    // determine sequence length and minimum valid code point
    if first <= 0x7F {
        width = 1;
        min = 0;
        code = first as u32;
    } else if first >= 0xC2 && first <= 0xDF {
        width = 2;
        min = 0x80;
        code = (first & 0x1F) as u32;
    } else if first >= 0xE0 && first <= 0xEF {
        width = 3;
        min = 0x800;
        code = (first & 0x0F) as u32;
    } else if first >= 0xF0 && first <= 0xF4 {
        width = 4;
        min = 0x10000;
        code = (first & 0x07) as u32;
    } else {
        return Err(NextCharError::InvalidUtf8);
    }

    if *offset + width > source.len() {
        return Err(NextCharError::EndOfSource);
    }

    // process continuation bytes
    for i in 1..width {
        let b = source[*offset + i];
        if (b & 0xC0) != 0x80 {
            return Err(NextCharError::InvalidUtf8);
        }
        code = (code << 6) | ((b & 0x3F) as u32);
    }

    // validate code point
    if code < min || code > 0x10FFFF || (code >= 0xD800 && code <= 0xDFFF) {
        return Err(NextCharError::InvalidUtf8);
    }

    *offset += width;

    Ok(unsafe { char::from_u32_unchecked(code) })
}
