use crate::{ast::*, wasm::*};
use alloc::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
    string::String,
    vec::Vec,
};
use core::cell::RefCell;

#[derive(Default)]
pub struct ModuleContext {
    pub included_modules: BTreeSet<String>,
    pub wasm_module: WasmModule,
    pub fn_defs: BTreeMap<String, FnDef>,
    pub fn_bodies: BTreeMap<String, FnBody>,
    pub fn_exports: BTreeMap<String, String>,
    pub memory_names: Vec<String>,
    pub struct_defs: BTreeMap<String, StructDef>,
    pub enum_kinds: BTreeMap<String, u32>,
    pub globals: BTreeMap<String, GlobalDef>,
    pub imported_fns_count: u32,
    pub data_size: Rc<RefCell<i32>>,
    pub string_pool: RefCell<BTreeMap<String, i32>>,
}

pub struct FnContext<'a> {
    pub module: &'a ModuleContext,
    pub fn_type: &'a WasmFnType,
    pub locals: &'a mut BTreeMap<String, LocalDef>,
    pub locals_last_index: u32,
    pub non_arg_locals: Vec<WasmValueType>,
    pub defers: BTreeMap<String, Vec<SExpr>>,
}

#[derive(Clone, Debug)]
pub enum LoleValueType {
    Primitive(WasmValueType),
    StructInstance { name: String },
}

pub struct LocalDef {
    pub index: u32,
    pub value_type: LoleValueType,
}

pub struct GlobalDef {
    pub index: u32,
    pub mutable: bool,
}

pub struct FnBody {
    pub fn_index: u32,
    pub locals: RefCell<BTreeMap<String, LocalDef>>,
    pub locals_last_index: u32,
    pub body: Vec<SExpr>,
}

#[derive(Clone)]
pub struct StructDef {
    pub fields: Vec<StructField>,
}

#[derive(Clone)]
pub struct StructField {
    pub name: String,
    pub value_type: LoleValueType,
    pub field_index: u32,
    pub byte_offset: u32,
}

pub struct FnDef {
    pub local: bool,
    pub fn_index: u32,
    pub type_index: u32,
}

impl FnDef {
    pub fn get_absolute_index(&self, ctx: &ModuleContext) -> u32 {
        if self.local {
            self.fn_index + ctx.imported_fns_count
        } else {
            self.fn_index
        }
    }
}

impl ModuleContext {
    pub fn insert_fn_type(&mut self, fn_type: WasmFnType) -> u32 {
        self.wasm_module
            .types
            .iter()
            .position(|ft| *ft == fn_type)
            .unwrap_or_else(|| {
                self.wasm_module.types.push(fn_type);
                self.wasm_module.types.len() - 1
            }) as u32
    }
}
