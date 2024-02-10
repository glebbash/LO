use crate::{lexer::*, utils::*, wasm::*};
use alloc::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    format,
    rc::Rc,
    string::String,
    vec::Vec,
};
use core::cell::RefCell;

#[derive(Default)]
pub struct ModuleContext {
    pub wasm_module: RefCell<WasmModule>,
    pub fn_defs: BTreeMap<String, FnDef>,
    pub fn_bodies: RefCell<Vec<FnBody>>,
    pub fn_exports: Vec<FnExport>,
    pub memories: BTreeMap<String, u32>,
    pub struct_defs: BTreeMap<String, StructDef>,
    pub globals: BTreeMap<String, GlobalDef>,
    pub imported_fns_count: u32,
    pub data_size: Rc<RefCell<u32>>,
    pub string_pool: RefCell<BTreeMap<String, u32>>,
    pub type_aliases: RefCell<BTreeMap<String, LoType>>,
    pub constants: RefCell<BTreeMap<String, LoInstr>>,
    pub included_modules: BTreeSet<String>,
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
pub struct LoFnType {
    pub inputs: Vec<LoType>,
    pub output: LoType,
}

pub struct FnContext<'a> {
    pub module: &'a ModuleContext,
    pub lo_fn_type: &'a LoFnType,
    pub locals_last_index: u32,
    pub non_arg_wasm_locals: Vec<WasmType>,
    pub defers: BTreeMap<String, Vec<LoInstr>>,
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
    pub fn push_local(&mut self, local_name: String, value_type: LoType) -> core::ops::Range<u32> {
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
pub enum LoType {
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
    Pointer(Box<LoType>),
    Tuple(Vec<LoType>),
    StructInstance { name: String },
}

impl core::fmt::Display for LoType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LoType::Void => f.write_str("void"),
            LoType::Bool => f.write_str("bool"),
            LoType::U8 => f.write_str("u8"),
            LoType::I8 => f.write_str("i8"),
            LoType::U16 => f.write_str("u16"),
            LoType::I16 => f.write_str("i16"),
            LoType::U32 => f.write_str("u32"),
            LoType::I32 => f.write_str("i32"),
            LoType::F32 => f.write_str("f32"),
            LoType::U64 => f.write_str("u64"),
            LoType::I64 => f.write_str("i64"),
            LoType::F64 => f.write_str("f64"),
            LoType::Pointer(pointee) => f.write_fmt(format_args!("&{pointee}")),
            LoType::Tuple(types) => {
                f.write_str("(")?;
                let mut types_iter = types.iter();
                if let Some(item) = types_iter.next() {
                    f.write_fmt(format_args!("{item}"))?;
                }
                for item in types_iter {
                    f.write_str(", ")?;
                    f.write_fmt(format_args!("{item}"))?;
                }
                f.write_str(")")
            }
            LoType::StructInstance { name } => f.write_str(name),
        }
    }
}

pub struct ValueComponent {
    pub byte_offset: u32,
    pub value_type: LoType,
}

#[derive(Default)]
pub struct EmitComponentStats {
    pub count: u32,
    pub byte_length: u32,
}

