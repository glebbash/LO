use core::fmt;

use alloc::{string::String, vec::Vec};

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
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

    fn end(&self) -> usize {
        self.offset + self.length
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

pub fn parse(script: &str) -> Result<Vec<SExpr>, ParseError> {
    let chars = &script.chars().collect::<Vec<_>>();

    let mut index = skip_space(chars, 0);

    let mut items = Vec::new();

    while index < chars.len() {
        if !is_list_start(char_at(chars, index)?) {
            return Err(ParseError::UnexpectedChar {
                loc: Location {
                    offset: index,
                    length: 1,
                },
            });
        }

        let res = parse_list(chars, index)?;
        index = skip_space(chars, res.loc().end());
        items.push(res);
    }

    Ok(items)
}

fn parse_expr(chars: &Vec<char>, index: usize) -> ParseResult {
    if is_list_start(char_at(chars, index)?) {
        parse_list(chars, index)
    } else {
        parse_atom(chars, index)
    }
}

fn parse_atom(chars: &Vec<char>, index: usize) -> ParseResult {
    let atom_text = chars[index..]
        .iter()
        .take_while(|&&c| !is_space(c) && !is_list_end(c) && c != ';')
        .collect::<String>();

    let atom_len = atom_text.len();

    Ok(SExpr::Atom {
        value: atom_text,
        loc: Location {
            offset: index,
            length: atom_len,
        },
    })
}

fn parse_list(chars: &Vec<char>, mut index: usize) -> ParseResult {
    let start_index = index;

    let list_start_char = char_at(chars, index)?;
    index += 1; // eat list start

    index = skip_space(chars, index);

    let mut items = Vec::new();

    while !is_list_end(char_at(chars, index)?) {
        let res = parse_expr(chars, index)?;
        index = skip_space(chars, res.loc().end());
        items.push(res);
    }

    let list_end_char = char_at(chars, index)?;
    index += 1; // eat list end

    if !is_valid_list_chars(list_start_char, list_end_char) {
        return Err(ParseError::UnexpectedChar {
            loc: Location {
                offset: index,
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
            offset: start_index,
            length: index - start_index,
        },
    })
}

fn skip_space(chars: &Vec<char>, mut index: usize) -> usize {
    while char_at(chars, index).map(is_space).unwrap_or(false) {
        index += 1
    }

    if let Ok(';') = char_at(chars, index) {
        index += 1;

        while char_at(chars, index).map(|c| c != '\n').unwrap_or(false) {
            index += 1
        }

        if let Ok('\n') = char_at(chars, index) {
            index += 1;
        }

        return skip_space(chars, index);
    }

    index
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

fn char_at(chars: &Vec<char>, index: usize) -> Result<char, ParseError> {
    chars
        .get(index)
        .copied()
        .ok_or_else(|| ParseError::UnexpectedEOF {
            loc: Location {
                offset: index,
                length: 1,
            },
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_complex_script() {
        let result = parse(
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
                            offset: 18,
                            length: 1
                        }
                    },
                    SExpr::Atom {
                        value: String::from("b"),
                        loc: Location {
                            offset: 20,
                            length: 1
                        }
                    },
                    SExpr::List {
                        value: vec![],
                        loc: Location {
                            offset: 46,
                            length: 2
                        }
                    },
                    SExpr::List {
                        value: vec![
                            SExpr::Atom {
                                value: String::from("a"),
                                loc: Location {
                                    offset: 50,
                                    length: 1
                                }
                            },
                            SExpr::Atom {
                                value: String::from("b"),
                                loc: Location {
                                    offset: 52,
                                    length: 1
                                }
                            },
                            SExpr::Atom {
                                value: String::from("cadwdawd-aw423f"),
                                loc: Location {
                                    offset: 54,
                                    length: 15
                                }
                            },
                            SExpr::Atom {
                                value: String::from("123"),
                                loc: Location {
                                    offset: 91,
                                    length: 3
                                }
                            }
                        ],
                        loc: Location {
                            offset: 49,
                            length: 46
                        }
                    },
                    SExpr::List {
                        value: vec![SExpr::List {
                            value: vec![SExpr::List {
                                value: vec![],
                                loc: Location {
                                    offset: 98,
                                    length: 2
                                }
                            }],
                            loc: Location {
                                offset: 97,
                                length: 21
                            }
                        }],
                        loc: Location {
                            offset: 96,
                            length: 23
                        }
                    }
                ],
                loc: Location {
                    offset: 17,
                    length: 103
                }
            }]
        );
    }

    #[test]
    fn it_fails_on_eof_reached() {
        let result = parse("(");

        assert_eq!(
            result,
            Err(ParseError::UnexpectedEOF {
                loc: Location {
                    offset: 1,
                    length: 1,
                },
            })
        );
    }

    #[test]
    fn it_fails_in_other_ways() {
        let result = parse("      )  ");

        assert_eq!(
            result,
            Err(ParseError::UnexpectedChar {
                loc: Location {
                    offset: 6,
                    length: 1,
                },
            })
        );
    }
}
