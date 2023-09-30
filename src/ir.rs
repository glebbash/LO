use crate::{ast::*, wasm::*};
use alloc::{boxed::Box, collections::BTreeMap, format, rc::Rc, string::String, vec::Vec};
use core::cell::RefCell;

#[derive(Default)]
pub struct ModuleContext {
    pub wasm_module: WasmModule,
    pub fn_defs: BTreeMap<String, FnDef>,
    pub fn_bodies: Vec<FnBody>,
    pub fn_exports: Vec<FnExport>,
    pub memory_names: Vec<String>,
    pub struct_defs: BTreeMap<String, StructDef>,
    pub globals: BTreeMap<String, GlobalDef>,
    pub imported_fns_count: u32,
    pub data_size: Rc<RefCell<i32>>,
    pub string_pool: RefCell<BTreeMap<String, i32>>,
}

impl ModuleContext {
    pub fn insert_fn_type(&mut self, fn_type: WasmFnType) -> u32 {
        let type_index = self.wasm_module.types.iter().position(|ft| *ft == fn_type);
        if let Some(type_index) = type_index {
            return type_index as u32;
        }

        self.wasm_module.types.push(fn_type);
        self.wasm_module.types.len() as u32 - 1
    }
}

pub struct FnContext<'a> {
    pub module: &'a ModuleContext,
    pub fn_type: &'a WasmFnType,
    pub locals: &'a mut BTreeMap<String, LocalDef>,
    pub locals_last_index: u32,
    pub non_arg_locals: Vec<WasmType>,
    pub defers: BTreeMap<String, Vec<SExpr>>,
}

#[derive(PartialEq)]
pub enum BlockType {
    Function,
    Loop,
    Block,
}

pub struct Block<'a> {
    pub block_type: BlockType,
    pub parent: Option<&'a Block<'a>>,
}

pub struct BlockContext<'a, 'b> {
    pub module: &'a ModuleContext,
    pub fn_ctx: &'a mut FnContext<'b>,
    pub block: Block<'a>,
}

#[derive(Clone, Debug)]
pub enum LolePrimitiveType {
    Bool,
    U8,
    I8,
    F8,
    U16,
    I16,
    F16,
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
}

impl LolePrimitiveType {
    pub fn byte_length(&self) -> u32 {
        match self {
            LolePrimitiveType::Bool => 1,
            LolePrimitiveType::U8 => 1,
            LolePrimitiveType::I8 => 1,
            LolePrimitiveType::F8 => 1,
            LolePrimitiveType::U16 => 2,
            LolePrimitiveType::I16 => 2,
            LolePrimitiveType::F16 => 2,
            LolePrimitiveType::U32 => 4,
            LolePrimitiveType::I32 => 4,
            LolePrimitiveType::F32 => 4,
            LolePrimitiveType::U64 => 8,
            LolePrimitiveType::I64 => 8,
            LolePrimitiveType::F64 => 8,
        }
    }

    pub fn to_wasm_type(&self) -> WasmType {
        match self {
            LolePrimitiveType::Bool => WasmType::I32,
            LolePrimitiveType::U8 => WasmType::I32,
            LolePrimitiveType::I8 => WasmType::I32,
            LolePrimitiveType::F8 => WasmType::F32,
            LolePrimitiveType::U16 => WasmType::I32,
            LolePrimitiveType::I16 => WasmType::I32,
            LolePrimitiveType::F16 => WasmType::F32,
            LolePrimitiveType::U32 => WasmType::I32,
            LolePrimitiveType::I32 => WasmType::I32,
            LolePrimitiveType::F32 => WasmType::F32,
            LolePrimitiveType::U64 => WasmType::I64,
            LolePrimitiveType::I64 => WasmType::I64,
            LolePrimitiveType::F64 => WasmType::F64,
        }
    }
}

#[derive(Clone, Debug)]
pub enum LoleType {
    Void,
    Primitive(LolePrimitiveType),
    Pointer(Box<LoleType>),
    StructInstance { name: String },
}

impl LoleType {
    pub fn to_wasm_type(&self) -> Option<WasmType> {
        match self {
            LoleType::Primitive(primitive) => Some(primitive.to_wasm_type()),
            LoleType::Pointer(_) => Some(WasmType::I32),
            _ => None,
        }
    }
}

pub struct ValueComponent {
    pub byte_offset: u32,
    pub value_type: LolePrimitiveType,
}

#[derive(Default)]
pub struct EmitComponentStats {
    pub count: u32,
    pub byte_length: u32,
}

