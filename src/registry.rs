use crate::{ast::*, common::*, lexer::*, parser::*, typer::*, wasm::*};

pub enum ValueKind {
    Local,
    Global,
    Const,
    EnumConstructor,
}

pub struct ValueInfo {
    pub kind: ValueKind,
    pub col_index: usize,
    pub type_id: TypeId,
}

pub struct LocalDef {
    pub local_name: Option<&'static str>,
    pub local_type: Type,
    pub wasm_local_index: u32,
}

pub struct FnInfo {
    pub name: &'static str,
    pub type_: FnType,
    pub params: Vec<FnParameter>,
    pub source: FnSource,
    pub exported_as: Vec<String>,
    pub locals_ids: Vec<usize>,
    pub wasm_fn_index: u32,
    pub wasm_locals_count: u32,
    pub definition_loc: Loc,
}

pub struct FnParameter {
    pub param_name: &'static IdentExpr,
    pub param_type: Type,
}

pub enum FnSource {
    Guest {
        module_id: ModuleId,
        lo_fn_index: usize,
        body: &'static CodeBlock,
    },
    Host {
        module_name: String,
        external_fn_name: &'static str,
    },
}

pub struct FnType {
    pub inputs: Vec<TypeId>,
    pub output: Type,
}

#[derive(Clone)]
pub struct ExprContext {
    pub module_id: ModuleId,
    pub fn_index: Option<usize>,
    pub addr_local_index: Option<u32>,
}

impl ExprContext {
    pub fn new(module_id: ModuleId, fn_index: Option<usize>) -> Self {
        Self {
            module_id,
            fn_index,
            addr_local_index: None,
        }
    }
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

#[derive(PartialEq)]
pub enum StructResolution {
    NotStarted,
    InProgress,
    Resolved,
}

pub struct StructDef {
    pub struct_name: &'static str,
    pub fields: Vec<StructField>,
    pub resolution: StructResolution,
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
    pub type_id: TypeId,
    pub expr: UBRef<CodeExpr>,
    pub inner_expr_id_offset: usize,
}

pub struct Module {
    pub id: usize,
    pub absolute_path: String,

    pub source: &'static [u8],
    pub _source: String,

    pub includes: Vec<ModuleInclude>,
    pub included_times: usize,

    pub parser: Parser,
}

pub type ModuleId = usize;

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

pub enum CallInfoValue {
    FnCall(/* fn_index */ usize),
    EnumCtor(/* variant_index */ i32),
}

pub struct CallInfo {
    pub value: CallInfoValue,
    pub return_type_id: TypeId,
}

pub struct InlineCallInfo {
    pub inline_fn_index: usize,
    pub inner_expr_id_offset: usize,
    pub return_type_id: TypeId,
}

pub struct StoredExprBytes {
    pub info: BytesSlice,
    pub expr_type_id: TypeId,
}

pub struct BytesSlice {
    pub ptr: u32,
    pub len: u32,
}

pub struct PooledString {
    pub value: String,
    pub ptr: u32,
}

pub struct BreakInfo {
    pub label_index: u32,
    pub expr_type_id: TypeId,
}

pub struct BlockInfo {
    pub needs_explicit_trap: bool,
}

pub struct BinaryOpInfo {
    pub is_compound_assignment: bool,
    pub op_kind: WasmBinaryOpKind,
    pub result_type_id: TypeId,
}

#[derive(Default)]
pub struct Registry {
    pub in_single_file_mode: bool,
    pub should_emit_dbg_local_names: bool,

    pub reporter: Box<Reporter>,

    pub modules: Vec<Module>, // indexed by `ModuleId`
    pub module_import_order: Vec<ModuleId>,

