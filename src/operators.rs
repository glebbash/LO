use crate::tokens::*;

pub enum OpTag {
    Assign,
    AddAssign,
    Less,
    Add,
    Sub,
    Mul,
    Dot,
}

impl OpTag {
    fn parse(token: &LoleToken) -> Option<Self> {
        Some(match token.value.as_str() {
            "=" => OpTag::Assign,
            "+=" => OpTag::AddAssign,
            "<" => OpTag::Less,
            "+" => OpTag::Add,
            "-" => OpTag::Sub,
            "*" => OpTag::Mul,
            "." => OpTag::Dot,
            _ => return None,
        })
    }

    fn get_info(&self) -> OpInfo {
        use OpAssoc::*;

        match self {
            OpTag::Assign => OpInfo { bp: 1, assoc: None },
            OpTag::AddAssign => OpInfo { bp: 1, assoc: None },
            OpTag::Less => OpInfo { bp: 2, assoc: L },
            OpTag::Add => OpInfo { bp: 3, assoc: L },
            OpTag::Sub => OpInfo { bp: 3, assoc: L },
            OpTag::Mul => OpInfo { bp: 4, assoc: L },
            OpTag::Dot => OpInfo { bp: 5, assoc: L },
        }
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

pub struct Op {
    pub tag: OpTag,
    pub info: OpInfo,
    pub token: LoleToken,
}

impl Op {
    pub fn parse(token: LoleToken) -> Option<Self> {
        let tag = OpTag::parse(&token)?;
        let info = tag.get_info();
        Some(Self { tag, info, token })
    }
}
