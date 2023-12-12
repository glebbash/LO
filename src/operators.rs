use crate::tokens::*;

pub enum InfixOpTag {
    Assign,
    AddAssign,
    SubAssign,
    And,
    Or,
    Equal,
    NotEqual,
    Less,
    Greater,
    GreaterEqual,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Cast,
    FieldAccess,
    RefFieldAccess,
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
            "=" => (Assign, OpInfo { bp: 1, assoc: None }),
            "+=" => (AddAssign, OpInfo { bp: 1, assoc: None }),
            "-=" => (SubAssign, OpInfo { bp: 1, assoc: None }),
            "||" => (Or, OpInfo { bp: 2, assoc: L }),
            "&&" => (And, OpInfo { bp: 3, assoc: L }),
            "==" => (Equal, OpInfo { bp: 4, assoc: None }),
            "!=" => (NotEqual, OpInfo { bp: 4, assoc: None }),
            "<" => (Less, OpInfo { bp: 4, assoc: L }),
            ">" => (Greater, OpInfo { bp: 4, assoc: L }),
            ">=" => (GreaterEqual, OpInfo { bp: 4, assoc: L }),
            "+" => (Add, OpInfo { bp: 5, assoc: L }),
            "-" => (Sub, OpInfo { bp: 5, assoc: L }),
            "*" => (Mul, OpInfo { bp: 6, assoc: L }),
            "/" => (Div, OpInfo { bp: 6, assoc: L }),
            "%" => (Mod, OpInfo { bp: 6, assoc: L }),
            "as" => (Cast, OpInfo { bp: 7, assoc: L }),
            "." => (FieldAccess, OpInfo { bp: 9, assoc: L }),
            "->" => (RefFieldAccess, OpInfo { bp: 9, assoc: L }),
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
