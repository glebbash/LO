use crate::{ast::*, common::*, lexer::*, parser::*, wasm::*};

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

pub struct Module {
    pub index: usize,
    pub source: &'static [u8],
    pub parser: Parser,
    pub includes: Vec<ModuleInclude>,
    pub scope_stack: Vec<Scope>,
    pub ctx: ExprContext,
}

pub struct ModuleInclude {
    pub module_index: usize,
    pub alias: Option<String>,
    pub with_extern: bool,
}

pub struct MemoryInfo {
    pub min_pages: Option<u32>,
    pub data_start: Option<u32>,
    pub exported: bool,
    pub imported_from: Option<String>,
    pub loc: Loc,
}

#[derive(Clone)]
pub struct PooledString {
    pub value: String,
    pub ptr: u32,
}

#[derive(Clone)]
pub struct Str {
    pub ptr: u32,
    pub len: u32,
}

pub enum VarInfo {
    Local(VarInfoLocal),
    Global(VarInfoGlobal),
    Const(VarInfoConst),
    Stored(VarInfoStored),
    StructValueField(VarInfoStructValueField),
}
pub struct VarInfoLocal {
    pub local_index: u32,
    pub var_type: Type,
    pub inspect_info: Option<InspectInfo>,
}

pub struct VarInfoGlobal {
    pub global_index: u32,
    pub var_type: Type,
    pub inspect_info: Option<InspectInfo>,
}

pub struct VarInfoConst {
    pub code_unit: &'static CodeUnit,
    pub inspect_info: Option<InspectInfo>,
    pub loc: Loc,
}

pub struct VarInfoStored {
    pub address: CodeUnit,
    pub offset: u32,
    pub var_type: Type,
    pub inspect_info: Option<InspectInfo>,
}

pub struct VarInfoStructValueField {
    pub struct_value: CodeUnit,
    pub drops_before: u32,
    pub drops_after: u32,
    pub var_type: Type,
    pub inspect_info: Option<InspectInfo>,
    pub loc: Loc,
}

impl VarInfo {
    pub fn get_type(&self) -> &Type {
        match self {
            VarInfo::Local(v) => &v.var_type,
            VarInfo::Global(v) => &v.var_type,
            VarInfo::Const(v) => &v.code_unit.type_,
            VarInfo::Stored(v) => &v.var_type,
            VarInfo::StructValueField(v) => &v.var_type,
        }
    }

    pub fn inspect_info(&self) -> &Option<InspectInfo> {
        match self {
            VarInfo::Local(v) => &v.inspect_info,
            VarInfo::Global(v) => &v.inspect_info,
            VarInfo::Const(v) => &v.inspect_info,
            VarInfo::Stored(v) => &v.inspect_info,
            VarInfo::StructValueField(v) => &v.inspect_info,
        }
    }
}

#[derive(Default)]
pub struct Registry {
    pub in_single_file_mode: bool,
    pub in_lex_only_mode: bool,
    pub should_emit_dbg_local_names: bool,

    pub fm: Box<FileManager>,
    pub reporter: Box<Reporter>,

    pub modules: Vec<Module>,             // indexed by `module_index`
    pub types: Vec<Type>,                 // indexed by `TypeId`
    pub expr_types: Vec<TypeId>,          // indexed by `expr.id()`
    pub symbols: Vec<Symbol>,             // indexed by `IdentExpr.symbol_id`
    pub globals: Vec<GlobalDef>,          // indexed by `col_index` when `type_ = Global`
    pub constants: Vec<ConstDef>,         // indexed by `col_index` when `type_ = Const`
    pub functions: Vec<FnInfo>,           // indexed by `col_index` when `type_ = Function`
    pub inline_fns: Vec<&'static FnExpr>, // indexed by `col_index` when `type_ = InlineFn`
    pub type_aliases: Vec<Type>,          // indexed by `col_index` when `type_ = TypeAlias`
    pub structs: Vec<StructDef>,          // indexed by `col_index` when `type_ = Struct`
    pub enums: Vec<EnumDef>,              // indexed by `col_index` when `type_ = Enum`
    pub enum_ctors: Vec<EnumConstructor>, // indexed by `col_index` when `type_ = EnumConstructor`

    pub const_slice_lens: Vec<ConstSliceLen>,
    pub allocated_strings: Vec<String>,
    pub memory: Option<MemoryInfo>,
    pub datas: UBCell<Vec<WasmData>>,
    pub data_size: UBCell<u32>,
    pub string_pool: UBCell<Vec<PooledString>>,
    pub first_string_usage: UBCell<Option<Loc>>,

    // TODO: remove this Typer is fully implemented
    pub get_expr_type_hits: UBCell<usize>,
    pub get_expr_type_cache_hits: UBCell<usize>,

    pub next_expr_id: usize,
    pub next_symbol_id: usize,
    pub next_scope_id: usize,
}

impl Registry {
    pub fn new() -> Self {
        let mut it = Self {
            next_scope_id: 1, // global scope id is 0
            ..Default::default()
        };
        it.reporter.fm = UBRef::new(&mut *it.fm);
        it
    }

    pub fn include_file(&mut self, relative_path: &str, loc: &Loc) -> Option<&Module> {
        let file_index = catch!(self.fm.include_file(relative_path, loc), err, {
            self.reporter.error(&err);
            return None;
        });

        let file_is_newly_added = self.fm.files[file_index].included_times == 1;

        if self.reporter.in_inspection_mode {
            if file_is_newly_added {
                let file_index = file_index;
                let file_path = &self.fm.files[file_index].absolute_path;
                stdout_writeln(format!(
                    "{{ \"type\": \"file\", \
                        \"index\": {file_index}, \
                        \"path\": \"{file_path}\" }},",
                ));
            }

            if loc.file_index != 0 {
                let source_index = loc.file_index;
                let source_range = RangeFmt(loc);
                let target_index = file_index;
                let target_range = "1:1-1:1";

                stdout_writeln(format!(
                    "{{ \"type\": \"info\", \
                        \"link\": \"{target_index}/{target_range}\", \
                        \"loc\": \"{source_index}/{source_range}\" }},",
                ));
            }
        }

        if !file_is_newly_added {
            return self.get_module_by_file_index(file_index);
        }

        let source = self.fm.files[file_index].source.as_bytes().relax();

        let mut lexer = Lexer::new(source, file_index);
        catch!(lexer.lex_file(), err, {
            self.reporter.error(&err);
            return None;
        });

        let parser = Parser::new(
            lexer,
            &mut self.reporter,
            self.next_expr_id,
            self.next_symbol_id,
        );
        if !self.in_lex_only_mode {
            catch!(parser.parse_file(), err, {
                self.reporter.error(&err);
                return None;
            });
            self.next_expr_id = *parser.next_expr_id;
            self.next_symbol_id = *parser.next_symbol_id;
        }

        let mut includes = Vec::new();

        if !self.in_single_file_mode {
            for expr in &*parser.ast {
                let TopLevelExpr::Intrinsic(InlineFnCallExpr {
                    id: _,
                    fn_name: intrinsic,
                    type_args,
                    args,
                    loc: _,
                }) = expr
                else {
                    continue;
                };
                if intrinsic.repr != "include" {
                    continue;
                }

                if type_args.len() != 0 {
                    self.reporter.error(&bad_signature(intrinsic));
                    continue;
                }

                let mut file_path = None;
                let mut alias = None;
                let mut with_extern = false;

                for arg in &args.items {
                    if let CodeExpr::StringLiteral(str) = arg {
                        file_path = Some(str.relax());
                        continue;
                    }

                    if let CodeExpr::Ident(ident) = arg
                        && ident.repr == "with_extern"
                    {
                        with_extern = true;
                    }

                    if let CodeExpr::Assign(AssignExpr { lhs, rhs, .. }) = arg
                        && let CodeExpr::Ident(IdentExpr { repr: "alias", .. }) = &**lhs
                        && let CodeExpr::StringLiteral(str) = &**rhs
                    {
                        alias = Some(str.value.clone())
                    }
                }

                let Some(file_path) = file_path else {
                    continue;
                };

                let module_index;
                match self.include_file(&file_path.value, &file_path.loc) {
                    Some(module) => module_index = module.index,
                    None => continue,
                }

                includes.push(ModuleInclude {
                    module_index,
                    alias,
                    with_extern,
                });

                fn bad_signature(fn_name: &IdentExpr) -> Error {
                    Error {
                        message: format!(
                            "Invalid call, expected signature: @{}(<str-literal>, [with_extern, alias = <str-literal>])",
                            fn_name.repr
                        ),
                        loc: fn_name.loc,
                    }
                }
            }
        }

        self.modules.push(Module {
            index: self.modules.len(),
            source: parser.source,
            scope_stack: Vec::new(),
            parser,
            ctx: ExprContext::new(self.modules.len(), None),
            includes,
        });

        let module = self.modules.relax_mut().last_mut().unwrap();

        Some(module)
    }