impl LoleType {
    pub fn emit_sized_component_stats(
        &self,
        ctx: &ModuleContext,
        stats: &mut EmitComponentStats,
        components: &mut Vec<ValueComponent>,
    ) -> Result<(), String> {
        match self {
            LoleType::Void => {}
            LoleType::Primitive(primitive) => {
                let component = ValueComponent {
                    byte_offset: stats.byte_length,
                    value_type: primitive.clone(),
                };

                stats.count += 1;
                stats.byte_length += component.value_type.byte_length();
                components.push(component);
            }
            LoleType::Pointer(_) => {
                let component = ValueComponent {
                    byte_offset: stats.byte_length,
                    value_type: LolePrimitiveType::U32,
                };
                stats.count += 1;
                stats.byte_length += component.value_type.byte_length();
                components.push(component);
            }
            LoleType::StructInstance { name } => {
                // safe, validation is done when creating StructInstance
                let struct_def = ctx.struct_defs.get(name).unwrap();

                for field in &struct_def.fields {
                    field
                        .value_type
                        .emit_sized_component_stats(ctx, stats, components)?;
                }
            }
        };
        Ok(())
    }

    pub fn emit_components(&self, ctx: &ModuleContext, components: &mut Vec<WasmType>) -> u32 {
        match self {
            LoleType::Void => 0,
            LoleType::Pointer(_) => {
                components.push(WasmType::I32);
                1
            }
            LoleType::Primitive(value_type) => {
                components.push(value_type.to_wasm_type());
                1
            }
            LoleType::StructInstance { name } => {
                // safe, validation is done when creating StructInstance
                let struct_def = ctx.struct_defs.get(name).unwrap();
                let mut count = 0;

                for field in &struct_def.fields {
                    count += field.value_type.emit_components(ctx, components);
                }

                count
            }
        }
    }

    pub fn sized_comp_stats(&self, ctx: &ModuleContext) -> Result<EmitComponentStats, String> {
        let mut stats = EmitComponentStats::default();
        self.emit_sized_component_stats(ctx, &mut stats, &mut Default::default())?;

        Ok(stats)
    }

    pub fn to_load_kind(&self) -> Result<WasmLoadKind, String> {
        match self {
            LoleType::Primitive(LolePrimitiveType::Bool) => return Ok(WasmLoadKind::I32),
            LoleType::Primitive(LolePrimitiveType::U32) => return Ok(WasmLoadKind::I32),
            LoleType::Primitive(LolePrimitiveType::I32) => return Ok(WasmLoadKind::I32),
            LoleType::Primitive(LolePrimitiveType::U8) => return Ok(WasmLoadKind::I32U8),
            LoleType::Pointer(_) => return Ok(WasmLoadKind::I32),
            _ => {}
        };
        return Err(format!("Unsupported type for load: {self:?}"));
    }
}

pub struct LocalDef {
    pub index: u32,
    pub value_type: LoleType,
}

pub struct GlobalDef {
    pub index: u32,
    pub mutable: bool,
    pub value_type: LoleType,
}

pub struct FnBody {
    pub fn_index: u32,
    pub type_index: u32,
    pub locals: RefCell<BTreeMap<String, LocalDef>>,
    pub locals_last_index: u32,
    pub body: Vec<SExpr>,
}

pub struct FnExport {
    pub in_name: String,
    pub out_name: String,
    pub loc: Location,
}

#[derive(Clone)]
pub struct StructDef {
    pub fields: Vec<StructField>,
}

#[derive(Clone)]
pub struct StructField {
    pub name: String,
    pub value_type: LoleType,
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

#[derive(Clone, Debug)]
pub enum LoleExpr {
    Unreachable,
    Drop {
        value: Box<LoleExpr>,
        drop_count: usize,
    },
    BinaryOp {
        kind: WasmBinaryOpKind,
        lhs: Box<LoleExpr>,
        rhs: Box<LoleExpr>,
    },
    MemorySize,
    MemoryGrow {
        size: Box<LoleExpr>,
    },
    Load {
        kind: LoleType,
        align: u32,
        offset: u32,
        address_instr: Box<LoleExpr>,
    },
    StructLoad {
        struct_name: String,
        address_instr: Box<LoleExpr>,
        address_local_index: u32,
        base_byte_offset: u32,
        primitive_loads: Vec<LoleExpr>,
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
        primitive_gets: Vec<LoleExpr>,
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
        bind: LoleSetBind,
    },
    Return {
        value: Box<LoleExpr>,
    },
    Block {
        block_type: LoleType,
        body: Vec<LoleExpr>,
    },
    Loop {
        block_type: LoleType,
        body: Vec<LoleExpr>,
    },
    If {
        block_type: LoleType,
        cond: Box<LoleExpr>,
        then_branch: Vec<LoleExpr>,
        else_branch: Option<Vec<LoleExpr>>,
    },
    Branch {
        label_index: u32,
    },
    Call {
        fn_index: u32,
        fn_type_index: u32, // for type-checker
        args: Vec<LoleExpr>,
    },
    MultiValueEmit {
        values: Vec<LoleExpr>,
    },
    // will not be written to binary, used for types only
    NoEmit {
        expr: Box<LoleExpr>,
    },
    // will be written to binary but emits no types
    NoTypeCheck {
        expr: Box<LoleExpr>,
    },
}

#[derive(Clone, Debug)]
pub enum LoleSetBind {
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
        address_instr: Box<LoleExpr>,
        value_local_index: u32,
    },
}