impl LoType {
    pub fn to_wasm_type(&self) -> Option<WasmType> {
        Some(match self {
            LoType::Bool | LoType::U8 | LoType::I8 | LoType::U16 => WasmType::I32,
            LoType::I16 | LoType::U32 | LoType::I32 | LoType::Pointer(_) => WasmType::I32,
            LoType::F32 => WasmType::F32,
            LoType::U64 | LoType::I64 => WasmType::I64,
            LoType::F64 => WasmType::F64,
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
            LoType::Void => {}
            LoType::Bool | LoType::U8 | LoType::I8 => byte_len = Some(1),
            LoType::U16 | LoType::I16 => byte_len = Some(2),
            LoType::U32 | LoType::I32 | LoType::F32 | LoType::Pointer(_) => byte_len = Some(4),
            LoType::U64 | LoType::I64 | LoType::F64 => byte_len = Some(8),
            LoType::Tuple(types) => {
                for lo_type in types {
                    lo_type.emit_sized_component_stats(ctx, stats, components)?;
                }
            }
            LoType::StructInstance { name } => {
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
            LoType::Void => 0,
            LoType::Tuple(types) => {
                let mut count = 0;
                for lo_type in types {
                    count += lo_type.emit_components(ctx, components);
                }
                count
            }
            LoType::StructInstance { name } => {
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
            LoType::Bool => return Ok(WasmLoadKind::I32),
            LoType::U8 => return Ok(WasmLoadKind::I32U8),
            LoType::I8 => return Ok(WasmLoadKind::I32I8),
            LoType::U16 => return Ok(WasmLoadKind::I32U16),
            LoType::I16 => return Ok(WasmLoadKind::I32I16),
            LoType::U32 => return Ok(WasmLoadKind::I32),
            LoType::I32 => return Ok(WasmLoadKind::I32),
            LoType::F32 => return Ok(WasmLoadKind::F32),
            LoType::U64 => return Ok(WasmLoadKind::I64),
            LoType::I64 => return Ok(WasmLoadKind::I64),
            LoType::F64 => return Ok(WasmLoadKind::F64),
            LoType::Pointer(_) => return Ok(WasmLoadKind::I32),
            _ => {}
        };
        return Err(format!("Unsupported type for load: {self:?}"));
    }
}

pub struct LocalDef {
    pub index: u32,
    pub value_type: LoType,
}

pub struct GlobalDef {
    pub index: u32,
    pub mutable: bool,
    pub value_type: LoType,
}

pub struct FnBody {
    pub fn_index: u32,
    pub type_index: u32,
    pub locals: BTreeMap<String, LocalDef>,
    pub locals_last_index: u32,
    pub body: LoTokenStream,
}

pub struct FnExport {
    pub in_name: String,
    pub out_name: String,
    pub loc: LoLocation,
}

#[derive(Clone)]
pub struct StructDef {
    pub fields: Vec<StructField>,
    pub fully_defined: bool, // used for self-reference checks
}

#[derive(Clone)]
pub struct StructField {
    pub name: String,
    pub value_type: LoType,
    pub field_index: u32,
    pub byte_offset: u32,
}

#[derive(Debug, Clone)]
pub struct FnDef {
    pub local: bool,
    pub fn_index: u32,
    pub type_index: u32,
    pub kind: LoFnType,
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
pub enum LoInstr {
    Unreachable,
    Drop {
        value: Box<LoInstr>,
        drop_count: u32,
    },
    BinaryOp {
        kind: WasmBinaryOpKind,
        lhs: Box<LoInstr>,
        rhs: Box<LoInstr>,
    },
    MemorySize,
    MemoryGrow {
        size: Box<LoInstr>,
    },
    Load {
        kind: LoType,
        align: u32,
        offset: u32,
        address_instr: Box<LoInstr>,
    },
    StructLoad {
        struct_name: String,
        address_instr: Box<LoInstr>,
        address_local_index: u32,
        base_byte_offset: u32,
        primitive_loads: Vec<LoInstr>,
    },
    LocalGet {
        local_index: u32,
        value_type: LoType,
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
        primitive_gets: Vec<LoInstr>,
    },
    U32ConstLazy {
        value: Rc<RefCell<u32>>,
    },
    U32Const {
        value: u32,
    },
    U64Const {
        value: u64,
    },
    I64Const {
        value: i64,
    },
    Set {
        bind: LoSetBind,
    },
    Return {
        value: Box<LoInstr>,
    },
    Block {
        block_type: LoType,
        body: Vec<LoInstr>,
    },
    Loop {
        block_type: LoType,
        body: Vec<LoInstr>,
    },
    If {
        block_type: LoType,
        cond: Box<LoInstr>,
        then_branch: Vec<LoInstr>,
        else_branch: Option<Vec<LoInstr>>,
    },
    Branch {
        label_index: u32,
    },
    Call {
        fn_index: u32,
        return_type: LoType,
        args: Vec<LoInstr>,
    },
    MultiValueEmit {
        values: Vec<LoInstr>,
    },
    Casted {
        value_type: LoType,
        expr: Box<LoInstr>,
    },
}

#[derive(Clone, Debug)]
pub enum LoSetBind {
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
        address_instr: Box<LoInstr>,
        value_local_index: u32,
    },
}

impl LoInstr {
    pub fn get_type(&self, ctx: &ModuleContext) -> LoType {
        match self {
            LoInstr::Unreachable => LoType::Void,
            LoInstr::U32ConstLazy { value: _ } => LoType::U32,
            LoInstr::U32Const { value: _ } => LoType::U32,
            LoInstr::U64Const { value: _ } => LoType::U64,
            LoInstr::I64Const { value: _ } => LoType::I64,
            LoInstr::UntypedLocalGet { local_index: _ } => unreachable!(),

            LoInstr::MultiValueEmit { values } => {
                let mut types = Vec::new();
                for value in values {
                    types.push(value.get_type(ctx));
                }
                LoType::Tuple(types)
            }
            LoInstr::StructLoad {
                struct_name,
                address_instr: _,
                address_local_index: _,
                base_byte_offset: _,
                primitive_loads: _,
            }
            | LoInstr::StructGet {
                struct_name,
                base_index: _,
                primitive_gets: _,
            } => LoType::StructInstance {
                name: struct_name.clone(),
            },

            // type-checked in the complier:
            LoInstr::Casted {
                value_type,
                expr: _,
            } => value_type.clone(),
            LoInstr::Set { bind: _ } => LoType::Void,
            LoInstr::Drop {
                value: _,
                drop_count: _,
            } => LoType::Void,
            LoInstr::Return { value: _ } => LoType::Void,
            LoInstr::MemorySize => LoType::I32,
            LoInstr::MemoryGrow { size: _ } => LoType::I32,

            LoInstr::BinaryOp {
                kind: _,
                lhs,
                rhs: _,
            } => lhs.get_type(ctx),
            LoInstr::Load {
                kind,
                align: _,
                offset: _,
                address_instr: _,
            } => kind.clone(),
            LoInstr::GlobalGet { global_index } => {
                let global_def = ctx
                    .globals
                    .values()
                    .find(|global| global.index == *global_index)
                    .unwrap();

                global_def.value_type.clone()
            }
            LoInstr::LocalGet {
                local_index: _,
                value_type,
            } => value_type.clone(),
            LoInstr::Call {
                return_type,
                fn_index: _,
                args: _,
            } => return_type.clone(),
            LoInstr::If {
                block_type,
                cond: _,
                then_branch: _,
                else_branch: _,
            }
            | LoInstr::Block {
                block_type,
                body: _,
            }
            | LoInstr::Loop {
                block_type,
                body: _,
            } => block_type.clone(),
            LoInstr::Branch { label_index: _ } => LoType::Void,
        }
    }
}

pub fn lower_exprs(out: &mut Vec<WasmInstr>, exprs: Vec<LoInstr>) {
    for expr in exprs.into_iter() {
        lower_expr(out, expr);
    }
}

pub fn lower_expr(out: &mut Vec<WasmInstr>, expr: LoInstr) {
    match expr {
        LoInstr::Unreachable => out.push(WasmInstr::Unreachable),
        LoInstr::Drop { value, drop_count } => {
            lower_expr(out, *value);
            for _ in 0..drop_count {
                out.push(WasmInstr::Drop);
            }
        }
        LoInstr::BinaryOp { kind, lhs, rhs } => {
            lower_expr(out, *lhs);
            lower_expr(out, *rhs);
            out.push(WasmInstr::BinaryOp { kind })
        }
        LoInstr::MemorySize => out.push(WasmInstr::MemorySize),
        LoInstr::MemoryGrow { size } => {
            lower_expr(out, *size);
            out.push(WasmInstr::MemoryGrow);
        }
        LoInstr::Load {
            kind,
            align,
            offset,
            address_instr,
        } => {
            lower_expr(out, *address_instr);
            out.push(WasmInstr::Load {
                kind: kind.to_load_kind().unwrap(),
                align,
                offset,
            });
        }
        LoInstr::StructLoad {
            struct_name: _,
            address_instr,
            address_local_index,
            base_byte_offset: _,
            primitive_loads,
        } => {
            lower_expr(out, *address_instr);
            out.push(WasmInstr::LocalSet {
                local_index: address_local_index,
            });
            lower_exprs(out, primitive_loads);
        }
        LoInstr::UntypedLocalGet { local_index }
        | LoInstr::LocalGet {
            local_index,
            value_type: _,
        } => out.push(WasmInstr::LocalGet { local_index }),
        LoInstr::GlobalGet { global_index } => out.push(WasmInstr::GlobalGet { global_index }),
        LoInstr::StructGet {
            struct_name: _,
            base_index: _,
            primitive_gets,
        } => {
            lower_exprs(out, primitive_gets);
        }
        LoInstr::U32ConstLazy { value } => out.push(WasmInstr::I32Const {
            value: *value.borrow() as i32,
        }),
        LoInstr::U32Const { value } => out.push(WasmInstr::I32Const {
            value: value as i32,
        }),
        LoInstr::I64Const { value } => out.push(WasmInstr::I64Const { value }),
        LoInstr::U64Const { value } => out.push(WasmInstr::I64Const {
            value: value as i64,
        }),
        LoInstr::Set { bind } => match bind {
            LoSetBind::Local { index } => out.push(WasmInstr::LocalSet { local_index: index }),
            LoSetBind::Global { index } => out.push(WasmInstr::GlobalSet {
                global_index: index,
            }),
            LoSetBind::Memory {
                align,
                offset,
                kind,
                address_instr,
                value_local_index,
            } => {
                out.push(WasmInstr::LocalSet {
                    local_index: value_local_index,
                });
                lower_expr(out, *address_instr);
                out.push(WasmInstr::LocalGet {
                    local_index: value_local_index,
                });
                out.push(WasmInstr::Store {
                    align,
                    offset,
                    kind,
                });
            }
        },
        LoInstr::Return { value } => {
            lower_expr(out, *value);
            out.push(WasmInstr::Return);
        }
        LoInstr::Block { block_type, body } => {
            out.push(WasmInstr::BlockStart {
                block_type: WasmBlockType::Block,
                return_type: block_type.to_wasm_type(),
            });
            lower_exprs(out, body);
            out.push(WasmInstr::BlockEnd);
        }
        LoInstr::Loop { block_type, body } => {
            out.push(WasmInstr::BlockStart {
                block_type: WasmBlockType::Loop,
                return_type: block_type.to_wasm_type(),
            });
            lower_exprs(out, body);
            out.push(WasmInstr::BlockEnd);
        }
        LoInstr::If {
            block_type,
            cond,
            then_branch,
            else_branch,
        } => {
            lower_expr(out, *cond);
            out.push(WasmInstr::BlockStart {
                block_type: WasmBlockType::If,
                return_type: block_type.to_wasm_type(),
            });
            lower_exprs(out, then_branch);
            if let Some(else_branch) = else_branch {
                out.push(WasmInstr::Else);
                lower_exprs(out, else_branch);
            }
            out.push(WasmInstr::BlockEnd);
        }
        LoInstr::Branch { label_index } => out.push(WasmInstr::Branch { label_index }),
        LoInstr::Call {
            fn_index,
            args,
            return_type: _,
        } => {
            for arg in args {
                lower_expr(out, arg);
            }
            out.push(WasmInstr::Call { fn_index });
        }
        LoInstr::MultiValueEmit { values } => {
            lower_exprs(out, values);
        }
        LoInstr::Casted {
            expr,
            value_type: _,
        } => {
            lower_expr(out, *expr);
        }
    }
}