    pub fn process_deferred_intrinsics(&mut self) {
        for module in &self.modules {
            for expr in &*module.parser.ast {
                if let TopLevelExpr::Intrinsic(intrinsic) = expr
                    && intrinsic.fn_name.repr == "inspect_stats"
                {
                    if !self.reporter.in_inspection_mode {
                        return;
                    }

                    let mut lines = 0;
                    for file in &self.reporter.fm.files {
                        lines += file
                            .source
                            .as_bytes()
                            .iter()
                            .filter(|&&b| b == b'\n')
                            .count()
                    }

                    let mut msg = String::new();
                    write!(&mut msg, "LOC: {}\n", lines).unwrap();
                    write!(&mut msg, "symbol count: {}\n", self.next_symbol_id).unwrap();
                    write!(&mut msg, "expr count: {}\n", self.next_expr_id).unwrap();
                    write!(
                        &mut msg,
                        "get_expr_type_hits: {}\n",
                        *self.get_expr_type_hits
                    )
                    .unwrap();
                    write!(
                        &mut msg,
                        "get_expr_type_cache_hits: {}\n",
                        *self.get_expr_type_cache_hits
                    )
                    .unwrap();
                    write!(&mut msg, "types: {}\n", TypeListFmt(self, &self.types)).unwrap();

                    self.reporter.print_inspection(&InspectInfo {
                        message: msg,
                        loc: intrinsic.fn_name.loc,
                        linked_loc: None,
                    });
                }
            }
        }
    }

    pub fn define_symbol(
        &mut self,
        ctx: &ExprContext,
        symbol_name: &'static str,
        symbol_type: SymbolType,
        symbol_col_index: usize,
        symbol_loc: Loc,
    ) -> Result<&Symbol, &Symbol> {
        let current_scope = self.relax_mut().current_scope_mut(ctx);

        if let Some(existing_symbol) = current_scope.relax().get_symbol(symbol_name)
            && existing_symbol.scope_id == current_scope.scope_id
        {
            self.reporter.error(&Error {
                message: format!(
                    "Cannot redefine {}, previously defined at {}",
                    symbol_name,
                    existing_symbol.loc.to_string(&self.reporter.fm)
                ),
                loc: symbol_loc,
            });
            return Err(&existing_symbol);
        }

        current_scope.symbols.push(Symbol {
            scope_id: current_scope.scope_id,
            name: symbol_name,
            type_: symbol_type,
            col_index: symbol_col_index,
            loc: symbol_loc,
        });
        Ok(&current_scope.symbols.last().unwrap())
    }

    pub fn enter_scope(&mut self, ctx: &ExprContext, scope_type: ScopeType) {
        let module = &mut self.relax_mut().modules[ctx.module_index];

        self.init_scope_from_parent_and_push(&mut module.scope_stack, scope_type);
    }

    pub fn exit_scope(&mut self, ctx: &ExprContext) -> Scope {
        let module = &mut self.modules[ctx.module_index];

        module.scope_stack.pop().unwrap()
    }

    pub fn current_scope(&self, ctx: &ExprContext) -> &Scope {
        let module = &self.modules[ctx.module_index];

        module.scope_stack.last().unwrap()
    }

    pub fn current_scope_mut(&mut self, ctx: &ExprContext) -> &mut Scope {
        let module = &mut self.modules[ctx.module_index];

        module.scope_stack.last_mut().unwrap()
    }

    pub fn init_scope_from_parent_and_push(
        &mut self,
        scope_stack: &mut Vec<Scope>,
        scope_type: ScopeType,
    ) {
        let scope_id = self.next_scope_id;
        self.next_scope_id += 1;

        let mut new_scope = Scope::new(scope_id, scope_type);
        if let Some(parent) = scope_stack.last() {
            new_scope.symbols.extend_from_slice(&parent.symbols);
        };
        scope_stack.push(new_scope);
    }

    pub fn is_64_bit_int_tag(&self, tag_type: &Type, loc: &Loc) -> Result<bool, Error> {
        match tag_type {
            Type::U64 | Type::I64 => Ok(true),
            Type::U8 | Type::I8 | Type::U16 | Type::I16 | Type::U32 | Type::I32 => Ok(false),
            other => Err(Error {
                message: format!("{} is not a valid int tag", TypeFmt(self, other)),
                loc: *loc,
            }),
        }
    }

    pub fn is_types_compatible(&self, slots: &Vec<Type>, values: &Vec<Type>) -> bool {
        if slots.len() != values.len() {
            return false;
        }

        for i in 0..slots.len() {
            if !self.is_type_compatible(&slots[i], &values[i]) {
                return false;
            }
        }

        true
    }

    pub fn is_type_compatible(&self, slot: &Type, value: &Type) -> bool {
        if let Type::Pointer { pointee } = slot {
            if *value == Type::Null {
                return true;
            }

            if **pointee == Type::Void {
                if let Type::Pointer { pointee: _ } = value {
                    return true;
                }

                if let Type::SequencePointer { pointee: _ } = value {
                    return true;
                }
            }

            if let Type::Pointer {
                pointee: value_pointee,
            } = value
            {
                return self.is_type_compatible(pointee, value_pointee);
            }
        }

        if let Type::Container(ContainerType { container, items }) = value {
            if let Type::Container(ContainerType {
                container: slot_container,
                items: slot_items,
            }) = slot
            {
                return self.is_type_compatible(slot_container, container)
                    && self.is_types_compatible(slot_items, items);
            }

            // TODO: allow this for self arguments only
            return self.is_type_compatible(slot, container);
        }

        if *value == Type::Never {
            return true;
        }

        slot == value
    }

