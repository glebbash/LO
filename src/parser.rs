use core::fmt;

use alloc::{string::String, vec::Vec};

#[derive(Debug, PartialEq)]
pub enum SExpr {
    Atom(String),
    List(Vec<SExpr>),
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedEOF,
    UnexpectedChar,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedChar => write!(f, "Unexpected character"),
            ParseError::UnexpectedEOF => write!(f, "Unexpected EOF"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct WithIndex<T> {
    pub data: T,
    pub index: usize,
}

pub type ParseResult = Result<WithIndex<SExpr>, WithIndex<ParseError>>;

pub fn parse(script: &str) -> Result<Vec<SExpr>, WithIndex<ParseError>> {
    let chars = &script.chars().collect::<Vec<_>>();

    let mut index = skip_space(chars, 0);

    let mut items = Vec::new();

    while index < chars.len() {
        if char_at(chars, index)? != '(' {
            return Err(WithIndex {
                data: ParseError::UnexpectedChar,
                index,
            });
        }

        let res = parse_list(chars, index)?;

        items.push(res.data);

        index = skip_space(chars, res.index);
    }

    Ok(items)
}

fn parse_expr(chars: &Vec<char>, index: usize) -> ParseResult {
    if char_at(chars, index)? == '(' {
        parse_list(chars, index)
    } else {
        parse_atom(chars, index)
    }
}

fn parse_atom(chars: &Vec<char>, index: usize) -> ParseResult {
    let atom_text = chars[index..]
        .iter()
        .take_while(|&&c| !is_space(c) && c != ')' && c != ';')
        .collect::<String>();

    Ok(WithIndex {
        index: index + atom_text.len(),
        data: SExpr::Atom(atom_text),
    })
}

fn parse_list(chars: &Vec<char>, mut index: usize) -> ParseResult {
    index += 1; // eat lparen

    index = skip_space(chars, index);

    let mut items = Vec::new();

    while char_at(chars, index)? != ')' {
        let res = parse_expr(chars, index)?;

        items.push(res.data);

        index = skip_space(chars, res.index);
    }

    index += 1; // eat rparen

    Ok(WithIndex {
        data: SExpr::List(items),
        index,
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

    return index;
}

fn is_space(c: char) -> bool {
    match c {
        ' ' | '\n' | '\t' => true,
        _ => false,
    }
}

fn char_at(chars: &Vec<char>, index: usize) -> Result<char, WithIndex<ParseError>> {
    chars.get(index).copied().ok_or_else(|| WithIndex {
        data: ParseError::UnexpectedEOF,
        index,
    })
}

pub fn index_to_position(script: &str, mut index: usize) -> (u32, u32) {
    let (mut line, mut col) = (1, 1);

    for char in script.chars() {
        index -= 1;

        if index == 0 {
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
            vec![SExpr::List(vec![
                SExpr::Atom(String::from("a")),
                SExpr::Atom(String::from("b")),
                SExpr::List(vec![]),
                SExpr::List(vec![
                    SExpr::Atom(String::from("a")),
                    SExpr::Atom(String::from("b")),
                    SExpr::Atom(String::from("cadwdawd-aw423f")),
                    SExpr::Atom(String::from("123"))
                ]),
                SExpr::List(vec![SExpr::List(vec![SExpr::List(vec![])])])
            ])]
        );
    }

    #[test]
    fn it_parses_42_example() {
        let result = parse(include_str!("../examples/42.lole")).unwrap();

        assert_eq!(
            result,
            vec![
                SExpr::List(vec![
                    SExpr::Atom(String::from("::")),
                    SExpr::Atom(String::from("main")),
                    SExpr::List(vec![]),
                    SExpr::List(vec![SExpr::Atom(String::from("i32"))])
                ]),
                SExpr::List(vec![
                    SExpr::Atom(String::from("fn")),
                    SExpr::Atom(String::from("main")),
                    SExpr::List(vec![]),
                    SExpr::List(vec![SExpr::List(vec![
                        SExpr::Atom(String::from("i32")),
                        SExpr::Atom(String::from("42"))
                    ]),])
                ]),
                SExpr::List(vec![
                    SExpr::Atom(String::from("export")),
                    SExpr::Atom(String::from("main")),
                    SExpr::Atom(String::from(":as")),
                    SExpr::Atom(String::from("main"))
                ])
            ]
        );
    }

    #[test]
    fn it_fails_on_eof_reached() {
        let result = parse("(");

        assert_eq!(
            result,
            Err(WithIndex {
                data: ParseError::UnexpectedEOF,
                index: 1,
            })
        );
    }

    #[test]
    fn it_fails_in_other_ways() {
        let result = parse("      )  ");

        assert_eq!(
            result,
            Err(WithIndex {
                data: ParseError::UnexpectedChar,
                index: 6,
            })
        );
    }
}
