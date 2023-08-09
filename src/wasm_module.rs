use crate::common::Location;
use alloc::{boxed::Box, rc::Rc, string::String, vec::Vec};
use core::cell::RefCell;

#[derive(Default)]
pub struct WasmModule {
    pub types: Vec<WasmFnType>,
    pub imports: Vec<WasmImport>,
    pub functions: Vec<u32>,
    pub memories: Vec<WasmLimits>,
    pub globals: Vec<WasmGlobal>,
    pub exports: Vec<WasmExport>,
    pub codes: Vec<WasmFn>,
    pub datas: RefCell<Vec<WasmData>>, // need RefCell for string support
}

pub struct WasmFnType {
    pub inputs: Vec<WasmValueType>,
    pub outputs: Vec<WasmValueType>,
}

pub struct WasmImport {
    pub module_name: String,
    pub item_name: String,
    pub item_desc: WasmImportDesc,
}

pub enum WasmImportDesc {
    Func { type_index: u32 },
}

pub struct WasmFn {
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

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum WasmBinaryOpKind {
    I32Equals = 0x46,
    I32LessThenSigned = 0x48,
    I32GreaterThenSigned = 0x4a,
    I32GreaterEqualSigned = 0x4e,
    I32NotEqual = 0x47,
    I32Add = 0x6a,
    I32Sub = 0x6b,
    I32Mul = 0x6c,
    I32DivUnsigned = 0x6e,
    I32RemUnsigned = 0x70,
    I32And = 0x71,
    I32Or = 0x72,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum WasmLoadKind {
    I32 = 0x28,
    I32U8 = 0x2d,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum WasmStoreKind {
    I32 = 0x36,
    I32U8 = 0x3A,
}

#[derive(Clone, Debug)]
pub enum WasmInstr {
    Unreachable,
    LoopBreak,
    LoopContinue,
    Drop {
        value: Box<WasmInstr>,
        drop_count: usize,
    },
    BinaryOp {
        kind: WasmBinaryOpKind,
        lhs: Box<WasmInstr>,
        rhs: Box<WasmInstr>,
    },
    // TODO: use single type for loads/gets?
    Load {
        kind: WasmLoadKind,
        align: u32,
        offset: u32,
        address_instr: Box<WasmInstr>,
    },
    StructLoad {
        struct_name: String,
        address_instr: Box<WasmInstr>,
        base_byte_offset: u32,
        primitive_loads: Vec<WasmInstr>,
    },
    LocalGet {
        local_index: u32,
    },
    GlobalGet {
        global_index: u32,
    },
    StructGet {
        struct_name: String,
        base_index: u32,
        primitive_gets: Vec<WasmInstr>,
    },
    I32ConstLazy {
        value: Rc<RefCell<i32>>,
    },
    I32Const {
        value: i32,
    },
    I64Const {
        value: i64,
    },
    Set {
        binds: Vec<WasmSetBind>,
        value: Box<WasmInstr>,
    },
    Return {
        value: Box<WasmInstr>,
        loc: Location,
    },
    Loop {
        instrs: Vec<WasmInstr>,
        loc: Location,
    },
    Call {
        fn_index: u32,
        args: Vec<WasmInstr>,
        loc: Location,
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
    MultiValueEmit {
        values: Vec<WasmInstr>,
    },
    // will not be written to binary, used for types only
    NoEmit {
        instr: Box<WasmInstr>,
    },
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WasmValueType {
    I32 = 0x7f,
    I64 = 0x7e,
    F32 = 0x7d,
    F64 = 0x7c,
    V128 = 0x7b,
    FuncRef = 0x70,
    ExternRef = 0x6f,
}

#[derive(Clone, Copy)]
pub struct WasmLimits {
    pub min: u32,
    pub max: Option<u32>,
}

pub struct WasmGlobal {
    pub kind: WasmGlobalKind,
    pub initial_value: WasmExpr,
}

#[derive(Clone, Copy)]
pub struct WasmGlobalKind {
    pub value_type: WasmValueType,
    pub mutable: bool,
}

pub struct WasmExport {
    pub export_type: WasmExportType,
    pub export_name: String,
    pub exported_item_index: u32,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum WasmExportType {
    Func = 0x00,
    Mem = 0x02,
}

pub enum WasmData {
    Active { offset: WasmExpr, bytes: Vec<u8> },
}

#[derive(Clone, Debug)]
pub enum WasmSetBind {
    Local {
        index: u32,
    },
    Global {
        index: u32,
    },
    Memory {
        align: u32,
        offset: u32,
        kind: WasmStoreKind,
        address_instr: Box<WasmInstr>,
        value_local_index: u32,
    },
}