    pub fn lower_type(&self, lo_type: &Type, wasm_types: &mut Vec<WasmType>) {
        match lo_type {
            Type::Never | Type::Void => {}
            Type::Null
            | Type::Bool
            | Type::U8
            | Type::I8
            | Type::U16
            | Type::I16
            | Type::U32
            | Type::I32 => wasm_types.push(WasmType::I32),
            Type::F32 => wasm_types.push(WasmType::F32),
            Type::U64 => wasm_types.push(WasmType::I64),
            Type::I64 => wasm_types.push(WasmType::I64),
            Type::F64 => wasm_types.push(WasmType::F64),
            Type::Pointer { pointee: _ } => wasm_types.push(WasmType::I32),
            Type::SequencePointer { pointee: _ } => wasm_types.push(WasmType::I32),
            Type::StructInstance { struct_index } => {
                let struct_def = &self.structs[*struct_index];

                for field in &struct_def.fields {
                    self.lower_type(&field.field_type, wasm_types);
                }
            }
            Type::EnumInstance { enum_index } => {
                let enum_def = &self.enums[*enum_index];

                self.lower_type(&Type::U32, wasm_types);
                self.lower_type(&enum_def.variant_type, wasm_types);
            }
            Type::Result(result) => {
                self.lower_type(&result.ok, wasm_types);
                self.lower_type(&result.err, wasm_types);
            }
            Type::Container(ContainerType {
                container,
                items: _,
            }) => {
                self.lower_type(container, wasm_types);
            }
        }
    }

    pub fn count_wasm_type_components(&self, lo_type: &Type) -> u32 {
        let layout = &mut TypeLayout::new();
        self.get_type_layout(lo_type, layout);
        layout.primities_count
    }

    pub fn get_type_layout(&self, lo_type: &Type, layout: &mut TypeLayout) {
        match lo_type {
            Type::Never | Type::Void => {
                layout.alignment = u32::max(layout.alignment, 1);
            }
            Type::Bool | Type::U8 | Type::I8 => {
                layout.primities_count += 1;
                layout.alignment = u32::max(layout.alignment, 1);
                layout.byte_size = align(layout.byte_size, 1) + 1;
            }
            Type::U16 | Type::I16 => {
                layout.primities_count += 1;
                layout.alignment = u32::max(layout.alignment, 2);
                layout.byte_size = align(layout.byte_size, 2) + 2;
            }
            Type::U32
            | Type::I32
            | Type::F32
            | Type::Null
            | Type::Pointer { pointee: _ }
            | Type::SequencePointer { pointee: _ } => {
                layout.primities_count += 1;
                layout.alignment = u32::max(layout.alignment, 4);
                layout.byte_size = align(layout.byte_size, 4) + 4;
            }
            Type::U64 | Type::I64 | Type::F64 => {
                layout.primities_count += 1;
                layout.alignment = u32::max(layout.alignment, 8);
                layout.byte_size = align(layout.byte_size, 8) + 8;
            }
            Type::StructInstance { struct_index } => {
                let struct_def = &self.structs[*struct_index];

                // append each field's layout to total struct layout
                for field in &struct_def.fields {
                    self.get_type_layout(&field.field_type, layout);
                }

                layout.alignment = u32::max(layout.alignment, 1);
                layout.byte_size = align(layout.byte_size, layout.alignment);
            }
            Type::EnumInstance { enum_index } => {
                let enum_def = &self.enums[*enum_index];

                self.get_type_layout(&Type::U32, layout);
                self.get_type_layout(&enum_def.variant_type, layout);

                layout.byte_size = align(layout.byte_size, layout.alignment);
            }
            Type::Result(result) => {
                self.get_type_layout(&result.ok, layout);
                self.get_type_layout(&result.err, layout);

                layout.byte_size = align(layout.byte_size, layout.alignment);
            }
            Type::Container(ContainerType {
                container,
                items: _,
            }) => {
                self.get_type_layout(container, layout);
            }
        }
    }

    pub fn get_cast_instr(&self, casted_from: &Type, casted_to: &Type) -> Option<WasmInstr> {
        if *casted_to == Type::I64 || *casted_to == Type::U64 {
            if *casted_from == Type::I8 || *casted_from == Type::I16 || *casted_from == Type::I32 {
                return Some(WasmInstr::I64ExtendI32s);
            }

            if *casted_from == Type::U8 || *casted_from == Type::U16 || *casted_from == Type::U32 {
                return Some(WasmInstr::I64ExtendI32u);
            }
        }

        if *casted_to == Type::I8
            || *casted_to == Type::U8
            || *casted_to == Type::I16
            || *casted_to == Type::U16
            || *casted_to == Type::I32
            || *casted_to == Type::U32
        {
            if *casted_from == Type::I64 || *casted_from == Type::U64 {
                return Some(WasmInstr::I32WrapI64);
            }
        }

        None
    }

    pub fn build_type(&self, ctx: &ExprContext, type_expr: &TypeExpr) -> Result<Type, Error> {
        return self.build_type_check_ref(ctx, type_expr, true, &Loc::internal());
    }

    // builds a type, asserting that it doesn't have infinite size
    pub fn build_type_check_ref(
        &self,
        ctx: &ExprContext,
        type_expr: &TypeExpr,
        is_referenced: bool,
        loc: &Loc,
    ) -> Result<Type, Error> {
        match type_expr {
            TypeExpr::Named(TypeExprNamed { name }) => {
                let lo_type = self.get_type_or_err(ctx, &name.repr, &name.loc)?;
                if let Type::StructInstance { struct_index } = &lo_type {
                    let struct_def = &self.structs[*struct_index];
                    if !is_referenced && !struct_def.fully_defined {
                        return Err(Error {
                            message: format!(
                                "Cannot use partially defined struct '{}' here",
                                struct_def.struct_name
                            ),
                            loc: *loc,
                        });
                    }
                }
                Ok(lo_type)
            }
            TypeExpr::Pointer(TypeExprPointer { pointee, loc: _ }) => {
                let pointee = Box::new(self.build_type_check_ref(ctx, &pointee, true, loc)?);

                Ok(Type::Pointer { pointee })
            }
            TypeExpr::SequencePointer(TypeExprSequencePointer { pointee, loc: _ }) => {
                let pointee = Box::new(self.build_type_check_ref(ctx, &pointee, true, loc)?);

                Ok(Type::SequencePointer { pointee })
            }
            TypeExpr::Container(TypeExprContainer {
                container,
                items,
                loc: _,
            }) => {
                if let TypeExpr::Named(ident) = &**container
                    && ident.name.repr == "Result"
                {
                    if items.len() != 2 {
                        return Err(Error {
                            message: format!(
                                "Expected exactly 2 type arguments, {} was found",
                                items.len()
                            ),
                            loc: ident.name.loc,
                        });
                    }

                    let ok = Box::new(self.build_type_check_ref(ctx, &items[0], false, loc)?);
                    let err = Box::new(self.build_type_check_ref(ctx, &items[1], false, loc)?);

                    return Ok(Type::Result(ResultType { ok, err }));
                }

                if let TypeExpr::Named(named) = &**container
                    && named.name.repr == "typeof"
                {
                    if items.len() != 1 {
                        return Err(Error {
                            message: format!(
                                "Expected exactly 1 type arguments, {} was found",
                                items.len()
                            ),
                            loc: named.name.loc,
                        });
                    }

                    let TypeExpr::Named(named) = &items[0] else {
                        return Err(Error {
                            message: format!("Symbol expected"),
                            loc: *items[0].loc(),
                        });
                    };

                    let symbol = self.current_scope(ctx).get_symbol(named.name.repr);
                    let Some(symbol) = symbol else {
                        return Err(Error {
                            message: format!("Unknown symbol"),
                            loc: *items[0].loc(),
                        });
                    };

                    let SymbolType::Const = symbol.type_ else {
                        return Err(Error {
                            message: format!("Expected const, got {:?}", symbol.type_),
                            loc: *items[0].loc(),
                        });
                    };

                    return Ok(self.constants[symbol.col_index].code_unit.type_.clone());
                }

                if let TypeExpr::Named(named) = &**container
                    && named.name.repr == "itemof"
                {
                    if items.len() != 1 {
                        return Err(Error {
                            message: format!(
                                "Expected exactly 1 type arguments, {} was found",
                                items.len()
                            ),
                            loc: named.name.loc,
                        });
                    }

                    let container = self.build_type_check_ref(ctx, &items[0], true, loc)?;
                    let container = container.deref_rec();

                    let Type::Container(ContainerType {
                        container: _,
                        items,
                    }) = container
                    else {
                        return Err(Error {
                            message: format!("Expected container type"),
                            loc: *items[0].loc(),
                        });
                    };

                    return Ok(items[0].clone());
                }

                let container = self.build_type_check_ref(ctx, container, is_referenced, loc)?;

                let mut type_items = Vec::new();
                for item in items {
                    type_items.push(self.build_type_check_ref(ctx, item, true, loc)?);
                }

                Ok(Type::Container(ContainerType {
                    container: Box::new(container),
                    items: type_items,
                }))
            }
        }
    }

