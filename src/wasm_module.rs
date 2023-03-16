use alloc::{boxed::Box, string::String, vec::Vec};

#[derive(Default)]
pub struct WasmModule {
    pub fn_types: Vec<WasmFnType>,
    pub fn_codes: Vec<WasmFnCode>,
    pub memories: Vec<WasmMemory>,
    pub exports: Vec<WasmExport>,
}

pub struct WasmFnType {
    pub inputs: Vec<WasmValueType>,
    pub outputs: Vec<WasmValueType>,
}

pub struct WasmFnCode {
    pub locals: Vec<WasmLocals>,
    pub expr: Expr,
}

pub struct WasmLocals {
    pub count: u32,
    pub value_type: WasmValueType,
}

pub struct Expr {
    pub instrs: Vec<Instr>,
}

pub enum Instr {
    I32Const(i32),
    Return {
        values: Vec<Instr>,
    },
    I32LessThenSigned {
        lhs: Box<Instr>,
        rhs: Box<Instr>,
    },
    I32GreaterEqualSigned {
        lhs: Box<Instr>,
        rhs: Box<Instr>,
    },
    I32NotEqual {
        lhs: Box<Instr>,
        rhs: Box<Instr>,
    },
    I32Add {
        lhs: Box<Instr>,
        rhs: Box<Instr>,
    },
    I32Sub {
        lhs: Box<Instr>,
        rhs: Box<Instr>,
    },
    I32Mul {
        lhs: Box<Instr>,
        rhs: Box<Instr>,
    },
    LocalGet(u32),
    LocalSet {
        local_idx: u32,
        value: Box<Instr>,
    },
    MultiValueLocalSet {
        local_idxs: Vec<u32>,
        value: Box<Instr>,
    },
    MultiValueEmit {
        values: Vec<Instr>,
    },
    Loop {
        instrs: Vec<Instr>,
    },
    LoopBreak,
    LoopContinue,
    I32Load {
        align: u32,
        offset: u32,
        address_instr: Box<Instr>,
    },
    I32Load8Unsigned {
        align: u32,
        offset: u32,
        address_instr: Box<Instr>,
    },
    Call {
        fn_idx: u32,
        args: Vec<Instr>,
    },
    If {
        block_type: WasmValueType,
        cond: Box<Instr>,
        then_branch: Box<Instr>,
        else_branch: Box<Instr>,
    },
    IfSingleBranch {
        cond: Box<Instr>,
        then_branch: Box<Instr>,
    },
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum WasmValueType {
    I32 = 0x7f,
    I64 = 0x7e,
    F32 = 0x7d,
    F64 = 0x7c,
    V128 = 0x7b,
    FuncRef = 0x70,
    ExternRef = 0x6f,
}

pub struct WasmMemory {
    pub min: u32,
    pub max: Option<u32>,
}

pub struct WasmExport {
    pub export_type: WasmExportType,
    pub export_name: String,
    pub exported_item_index: usize,
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum WasmExportType {
    Func = 0x00,
    Table = 0x01,
    Mem = 0x02,
    Global = 0x03,
}
