use alloc::string::String;
use alloc::vec::Vec;

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
        .take_while(|&&c| !is_space(c) && c != ')')
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
    while is_space(char_at(chars, index).unwrap_or('x')) {
        index += 1
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
