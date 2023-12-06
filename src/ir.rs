use crate::{ast::*, tokens::*, wasm::*};
use alloc::{boxed::Box, collections::BTreeMap, format, rc::Rc, string::String, vec::Vec};
use core::cell::RefCell;

#[derive(Default)]
pub struct ModuleContext {
    pub wasm_module: RefCell<WasmModule>,
    pub fn_defs: BTreeMap<String, FnDef>,
    pub fn_bodies: RefCell<Vec<FnBody>>,
    pub fn_exports: Vec<FnExport>,
    pub memory_names: Vec<String>,
    pub struct_defs: BTreeMap<String, StructDef>,
    pub globals: BTreeMap<String, GlobalDef>,
    pub imported_fns_count: u32,
    pub data_size: Rc<RefCell<u32>>,
    pub string_pool: RefCell<BTreeMap<String, u32>>,
}

// v2

#[derive(Default)]
pub struct ModuleContextV2 {
    pub fn_defs: RefCell<Vec<FnDef2>>,
}

pub struct FnDef2 {
    // TODO: look for a lighter solution
    pub body: Vec<LoleTokenStream>,
}

impl ModuleContext {
    pub fn insert_fn_type(&mut self, fn_type: WasmFnType) -> u32 {
        let mut wasm_module = self.wasm_module.borrow_mut();

        let type_index = wasm_module.types.iter().position(|ft| *ft == fn_type);
        if let Some(type_index) = type_index {
            return type_index as u32;
        }

        wasm_module.types.push(fn_type);
        wasm_module.types.len() as u32 - 1
    }
}

#[derive(Debug, Clone)]
pub struct LoleFnType {
    pub inputs: Vec<LoleType>,
    pub output: LoleType,
}

pub struct FnContext<'a> {
    pub module: &'a ModuleContext,
    pub fn_lole_type: &'a LoleFnType,
    pub locals_last_index: u32,
    pub non_arg_wasm_locals: Vec<WasmType>,
    pub defers: BTreeMap<String, Vec<SExpr>>,
}

#[derive(PartialEq)]
pub enum BlockType {
    Function,
    Block,
    Loop,
}

pub struct Block<'a> {
    pub block_type: BlockType,
    pub locals: BTreeMap<String, LocalDef>,
    pub parent: Option<&'a Block<'a>>,
}

impl Block<'_> {
    pub fn get_local(&self, local_name: &str) -> Option<&LocalDef> {
        if let Some(local_def) = self.locals.get(local_name) {
            return Some(local_def);
        }

        if let Some(parent) = self.parent {
            return parent.get_local(local_name);
        }

        return None;
    }

    pub fn get_own_local(&self, local_name: &str) -> Option<&LocalDef> {
        if let Some(local_def) = self.locals.get(local_name) {
            return Some(local_def);
        }

        if self.block_type == BlockType::Function {
            if let Some(parent) = self.parent {
                return parent.get_local(local_name);
            }
        }

        return None;
    }
}

pub struct BlockContext<'a, 'b> {
    pub module: &'a ModuleContext,
    pub fn_ctx: &'a mut FnContext<'b>,
    pub block: Block<'a>,
}

impl BlockContext<'_, '_> {
    pub fn push_local(
        &mut self,
        local_name: String,
        value_type: LoleType,
    ) -> core::ops::Range<u32> {
        let local_index = self.fn_ctx.locals_last_index;
        let comp_count =
            value_type.emit_components(&self.module, &mut self.fn_ctx.non_arg_wasm_locals);

        self.fn_ctx.locals_last_index += comp_count;

        self.block.locals.insert(
            local_name.clone(),
            LocalDef {
                index: local_index,
                value_type,
            },
        );

        local_index..local_index + comp_count
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoleType {
    Void,
    Bool,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
    Pointer(Box<LoleType>),
    Tuple(Vec<LoleType>),
    StructInstance { name: String },
}

impl core::fmt::Display for LoleType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use LoleType::*;
        match self {
            Void => f.write_str("void"),
            Bool => f.write_str("bool"),
            U8 => f.write_str("u8"),
            I8 => f.write_str("i8"),
            U16 => f.write_str("u16"),
            I16 => f.write_str("i16"),
            U32 => f.write_str("u32"),
            I32 => f.write_str("i32"),
            F32 => f.write_str("f32"),
            U64 => f.write_str("u64"),
            I64 => f.write_str("i64"),
            F64 => f.write_str("f64"),
            Pointer(pointee) => f.write_fmt(format_args!("(& {pointee})")),
            Tuple(types) => {
                f.write_str("(tuple")?;
                for item in types {
                    f.write_str(" ")?;
                    f.write_fmt(format_args!("{item}"))?;
                }
                f.write_str(")")
            }
            StructInstance { name } => f.write_str(name),
        }
    }
}

