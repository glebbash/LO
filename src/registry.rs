use crate::{ast::*, common::*, lexer::*, parser::*, typer::*, wasm::*};

#[derive(Clone)]
pub struct Symbol {
    pub scope_id: usize,
    pub name: &'static str,
    pub kind: SymbolKind,
    pub col_index: usize,
    pub loc: Loc,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum SymbolKind {
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
    pub name: &'static str,
    pub type_: FnType,
    pub params: Vec<FnParameter>,
    pub source: FnSource,
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
        module_id: usize,
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
    pub module_id: usize,
    pub fn_index: Option<usize>,
    pub locals: Vec<Local>,
    pub next_local_index: u32,
    pub addr_local_index: Option<u32>,
}

impl ExprContext {
    pub fn new(module_id: usize, fn_index: Option<usize>) -> Self {
        Self {
            module_id,
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
pub enum ScopeKind {
    Global,
    Function,
    Block,
    Loop,
    ForLoop,
    InlineFn,
}

impl Default for ScopeKind {
    fn default() -> Self {
        ScopeKind::Block
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
    pub id: usize,
    pub kind: ScopeKind,
    pub symbols: Vec<Symbol>,
    pub deferred_exprs: Vec<CodeUnit>,
    pub inline_fn_call_loc: Option<Loc>,

    pub expr_info_offset: usize,
}

impl Scope {
    pub fn new(scope_id: usize, scope_type: ScopeKind) -> Self {
        Self {
            id: scope_id,
            kind: scope_type,
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

#[derive(Clone)]
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
    pub variant_type_id: TypeId,
    pub loc: Loc,
}

pub struct EnumConstructor {
    pub enum_index: usize,
    pub variant_index: usize,
    pub loc: Loc,
}

pub struct GlobalDef {
    pub module_id: ModuleId,
    pub value: &'static CodeExpr,
    pub type_id: TypeId,
    pub wasm_global_index: u32,
}

#[derive(Clone)]
pub struct ConstDef {
    pub const_name: &'static str,
    pub code_unit: CodeUnit,
    pub loc: Loc,
}

pub struct Module {
    pub id: usize,
    pub source: &'static [u8],
    pub parser: Parser,
    pub includes: Vec<ModuleInclude>,
    pub scope_stack: Vec<Scope>,

    // TODO: remove after migration
    pub ctx: ExprContext,
}

type ModuleId = usize;

pub struct ModuleInclude {
    pub module_id: ModuleId,
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

#[derive(Default)]
pub struct Registry {
    pub in_single_file_mode: bool,
    pub should_emit_dbg_local_names: bool,

    pub fm: Box<FileManager>,
    pub reporter: Box<Reporter>,

    pub modules: Vec<Module>,                // indexed by `module_id`
    pub expr_info: Vec<ExprInfo>,            // indexed by `expr.id()`
    pub types: Vec<Type>,                    // indexed by `ExprInfoId` for most of the expressions
    pub inline_fn_call_info: Vec<ICallInfo>, // indexed by `ExprInfoId` for `::InlineFnCall` and `::InlineMethodCall`
    pub globals: Vec<GlobalDef>,             // indexed by `col_index` when `kind = Global`
    pub constants: Vec<ConstDef>,            // indexed by `col_index` when `kind = Const`
    pub functions: Vec<FnInfo>,              // indexed by `col_index` when `kind = Function`
    pub inline_fns: Vec<&'static FnExpr>,    // indexed by `col_index` when `kind = InlineFn`
    pub type_aliases: Vec<Type>,             // indexed by `col_index` when `kind = TypeAlias`
    pub structs: Vec<StructDef>,             // indexed by `col_index` when `kind = Struct`
    pub enums: Vec<EnumDef>,                 // indexed by `col_index` when `kind = Enum`
    pub enum_ctors: Vec<EnumConstructor>,    // indexed by `col_index` when `kind = EnumConstructor`

    pub memory: Option<MemoryInfo>,
    pub data_size: UBCell<u32>,

    pub expr_count: usize,
    pub scope_count: usize,
}

impl Registry {
    pub fn new() -> Self {
        let mut it = Self::default();
        it.reporter.fm = UBRef::new(&mut *it.fm);

        // TODO: remove this when symbols will become resolved through typer
        it.scope_count = 1; // global scope id is 0

        it
    }

    pub fn include_file(&mut self, relative_path: &str, loc: &Loc) -> Option<ModuleId> {
        let file_id = catch!(self.fm.include_file(relative_path, loc), err, {
            self.report_error(&err);
            return None;
        });

        let file_is_newly_added = self.fm.files[file_id].included_times == 1;

        if self.reporter.in_inspection_mode {
            self.reporter
                .print_include_info(file_is_newly_added, file_id, loc);
        }

        if !file_is_newly_added {
            return Some(self.get_module_id_by_file_id(file_id));
        }

        let source = self.fm.files[file_id].source.as_bytes().relax();

        let mut lexer = Lexer::new(source, file_id);
        catch!(lexer.lex_file(), err, {
            self.report_error(&err);
            return None;
        });

        let parser = Parser::new(lexer, &mut self.reporter);

        *parser.expr_count.be_mut() = self.expr_count;

        catch!(parser.parse_file(), err, {
            self.report_error(&err);
            return None;
        });

        self.expr_count = *parser.expr_count;

        let mut includes = Vec::new();

        if !self.in_single_file_mode {
            for expr in &*parser.ast {
                let Some(include_info) = catch!(get_include_info(expr), err, {
                    self.report_error(&err);
                    continue;
                }) else {
                    continue;
                };

                let Some(module_id) =
                    self.include_file(&include_info.file_path.value, &include_info.file_path.loc)
                else {
                    continue;
                };

                includes.push(ModuleInclude {
                    module_id,
                    alias: include_info.alias,
                    with_extern: include_info.with_extern,
                });
            }
        }

        let module_id = self.modules.len();

        self.modules.push(Module {
            id: module_id,
            source: parser.source,
            scope_stack: Vec::new(),
            parser,
            ctx: ExprContext::new(self.modules.len(), None),
            includes,
        });

        Some(module_id)
    }

    // TODO: move to codegen
    pub fn define_symbol(
        &mut self,
        ctx: &ExprContext,
        symbol_name: &'static str,
        symbol_kind: SymbolKind,
        symbol_loc: Loc,
    ) -> Result<(), &Symbol> {
        let symbol_col_index = match symbol_kind {
            SymbolKind::TypeAlias => self.type_aliases.len(),
            SymbolKind::Struct => self.structs.len(),
            SymbolKind::Enum => self.enums.len(),
            SymbolKind::Local => ctx.locals.len(),
            SymbolKind::Global => self.globals.len(),
            SymbolKind::Const => self.constants.len(),
            SymbolKind::InlineFn => self.inline_fns.len(),
            SymbolKind::Function => self.functions.len(),
            SymbolKind::EnumConstructor => self.enum_ctors.len(),
        };

        let current_scope = self.current_scope(ctx).relax().be_mut();

        if let Some(existing_symbol) = current_scope.relax().get_symbol(symbol_name)
            && existing_symbol.scope_id == current_scope.id
        {
            self.report_error(&Error {
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
            scope_id: current_scope.id,
            name: symbol_name,
            kind: symbol_kind,
            col_index: symbol_col_index,
            loc: symbol_loc,
        });
        Ok(())
    }

    // TODO: move to codegen
    pub fn enter_scope(&mut self, ctx: &ExprContext, scope_type: ScopeKind) {
        let module = &mut self.relax_mut().modules[ctx.module_id];

        self.init_scope_from_parent_and_push(&mut module.scope_stack, scope_type);
    }

    // TODO: move to codegen
    pub fn exit_scope(&mut self, ctx: &ExprContext) -> Scope {
        let module = &mut self.modules[ctx.module_id];

        module.scope_stack.pop().unwrap()
    }

    // TODO: move to codegen
    pub fn current_scope(&self, ctx: &ExprContext) -> &Scope {
        let module = &self.modules[ctx.module_id];

        module.scope_stack.last().unwrap()
    }

    // TODO: move to codegen
    pub fn init_scope_from_parent_and_push(
        &mut self,
        scope_stack: &mut Vec<Scope>,
        scope_type: ScopeKind,
    ) {
        let scope_id = self.scope_count;
        self.scope_count += 1;

        let mut new_scope = Scope::new(scope_id, scope_type);
        if let Some(parent) = scope_stack.last() {
            new_scope.symbols.extend_from_slice(&parent.symbols);
            new_scope.expr_info_offset = parent.expr_info_offset;
        };
        scope_stack.push(new_scope);
    }

    // TODO: move to typer
    pub fn build_type(&self, ctx: &ExprContext, type_expr: &TypeExpr) -> Result<Type, Error> {
        return self.build_type_check_ref(ctx, type_expr, true, &Loc::internal());
    }

    // builds a type, asserting that it doesn't have infinite size
    fn build_type_check_ref(
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

                    let SymbolKind::Const = symbol.kind else {
                        return Err(Error {
                            message: format!("Expected const, got {:?}", symbol.kind),
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

        match symbol.kind {
            SymbolKind::Struct => {
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
            SymbolKind::Enum => {
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
            SymbolKind::TypeAlias => {
                let type_ = &self.type_aliases[symbol.col_index];

                // don't print inspection for built-ins
                if self.reporter.in_inspection_mode && symbol.loc.file_id != 0 {
                    self.reporter.print_inspection(&InspectInfo {
                        message: format!("type {type_name} = {}", TypeFmt(self, &type_)),
                        loc: *loc,
                        linked_loc: Some(symbol.loc),
                    });
                }

                Ok(type_.clone())
            }
            SymbolKind::Local
            | SymbolKind::Global
            | SymbolKind::Const
            | SymbolKind::Function
            | SymbolKind::InlineFn
            | SymbolKind::EnumConstructor => Err(Error {
                message: format!("Symbol is not a type: {}", type_name),
                loc: *loc,
            }),
        }
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

    pub fn get_module_id_by_file_id(&self, file_id: usize) -> ModuleId {
        for module in &self.modules {
            if module.parser.lexer.file_id == file_id {
                return module.id;
            }
        }

        unreachable!()
    }

    // TODO: remove tag after migration
    fn report_error(&self, err: &Error) {
        let marked_error = Error {
            message: format!("(reg) {}", err.message),
            loc: err.loc.clone(),
        };
        self.reporter.error(&marked_error);
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