    pub expr_id_count: usize,
    pub expr_info: Vec<ExprInfo>,              // indexed by `ExprId`
    pub types: Vec<Type>, //                      indexed by `ExprInfo` for most of the expressions
    pub inline_call_info: Vec<InlineCallInfo>, // indexed by `ExprInfo` for `::InlineFnCall` and `::InlineMethodCall`
    pub call_info: Vec<CallInfo>, //              indexed by `ExprInfo` for `::FnCall` and `::MethodCall`
    pub value_info: Vec<ValueInfo>, //            indexed by `ExprInfo` for `::IdentExpr`
    pub stored_bytes: Vec<StoredExprBytes>, //    indexed by `ExprInfo` for `::StringLiteral` and `::ArrayLiteral`
    pub breaks: Vec<BreakInfo>, //                indexed by `ExprInfo` for `::Break` and `::Continue`
    pub block_info: Vec<BlockInfo>, //            indexed by `ExprInfo` for `::CodeBlock`
    pub binary_op_info: Vec<BinaryOpInfo>, //     indexed by `ExprInfo` for `::InfixOp`

    pub globals: Vec<GlobalDef>, //          indexed by `col_index` when `kind = Global`
    pub locals: Vec<LocalDef>,   //          indexed by `col_index` when `kind = Local`
    pub constants: Vec<ConstDef>, //         indexed by `col_index` when `kind = Const`
    pub functions: Vec<FnInfo>,  //          indexed by `col_index` when `kind = Function`
    pub inline_fns: Vec<&'static FnExpr>, // indexed by `col_index` when `kind = InlineFn`
    pub structs: Vec<StructDef>, //          indexed by `col_index` when `kind = Struct`
    pub enums: Vec<EnumDef>,     //          indexed by `col_index` when `kind = Enum`
    pub enum_ctors: Vec<EnumConstructor>, // indexed by `col_index` when `kind = EnumConstructor`

    pub memory: Option<MemoryInfo>,
    pub data_size: UBCell<u32>,
    pub datas: UBCell<Vec<WasmData>>,
    pub string_pool: UBCell<Vec<PooledString>>,
    pub str_literal_type_id: Option<TypeId>,
}

impl Registry {
    pub fn new() -> Box<Self> {
        let mut it = Box::new(Self::default());
        it.reporter.registry = UBRef::new(&*it);
        it.modules.push(Module {
            id: 0,
            absolute_path: String::from("<internal>"),
            source: "".as_bytes(),
            _source: String::from(""),
            includes: Vec::new(),
            included_times: 0,
            parser: Parser::new(Lexer::new("".as_bytes(), 0), &mut it.reporter),
        });
        it
    }

    pub fn include(&mut self, relative_path: &str, loc: &Loc) -> Option<ModuleId> {
        let absolute_path = self.resolve_path(relative_path, loc);

        for module in &mut self.modules {
            if module.absolute_path == absolute_path {
                module.included_times += 1;

                if self.reporter.in_inspection_mode {
                    self.reporter.print_include_info(false, module.id, loc);
                }

                return Some(module.id);
            }
        }

        let module_id = self.modules.len();

        let _source = catch!(fs::file_read_utf8(&absolute_path), message, {
            self.reporter.error(Error { message, loc: *loc });
            return None;
        });
        let source = _source.as_bytes().relax();

        let lexer = Lexer::new(source, module_id);
        let parser = Parser::new(lexer, &mut self.reporter);
        let module = Module {
            id: module_id,
            absolute_path,
            source,
            _source,
            parser,
            includes: Vec::new(),
            included_times: 0,
        };
        self.modules.push(module);

        if self.reporter.in_inspection_mode {
            self.reporter.print_include_info(true, module_id, loc);
        }

        catch!(self.modules[module_id].parser.lexer.lex_file(), err, {
            self.reporter.error(err);
            return None;
        });

        {
            *self.modules[module_id].parser.expr_id_count = self.expr_id_count;

            catch!(self.modules[module_id].parser.parse_file(), err, {
                self.reporter.error(err);
                return None;
            });

            self.expr_id_count = *self.modules[module_id].parser.expr_id_count;
        }

        if !self.in_single_file_mode {
            for expr in &*self.modules[module_id].relax().parser.ast {
                let Some(include_info) = catch!(get_include_info(expr), err, {
                    self.reporter.error(err);
                    continue;
                }) else {
                    continue;
                };

                let Some(included_module_id) =
                    self.include(&include_info.file_path.value, &include_info.file_path.loc)
                else {
                    continue;
                };

                self.modules[module_id].includes.push(ModuleInclude {
                    module_id: included_module_id,
                    alias: include_info.alias,
                    with_extern: include_info.with_extern,
                });
            }
        }

        self.module_import_order.push(module_id);

        Some(module_id)
    }

