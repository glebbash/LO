use alloc::{string::String, vec::Vec};

#[derive(Default)]
pub struct WasmModule {
    pub fn_types: Vec<FnType>,
    pub fn_codes: Vec<FnCode>,
    pub exports: Vec<Export>,
}

pub struct FnType {
    pub inputs: Vec<ValueType>,
    pub outputs: Vec<ValueType>,
}

pub struct FnCode {
    pub locals: Vec<Locals>,
    pub expr: Expr,
}

pub struct Locals {
    pub count: u32,
    pub value_type: ValueType,
}

pub struct Expr {
    pub instrs: Vec<Instr>,
}

pub enum Instr {
    Return,
    I32LTS,
    I32Sub,
    I32Mul,
    I32Const(i32),
    LocalGet(u32),
    Call(u32),
    If(ValueType, Vec<Instr>, Vec<Instr>),
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum ValueType {
    I32 = 0x7f,
    I64 = 0x7e,
    F32 = 0x7d,
    F64 = 0x7c,
    V128 = 0x7b,
    FuncRef = 0x70,
    ExternRef = 0x6f,
}

pub struct Export {
    pub export_type: ExportType,
    pub export_name: String,
    pub exported_item_index: usize,
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum ExportType {
    Func = 0x00,
    Table = 0x01,
    Mem = 0x02,
    Global = 0x03,
}
