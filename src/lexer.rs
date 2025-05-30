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

#[derive(Debug)]
pub struct Comment {
    pub content: String,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct Backslash {
    pub loc: LoLocation,
}

pub struct Lexer {
    file_index: u32,
    chars: Vec<char>,
    index: usize,
    line: usize,
    col: usize,
    was_newline: bool,
    comments: Vec<Comment>,
    backslashes: Vec<Backslash>,
}

pub struct LoTokens {
    pub tokens: Vec<LoToken>,
    pub end_loc: LoLocation,
    pub comments: Vec<Comment>,
    pub backslashes: Vec<Backslash>,
}

impl Lexer {
    pub fn lex(file_index: u32, chars: &str) -> Result<LoTokens, LoError> {
        let mut lexer = Lexer {
            file_index,
            chars: chars.chars().collect::<Vec<_>>(),
            index: 0,
            line: 1,
            col: 1,
            was_newline: false,
            comments: Vec::new(),
            backslashes: Vec::new(),
        };
        let tokens = lexer.lex_file()?;

        Ok(LoTokens {
            tokens,
            end_loc: lexer.loc(),
            comments: lexer.comments,
            backslashes: lexer.backslashes,
        })
    }

    fn lex_file(&mut self) -> Result<Vec<LoToken>, LoError> {
        let mut tokens = Vec::new();

        self.skip_space();

        while self.index < self.chars.len() {
            tokens.push(self.lex_token()?);
            self.skip_space();
        }

        Ok(tokens)
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
            message: format!("Unexpected char: {char}"),
            loc: self.loc(),
        })
    }

    fn lex_symbol(&mut self) -> Result<LoToken, LoError> {
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

        loc.end_pos = self.pos();

        Ok(LoToken {
            type_: LoTokenType::CharLiteral,
            value: self.chars[loc.pos.offset..self.index].iter().collect(),
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

        let hex = match (self.current_char(), self.peek_next_char()) {
            (Ok('0'), Ok('x')) => {
                self.next_char();
                self.next_char();
                true
            }
            _ => false,
        };

        loop {
            match self.current_char() {
                Ok('_') | Ok('0'..='9') => {}
                Ok('A'..='F') if hex => {}
                _ => break,
            }
            self.next_char();
        }

        loc.end_pos = self.pos();

        Ok(LoToken {
            type_: LoTokenType::IntLiteral,
            value: self.chars[loc.pos.offset..self.index].iter().collect(),
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

        loc.end_pos = self.pos();

        Ok(LoToken {
            type_: LoTokenType::StringLiteral,
            value: self.chars[loc.pos.offset..self.index].iter().collect(),
            loc,
        })
    }

    pub fn unescape_string(escaped: &str) -> String {
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

    fn lex_delim(&mut self) -> Result<LoToken, LoError> {
        let mut loc = self.loc();

        self.next_char(); // skip delimiter char

        loc.end_pos = self.pos();

        Ok(LoToken {
            type_: LoTokenType::Delim,
            value: self.chars[loc.pos.offset].into(),
            loc,
        })
    }

    fn lex_operator(&mut self) -> Result<LoToken, LoError> {
        let mut loc = self.loc();
        let mut value = String::new();

        loop {
            value.push(self.current_char()?);

            let mut is_start_of_operator = false;
            for operator in OPERATORS {
                if operator.starts_with(&value) {
                    is_start_of_operator = true;
                    break;
                }
            }

            if !is_start_of_operator {
                value.pop();
                break;
            };

            self.next_char();
        }

        let mut matched_fully = false;
        for operator in OPERATORS {
            if operator == &value {
                matched_fully = true;
                break;
            }
        }

        if !matched_fully {
            return Err(LoError {
                message: format!("Unexpected char: '{}'", self.current_char()?),
                loc: self.loc(),
            });
        };

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

        if self.current_char() == Ok('/') && self.peek_next_char() == Ok('/') {
            let comment = self.lex_comment();
            self.comments.push(comment);
            self.skip_space();
        }

        if self.current_char() == Ok('\\') {
            self.backslashes.push(Backslash { loc: self.loc() });
            self.next_char();
            self.skip_space();
        }
    }

    fn lex_comment(&mut self) -> Comment {
        let mut loc = self.loc();

        self.next_char(); /* `/` */
        self.next_char(); /* `/` */

        loop {
            let Ok(char) = self.current_char() else {
                break;
            };

            if char == '\n' {
                break;
            }

            self.next_char();
        }

        loc.end_pos = self.pos();

        Comment {
            content: self.chars[loc.pos.offset..self.index].iter().collect(),
            loc,
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
        let pos = self.pos();

        let mut end_pos = pos.clone();
        end_pos.col += 1;

        LoLocation {
            file_index: self.file_index,
            pos,
            end_pos,
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

#[derive(Debug, PartialEq)]
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

impl InfixOpTag {
    pub fn to_str(&self) -> &str {
        match self {
            InfixOpTag::Equal => "==",
            InfixOpTag::NotEqual => "!=",
            InfixOpTag::Less => "<",
            InfixOpTag::Greater => ">",
            InfixOpTag::LessEqual => "<=",
            InfixOpTag::GreaterEqual => ">=",
            InfixOpTag::Add => "+",
            InfixOpTag::Sub => "-",
            InfixOpTag::Mul => "*",
            InfixOpTag::Div => "/",
            InfixOpTag::Mod => "%",
            InfixOpTag::And => "&&",
            InfixOpTag::BitAnd => "&",
            InfixOpTag::Or => "||",
            InfixOpTag::BitOr => "|",
            InfixOpTag::ShiftLeft => "<<",
            InfixOpTag::ShiftRight => ">>",
            InfixOpTag::Assign => "=",
            InfixOpTag::AddAssign => "+=",
            InfixOpTag::SubAssign => "-=",
            InfixOpTag::MulAssign => "*=",
            InfixOpTag::DivAssign => "/=",
            InfixOpTag::ModAssign => "%=",
            InfixOpTag::BitAndAssign => "&=",
            InfixOpTag::BitOrAssign => "|=",
            InfixOpTag::ShiftLeftAssign => "<<=",
            InfixOpTag::ShiftRightAssign => ">>=",
            InfixOpTag::Cast => "as",
            InfixOpTag::FieldAccess => ".",
            InfixOpTag::Catch => "catch",
            InfixOpTag::ErrorPropagation => "?",
        }
    }
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
            "catch" => (Catch, OpInfo { bp: 13, assoc: L }),

            "." => (FieldAccess, OpInfo { bp: 12, assoc: L }),

            "?" => (
                ErrorPropagation,
                OpInfo {
                    bp: 11,
                    assoc: None,
                },
            ),

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

#[derive(Debug)]
pub enum PrefixOpTag {
    Not,
    Dereference,
    Positive,
    Negative,
}

impl PrefixOpTag {
    pub fn to_str(&self) -> &str {
        match self {
            PrefixOpTag::Not => "!",
            PrefixOpTag::Dereference => "*",
            PrefixOpTag::Positive => "+",
            PrefixOpTag::Negative => "-",
        }
    }
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