    pub fn add_local(
        &self,
        fn_index: usize,
        local_name: Option<&'static str>,
        local_type: &Type,
    ) -> u32 {
        let fn_info = self.functions[fn_index].be_mut();

        let wasm_local_index = fn_info.wasm_locals_count;

        fn_info.locals_ids.push(self.locals.len());
        self.locals.be_mut().push(LocalDef {
            local_name,
            local_type: local_type.clone(),
            wasm_local_index,
        });
        fn_info.wasm_locals_count += count_primitive_components(&self, local_type);

        wasm_local_index
    }

    pub fn resolve_path(&self, file_path: &str, loc: &Loc) -> String {
        if !file_path.starts_with('.') {
            return file_path.into();
        }

        let relative_to = &self.modules[loc.module_id].absolute_path;

        let mut path_items = relative_to.split('/').collect::<Vec<_>>();
        path_items.pop(); // remove `relative_to`'s file name

        path_items.extend(file_path.split('/'));

        let mut i = 0;
        loop {
            if i >= path_items.len() {
                break;
            }

            if path_items[i] == "." {
                path_items.remove(i);
                continue;
            }

            if path_items[i] == ".." && i > 0 {
                i -= 1;
                path_items.remove(i);
                path_items.remove(i);
                continue;
            }

            i += 1;
        }

        path_items.join("/")
    }

    pub fn get_expr_type(&self, expr_id_offset: usize, expr: &CodeExpr) -> Option<TypeId> {
        let Some(expr_info) = self.get_expr_info(expr_id_offset, expr.id()) else {
            return None;
        };

        Some(match expr {
            CodeExpr::Ident(_) => {
                let symbol = &self.value_info[expr_info];
                symbol.type_id
            }
            CodeExpr::FnCall(_) | CodeExpr::MethodCall(_) => {
                let call_info = &self.call_info[expr_info];
                call_info.return_type_id
            }
            CodeExpr::InlineFnCall(_) | CodeExpr::InlineMethodCall(_) => {
                let call_info = &self.inline_call_info[expr_info];
                call_info.return_type_id
            }
            CodeExpr::StringLiteral(_) | CodeExpr::ArrayLiteral(_) => {
                let stored_bytes = &self.stored_bytes[expr_info];
                stored_bytes.expr_type_id
            }
            CodeExpr::Break(_) | CodeExpr::Continue(_) => {
                let break_info = &self.breaks[expr_info];
                break_info.expr_type_id
            }
            CodeExpr::InfixOp(_) => {
                let binary_op_info = &self.binary_op_info[expr_info];
                binary_op_info.result_type_id
            }
            _ => expr_info,
        })
    }

    pub fn get_expr_info(&self, expr_id_offset: usize, expr_id: usize) -> Option<ExprInfo> {
        let expr_info = self.expr_info[expr_id_offset + expr_id];
        if expr_info == EXPR_INFO_INVALID {
            return None;
        }
        Some(expr_info)
    }

    pub fn get_type(&self, type_id: TypeId) -> &'static Type {
        self.types[type_id].relax()
    }

    pub fn fmt<'a>(&'a self, type_: &'a Type) -> TypeFmt<'a> {
        TypeFmt {
            registry: self,
            type_,
        }
    }

    pub fn fmt_many<'a>(&'a self, type_ids: &'a [TypeId]) -> TypeListFmt<'a> {
        TypeListFmt {
            registry: self,
            type_ids,
        }
    }
}

pub fn align(value: u32, alignment: u32) -> u32 {
    return (value + alignment - 1) / alignment * alignment;
}
