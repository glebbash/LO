#![allow(dead_code)] // TODO: remove

use crate::{ast::*, core::*, wasm::*};
use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Type {
    Never,
    Null,
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
    Pointer { pointee: Box<Type> },
    SequencePointer { pointee: Box<Type> },
    StructInstance { struct_index: usize },
    EnumInstance { enum_index: usize },
    Result(ResultType),
    Container(ContainerType),
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct ResultType {
    pub ok: Box<Type>,
    pub err: Box<Type>,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct ContainerType {
    pub container: Box<Type>,
    pub items: Vec<Type>,
}

impl Type {
    pub fn to_str(&self) -> Option<&'static str> {
        Some(match self {
            Type::Never => "never",
            Type::Null => "null",
            Type::Void => "void",
            Type::Bool => "bool",
            Type::U8 => "u8",
            Type::I8 => "i8",
            Type::U16 => "u16",
            Type::I16 => "i16",
            Type::U32 => "u32",
            Type::I32 => "i32",
            Type::F32 => "f32",
            Type::U64 => "u64",
            Type::I64 => "i64",
            Type::F64 => "f64",
            _ => return None,
        })
    }

    pub fn deref_rec(&self) -> &Type {
        match self {
            Type::Pointer { pointee } => pointee.deref_rec(),
            Type::SequencePointer { pointee } => pointee.deref_rec(),
            other => other,
        }
    }
}

#[derive(Clone)]
pub struct Symbol {
    pub scope_id: usize,
    pub name: &'static str,
    pub type_: SymbolType,
    pub col_index: usize,
    pub loc: Loc,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SymbolType {
    TypeAlias,
    Struct,
    Enum,

    Local,
    Global,
    Const,

    InlineFn,
    Function,
    EnumConstructor,
}

pub struct FnInfo {
    pub fn_name: &'static str,
    pub fn_type: FnType,
    pub fn_params: Vec<FnParameter>,
    pub fn_source: FnSource,
    pub exported_as: Vec<String>,
    pub wasm_fn_index: u32,
    pub definition_loc: Loc,
}

pub struct FnParameter {
    pub param_name: &'static str,
    pub param_type: Type,
    pub loc: Loc,
}

pub enum FnSource {
    Guest {
        module_index: usize,
        lo_fn_index: usize,
        body: &'static CodeBlock,
    },
    Host {
        module_name: String,
        external_fn_name: &'static str,
    },
}

pub struct FnType {
    pub inputs: Vec<Type>,
    pub output: Type,
}

#[derive(Clone)]
pub struct ExprContext {
    pub module_index: usize,
    pub fn_index: Option<usize>,
    pub locals: Vec<Local>,
    pub next_local_index: u32,
    pub addr_local_index: Option<u32>,
}

impl ExprContext {
    pub fn new(module_index: usize, fn_index: Option<usize>) -> Self {
        Self {
            module_index,
            fn_index,
            locals: Vec::new(),
            next_local_index: 0,
            addr_local_index: None,
        }
    }
}

#[derive(Clone)]
pub struct Local {
    pub local_index: u32,
    pub local_type: Type,
    pub definition_loc: Loc,
}

#[derive(Clone, PartialEq)]
pub enum ScopeType {
    Global,
    Function,
    Block,
    Loop,
    ForLoop,
    InlineFn,
}

impl Default for ScopeType {
    fn default() -> Self {
        ScopeType::Block
    }
}

#[derive(Clone)]
pub struct CodeUnit {
    pub type_: Type,
    pub instrs: Vec<WasmInstr>,
}

pub struct ConstSliceLen {
    pub slice_ptr: u32,
    pub slice_len: usize,
}

#[derive(Clone, Default)]
pub struct Scope {
    pub scope_id: usize,
    pub scope_type: ScopeType,
    pub symbols: Vec<Symbol>,
    pub deferred_exprs: Vec<CodeUnit>,
    pub inline_fn_call_loc: Option<Loc>,
}

impl Scope {
    pub fn new(scope_id: usize, scope_type: ScopeType) -> Self {
        Self {
            scope_id,
            scope_type,
            ..Default::default()
        }
    }

    pub fn get_symbol(&self, symbol_name: &str) -> Option<&Symbol> {
        for symbol in self.symbols.iter().rev() {
            if symbol.name == symbol_name {
                return Some(symbol);
            }
        }

        None
    }
}

pub struct StructDef {
    pub struct_name: &'static str,
    pub fields: Vec<StructField>,
    pub fully_defined: bool, // used for self-reference checks
}

pub struct StructField {
    pub field_name: &'static str,
    pub field_type: Type,
    pub field_layout: TypeLayout,
    pub field_index: u32,
    pub byte_offset: u32,
    pub loc: Loc,
}

pub struct EnumDef {
    pub enum_name: &'static str,
    pub variant_type: Type,
    pub variants: Vec<EnumVariant>,
}

pub struct EnumVariant {
    pub variant_name: &'static str,
    pub variant_type: Type,
    pub loc: Loc,
}

pub struct EnumConstructor {
    pub enum_index: usize,
    pub variant_index: usize,
}

pub struct GlobalDef {
    pub module_ctx: &'static ExprContext,
    pub def_expr: &'static LetExpr,
    pub global_type: Type,
    pub global_index: u32,
}

#[derive(Clone)]
pub struct ConstDef {
    pub const_name: &'static str,
    pub code_unit: CodeUnit,
    pub loc: Loc,
}

pub struct TypeLayout {
    pub primities_count: u32,
    pub byte_size: u32,
    pub alignment: u32,
}

impl TypeLayout {
    pub fn new() -> Self {
        Self {
            primities_count: 0,
            byte_size: 0,
            alignment: 0,
        }
    }
}

pub type TypeId = usize;

#[derive(Default)]
pub struct Registry {
    pub types: Vec<Type>,                 // indexed by `type_id`
    pub node_types: Vec<TypeId>,          // indexed by `node_id`
    pub symbols: Vec<Symbol>,             // indexed by `symbol_id`
    pub globals: Vec<GlobalDef>,          // indexed by `col_index` when `type_ = Global`
    pub constants: Vec<ConstDef>,         // indexed by `col_index` when `type_ = Const`
    pub functions: Vec<FnInfo>,           // indexed by `col_index` when `type_ = Function`
    pub inline_fns: Vec<&'static FnExpr>, // indexed by `col_index` when `type_ = InlineFn`
    pub type_aliases: Vec<Type>,          // indexed by `col_index` when `type_ = TypeAlias`
    pub structs: Vec<StructDef>,          // indexed by `col_index` when `type_ = Struct`
    pub enums: Vec<EnumDef>,              // indexed by `col_index` when `type_ = Enum`
    pub enum_ctors: Vec<EnumConstructor>, // indexed by `col_index` when `type_ = EnumConstructor`
}

pub struct TypeFmt<'a>(pub &'a Registry, pub &'a Type);

impl<'a> core::fmt::Display for TypeFmt<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.1 {
            Type::Never
            | Type::Null
            | Type::Void
            | Type::Bool
            | Type::U8
            | Type::I8
            | Type::U16
            | Type::I16
            | Type::U32
            | Type::I32
            | Type::F32
            | Type::U64
            | Type::I64
            | Type::F64 => write!(f, "{}", self.1.to_str().unwrap()),
            Type::Pointer { pointee } => write!(f, "&{}", TypeFmt(self.0, pointee)),
            Type::SequencePointer { pointee } => {
                write!(f, "*&{}", TypeFmt(self.0, pointee))
            }
            Type::StructInstance { struct_index } => {
                f.write_str(&self.0.structs[*struct_index].struct_name)
            }
            Type::EnumInstance { enum_index } => f.write_str(&self.0.enums[*enum_index].enum_name),
            Type::Result(result) => {
                write!(
                    f,
                    "Result({}, {})",
                    TypeFmt(self.0, &result.ok),
                    TypeFmt(self.0, &result.err)
                )
            }
            Type::Container(ContainerType { container, items }) => {
                write!(f, "{}", TypeFmt(self.0, container))?;
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", TypeFmt(self.0, item))?;
                }
                write!(f, ")")
            }
        }
    }
}

pub struct TypeListFmt<'a>(pub &'a Registry, pub &'a [Type]);

impl<'a> core::fmt::Display for TypeListFmt<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (item, i) in self.1.iter().zip(0..) {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", TypeFmt(self.0, item))?;
        }
        Ok(())
    }
}

pub struct CodeBlockContext {
    pub exprs: &'static [CodeExpr],
    pub node_id_offset: usize,
    pub symbol_id_offset: usize,
}

pub struct TypeChecker {
    pub registry: UBRef<Registry>,
    pub type_lookup: BTreeMap<Type, TypeId>,
}

impl TypeChecker {
    pub fn build_type_id(&mut self, type_: Type) -> TypeId {
        if let Some(&id) = self.type_lookup.get(&type_) {
            return id;
        }

        let id = self.registry.types.len();
        self.type_lookup.insert(type_.clone(), id);
        self.registry.types.push(type_);
        id
    }
}
