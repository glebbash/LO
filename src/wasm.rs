use alloc::{string::String, vec::Vec};
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

#[derive(PartialEq)]
pub struct WasmFnType {
    pub inputs: Vec<WasmType>,
    pub outputs: Vec<WasmType>,
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
    pub value_type: WasmType,
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
    I64 = 0x29,
    F32 = 0x2A,
    F64 = 0x2B,
    I32I8 = 0x2c,
    I32U8 = 0x2d,
    I32I16 = 0x2e,
    I32U16 = 0x2f,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum WasmStoreKind {
    I32 = 0x36,
    I64 = 0x37,
    F32 = 0x38,
    F64 = 0x39,
    I32U8 = 0x3A,
    I32U16 = 0x3B,
}

impl WasmStoreKind {
    pub fn from_load_kind(kind: &WasmLoadKind) -> Self {
        match kind {
            WasmLoadKind::I32 => Self::I32,
            WasmLoadKind::I64 => Self::I64,
            WasmLoadKind::F32 => Self::F32,
            WasmLoadKind::F64 => Self::F64,
            WasmLoadKind::I32I8 => Self::I32U8,
            WasmLoadKind::I32U8 => Self::I32U8,
            WasmLoadKind::I32I16 => Self::I32U16,
            WasmLoadKind::I32U16 => Self::I32U16,
        }
    }
}

#[derive(Clone, Debug)]
pub enum WasmInstr {
    Unreachable,
    Drop,
    BinaryOp {
        kind: WasmBinaryOpKind,
    },
    MemorySize,
    MemoryGrow,
    I32Const {
        value: i32,
    },
    I64Const {
        value: i64,
    },
    LocalGet {
        local_index: u32,
    },
    GlobalGet {
        global_index: u32,
    },
    LocalSet {
        local_index: u32,
    },
    GlobalSet {
        global_index: u32,
    },
    Load {
        kind: WasmLoadKind,
        align: u32,
        offset: u32,
    },
    Store {
        kind: WasmStoreKind,
        align: u32,
        offset: u32,
    },
    Return,
    BlockStart {
        block_type: WasmBlockType,
        return_type: Option<WasmType>,
    },
    Else,
    BlockEnd,
    Branch {
        label_index: u32,
    },
    Call {
        fn_index: u32,
    },
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum WasmBlockType {
    Block = 0x02,
    Loop = 0x03,
    If = 0x04,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WasmType {
    I32 = 0x7f,
    I64 = 0x7e,
    F32 = 0x7d,
    F64 = 0x7c,
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
    pub value_type: WasmType,
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
