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

#[derive(Clone)]
struct ParseContext<'a> {
    file_name: &'a str,
    chars: &'a Vec<char>,
    index: usize,
}

pub fn parse(file_name: &str, script: &str) -> Result<Vec<SExpr>, ParseError> {
    let chars = &script.chars().collect::<Vec<_>>();

    let mut ctx = ParseContext {
        file_name,
        chars,
        index: 0,
    };

    ctx.index = skip_space(&ctx);

    let mut items = Vec::new();

    while ctx.index < chars.len() {
        if !is_list_start(char_at(&ctx)?) {
            return Err(ParseError::UnexpectedChar {
                loc: Location {
                    file_name: file_name.into(),
                    offset: ctx.index,
                    length: 1,
                },
            });
        }

        let res = parse_list(&ctx)?;
        ctx.index = res.loc().end();
        ctx.index = skip_space(&ctx);
        items.push(res);
    }

    Ok(items)
}

fn parse_expr(ctx: &ParseContext) -> ParseResult {
    if is_list_start(char_at(ctx)?) {
        parse_list(ctx)
    } else {
        parse_atom(ctx)
    }
}

fn parse_atom(ctx: &ParseContext) -> ParseResult {
    let atom_text = ctx.chars[ctx.index..]
        .iter()
        .take_while(|&&c| !is_space(c) && !is_list_end(c) && c != ';')
        .collect::<String>();

    let atom_len = atom_text.len();

    Ok(SExpr::Atom {
        value: atom_text,
        loc: Location {
            file_name: ctx.file_name.into(),
            offset: ctx.index,
            length: atom_len,
        },
    })
}

fn parse_list(ctx: &ParseContext) -> ParseResult {
    let mut ctx = (*ctx).clone();
    let start_index = ctx.index;

    let list_start_char = char_at(&ctx)?;
    ctx.index += 1; // eat list start

    ctx.index = skip_space(&ctx);

    let mut items = Vec::new();

    while !is_list_end(char_at(&ctx)?) {
        let res = parse_expr(&ctx)?;
        ctx.index = res.loc().end();
        ctx.index = skip_space(&ctx);
        items.push(res);
    }

    let list_end_char = char_at(&ctx)?;
    ctx.index += 1; // eat list end

    if !is_valid_list_chars(list_start_char, list_end_char) {
        return Err(ParseError::UnexpectedChar {
            loc: Location {
                file_name: ctx.file_name.into(),
                offset: ctx.index,
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
            file_name: ctx.file_name.into(),
            offset: start_index,
            length: ctx.index - start_index,
        },
    })
}

fn skip_space(ctx: &ParseContext) -> usize {
    let mut ctx = (*ctx).clone();

    while char_at(&ctx).map(is_space).unwrap_or(false) {
        ctx.index += 1
    }

    if let Ok(';') = char_at(&ctx) {
        ctx.index += 1;

        while char_at(&ctx).map(|c| c != '\n').unwrap_or(false) {
            ctx.index += 1
        }

        if let Ok('\n') = char_at(&ctx) {
            ctx.index += 1;
        }

        return skip_space(&ctx);
    }

    ctx.index
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

fn char_at(ctx: &ParseContext) -> Result<char, ParseError> {
    ctx.chars
        .get(ctx.index)
        .copied()
        .ok_or_else(|| ParseError::UnexpectedEOF {
            loc: Location {
                file_name: ctx.file_name.into(),
                offset: ctx.index,
                length: 1,
            },
        })
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
