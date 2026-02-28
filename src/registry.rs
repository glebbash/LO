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
    pub inputs: Vec<TypeId>,
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
    pub instrs: Vec<WasmInstr>,
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
    pub type_id: TypeId,
    pub expr: UBRef<CodeExpr>,
    pub inner_expr_id_offset: usize,
}

pub struct Module {
    pub id: usize,
    pub source: &'static [u8],
    pub parser: Parser,
    pub includes: Vec<ModuleInclude>,
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

#[derive(Default)]
pub struct Registry {
    pub in_single_file_mode: bool,
    pub should_emit_dbg_local_names: bool,

    pub reporter: Box<Reporter>,

    pub files: Vec<FileInfo>,                  // indexed by `Loc::file_id`
    pub modules: Vec<Module>,                  // indexed by `*::module_id`
    pub expr_info: Vec<ExprInfo>,              // indexed by `CodeExpr.id()` and `TypeExpr.id()`
    pub types: Vec<Type>, //                      indexed by `ExprInfo` for most of the expressions
    pub inline_call_info: Vec<InlineCallInfo>, // indexed by `ExprInfo` for `::InlineFnCall` and `::InlineMethodCall`
    pub call_info: Vec<CallInfo>, //              indexed by `ExprInfo` for `::FnCall` and `::MethodCall`
    pub value_info: Vec<ValueInfo>, //            indexed by `ExprInfo` for `::IdentExpr`
    pub globals: Vec<GlobalDef>,  //              indexed by `col_index` when `kind = Global`
    pub constants: Vec<ConstDef>, //              indexed by `col_index` when `kind = Const`
    pub functions: Vec<FnInfo>,   //              indexed by `col_index` when `kind = Function`
    pub inline_fns: Vec<&'static FnExpr>, //      indexed by `col_index` when `kind = InlineFn`
    pub structs: Vec<StructDef>,  //              indexed by `col_index` when `kind = Struct`
    pub enums: Vec<EnumDef>,      //              indexed by `col_index` when `kind = Enum`
    pub enum_ctors: Vec<EnumConstructor>, //      indexed by `col_index` when `kind = EnumConstructor`

    pub memory: Option<MemoryInfo>,
    pub data_size: UBCell<u32>,

    pub str_literal_type: Option<Type>,

    pub expr_id_count: usize,
}

impl Registry {
    pub fn new() -> Box<Self> {
        let mut it = Box::new(Self::default());
        it.reporter.registry = UBRef::new(&*it);
        it.files.push(FileInfo {
            index: 0,
            included_times: 0,
            absolute_path: String::from("<internal>"),
            source: String::from(""),
        });
        it
    }

    pub fn include_file(&mut self, relative_path: &str, loc: &Loc) -> Option<ModuleId> {
        let file_id = catch!(self.include_file_contents(relative_path, loc), err, {
            self.reporter.error(err);
            return None;
        });

        let file_is_newly_added = self.files[file_id].included_times == 1;

        if self.reporter.in_inspection_mode {
            self.reporter
                .print_include_info(file_is_newly_added, file_id, loc);
        }

        if !file_is_newly_added {
            return Some(self.get_module_id_by_file_id(file_id));
        }

        let source = self.files[file_id].source.as_bytes().relax();

        let mut lexer = Lexer::new(source, file_id);
        catch!(lexer.lex_file(), err, {
            self.reporter.error(err);
            return None;
        });

        let parser = Parser::new(lexer, &mut self.reporter);

        *parser.expr_id_count.be_mut() = self.expr_id_count;

        catch!(parser.parse_file(), err, {
            self.reporter.error(err);
            return None;
        });

        self.expr_id_count = *parser.expr_id_count;

        let mut includes = Vec::new();

        if !self.in_single_file_mode {
            for expr in &*parser.ast {
                let Some(include_info) = catch!(get_include_info(expr), err, {
                    self.reporter.error(err);
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
            parser,
            includes,
        });

        Some(module_id)
    }

    fn include_file_contents(&mut self, relative_path: &str, loc: &Loc) -> Result<usize, Error> {
        let absolute_path = self.resolve_path(relative_path, loc);

        for file in &mut self.files {
            if file.absolute_path == absolute_path {
                file.included_times += 1;
                return Ok(file.index);
            }
        }

        let file_contents =
            fs::file_read_utf8(&absolute_path).map_err(|message| Error { message, loc: *loc })?;

        let file_id = self.files.len();
        self.files.push(FileInfo {
            index: file_id,
            included_times: 1,
            absolute_path: absolute_path.into(),
            source: file_contents,
        });

        Ok(file_id)
    }

    pub fn resolve_path(&self, file_path: &str, loc: &Loc) -> String {
        let relative_to = &self.files[loc.file_id].absolute_path;

        if !file_path.starts_with('.') {
            return file_path.into();
        }

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

    pub fn get_module_id_by_file_id(&self, file_id: usize) -> ModuleId {
        for module in &self.modules {
            if module.parser.lexer.file_id == file_id {
                return module.id;
            }
        }

        unreachable!()
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