    pub fn get_type_or_err(
        &self,
        ctx: &ExprContext,
        type_name: &str,
        loc: &Loc,
    ) -> Result<Type, Error> {
        let Some(symbol) = self.current_scope(ctx).get_symbol(type_name) else {
            return Err(Error {
                message: format!("Unknown type: {}", type_name),
                loc: *loc,
            });
        };

        match symbol.type_ {
            SymbolType::Struct => {
                if self.reporter.in_inspection_mode {
                    self.reporter.print_inspection(&InspectInfo {
                        message: format!("struct {type_name} {{ ... }}"),
                        loc: *loc,
                        linked_loc: Some(symbol.loc),
                    });
                }

                Ok(Type::StructInstance {
                    struct_index: symbol.col_index,
                })
            }
            SymbolType::Enum => {
                if self.reporter.in_inspection_mode {
                    self.reporter.print_inspection(&InspectInfo {
                        message: format!("enum {type_name} {{ ... }}"),
                        loc: *loc,
                        linked_loc: Some(symbol.loc),
                    });
                }

                Ok(Type::EnumInstance {
                    enum_index: symbol.col_index,
                })
            }
            SymbolType::TypeAlias => {
                let type_ = &self.type_aliases[symbol.col_index];

                // don't print inspection for built-ins
                if self.reporter.in_inspection_mode && symbol.loc.file_index != 0 {
                    self.reporter.print_inspection(&InspectInfo {
                        message: format!("type {type_name} = {}", TypeFmt(self, &type_)),
                        loc: *loc,
                        linked_loc: Some(symbol.loc),
                    });
                }

                Ok(type_.clone())
            }
            SymbolType::Local
            | SymbolType::Global
            | SymbolType::Const
            | SymbolType::Function
            | SymbolType::InlineFn
            | SymbolType::EnumConstructor => Err(Error {
                message: format!("Symbol is not a type: {}", type_name),
                loc: *loc,
            }),
        }
    }

    pub fn get_fn_info_for_call(
        &self,
        ctx: &ExprContext,
        fn_name: &str,
        loc: &Loc,
    ) -> Result<&FnInfo, Error> {
        let Some(symbol) = self.current_scope(ctx).get_symbol(fn_name) else {
            return Err(Error {
                message: format!("Unknown function: {}", fn_name),
                loc: *loc,
            });
        };

        let SymbolType::Function = symbol.type_ else {
            return Err(Error {
                message: format!(
                    "Trying to call {} which is not a function, defined at: {}",
                    fn_name,
                    symbol.loc.to_string(&self.reporter.fm)
                ),
                loc: *loc,
            });
        };

        Ok(&self.functions[symbol.col_index])
    }

    pub fn get_fn_self_type(
        &self,
        ctx: &ExprContext,
        fn_name: &IdentExpr,
        fn_params: &Vec<FnParam>,
    ) -> Option<Type> {
        let mut has_self_param = false;
        for fn_param in fn_params {
            let (FnParamType::Self_ | FnParamType::SelfRef) = fn_param.param_type else {
                continue;
            };

            has_self_param = true;

            if fn_name.parts.len() == 1 {
                self.reporter.error(&Error {
                    message: format!("Cannot use self param in non-method function"),
                    loc: fn_param.loc,
                });
                return Some(Type::Never);
            }
        }
        if !has_self_param {
            return None;
        }

        let mut module = &self.modules[ctx.module_index];
        if fn_name.loc.file_index != module.parser.lexer.file_index {
            // fn imported from other module
            module = &self
                .get_module_by_file_index(fn_name.loc.file_index)
                .unwrap();
        }

        let mut self_type_loc = fn_name.parts[0];
        self_type_loc.end_pos = fn_name.parts[fn_name.parts.len() - 2].end_pos;

        let self_type_name = self_type_loc.read_span(module.source);

        let self_type = catch!(
            self.get_type_or_err(ctx, &self_type_name, &self_type_loc),
            err,
            {
                self.reporter.error(&err);
                return Some(Type::Never);
            }
        );

        Some(self_type)
    }

    pub fn get_fn_param_type(
        &self,
        ctx: &ExprContext,
        fn_param: &FnParam,
        self_type: &Option<Type>,
        infer_allowed: bool,
    ) -> Result<Type, Error> {
        match &fn_param.param_type {
            FnParamType::Self_ | FnParamType::SelfRef => {
                // SAFETY: `get_fn_self_type` does the check
                let self_type = self_type.clone().unwrap();

                if let FnParamType::Self_ = fn_param.param_type {
                    return Ok(self_type);
                }

                return Ok(Type::Pointer {
                    pointee: Box::new(self_type),
                });
            }
            FnParamType::Type { expr } => {
                if let Some(infer_type_name) = self.get_infer_type_name(fn_param)? {
                    if !infer_allowed {
                        return Err(Error {
                            message: format!("Infer is only allowed in inline fns"),
                            loc: fn_param.param_name.loc,
                        });
                    }

                    return self.get_type_or_err(ctx, infer_type_name, &fn_param.param_name.loc);
                }

                self.build_type(ctx, &expr)
            }
        }
    }

    pub fn get_infer_type_name(&self, fn_param: &FnParam) -> Result<Option<&'static str>, Error> {
        let FnParamType::Type {
            expr: TypeExpr::Container(container),
        } = &fn_param.param_type
        else {
            return Ok(None);
        };