pub struct ValueComponent {
    pub byte_offset: u32,
    pub value_type: LoleType,
}

#[derive(Default)]
pub struct EmitComponentStats {
    pub count: u32,
    pub byte_length: u32,
}

impl LoleType {
    pub fn to_wasm_type(&self) -> Option<WasmType> {
        Some(match self {
            LoleType::Bool | LoleType::U8 | LoleType::I8 | LoleType::U16 => WasmType::I32,
            LoleType::I16 | LoleType::U32 | LoleType::I32 | LoleType::Pointer(_) => WasmType::I32,
            LoleType::F32 => WasmType::F32,
            LoleType::U64 | LoleType::I64 => WasmType::I64,
            LoleType::F64 => WasmType::F64,
            _ => return None,
        })
    }

    pub fn emit_sized_component_stats(
        &self,
        ctx: &ModuleContext,
        stats: &mut EmitComponentStats,
        components: &mut Vec<ValueComponent>,
    ) -> Result<(), String> {
        let mut byte_len = None;
        match self {
            LoleType::Void => {}
            LoleType::Bool | LoleType::U8 | LoleType::I8 => byte_len = Some(1),
            LoleType::U16 | LoleType::I16 => byte_len = Some(2),
            LoleType::U32 | LoleType::I32 | LoleType::F32 | LoleType::Pointer(_) => {
                byte_len = Some(4)
            }
            LoleType::U64 | LoleType::I64 | LoleType::F64 => byte_len = Some(8),
            LoleType::Tuple(types) => {
                for lole_type in types {
                    lole_type.emit_sized_component_stats(ctx, stats, components)?;
                }
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

        if let Some(byte_len) = byte_len {
            let component = ValueComponent {
                byte_offset: stats.byte_length,
                value_type: self.clone(),
            };

            stats.count += 1;
            stats.byte_length += byte_len;
            components.push(component);
        }
        Ok(())
    }

    pub fn emit_components(&self, ctx: &ModuleContext, components: &mut Vec<WasmType>) -> u32 {
        if let Some(wasm_type) = self.to_wasm_type() {
            components.push(wasm_type);
            return 1;
        }

        match self {
            LoleType::Void => 0,
            LoleType::Tuple(types) => {
                let mut count = 0;
                for lole_type in types {
                    count += lole_type.emit_components(ctx, components);
                }
                count
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
            _ => unreachable!(),
        }
    }

    pub fn sized_comp_stats(&self, ctx: &ModuleContext) -> Result<EmitComponentStats, String> {
        let mut stats = EmitComponentStats::default();
        self.emit_sized_component_stats(ctx, &mut stats, &mut Default::default())?;

        Ok(stats)
    }

    pub fn to_load_kind(&self) -> Result<WasmLoadKind, String> {
        match self {
            LoleType::Bool => return Ok(WasmLoadKind::I32),
            LoleType::U8 => return Ok(WasmLoadKind::I32U8),
            LoleType::I8 => return Ok(WasmLoadKind::I32I8),
            LoleType::U16 => return Ok(WasmLoadKind::I32U16),
            LoleType::I16 => return Ok(WasmLoadKind::I32I16),
            LoleType::U32 => return Ok(WasmLoadKind::I32),
            LoleType::I32 => return Ok(WasmLoadKind::I32),
            LoleType::F32 => return Ok(WasmLoadKind::F32),
            LoleType::U64 => return Ok(WasmLoadKind::I64),
            LoleType::I64 => return Ok(WasmLoadKind::I64),
            LoleType::F64 => return Ok(WasmLoadKind::F64),
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
    pub locals: BTreeMap<String, LocalDef>,
    pub locals_last_index: u32,
    pub body: FnBodyExprs,
}

pub enum FnBodyExprs {
    V1(Vec<SExpr>),
    V2(Vec<LoleTokenStream>),
}

pub struct FnExport {
    pub in_name: String,
    pub out_name: String,
    pub loc: LoleLocation,
}

#[derive(Clone)]
pub struct StructDef {
    pub fields: Vec<StructField>,
    pub fully_defined: bool, // used for self-reference checks
}

#[derive(Clone)]
pub struct StructField {
    pub name: String,
    pub value_type: LoleType,
    pub field_index: u32,
    pub byte_offset: u32,
}

#[derive(Debug, Clone)]
pub struct FnDef {
    pub local: bool,
    pub fn_index: u32,
    pub type_index: u32,
    pub kind: LoleFnType,
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
pub enum LoleInstr {
    Unreachable,
    Drop {
        value: Box<LoleInstr>,
        drop_count: u32,
    },
    BinaryOp {
        kind: WasmBinaryOpKind,
        lhs: Box<LoleInstr>,
        rhs: Box<LoleInstr>,
    },
    MemorySize,
    MemoryGrow {
        size: Box<LoleInstr>,
    },
    Load {
        kind: LoleType,
        align: u32,
        offset: u32,
        address_instr: Box<LoleInstr>,
    },
    StructLoad {
        struct_name: String,
        address_instr: Box<LoleInstr>,
        address_local_index: u32,
        base_byte_offset: u32,
        primitive_loads: Vec<LoleInstr>,
    },
    LocalGet {
        local_index: u32,
        value_type: LoleType,
    },
    UntypedLocalGet {
        local_index: u32,
    },
    GlobalGet {
        global_index: u32,
    },
    StructGet {
        struct_name: String,
        base_index: u32,
        primitive_gets: Vec<LoleInstr>,
    },
    U32ConstLazy {
        value: Rc<RefCell<u32>>,
    },
    U32Const {
        value: u32,
    },
    I64Const {
        value: i64,
    },
    Set {
        bind: LoleSetBind,
    },
    Return {
        value: Box<LoleInstr>,
    },
    Block {
        block_type: LoleType,
        body: Vec<LoleInstr>,
    },
    Loop {
        block_type: LoleType,
        body: Vec<LoleInstr>,
    },
    If {
        block_type: LoleType,
        cond: Box<LoleInstr>,
        then_branch: Vec<LoleInstr>,
        else_branch: Option<Vec<LoleInstr>>,
    },
    Branch {
        label_index: u32,
    },
    Call {
        fn_index: u32,
        return_type: LoleType,
        args: Vec<LoleInstr>,
    },
    MultiValueEmit {
        values: Vec<LoleInstr>,
    },
    Casted {
        value_type: LoleType,
        expr: Box<LoleInstr>,
    },
    // will not be written to binary, used for types only
    NoEmit {
        expr: Box<LoleInstr>,
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
        address_instr: Box<LoleInstr>,
        value_local_index: u32,
    },
}

impl LoleInstr {
    pub fn get_type(&self, ctx: &ModuleContext) -> LoleType {
        match self {
            LoleInstr::Unreachable => LoleType::Void,
            LoleInstr::U32ConstLazy { value: _ } => LoleType::U32,
            LoleInstr::U32Const { value: _ } => LoleType::U32,
            LoleInstr::I64Const { value: _ } => LoleType::I64,
            LoleInstr::UntypedLocalGet { local_index: _ } => unreachable!(),

            LoleInstr::MultiValueEmit { values } => {
                let mut types = Vec::new();
                for value in values {
                    types.push(value.get_type(ctx));
                }
                LoleType::Tuple(types)
            }
            LoleInstr::NoEmit { expr } => expr.get_type(ctx),
            LoleInstr::StructLoad {
                struct_name,
                address_instr: _,
                address_local_index: _,
                base_byte_offset: _,
                primitive_loads: _,
            }
            | LoleInstr::StructGet {
                struct_name,
                base_index: _,
                primitive_gets: _,
            } => LoleType::StructInstance {
                name: struct_name.clone(),
            },

            // type-checked in the complier:
            LoleInstr::Casted {
                value_type,
                expr: _,
            } => value_type.clone(),
            LoleInstr::Set { bind: _ } => LoleType::Void,
            LoleInstr::Drop {
                value: _,
                drop_count: _,
            } => LoleType::Void,
            LoleInstr::Return { value: _ } => LoleType::Void,
            LoleInstr::MemorySize => LoleType::I32,
            LoleInstr::MemoryGrow { size: _ } => LoleType::I32,

            LoleInstr::BinaryOp {
                kind: _,
                lhs,
                rhs: _,
            } => lhs.get_type(ctx),
            LoleInstr::Load {
                kind,
                align: _,
                offset: _,
                address_instr: _,
            } => kind.clone(),
            LoleInstr::GlobalGet { global_index } => {
                let global_def = ctx
                    .globals
                    .values()
                    .find(|global| global.index == *global_index)
                    .unwrap();

                global_def.value_type.clone()
            }
            LoleInstr::LocalGet {
                local_index: _,
                value_type,
            } => value_type.clone(),
            LoleInstr::Call {
                return_type,
                fn_index: _,
                args: _,
            } => return_type.clone(),
            LoleInstr::If {
                block_type,
                cond: _,
                then_branch: _,
                else_branch: _,
            }
            | LoleInstr::Block {
                block_type,
                body: _,
            }
            | LoleInstr::Loop {
                block_type,
                body: _,
            } => block_type.clone(),
            LoleInstr::Branch { label_index: _ } => LoleType::Void,
        }
    }
}
