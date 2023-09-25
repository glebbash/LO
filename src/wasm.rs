use crate::ir::*;
use alloc::{boxed::Box, format, rc::Rc, string::String, vec::Vec};
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
    Drop {
        value: Box<WasmInstr>,
        drop_count: usize,
    },
    BinaryOp {
        kind: WasmBinaryOpKind,
        lhs: Box<WasmInstr>,
        rhs: Box<WasmInstr>,
    },
    MemorySize,
    MemoryGrow {
        size: Box<WasmInstr>,
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
        address_local_index: u32,
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
        bind: WasmSetBind,
    },
    Return {
        value: Box<WasmInstr>,
    },
    Loop {
        block_type: Option<WasmType>,
        body: Vec<WasmInstr>,
    },
    Block {
        block_type: Option<WasmType>,
        body: Vec<WasmInstr>,
    },
    If {
        block_type: Option<WasmType>,
        cond: Box<WasmInstr>,
        then_branch: Vec<WasmInstr>,
        else_branch: Option<Vec<WasmInstr>>,
    },
    Branch {
        label_index: u32,
    },
    Call {
        fn_index: u32,
        fn_type_index: u32, // for type-checker
        args: Vec<WasmInstr>,
    },
    MultiValueEmit {
        values: Vec<WasmInstr>,
    },
    // will not be written to binary, used for types only
    NoEmit {
        instr: Box<WasmInstr>,
    },
    // will be written to binary but emits no types
    NoTypeCheck {
        instr: Box<WasmInstr>,
    },
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

impl WasmStoreKind {
    pub fn from_load_kind(kind: &WasmLoadKind) -> Self {
        match kind {
            WasmLoadKind::I32 => Self::I32,
            WasmLoadKind::I32U8 => Self::I32U8,
        }
    }
}

impl WasmLoadKind {
    pub fn get_primitive_type(&self) -> LolePrimitiveType {
        match &self {
            Self::I32 => LolePrimitiveType::I32,
            Self::I32U8 => LolePrimitiveType::U8,
        }
    }

    pub fn from_primitive_type(value_type: &LolePrimitiveType) -> Result<Self, String> {
        match value_type {
            LolePrimitiveType::I32 => Ok(Self::I32),
            LolePrimitiveType::U8 => Ok(Self::I32U8),
            _ => return Err(format!("Unsupported type for load: {value_type:?}")),
        }
    }
}
