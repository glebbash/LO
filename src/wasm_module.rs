use alloc::{boxed::Box, string::String, vec::Vec};

#[derive(Default)]
pub struct WasmModule {
    pub fn_types: Vec<WasmFnType>,
    pub fn_codes: Vec<WasmFnCode>,
    pub memories: Vec<WasmMemory>,
    pub globals: Vec<WasmGlobal>,
    pub exports: Vec<WasmExport>,
}

pub struct WasmFnType {
    pub inputs: Vec<WasmValueType>,
    pub outputs: Vec<WasmValueType>,
}

pub struct WasmFnCode {
    pub locals: Vec<WasmLocals>,
    pub expr: WasmExpr,
}

pub struct WasmLocals {
    pub count: u32,
    pub value_type: WasmValueType,
}

pub struct WasmExpr {
    pub instrs: Vec<WasmInstr>,
}

pub enum WasmInstr {
    Nop,
    I32Const(i32),
    Return {
        values: Vec<WasmInstr>,
    },
    I32LessThenSigned {
        lhs: Box<WasmInstr>,
        rhs: Box<WasmInstr>,
    },
    I32GreaterEqualSigned {
        lhs: Box<WasmInstr>,
        rhs: Box<WasmInstr>,
    },
    I32NotEqual {
        lhs: Box<WasmInstr>,
        rhs: Box<WasmInstr>,
    },
    I32Add {
        lhs: Box<WasmInstr>,
        rhs: Box<WasmInstr>,
    },
    I32Sub {
        lhs: Box<WasmInstr>,
        rhs: Box<WasmInstr>,
    },
    I32Mul {
        lhs: Box<WasmInstr>,
        rhs: Box<WasmInstr>,
    },
    LocalGet(u32),
    LocalSet {
        local_idx: u32,
        value: Box<WasmInstr>,
    },
    GlobalGet(u32),
    GlobalSet {
        global_idx: u32,
        value: Box<WasmInstr>,
    },
    MultiValueLocalSet {
        local_idxs: Vec<u32>,
        value: Box<WasmInstr>,
    },
    MultiValueEmit {
        values: Vec<WasmInstr>,
    },
    Loop {
        instrs: Vec<WasmInstr>,
    },
    LoopBreak,
    LoopContinue,
    I32Load {
        align: u32,
        offset: u32,
        address_instr: Box<WasmInstr>,
    },
    I32Load8Unsigned {
        align: u32,
        offset: u32,
        address_instr: Box<WasmInstr>,
    },
    Call {
        fn_idx: u32,
        args: Vec<WasmInstr>,
    },
    If {
        block_type: WasmValueType,
        cond: Box<WasmInstr>,
        then_branch: Box<WasmInstr>,
        else_branch: Box<WasmInstr>,
    },
    IfSingleBranch {
        cond: Box<WasmInstr>,
        then_branch: Box<WasmInstr>,
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

pub struct WasmGlobal {
    pub value_type: WasmValueType,
    pub mutable: bool,
    pub initial_value: WasmExpr,
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