        let TypeExpr::Named(named) = &*container.container else {
            return Ok(None);
        };

        if named.name.repr != "infer" {
            return Ok(None);
        }

        if container.items.len() != 1 {
            return Err(Error {
                message: format!("Invalid `infer` call, expected 1 named type argument"),
                loc: container.loc,
            });
        }

        let TypeExpr::Named(named) = &container.items[0] else {
            return Err(Error {
                message: format!("Invalid `infer` call, expected 1 named type argument"),
                loc: container.loc,
            });
        };

        Ok(Some(named.name.repr))
    }

    pub fn get_expr_type(&self, ctx: &ExprContext, expr: &CodeExpr) -> Result<Type, Error> {
        *self.get_expr_type_hits.be_mut() += 1;

        let expr_type_id = self.expr_types[expr.id()];
        if expr_type_id != 0 {
            *self.get_expr_type_cache_hits.be_mut() += 1;

            return Ok(self.types[expr_type_id].clone());
        }

        match expr {
            CodeExpr::BoolLiteral(_) => Ok(Type::Bool),
            CodeExpr::CharLiteral(_) => Ok(Type::U8),
            CodeExpr::NullLiteral(_) => Ok(Type::Null),
            CodeExpr::IntLiteral(IntLiteralExpr {
                id: _,
                repr: _,
                value: _,
                tag,
                loc,
            }) => {
                let Some(tag) = tag else { return Ok(Type::U32) };

                let tag_type = catch!(self.get_type_or_err(ctx, tag, loc), err, {
                    self.reporter.error(&err);
                    return Ok(Type::U32);
                });

                catch!(self.is_64_bit_int_tag(&tag_type, loc), err, {
                    self.reporter.error(&err);
                    return Ok(Type::U32);
                });

                Ok(tag_type)
            }
            CodeExpr::StringLiteral(StringLiteralExpr {
                id: _,
                repr: _,
                value: _,
                loc,
            }) => {
                let Some(symbol) = self.current_scope(ctx).get_symbol("str") else {
                    return Err(Error {
                        message: format!("Cannot use strings with no `str` struct defined"),
                        loc: *loc,
                    });
                };

                Ok(Type::StructInstance {
                    struct_index: symbol.col_index,
                })
            }
            CodeExpr::StructLiteral(StructLiteralExpr {
                id: _,
                struct_name,
                body: _,
                loc,
            }) => {
                let Some(symbol) = self.current_scope(ctx).get_symbol(&struct_name.repr) else {
                    return Err(Error {
                        message: format!("Unknown struct: {}", struct_name.repr),
                        loc: *loc,
                    });
                };

                return Ok(Type::StructInstance {
                    struct_index: symbol.col_index,
                });
            }
            CodeExpr::ArrayLiteral(ArrayLiteralExpr {
                id: _,
                item_type,
                items: _,
                has_trailing_comma: _,
                loc: _,
            }) => {
                let item_type = self.build_type(ctx, item_type)?;
                return Ok(Type::SequencePointer {
                    pointee: Box::new(item_type),
                });
            }
            CodeExpr::ResultLiteral(ResultLiteralExpr {
                id: _,
                is_ok: _,
                result_type,
                value: _,
                loc,
            }) => {
                let result = self.get_result_literal_type(ctx, result_type, loc)?;
                return Ok(Type::Result(result));
            }
            CodeExpr::Ident(ident) => {
                let Some(symbol) = self.current_scope(ctx).get_symbol(ident.repr) else {
                    return Err(Error {
                        message: format!("Unknown variable: {}", ident.repr),
                        loc: ident.loc,
                    });
                };

                match symbol.type_ {
                    SymbolType::Local => {
                        let local = &ctx.locals[symbol.col_index];

                        Ok(local.local_type.clone())
                    }
                    SymbolType::Global => {
                        let global = &self.globals[symbol.col_index];

                        Ok(global.global_type.clone())
                    }
                    SymbolType::Const => {
                        let const_def = &self.constants[symbol.col_index];

                        Ok(const_def.code_unit.type_.clone())
                    }
                    SymbolType::TypeAlias
                    | SymbolType::Struct
                    | SymbolType::Enum
                    | SymbolType::InlineFn
                    | SymbolType::Function
                    | SymbolType::EnumConstructor => Err(Error {
                        message: format!(
                            "Expected variable, found {:?} '{}'",
                            symbol.type_, ident.repr
                        ),
                        loc: ident.loc,
                    }),
                }
            }
            CodeExpr::InfixOp(InfixOpExpr {
                id: _,
                op_tag,
                op_loc: _,
                lhs,
                rhs: _,
                loc: _,
            }) => match op_tag {
                InfixOpTag::Equal
                | InfixOpTag::NotEqual
                | InfixOpTag::Less
                | InfixOpTag::Greater
                | InfixOpTag::LessEqual
                | InfixOpTag::GreaterEqual
                | InfixOpTag::And
                | InfixOpTag::Or => Ok(Type::Bool),

                InfixOpTag::Add
                | InfixOpTag::Sub
                | InfixOpTag::Mul
                | InfixOpTag::Div
                | InfixOpTag::Mod
                | InfixOpTag::BitAnd
                | InfixOpTag::BitOr
                | InfixOpTag::ShiftLeft
                | InfixOpTag::ShiftRight => Ok(self.get_expr_type(ctx, lhs)?),

                InfixOpTag::AddAssign
                | InfixOpTag::SubAssign
                | InfixOpTag::MulAssign
                | InfixOpTag::DivAssign
                | InfixOpTag::ModAssign
                | InfixOpTag::BitAndAssign
                | InfixOpTag::BitOrAssign
                | InfixOpTag::ShiftLeftAssign
                | InfixOpTag::ShiftRightAssign => Ok(Type::Void),

                // have their own CodeExpr variants
                InfixOpTag::Cast
                | InfixOpTag::Assign
                | InfixOpTag::FieldAccess
                | InfixOpTag::Catch
                | InfixOpTag::ErrorPropagation
                | InfixOpTag::Pipe => unreachable!(),
            },
            CodeExpr::PrefixOp(PrefixOpExpr {
                id: _,
                op_tag,
                expr,
                op_loc: _,
                loc,
            }) => match op_tag {
                PrefixOpTag::Not => Ok(Type::Bool),
                PrefixOpTag::Reference => {
                    let expr_type = self.get_expr_type(ctx, expr)?;
                    Ok(Type::Pointer {
                        pointee: Box::new(expr_type),
                    })
                }
                PrefixOpTag::Dereference => {
                    let expr_type = self.get_expr_type(ctx, expr)?;
                    let (Type::Pointer { pointee } | Type::SequencePointer { pointee }) = expr_type
                    else {
                        return Err(Error {
                            message: format!(
                                "Cannot dereference expr of type {}",
                                TypeFmt(self, &expr_type)
                            ),
                            loc: *loc,
                        });
                    };
                    Ok(*pointee)
                }
                PrefixOpTag::Positive | PrefixOpTag::Negative => {
                    let expr_type = self.get_expr_type(ctx, expr)?;

                    match expr_type {
                        Type::U8 => Ok(Type::I8),
                        Type::U16 => Ok(Type::I16),
                        Type::U32 => Ok(Type::I32),
                        Type::U64 => Ok(Type::I64),
                        Type::Never
                        | Type::Null
                        | Type::Void
                        | Type::Bool
                        | Type::I8
                        | Type::I16
                        | Type::I32
                        | Type::F32
                        | Type::I64
                        | Type::F64
                        | Type::Pointer { pointee: _ }
                        | Type::SequencePointer { pointee: _ }
                        | Type::StructInstance { struct_index: _ }
                        | Type::EnumInstance { enum_index: _ }
                        | Type::Result(_) => Ok(expr_type),
                        Type::Container(_) => Ok(expr_type),
                    }
                }
            },
            CodeExpr::Cast(CastExpr {
                id: _,
                expr: _,
                casted_to,
                loc: _,
            }) => self.build_type(ctx, casted_to),
            CodeExpr::FieldAccess(field_access) => {
                let lhs_type = self.get_expr_type(ctx, &field_access.lhs)?;
                let field = self.get_struct_or_struct_ref_field(&lhs_type, field_access)?;
                Ok(field.field_type.clone())
            }
            CodeExpr::FnCall(FnCallExpr {
                id: _,
                fn_name,
                args: _,
                loc: _,
            }) => {
                if let Some(symbol) = self.current_scope(ctx).get_symbol(&fn_name.repr) {
                    if let SymbolType::EnumConstructor = symbol.type_ {
                        let ctor = &self.enum_ctors[symbol.col_index];

                        return Ok(Type::EnumInstance {
                            enum_index: ctor.enum_index,
                        });
                    }
                }

                let fn_info = self.get_fn_info_for_call(ctx, &fn_name.repr, &fn_name.loc)?;

                Ok(fn_info.fn_type.output.clone())
            }
            CodeExpr::MethodCall(MethodCallExpr {
                id: _,
                lhs,
                field_name,
                args: _,
                loc: _,
            }) => {
                let lhs_type = self.get_expr_type(ctx, lhs)?;
                let fn_name = self.get_fn_name_from_method(&lhs_type, &field_name.repr);

                let fn_info = self.get_fn_info_for_call(ctx, &fn_name, &field_name.loc)?;

                Ok(fn_info.fn_type.output.clone())
            }
            CodeExpr::InlineFnCall(InlineFnCallExpr {
                id: _,
                fn_name,
                type_args,
                args,
                loc,
            }) => {
                self.be_mut().enter_scope(ctx, ScopeType::InlineFn);
                let expr_type = self.get_inline_fn_return_type(
                    ctx.be_mut(),
                    &fn_name.repr,
                    type_args,
                    &args.items,
                    None,
                    loc,
                );
                self.be_mut().exit_scope(ctx);
                expr_type
            }
            CodeExpr::IntrinsicCall(InlineFnCallExpr {
                id: _,
                fn_name,
                type_args: _,
                args,
                loc: _,
            }) => {
                if fn_name.repr == "unreachable" {
                    return Ok(Type::Never);
                }

                if fn_name.repr == "memory_size" {
                    return Ok(Type::I32);
                }

                if fn_name.repr == "memory_grow" {
                    return Ok(Type::I32);
                }

                if fn_name.repr == "memory_copy" {
                    return Ok(Type::Void);
                }

                if fn_name.repr == "data_size" {
                    return Ok(Type::U32);
                }

                if fn_name.repr == "embed_file" {
                    return Ok(Type::SequencePointer {
                        pointee: Box::new(Type::U8),
                    });
                }

                if fn_name.repr == "const_slice_len" {
                    return Ok(Type::U32);
                }

                if fn_name.repr == "inline_fn_call_loc" {
                    return Ok(Type::StructInstance {
                        struct_index: self.current_scope(ctx).get_symbol("str").unwrap().col_index,
                    });
                }

                if fn_name.repr == "get_ok" {
                    if args.items.len() == 1 {
                        let arg_type = self.get_expr_type(ctx, &args.items[0])?;

                        if let Type::Result(ResultType { ok, err: _ }) = arg_type {
                            return Ok(*ok.clone());
                        };
                    }

                    return Err(Error {
                        message: format!(
                            "Invalid arguments for @{}(items: Result(T, E)): T",
                            fn_name.repr,
                        ),
                        loc: fn_name.loc,
                    });
                }

                if fn_name.repr == "get_err" {
                    if args.items.len() == 1 {
                        let arg_type = self.get_expr_type(ctx, &args.items[0])?;

                        if let Type::Result(ResultType { ok: _, err }) = arg_type {
                            return Ok(*err.clone());
                        };
                    }

                    return Err(Error {
                        message: format!(
                            "Invalid arguments for @{}(items: Result(T, E)): E",
                            fn_name.repr,
                        ),
                        loc: fn_name.loc,
                    });
                }

                if fn_name.repr.starts_with("inspect_") {
                    return Ok(Type::Void);
                }

                Err(Error {
                    message: format!("Unknown intrinsic: {}", fn_name.repr),
                    loc: fn_name.loc,
                })
            }
            CodeExpr::InlineMethodCall(InlineMethodCallExpr {
                id: _,
                lhs,
                field_name,
                type_args,
                args,
                loc,
            }) => {
                let lhs_type = self.get_expr_type(ctx, lhs)?;
                let inline_fn_name = self.get_fn_name_from_method(&lhs_type, &field_name.repr);

                self.be_mut().enter_scope(ctx, ScopeType::InlineFn);
                let expr_type = self.get_inline_fn_return_type(
                    ctx.be_mut(),
                    &inline_fn_name,
                    type_args,
                    &args.items,
                    Some(&lhs),
                    loc,
                );
                self.be_mut().exit_scope(ctx);
                expr_type
            }
            CodeExpr::Catch(CatchExpr {
                id: _,
                lhs,
                error_bind: _,
                catch_body: _,
                catch_loc,
                loc: _,
            }) => {
                let expr_type = self.get_expr_type(ctx, lhs)?;
                let result = self.assert_catchable_type(&expr_type, catch_loc)?;
                Ok(result.ok.as_ref().clone())
            }
            CodeExpr::PropagateError(PropagateErrorExpr { id: _, expr, loc }) => {
                let expr_type = self.get_expr_type(ctx, expr)?;
                let result = self.assert_catchable_type(&expr_type, loc)?;
                Ok(result.ok.as_ref().clone())
            }
            CodeExpr::Pipe(PipeExpr {
                id: _,
                lhs,
                rhs,
                op_loc,
                loc: _,
            }) => {
                let ctx = ctx.be_mut();

                let lhs_type = catch!(self.get_expr_type(ctx, &lhs), err, {
                    self.reporter.error(&err);
                    return Ok(Type::Never);
                });

                self.be_mut().enter_scope(ctx, ScopeType::Block);

                self.be_mut().register_block_const(
                    ctx,
                    ConstDef {
                        const_name: "it",
                        code_unit: CodeUnit {
                            type_: lhs_type,
                            instrs: Vec::new(),
                        },
                        loc: op_loc.clone(),
                    },
                );

                let rhs_type = catch!(self.get_expr_type(ctx, &rhs), err, {
                    self.reporter.error(&err);
                    return Ok(Type::Never);
                });

                self.be_mut().exit_scope(ctx);

                return Ok(rhs_type);
            }
            CodeExpr::Sizeof(_) => Ok(Type::U32),
            CodeExpr::Let(let_) => {
                let value_type = catch!(self.get_expr_type(ctx, &let_.value), _err, {
                    // `_err` will be reported in `codegen`
                    return Ok(Type::Void);
                });

                if value_type == Type::Never {
                    return Ok(Type::Never);
                }

                Ok(Type::Void)
            }
            CodeExpr::Assign(_) => Ok(Type::Void),
            CodeExpr::Defer(_) => Ok(Type::Void),
            CodeExpr::If(IfExpr {
                id: _,
                cond,
                then_block,
                else_block,
                loc: _,
            }) => {
                let mut should_exit_match_scope = false;

                match cond {
                    IfCond::Expr(e) => {
                        if self.get_expr_type(ctx, e)? == Type::Never {
                            return Ok(Type::Never);
                        }
                    }
                    IfCond::Match(header) => {
                        if let Some(symbol) = self
                            .current_scope(ctx)
                            .get_symbol(&header.variant_name.repr)
                        {
                            if let SymbolType::EnumConstructor = symbol.type_ {
                                let enum_ctor = &self.enum_ctors[symbol.col_index];
                                let enum_variant = &self.enums[enum_ctor.enum_index].variants
                                    [enum_ctor.variant_index];

                                should_exit_match_scope = true;
                                self.be_mut().enter_scope(ctx, ScopeType::Block);
                                self.be_mut().register_block_const(
                                    ctx,
                                    ConstDef {
                                        const_name: header.variant_bind.repr,
                                        code_unit: CodeUnit {
                                            type_: enum_variant.variant_type.clone(),
                                            instrs: Vec::new(),
                                        },
                                        loc: header.variant_bind.loc,
                                    },
                                );
                            };
                        }
                    }
                };

                let then_type = self.get_code_block_type(ctx, &then_block.exprs);
                if should_exit_match_scope {
                    self.be_mut().exit_scope(ctx);
                }
                let then_type = then_type?;

                let mut else_type = Type::Void;
                match else_block {
                    ElseBlock::None => {}
                    ElseBlock::Else(else_) => {
                        else_type = self.get_code_block_type(ctx, &else_.exprs)?;
                    }
                    ElseBlock::ElseIf(e) => {
                        else_type = self.get_expr_type(ctx, e)?;
                    }
                }

                if then_type == Type::Never && else_type == Type::Never {
                    return Ok(Type::Never);
                }

                Ok(Type::Void)
            }
            CodeExpr::Match(_) => Ok(Type::Void),
            CodeExpr::While(_) => Ok(Type::Void),
            CodeExpr::For(_) => Ok(Type::Void),
            CodeExpr::Break(_) => Ok(Type::Never),
            CodeExpr::Continue(_) => Ok(Type::Never),
            CodeExpr::DoWith(_) => Ok(Type::Void),
            CodeExpr::Return(_) => Ok(Type::Never),
            CodeExpr::Paren(ParenExpr {
                id: _,
                expr,
                has_trailing_comma: _,
                loc: _,
            }) => self.get_expr_type(ctx, expr),
        }
    }

    pub fn get_code_block_type(
        &self,
        ctx: &ExprContext,
        exprs: &Vec<CodeExpr>,
    ) -> Result<Type, Error> {
        self.be_mut().enter_scope(ctx, ScopeType::Block);
        let res = self.get_code_block_type_(ctx, exprs);
        self.be_mut().exit_scope(ctx);
        res
    }

    pub fn get_code_block_type_(
        &self,
        ctx: &ExprContext,
        exprs: &Vec<CodeExpr>,
    ) -> Result<Type, Error> {
        let mut diverges = false;

        for expr in exprs {
            if let CodeExpr::Let(LetExpr {
                id: _,
                is_inline: _,
                name,
                value,
                loc,
            }) = expr
            {
                let value_type = self.get_expr_type(ctx, value)?;
                diverges = diverges || value_type == Type::Never;

                self.be_mut().register_block_const(
                    ctx,
                    ConstDef {
                        const_name: name.repr,
                        code_unit: CodeUnit {
                            type_: value_type,
                            instrs: Vec::new(),
                        },
                        loc: *loc,
                    },
                );

                continue;
            }

            diverges = diverges || self.get_expr_type(ctx, expr)? == Type::Never;
        }

        if diverges {
            return Ok(Type::Never);
        }

        Ok(Type::Void)
    }

    fn get_inline_fn_return_type(
        &self,
        ctx: &mut ExprContext,
        inline_fn_name: &str,
        type_args: &Vec<TypeExpr>,
        args: &Vec<CodeExpr>,
        receiver_arg: Option<&CodeExpr>,
        loc: &Loc,
    ) -> Result<Type, Error> {
        let inline_fn_def = self.be_mut().populate_ctx_from_inline_fn_call(
            ctx,
            inline_fn_name,
            type_args,
            receiver_arg,
            args,
            loc,
        )?;

        let return_type = if let Some(return_type) = &inline_fn_def.decl.return_type {
            self.build_type(ctx, return_type)?
        } else {
            Type::Void
        };

        Ok(return_type)
    }

    fn populate_ctx_from_inline_fn_call(
        &mut self,
        ctx: &mut ExprContext,
        inline_fn_name: &str,
        type_args: &Vec<TypeExpr>,
        receiver_arg: Option<&CodeExpr>,
        args: &Vec<CodeExpr>,
        loc: &Loc,
    ) -> Result<&FnExpr, Error> {
        let Some(symbol) = self.current_scope(ctx).get_symbol(inline_fn_name) else {
            return Err(Error {
                message: format!("Unknown inline fn: {}", inline_fn_name),
                loc: *loc,
            });
        };

        let inline_fn_def = self.inline_fns[symbol.col_index];

        let mut all_args = Vec::new();
        if let Some(receiver_arg) = receiver_arg {
            all_args.push(CodeUnit {
                type_: self.get_expr_type(ctx, receiver_arg)?,
                instrs: Vec::new(),
            });
        }
        for arg in args {
            all_args.push(CodeUnit {
                type_: self.get_expr_type(ctx, arg)?,
                instrs: Vec::new(),
            });
        }

        let mut lo_type_args = Vec::new();
        for type_arg in type_args {
            lo_type_args.push(self.build_type(ctx, &type_arg)?);
        }
        if lo_type_args.len() != inline_fn_def.decl.type_params.len() {
            return Err(Error {
                message: format!(
                    "Invalid number of type args, expected {}, got {}",
                    inline_fn_def.decl.type_params.len(),
                    type_args.len()
                ),
                loc: *loc,
            });
        }

        for (i, (type_param, type_arg)) in inline_fn_def
            .decl
            .type_params
            .iter()
            .zip(lo_type_args.iter())
            .enumerate()
        {
            self.register_block_type(ctx, type_param, type_arg.clone(), *type_args[i].loc());
        }

        if all_args.len() != inline_fn_def.decl.params.len() {
            return Err(Error {
                message: format!(
                    "Invalid number of inline fn args, expected {}, got {}",
                    inline_fn_def.decl.params.len(),
                    all_args.len()
                ),
                loc: *loc,
            });
        }

        let mut arg_types = Vec::<Type>::new();
        for arg in &all_args {
            arg_types.push(arg.type_.clone());
        }

        for (inline_fn_param, inline_fn_arg) in
            inline_fn_def.decl.params.iter().zip(all_args.into_iter())
        {
            let const_def = ConstDef {
                const_name: inline_fn_param.param_name.repr,
                code_unit: inline_fn_arg,
                loc: inline_fn_param.loc,
            };

            if let Some(type_name) = self.get_infer_type_name(inline_fn_param)? {
                self.register_block_type(
                    ctx,
                    type_name,
                    const_def.code_unit.type_.clone(),
                    inline_fn_param.loc,
                );
            }

            self.register_block_const(ctx, const_def);
        }

        let self_type =
            self.get_fn_self_type(ctx, &inline_fn_def.decl.fn_name, &inline_fn_def.decl.params);

        let mut inline_fn_types = Vec::<Type>::new();
        for inline_fn_param in &inline_fn_def.decl.params {
            let inline_fn_type = self.get_fn_param_type(ctx, inline_fn_param, &self_type, true)?;
            inline_fn_types.push(inline_fn_type);
        }

        if !self.is_types_compatible(&inline_fn_types, &arg_types) {
            return Err(Error {
                message: format!(
                    "Invalid inline fn args, expected [{}], got [{}]",
                    TypeListFmt(&*self, &inline_fn_types),
                    TypeListFmt(&*self, &arg_types)
                ),
                loc: *loc,
            });
        }

        Ok(inline_fn_def)
    }

    pub fn get_result_literal_type(
        &self,
        ctx: &ExprContext,
        explicit_type: &Option<ResultTypeExpr>,
        loc: &Loc,
    ) -> Result<ResultType, Error> {
        if let Some(result_type) = explicit_type {
            let ok = Box::new(self.build_type(ctx, &result_type.ok)?);
            let err = Box::new(self.build_type(ctx, &result_type.err)?);
            return Ok(ResultType { ok, err });
        }

        let Some(fn_index) = ctx.fn_index else {
            return Err(Error {
                message: format!("Cannot create implicitly typed result in const context"),
                loc: *loc,
            });
        };

        let fn_info = &self.functions[fn_index];
        let Type::Result(result) = &fn_info.fn_type.output else {
            return Err(Error {
                message: format!(
                    "Cannot create implicitly typed result: function does not return result"
                ),
                loc: *loc,
            });
        };

        Ok(ResultType {
            ok: result.ok.clone(),
            err: result.err.clone(),
        })
    }

    pub fn get_struct_or_struct_ref_field(
        &self,
        mut lhs_type: &Type,
        field_access: &FieldAccessExpr,
    ) -> Result<&StructField, Error> {
        if let Type::Pointer { pointee } = &lhs_type {
            lhs_type = pointee;
        }

        let struct_index: usize;
        if let Type::StructInstance { struct_index: si } = lhs_type {
            struct_index = *si;
        } else if let Type::Container(ContainerType {
            container,
            items: _,
        }) = lhs_type
            && let Type::StructInstance { struct_index: si } = &**container
        {
            struct_index = *si;
        } else {
            return Err(Error {
                message: format!(
                    "Cannot get field '{}' on non struct: {}",
                    field_access.field_name.repr,
                    TypeFmt(self, lhs_type),
                ),
                loc: field_access.field_name.loc,
            });
        };

        let struct_def = &self.structs[struct_index];
        let Some(field) = struct_def
            .fields
            .iter()
            .find(|f| &f.field_name == &field_access.field_name.repr)
        else {
            return Err(Error {
                message: format!(
                    "Unknown field {} in struct {}",
                    field_access.field_name.repr, struct_def.struct_name
                ),
                loc: field_access.field_name.loc,
            });
        };

        Ok(field)
    }

    pub fn get_fn_name_from_method(&self, receiver_type: &Type, method_name: &str) -> String {
        let receiver_type_base = receiver_type.deref_rec();

        if let Type::Container(ContainerType {
            container,
            items: _,
        }) = receiver_type_base
        {
            return format!("{}::{method_name}", TypeFmt(self, container));
        }

        format!("{}::{method_name}", TypeFmt(self, receiver_type_base))
    }

    pub fn create_or_get_addr_local(&self, ctx: &mut ExprContext) -> u32 {
        if let Some(addr_local_index) = ctx.addr_local_index {
            return addr_local_index;
        }

        let addr_local_index = self.define_unnamed_local(ctx, Loc::internal(), &Type::U32);

        return addr_local_index;
    }

    pub fn define_local(
        &mut self,
        ctx: &mut ExprContext,
        loc: Loc,
        local_name: &'static str,
        local_type: &Type,
    ) -> u32 {
        let res = self.define_symbol(ctx, local_name, SymbolType::Local, ctx.locals.len(), loc);

        if let Err(existing) = res
            && existing.type_ == SymbolType::Local
        {
            return ctx.locals[existing.col_index].local_index;
        }

        let local_index = self.define_unnamed_local(ctx, loc, local_type);
        local_index
    }

    pub fn define_unnamed_local(&self, ctx: &mut ExprContext, loc: Loc, local_type: &Type) -> u32 {
        let local_index = ctx.next_local_index;
        ctx.locals.push(Local {
            local_index,
            local_type: local_type.clone(),
            definition_loc: loc,
        });
        ctx.next_local_index += self.count_wasm_type_components(local_type);

        local_index
    }

    pub fn assert_catchable_type<'a>(
        &self,
        expr_type: &'a Type,
        loc: &Loc,
    ) -> Result<&'a ResultType, Error> {
        let Type::Result(result) = expr_type else {
            return Err(Error {
                message: format!(
                    "Cannot catch error from expr of type {}",
                    TypeFmt(self, &expr_type)
                ),
                loc: *loc,
            });
        };

        let mut err_type_components = Vec::new();
        self.lower_type(&result.err, &mut err_type_components);
        if err_type_components != [WasmType::I32] {
            return Err(Error {
                message: format!(
                    "Invalid Result error type: {}, must lower to i32",
                    TypeFmt(self, &result.err)
                ),
                loc: *loc,
            });
        }

        Ok(result)
    }

    pub fn register_block_const(&mut self, ctx: &ExprContext, const_def: ConstDef) {
        if const_def.const_name == "_" {
            return;
        }

        let _ = self.define_symbol(
            ctx,
            const_def.const_name,
            SymbolType::Const,
            self.constants.len(),
            const_def.loc,
        );
        self.constants.push(const_def);
    }

    pub fn register_block_type(
        &mut self,
        ctx: &ExprContext,
        name: &'static str,
        type_: Type,
        loc: Loc,
    ) {
        let _ = self.define_symbol(
            ctx,
            name,
            SymbolType::TypeAlias,
            self.type_aliases.len(),
            loc,
        );
        self.type_aliases.push(type_);
    }

    pub fn get_module_by_file_index(&self, file_index: usize) -> Option<&Module> {
        for module in &self.modules {
            if module.parser.lexer.file_index == file_index {
                return Some(module);
            }
        }

        None
    }
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

pub fn align(value: u32, alignment: u32) -> u32 {
    return (value + alignment - 1) / alignment * alignment;
}
