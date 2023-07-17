use core::fmt;

use alloc::{boxed::Box, string::String, vec::Vec};

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub file_name: Box<str>,
    pub offset: usize,
    pub length: usize,
}

impl Location {
    pub fn position_in(&self, script: &str) -> (usize, usize) {
        let mut offset = self.offset;
        let (mut line, mut col) = (1, 1);

        for char in script.chars() {
            offset -= 1;

            if offset == 0 {
                break;
            }

            if char == '\n' {
                col = 1;
                line += 1;
                continue;
            }

            col += 1;
        }

        (line, col)
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
        }
    }

    fn parse_all(&mut self) -> Result<Vec<SExpr>, ParseError> {
        self.skip_space();

        let mut items = Vec::new();

        while self.index < self.chars.len() {
            if !is_list_start(self.current_char()?) {
                return Err(ParseError::UnexpectedChar {
                    loc: Location {
                        file_name: self.file_name.clone(),
                        offset: self.index,
                        length: 1,
                    },
                });
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
        let start_index = self.index;

        loop {
            let c = self.current_char()?;
            if is_space(c) || is_list_end(c) || c == ';' {
                break;
            }
            self.next_char();
        }

        Ok(SExpr::Atom {
            value: self.chars[start_index..self.index].iter().collect(),
            loc: Location {
                file_name: self.file_name.clone(),
                offset: start_index,
                length: self.index - start_index,
            },
        })
    }

    fn parse_list(&mut self) -> ParseResult {
        let start_index = self.index;

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
            return Err(ParseError::UnexpectedChar {
                loc: Location {
                    file_name: self.file_name.clone(),
                    offset: self.index,
                    length: 1,
                },
            });
        }

        if list_start_char == '{' && items.len() >= 2 {
            items.swap(0, 1);
        }

        Ok(SExpr::List {
            value: items,
            loc: Location {
                file_name: self.file_name.clone(),
                offset: start_index,
                length: self.index - start_index,
            },
        })
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
    }

    fn current_char(&mut self) -> Result<char, ParseError> {
        self.chars
            .get(self.index)
            .copied()
            .ok_or_else(|| ParseError::UnexpectedEOF {
                loc: Location {
                    file_name: self.file_name.clone(),
                    offset: self.index,
                    length: 1,
                },
            })
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

// FIXME: make this tests run or move them to test.mjs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_complex_script() {
        let file_name = "<input>";
        let result = parse(
            file_name,
            r"
                (a b
                        () (a b cadwdawd-aw423f
                     123) ((()
                )))
            ",
        )
        .unwrap();

        assert_eq!(
            result,
            vec![SExpr::List {
                value: vec![
                    SExpr::Atom {
                        value: String::from("a"),
                        loc: Location {
                            file_name: file_name.into(),
                            offset: 18,
                            length: 1
                        }
                    },
                    SExpr::Atom {
                        value: String::from("b"),
                        loc: Location {
                            file_name: file_name.into(),
                            offset: 20,
                            length: 1
                        }
                    },
                    SExpr::List {
                        value: vec![],
                        loc: Location {
                            file_name: file_name.into(),
                            offset: 46,
                            length: 2
                        }
                    },
                    SExpr::List {
                        value: vec![
                            SExpr::Atom {
                                value: String::from("a"),
                                loc: Location {
                                    file_name: file_name.into(),
                                    offset: 50,
                                    length: 1
                                }
                            },
                            SExpr::Atom {
                                value: String::from("b"),
                                loc: Location {
                                    file_name: file_name.into(),
                                    offset: 52,
                                    length: 1
                                }
                            },
                            SExpr::Atom {
                                value: String::from("cadwdawd-aw423f"),
                                loc: Location {
                                    file_name: file_name.into(),
                                    offset: 54,
                                    length: 15
                                }
                            },
                            SExpr::Atom {
                                value: String::from("123"),
                                loc: Location {
                                    file_name: file_name.into(),
                                    offset: 91,
                                    length: 3
                                }
                            }
                        ],
                        loc: Location {
                            file_name: file_name.into(),
                            offset: 49,
                            length: 46
                        }
                    },
                    SExpr::List {
                        value: vec![SExpr::List {
                            value: vec![SExpr::List {
                                value: vec![],
                                loc: Location {
                                    file_name: file_name.into(),
                                    offset: 98,
                                    length: 2
                                }
                            }],
                            loc: Location {
                                file_name: file_name.into(),
                                offset: 97,
                                length: 21
                            }
                        }],
                        loc: Location {
                            file_name: file_name.into(),
                            offset: 96,
                            length: 23
                        }
                    }
                ],
                loc: Location {
                    file_name: file_name.into(),
                    offset: 17,
                    length: 103
                }
            }]
        );
    }

    #[test]
    fn it_fails_on_eof_reached() {
        let file_name = "<input>";
        let result = parse(file_name, "(");

        assert_eq!(
            result,
            Err(ParseError::UnexpectedEOF {
                loc: Location {
                    file_name: file_name.into(),
                    offset: 1,
                    length: 1,
                },
            })
        );
    }

    #[test]
    fn it_fails_in_other_ways() {
        let file_name = "<input>";
        let result = parse(file_name, "      )  ");

        assert_eq!(
            result,
            Err(ParseError::UnexpectedChar {
                loc: Location {
                    file_name: file_name.into(),
                    offset: 6,
                    length: 1,
                },
            })
        );
    }
}
