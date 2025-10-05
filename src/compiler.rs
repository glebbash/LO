use crate::{ast::*, core::*, lexer::*, parser::*, wasm::*};
use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::{cell::RefCell, fmt::Write};

// TODO: make globals, macros and consts be stored in module's items
// TODO!: change all magic functions to have `@some_magic_fn<...type_args>(...args)` syntax:
//   @sizeof<T>(), @data_size(), @__inspect(), @memory_size(), etc...
// TODO: make sure function names can't collide with intrinsics

#[derive(Clone, PartialEq)]
pub enum LoType {
    Never,
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
    Pointer { pointee: Box<LoType> },
    SequencePointer { pointee: Box<LoType> },
    StructInstance { struct_name: String },
    Result(LoResultType),
}

#[derive(Clone, PartialEq)]
pub struct LoResultType {
    ok: Box<LoType>,
    err: Box<LoType>,
}

impl core::fmt::Display for LoType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LoType::Never => f.write_str("never"),
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
            LoType::Pointer { pointee } => write!(f, "&{pointee}"),
            LoType::SequencePointer { pointee } => write!(f, "*&{pointee}"),
            LoType::StructInstance { struct_name } => f.write_str(&struct_name),
            LoType::Result(result) => write!(f, "Result<{}, {}>", result.ok, result.err),
        }
    }
}

impl LoType {
    fn deref_rec(&self) -> &LoType {
        match self {
            LoType::Pointer { pointee } => pointee.deref_rec(),
            LoType::SequencePointer { pointee } => pointee.deref_rec(),
            other => other,
        }
    }
}

struct FnInfo {
    fn_name: String,
    fn_type: LoFnType,
    fn_params: Vec<LoFnParam>,
    fn_source: LoFnSource,
    exported_as: Vec<String>,
    wasm_fn_index: u32,
    definition_loc: LoLocation,
}

struct LoFnParam {
    param_name: String,
    param_type: LoType,
}

impl core::fmt::Display for LoFnParam {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {}", self.param_name, self.param_type)
    }
}

enum LoFnSource {
    Guest {
        ctx: LoExprContext,
        body: &'static CodeBlockExpr,
    },
    Host {
        module_name: String,
        external_fn_name: String,
    },
}

struct LoFnType {
    inputs: Vec<LoType>,
    output: LoType,
}

#[derive(Clone)]
struct LoExprContext {
    module_index: usize,
    fn_index: Option<usize>,
    locals: Vec<LoLocal>,
    next_local_index: u32,
    addr_local_index: Option<u32>,
    scopes: Vec<LoScope>,
}

impl LoExprContext {
    fn new(module_index: usize, fn_index: Option<usize>) -> Self {
        Self {
            module_index,
            fn_index,
            locals: Vec::new(),
            next_local_index: 0,
            addr_local_index: None,
            scopes: Vec::new(),
        }
    }
}

#[derive(Clone)]
struct LoLocal {
    local_index: u32,
    local_type: LoType,
    definition_loc: LoLocation,
    is_fn_param: bool,
}

#[derive(Clone)]
enum LoScopeType {
    Function,
    Block,
    Loop,
    ForLoop,
    Macro,
}

#[derive(Clone)]
struct LoCodeUnit {
    type_: LoType,
    instrs: Vec<WasmInstr>,
}

#[derive(Clone)]
struct LoMacroTypeArg {
    name: String,
    type_: LoType,
}

#[derive(Clone)]
struct LoScope {
    scope_type: LoScopeType,
    locals: Vec<LoScopedLocal>,
    deferred: Vec<LoCodeUnit>,
    macro_args: Vec<LoConstDef>,
    macro_type_args: Vec<LoMacroTypeArg>,
}

#[derive(Clone)]
struct LoScopedLocal {
    local_name: String,
    lo_local_index: usize,
    defined_in_this_scope: bool,
}

impl LoExprContext {
    fn enter_scope(&mut self, scope_type: LoScopeType) {
        let mut new_scope = LoScope {
            scope_type,
            locals: Vec::new(),
            deferred: Vec::new(),
            macro_args: Vec::new(),
            macro_type_args: Vec::new(),
        };

        if let Some(parent_scope) = self.scopes.last() {
            for local in &parent_scope.locals {
                new_scope.locals.push(LoScopedLocal {
                    local_name: local.local_name.clone(),
                    lo_local_index: local.lo_local_index,
                    defined_in_this_scope: false,
                });
            }
        };

        self.scopes.push(new_scope);
    }

    fn exit_scope(&mut self) {
        self.scopes.pop().unwrap();
    }

    fn current_scope(&self) -> &LoScope {
        self.scopes.last().unwrap()
    }

    fn current_scope_mut(&mut self) -> &mut LoScope {
        self.scopes.last_mut().unwrap()
    }

    fn get_local(&self, local_name: &str) -> Option<&LoLocal> {
        for local in &self.current_scope().locals {
            if local.local_name == local_name {
                return Some(&self.locals[local.lo_local_index]);
            }
        }

        None
    }

    fn get_macro_type_arg(&self, type_name: &str) -> Option<&LoType> {
        for scope in self.scopes.iter().rev() {
            for macro_type_arg in &scope.macro_type_args {
                if macro_type_arg.name == type_name {
                    return Some(&macro_type_arg.type_);
                }
            }
        }

        None
    }

    fn get_macro_arg(&self, arg_name: &str) -> Option<&LoConstDef> {
        for scope in self.scopes.iter().rev() {
            for macro_arg in &scope.macro_args {
                if macro_arg.const_name == arg_name {
                    return Some(&macro_arg);
                }
            }
        }

        None
    }
}

#[derive(Clone)]
enum LoTypeDefKind {
    Builtin,
    Struct,
    Alias,
}

#[derive(Clone)]
struct LoTypeDef {
    kind: LoTypeDefKind,
    name: String,
    value: LoType,
    loc: LoLocation,
}

struct LoStructDef {
    struct_name: String,
    fields: Vec<LoStructField>,
    fully_defined: bool, // used for self-reference checks
}

pub struct LoStructField {
    field_name: String,
    field_type: LoType,
    field_layout: LoTypeLayout,
    field_index: u32,
    byte_offset: u32,
    loc: LoLocation,
}

struct LoGlobalDef {
    module_ctx: &'static LoExprContext,
    def_expr: &'static GlobalDefExpr,
    global_type: LoType,
    global_index: u32,
}

#[derive(Default, Debug)]
struct LoTypeLayout {
    primities_count: u32,
    byte_size: u32,
    alignment: u32,
}

enum LoVariableInfo {
    Local {
        local_index: u32,
        local_type: LoType,
        inspect_info: Option<InspectInfo>,
    },
    Global {
        global_index: u32,
        global_type: LoType,
        inspect_info: Option<InspectInfo>,
    },
    Const {
        code_unit: &'static LoCodeUnit,
        loc: LoLocation,
        inspect_info: Option<InspectInfo>,
    },
    Stored {
        address: LoCodeUnit,
        offset: u32,
        value_type: LoType,
        inspect_info: Option<InspectInfo>,
    },
    StructValueField {
        struct_value: LoCodeUnit,
        field_type: LoType,
        drops_before: u32,
        drops_after: u32,
        loc: LoLocation,
        inspect_info: Option<InspectInfo>,
    },
}

impl LoVariableInfo {
    fn get_type(&self) -> &LoType {
        match self {
            LoVariableInfo::Local {
                local_index: _,
                local_type,
                inspect_info: _,
            } => local_type,
            LoVariableInfo::Global {
                global_index: _,
                global_type,
                inspect_info: _,
            } => global_type,
            LoVariableInfo::Const {
                code_unit,
                loc: _,
                inspect_info: _,
            } => &code_unit.type_,
            LoVariableInfo::Stored {
                address: _,
                offset: _,
                value_type,
                inspect_info: _,
            } => value_type,
            LoVariableInfo::StructValueField {
                struct_value: _,
                field_type,
                drops_before: _,
                drops_after: _,
                loc: _,
                inspect_info: _,
            } => field_type,
        }
    }

    fn inspect_info(&self) -> &Option<InspectInfo> {
        match self {
            LoVariableInfo::Local {
                local_index: _,
                local_type: _,
                inspect_info,
            }
            | LoVariableInfo::Global {
                global_index: _,
                global_type: _,
                inspect_info,
            }
            | LoVariableInfo::Const {
                code_unit: _,
                loc: _,
                inspect_info,
            }
            | LoVariableInfo::Stored {
                address: _,
                offset: _,
                value_type: _,
                inspect_info,
            }
            | LoVariableInfo::StructValueField {
                struct_value: _,
                field_type: _,
                drops_before: _,
                drops_after: _,
                loc: _,
                inspect_info,
            } => inspect_info,
        }
    }
}

struct InspectInfo {
    message: String,
    loc: LoLocation,
    linked_loc: Option<LoLocation>,
}

#[derive(Clone)]
struct LoConstDef {
    const_name: String,
    code_unit: LoCodeUnit,
    loc: LoLocation,
}

#[derive(Clone)]
struct PooledString {
    value: String,
    ptr: u32,
}

#[derive(Clone)]
struct LoStr {
    ptr: u32,
    len: u32,
}

pub struct Module {
    pub index: usize,
    pub file_index: u32,
    pub source: &'static [u8],
    pub ast: AST,
    includes: Vec<ModuleInclude>,
    own_items: Vec<ModuleItem>,
    all_items: Vec<ModuleItem>,
    ctx: LoExprContext,
}

pub struct ModuleInclude {
    module_index: usize,
    include_expr: &'static IncludeExpr,
}

#[derive(Clone)]
struct ModuleItem {
    name: String,
    collection: ModuleItemCollection,
    collection_index: usize,
    loc: LoLocation,
}

#[derive(Clone, Debug)]
enum ModuleItemCollection {
    Type,
    Function,
}

impl Module {
    fn get_item(&self, item_name: &str) -> Option<&ModuleItem> {
        for item in &self.all_items {
            if item.name == item_name {
                return Some(item);
            }
        }

        None
    }

    fn get_own_item(&self, item_name: &str) -> Option<&ModuleItem> {
        for item in &self.own_items {
            if item.name == item_name {
                return Some(item);
            }
        }

        None
    }
}

#[derive(Default)]
pub struct Compiler {
    pub in_inspection_mode: bool,
    pub in_single_file_mode: bool,

    pub fm: FileManager,
    pub modules: Vec<Module>,
    pub error_count: RefCell<u32>,
    pub warning_count: RefCell<u32>,

    type_defs: Vec<LoTypeDef>,
    struct_defs: Vec<LoStructDef>,
    globals: Vec<LoGlobalDef>,
    const_defs: Vec<LoConstDef>,
    macro_defs: Vec<&'static MacroDefExpr>,

    functions: Vec<FnInfo>,

    memory: Option<MemoryDefExpr>,
    memory_imported_from: Option<String>,
    datas: RefCell<Vec<WasmData>>,
    string_pool: RefCell<Vec<PooledString>>,
    data_size: RefCell<u32>,

    wasm_types: RefCell<Vec<WasmFnType>>,
}

impl Compiler {
    pub fn new() -> Self {
        let mut self_ = Self::default();

        self_.type_defs.push(LoTypeDef {
            kind: LoTypeDefKind::Builtin,
            name: String::from("never"),
            value: LoType::Never,
            loc: LoLocation::internal(),
        });
        self_.type_defs.push(LoTypeDef {
            kind: LoTypeDefKind::Builtin,
            name: String::from("void"),
            value: LoType::Void,
            loc: LoLocation::internal(),
        });
        self_.type_defs.push(LoTypeDef {
            kind: LoTypeDefKind::Builtin,
            name: String::from("bool"),
            value: LoType::Bool,
            loc: LoLocation::internal(),
        });
        self_.type_defs.push(LoTypeDef {
            kind: LoTypeDefKind::Builtin,
            name: String::from("u8"),
            value: LoType::U8,
            loc: LoLocation::internal(),
        });
        self_.type_defs.push(LoTypeDef {
            kind: LoTypeDefKind::Builtin,
            name: String::from("i8"),
            value: LoType::I8,
            loc: LoLocation::internal(),
        });
        self_.type_defs.push(LoTypeDef {
            kind: LoTypeDefKind::Builtin,
            name: String::from("u16"),
            value: LoType::U16,
            loc: LoLocation::internal(),
        });
        self_.type_defs.push(LoTypeDef {
            kind: LoTypeDefKind::Builtin,
            name: String::from("i16"),
            value: LoType::I16,
            loc: LoLocation::internal(),
        });
        self_.type_defs.push(LoTypeDef {
            kind: LoTypeDefKind::Builtin,
            name: String::from("u32"),
            value: LoType::U32,
            loc: LoLocation::internal(),
        });
        self_.type_defs.push(LoTypeDef {
            kind: LoTypeDefKind::Builtin,
            name: String::from("i32"),
            value: LoType::I32,
            loc: LoLocation::internal(),
        });
        self_.type_defs.push(LoTypeDef {
            kind: LoTypeDefKind::Builtin,
            name: String::from("f32"),
            value: LoType::F32,
            loc: LoLocation::internal(),
        });
        self_.type_defs.push(LoTypeDef {
            kind: LoTypeDefKind::Builtin,
            name: String::from("u64"),
            value: LoType::U64,
            loc: LoLocation::internal(),
        });
        self_.type_defs.push(LoTypeDef {
            kind: LoTypeDefKind::Builtin,
            name: String::from("i64"),
            value: LoType::I64,
            loc: LoLocation::internal(),
        });
        self_.type_defs.push(LoTypeDef {
            kind: LoTypeDefKind::Builtin,
            name: String::from("f64"),
            value: LoType::F64,
            loc: LoLocation::internal(),
        });

        return self_;
    }

    pub fn begin_inspection(&mut self) {
        self.in_inspection_mode = true;
        stdout_enable_buffering();
        stdout_writeln("[");
    }

    pub fn end_inspection(&mut self) {
        self.in_inspection_mode = false;
        // this item is a stub to make json array valid
        //   as last inspection ended with a comma
        stdout_writeln("{ \"type\": \"end\" }");
        stdout_writeln("]");
        stdout_disable_buffering();
    }

    pub fn import(&mut self, relative_path: &str, loc: &LoLocation) -> Option<&Module> {
        let file = catch!(self.fm.include_file(relative_path, loc), err, {
            self.report_error(&err);
            return None;
        });

        if self.in_inspection_mode {
            if file.is_newly_added {
                let file_index = file.index;
                let file_path = self.fm.get_file_path(file.index);
                stdout_writeln(format!(
                    "{{ \"type\": \"file\", \
                        \"index\": {file_index}, \
                        \"path\": \"{file_path}\" }},",
                ));
            }

            if loc.file_index != 0 {
                let source_index = loc.file_index;
                let source_range = RangeDisplay(loc);
                let target_index = file.index;
                let target_range = "1:1-1:1";

                stdout_writeln(format!(
                    "{{ \"type\": \"info\", \
                    \"link\": \"{target_index}/{target_range}\", \
                    \"loc\": \"{source_index}/{source_range}\" }},",
                ));
            }
        }

        if !file.is_newly_added {
            return self.get_module_by_file_index(file.index);
        }

        let source = self.fm.get_file_source(file.index);

        let tokens = catch!(Lexer::lex(source, file.index), err, {
            self.report_error(&err);
            return None;
        });

        let ast = catch!(Parser::parse(source, tokens), err, {
            self.report_error(&err);
            return None;
        });

        let mut includes = Vec::new();

        if !self.in_single_file_mode {
            for expr in &ast.exprs {
                let TopLevelExpr::Include(include) = expr else {
                    continue;
                };

                let Some(module) = self.import(&include.file_path.unescape(source), &include.loc)
                else {
                    continue;
                };

                includes.push(ModuleInclude {
                    module_index: module.index,
                    include_expr: include.relax(),
                });
            }
        }

        let module_index = self.modules.len();
        let ctx = LoExprContext::new(module_index, None);

        self.modules.push(Module {
            index: module_index,
            file_index: file.index,
            source,
            ast,
            ctx,
            includes,
            own_items: Vec::new(),
            all_items: Vec::new(),
        });

        Some(self.modules.last().unwrap())
    }

    pub fn run_passes(&mut self) {
        for module in self.modules.relax_mut() {
            self.pass_collect_own_items(module);
        }

        for module in self.modules.relax_mut() {
            self.pass_collect_all_items(module);
        }

        for module in self.modules.relax() {
            self.pass_build_structs(module);
        }

        for module in self.modules.relax() {
            self.pass_main(module);
        }
    }

    fn pass_collect_own_items(&mut self, module: &mut Module) {
        for expr in &module.ast.exprs {
            match expr {
                TopLevelExpr::StructDef(StructDefExpr {
                    struct_name,
                    fields: _,
                    loc: _,
                }) => {
                    if let Some(item) = module.get_own_item(&struct_name.repr) {
                        self.report_error(&LoError {
                            message: format!(
                                "Cannot redefine {}, already defined at {}",
                                struct_name.repr,
                                item.loc.to_string(&self.fm)
                            ),
                            loc: struct_name.loc.clone(),
                        });
                        continue;
                    }

                    self.struct_defs.push(LoStructDef {
                        struct_name: struct_name.repr.clone(),
                        fields: Vec::new(),
                        fully_defined: false,
                    });

                    self.type_defs.push(LoTypeDef {
                        name: struct_name.repr.clone(),
                        value: LoType::StructInstance {
                            struct_name: struct_name.repr.clone(),
                        },
                        kind: LoTypeDefKind::Struct,
                        loc: struct_name.loc.clone(),
                    });

                    module.own_items.push(ModuleItem {
                        name: struct_name.repr.clone(),
                        collection: ModuleItemCollection::Type,
                        collection_index: self.type_defs.len() - 1,
                        loc: struct_name.loc.clone(),
                    })
                }
                TopLevelExpr::TypeDef(TypeDefExpr {
                    type_name,
                    type_value,
                    loc: _,
                }) => {
                    if let Some(item) = module.get_own_item(&type_name.repr) {
                        self.report_error(&LoError {
                            message: format!(
                                "Cannot redefine {}, already defined at {}",
                                type_name.repr,
                                item.loc.to_string(&self.fm)
                            ),
                            loc: type_name.loc.clone(),
                        });
                        continue;
                    }

                    let type_value = self.build_type(&module.ctx, &type_value);
                    let type_value = catch!(type_value, err, {
                        self.report_error(&err);
                        continue;
                    });

                    self.type_defs.push(LoTypeDef {
                        kind: LoTypeDefKind::Alias,
                        name: type_name.repr.clone(),
                        value: type_value,
                        loc: type_name.loc.clone(),
                    });

                    module.own_items.push(ModuleItem {
                        name: type_name.repr.clone(),
                        collection: ModuleItemCollection::Type,
                        collection_index: self.type_defs.len() - 1,
                        loc: type_name.loc.clone(),
                    })
                }
                TopLevelExpr::FnDef(fn_def) => {
                    if let Some(item) = module.get_own_item(&fn_def.decl.fn_name.repr) {
                        self.report_error(&LoError {
                            message: format!(
                                "Cannot redefine {}, already defined at {}",
                                fn_def.decl.fn_name.repr,
                                item.loc.to_string(&self.fm)
                            ),
                            loc: fn_def.decl.fn_name.loc.clone(),
                        });

                        // continue processing, this will make an item with duplicate name
                        //   but this is fine because these items will be linked to different functions
                        //   and only the first defined item will be accessible by name
                    }

                    let mut ctx = LoExprContext::new(module.index, Some(self.functions.len()));
                    ctx.enter_scope(LoScopeType::Function);

                    let mut exported_as = Vec::new();
                    if fn_def.exported {
                        exported_as.push(fn_def.decl.fn_name.repr.clone());
                    }

                    // fill in as much as possible before types can be resolved
                    self.functions.push(FnInfo {
                        fn_name: fn_def.decl.fn_name.repr.clone(),
                        fn_type: LoFnType {
                            inputs: Vec::new(),
                            output: LoType::Void,
                        },
                        fn_params: Vec::new(),
                        fn_source: LoFnSource::Guest {
                            ctx,
                            body: fn_def.body.relax(),
                        },
                        exported_as,
                        wasm_fn_index: u32::MAX, // not known at this point
                        definition_loc: fn_def.decl.fn_name.loc.clone(),
                    });

                    module.own_items.push(ModuleItem {
                        name: fn_def.decl.fn_name.repr.clone(),
                        collection: ModuleItemCollection::Function,
                        collection_index: self.functions.len() - 1,
                        loc: fn_def.decl.fn_name.loc.clone(),
                    });
                }
                TopLevelExpr::Import(ImportExpr {
                    module_name,
                    items,
                    loc: _,
                }) => {
                    let module_name = module_name.unescape(module.source);

                    for item in items {
                        let ImportItem::FnDecl(fn_decl) = item else {
                            continue;
                        };

                        if let Some(item) = module.get_own_item(&fn_decl.fn_name.repr) {
                            self.report_error(&LoError {
                                message: format!(
                                    "Cannot redefine {}, already defined at {}",
                                    fn_decl.fn_name.repr,
                                    item.loc.to_string(&self.fm)
                                ),
                                loc: fn_decl.fn_name.loc.clone(),
                            });

                            // continue processing, this will make an item with duplicate name
                            //   but this is fine because these items will be linked to different functions
                            //   and only the first defined item will be accessible by name
                        }

                        self.functions.push(FnInfo {
                            fn_name: fn_decl.fn_name.repr.clone(),
                            fn_type: LoFnType {
                                inputs: Vec::new(),
                                output: LoType::Void,
                            },
                            fn_params: Vec::new(),
                            fn_source: LoFnSource::Host {
                                module_name: module_name.clone(),
                                external_fn_name: fn_decl.fn_name.parts.last().unwrap().clone(),
                            },
                            exported_as: Vec::new(),
                            wasm_fn_index: u32::MAX, // not known at this point
                            definition_loc: fn_decl.fn_name.loc.clone(),
                        });

                        module.own_items.push(ModuleItem {
                            name: fn_decl.fn_name.repr.clone(),
                            collection: ModuleItemCollection::Function,
                            collection_index: self.functions.len() - 1,
                            loc: fn_decl.fn_name.loc.clone(),
                        });
                    }
                }
                _ => {} // skip, not interested
            }
        }
    }

    fn pass_collect_all_items(&mut self, module: &mut Module) {
        self.inline_includes(module.relax_mut(), module, &mut String::from(""), true);
    }

    fn inline_includes(
        &mut self,
        includer: &mut Module,
        includee: &Module,
        prefix: &mut String,
        force_go_deeper: bool,
    ) {
        for item in &includee.own_items {
            includer.all_items.push(ModuleItem {
                name: format!("{}{}", prefix, item.name),
                ..item.clone()
            })
        }

        let original_prefix_len = prefix.len();
        for include in &includee.includes {
            if !(include.include_expr.with_extern || force_go_deeper) {
                continue;
            }

            if let Some(alias) = &include.include_expr.alias {
                prefix.push_str(&alias.repr);
                prefix.push_str("::");
            }

            let included_module = self.modules[include.module_index].relax();
            self.inline_includes(includer, included_module, prefix, false);
            prefix.truncate(original_prefix_len);
        }
    }

    fn pass_build_structs(&mut self, module: &Module) {
        'exprs: for expr in &module.ast.exprs {
            match expr {
                TopLevelExpr::StructDef(StructDefExpr {
                    struct_name,
                    fields,
                    loc: _,
                }) => {
                    let mut struct_fields = Vec::<LoStructField>::new();
                    let mut struct_primitives_count = 0;
                    let mut struct_aligment = 1;

                    'fields: for field in fields {
                        for existing_field in &struct_fields {
                            if existing_field.field_name == field.field_name.repr {
                                self.report_error(&LoError {
                                    message: format!(
                                        "Cannot redefine struct field '{}', already defined at {}",
                                        field.field_name.repr,
                                        existing_field.loc.to_string(&self.fm),
                                    ),
                                    loc: field.field_name.loc.clone(),
                                });
                                continue 'fields;
                            }
                        }

                        let field_index = struct_primitives_count;
                        let field_type_res = self.build_type_check_ref(
                            &module.ctx,
                            &field.field_type,
                            false,
                            field.field_type.loc(),
                        );
                        let field_type = catch!(field_type_res, err, {
                            self.report_error(&err);
                            continue 'exprs;
                        });
                        let mut field_layout = LoTypeLayout::default();
                        self.get_type_layout(&field_type, &mut field_layout);

                        struct_aligment = u32::max(struct_aligment, field_layout.alignment);
                        struct_primitives_count += field_layout.primities_count;

                        struct_fields.push(LoStructField {
                            field_name: field.field_name.repr.clone(),
                            field_type: field_type.clone(),
                            field_layout,
                            field_index,
                            byte_offset: 0, // will be set during field alignment
                            loc: field.field_name.loc.clone(),
                        });
                    }

                    // align fields
                    let mut byte_offset = 0;
                    for field in &mut struct_fields {
                        byte_offset = align(byte_offset, field.field_layout.alignment);

                        field.byte_offset = byte_offset;

                        byte_offset += field.field_layout.byte_size;
                    }

                    let struct_def = self.get_struct_def_mut(&struct_name.repr).unwrap();
                    struct_def.fields.append(&mut struct_fields);
                    struct_def.fully_defined = true;
                }
                _ => {} // skip, not interested
            }
        }
    }

    fn pass_main(&mut self, module: &Module) {
        for expr in &module.ast.exprs {
            match expr {
                TopLevelExpr::Include(_) => {}   // skip, processed in parse_file_tree
                TopLevelExpr::StructDef(_) => {} // skip, processed in pass_build_structs
                // debug tools, will have different syntax or just be removed later
                TopLevelExpr::TypeDef(TypeDefExpr {
                    type_name,
                    type_value: _,
                    loc: _,
                }) => {
                    if type_name.repr.starts_with("__inspect_") {
                        let command = type_name.repr.replacen("__inspect_", "", 1);

                        if self.in_inspection_mode {
                            match command.as_str() {
                                "items" => {
                                    let mut message = String::new();
                                    for item in &module.all_items {
                                        write!(message, "{}\n", item.name).unwrap();
                                    }

                                    self.print_inspection(&InspectInfo {
                                        message,
                                        loc: type_name.loc.clone(),
                                        linked_loc: None,
                                    });
                                }
                                "includes" => {
                                    let mut message = String::new();
                                    write!(
                                        message,
                                        "current: {}\n\n",
                                        self.fm.get_file_path(module.file_index)
                                    )
                                    .unwrap();

                                    for include in &module.includes {
                                        write!(
                                            message,
                                            "{}\n",
                                            include.include_expr.file_path.unescape(module.source)
                                        )
                                        .unwrap();
                                    }

                                    self.print_inspection(&InspectInfo {
                                        message,
                                        loc: type_name.loc.clone(),
                                        linked_loc: None,
                                    });
                                }
                                "modules" => {
                                    let mut message = String::new();

                                    for module in &self.modules {
                                        write!(
                                            message,
                                            "module: {}:\n",
                                            self.fm.get_file_path(module.file_index)
                                        )
                                        .unwrap();

                                        for include in &module.includes {
                                            write!(
                                                message,
                                                "- {}\n",
                                                include
                                                    .include_expr
                                                    .file_path
                                                    .unescape(module.source)
                                            )
                                            .unwrap();
                                        }
                                    }

                                    self.print_inspection(&InspectInfo {
                                        message,
                                        loc: type_name.loc.clone(),
                                        linked_loc: None,
                                    });
                                }
                                _ => {
                                    self.print_inspection(&InspectInfo {
                                        message: format!("Invalid inspection call"),
                                        loc: type_name.loc.clone(),
                                        linked_loc: None,
                                    });
                                }
                            }
                        }
                    }
                }
                TopLevelExpr::FnDef(fn_def) => {
                    let item = module.get_own_item(&fn_def.decl.fn_name.repr).unwrap();
                    let fn_info = self.functions[item.collection_index].relax_mut();

                    fn_info.fn_type.output = match &fn_def.decl.return_type {
                        Some(return_type) => {
                            catch!(self.build_type(&module.ctx, return_type), err, {
                                self.report_error(&err);
                                continue;
                            })
                        }
                        _ => LoType::Void,
                    };

                    let LoFnSource::Guest {
                        ref mut ctx,
                        body: _,
                    } = &mut fn_info.fn_source
                    else {
                        unreachable!()
                    };

                    'param_loop: for fn_param in &fn_def.decl.fn_params {
                        for var in &ctx.current_scope().locals {
                            if var.local_name == fn_param.param_name.repr {
                                self.report_error(&LoError {
                                    message: format!(
                                        "Duplicate function parameter name: {}",
                                        fn_param.param_name.repr
                                    ),
                                    loc: fn_param.loc.clone(),
                                });
                                continue 'param_loop;
                            }
                        }

                        let param_type =
                            self.get_fn_param_type(&module.ctx, &fn_def.decl.fn_name, fn_param);
                        let param_type = catch!(param_type, err, {
                            self.report_error(&err);
                            continue 'param_loop;
                        });
                        fn_info.fn_type.inputs.push(param_type.clone());

                        fn_info.fn_params.push(LoFnParam {
                            param_name: fn_param.param_name.repr.clone(),
                            param_type: param_type.clone(),
                        });

                        let res = self.define_local(
                            ctx,
                            fn_param.param_name.loc.clone(),
                            fn_param.param_name.repr.clone(),
                            &param_type,
                            true,
                        );
                        catch!(res, err, {
                            self.report_error(&err);
                            continue;
                        });
                    }
                }
                TopLevelExpr::TryExport(TryExportExpr {
                    in_name,
                    out_name,
                    from_root,
                    loc,
                }) => {
                    let mut target_module = module;
                    if *from_root {
                        target_module = &self.modules.last().unwrap();
                    }

                    // TODO: same syntax should probably also be able to export named memories
                    let Ok(fn_info) =
                        self.get_fn_info_for_call(&target_module.ctx, &in_name.repr, loc)
                    else {
                        return; // ignore if it doesn't exist or is not a function
                    };

                    if self.in_inspection_mode {
                        self.print_inspection(&InspectInfo {
                            message: in_name.repr.clone(),
                            loc: loc.clone(),
                            linked_loc: Some(fn_info.definition_loc.clone()),
                        });
                    }

                    fn_info
                        .be_mut()
                        .exported_as
                        .push(out_name.unescape(module.source));
                }
                TopLevelExpr::Import(ImportExpr {
                    module_name,
                    items,
                    loc: _,
                }) => {
                    let module_name = module_name.unescape(module.source);

                    'items: for item in items {
                        let fn_decl = match item {
                            ImportItem::FnDecl(fn_decl) => fn_decl,
                            ImportItem::Memory(memory) => {
                                let res =
                                    self.define_memory(memory.clone(), Some(module_name.clone()));
                                catch!(res, err, {
                                    self.report_error(&err);
                                });
                                continue;
                            }
                        };

                        let item = module.get_own_item(&fn_decl.fn_name.repr).unwrap();
                        let fn_info = self.functions[item.collection_index].relax_mut();

                        for fn_param in &fn_decl.fn_params {
                            let param_type =
                                self.get_fn_param_type(&module.ctx, &fn_decl.fn_name, fn_param);
                            let param_type = catch!(param_type, err, {
                                self.report_error(&err);
                                continue 'items;
                            });
                            fn_info.fn_type.inputs.push(param_type.clone());
                            fn_info.fn_params.push(LoFnParam {
                                param_name: fn_param.param_name.repr.clone(),
                                param_type: param_type.clone(),
                            });
                        }

                        if let Some(return_type) = &fn_decl.return_type {
                            fn_info.fn_type.output =
                                catch!(self.build_type(&module.ctx, &return_type), err, {
                                    self.report_error(&err);
                                    continue 'items;
                                });
                        }
                    }
                }
                TopLevelExpr::GlobalDef(global) => {
                    let existing_global = self.get_global(&global.global_name.repr);
                    if let Some(existing_global) = existing_global {
                        self.report_error(&LoError {
                            message: format!(
                                "Cannot redefine global {}, previously defined at {}",
                                global.global_name.repr,
                                existing_global.def_expr.global_name.loc.to_string(&self.fm),
                            ),
                            loc: global.loc.clone(),
                        });
                        continue;
                    }

                    let value_type = match &global.global_value {
                        GlobalDefValue::Expr(expr) => {
                            catch!(self.ensure_const_expr(expr), err, {
                                self.report_error(&err);
                                // continue processing global def
                            });
                            let value_type = self.get_expr_type(&module.ctx, expr);
                            let value_type = catch!(value_type, err, {
                                self.report_error(&err);
                                continue;
                            });
                            let value_comp_count = self.count_wasm_type_components(&value_type);
                            if value_comp_count != 1 {
                                self.report_error(&LoError {
                                    message: format!(
                                        "Cannot define global with non-primitive type {value_type}",
                                    ),
                                    loc: global.loc.clone(),
                                });
                                continue;
                            }
                            value_type
                        }
                        GlobalDefValue::DataSize => LoType::U32,
                    };

                    if self.in_inspection_mode {
                        let global_name = &global.global_name.repr;

                        self.print_inspection(&InspectInfo {
                            message: format!("global {global_name}: {value_type}"),
                            loc: global.global_name.loc.clone(),
                            linked_loc: None,
                        });
                    }

                    self.globals.push(LoGlobalDef {
                        module_ctx: module.ctx.relax(),
                        def_expr: global.relax(),
                        global_type: value_type,
                        global_index: self.globals.len() as u32,
                    });
                }
                TopLevelExpr::ConstDef(const_def) => {
                    if let Some(existing_const) = self.get_const_def(&const_def.const_name.repr) {
                        self.report_error(&LoError {
                            message: format!(
                                "Cannot redefine constant {}, already defined at {}",
                                const_def.const_name.repr,
                                existing_const.loc.to_string(&self.fm)
                            ),
                            loc: const_def.loc.clone(),
                        });
                        continue;
                    }

                    let code_unit =
                        self.build_code_unit(module.ctx.be_mut(), &const_def.const_value);
                    let code_unit = catch!(code_unit, err, {
                        self.report_error(&err);
                        continue;
                    });

                    if self.in_inspection_mode {
                        let const_name = &const_def.const_name.repr;
                        let const_type = &code_unit.type_;

                        self.print_inspection(&InspectInfo {
                            message: format!("const {const_name}: {const_type}"),
                            loc: const_def.const_name.loc.clone(),
                            linked_loc: None,
                        });
                    }

                    self.const_defs.push(LoConstDef {
                        const_name: const_def.const_name.repr.clone(),
                        code_unit,
                        loc: const_def.loc.clone(),
                    });
                }
                TopLevelExpr::MemoryDef(memory) => {
                    catch!(self.define_memory(memory.clone(), None), err, {
                        self.report_error(&err);
                        continue;
                    });
                }
                TopLevelExpr::MacroDef(macro_def) => {
                    if let Some(existing_macro) = self.get_macro_def(&macro_def.macro_name.repr) {
                        self.report_error(&LoError {
                            message: format!(
                                "Cannot redefine macro {}, already defined at {}",
                                macro_def.macro_name.repr,
                                existing_macro.loc.to_string(&self.fm)
                            ),
                            loc: macro_def.loc.clone(),
                        });
                        continue;
                    }

                    self.macro_defs.push(macro_def.relax());
                }
            }
        }
    }

    // TODO: add local names to debug info
    pub fn generate(&mut self, wasm_module: &mut WasmModule) {
        let mut fn_imports_count = 0;
        for fn_info in &self.functions {
            if let LoFnSource::Host { .. } = fn_info.fn_source {
                fn_imports_count += 1;
            }
        }

        // resolve wasm fn indicies and populate type, import and export sections
        let mut wasm_import_fn_index = 0;
        let mut wasm_fn_index = fn_imports_count;
        for fn_index in 0..self.functions.len() {
            let fn_info = self.functions[fn_index].relax_mut();

            let mut wasm_fn_type = WasmFnType {
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            for lo_input_type in &fn_info.fn_type.inputs {
                self.lower_type(lo_input_type, &mut wasm_fn_type.inputs);
            }
            self.lower_type(&fn_info.fn_type.output, &mut wasm_fn_type.outputs);

            let mut fn_type_index = self.wasm_types.borrow().len() as u32;
            for (existing_fn_type, existing_type_index) in self.wasm_types.borrow().iter().zip(0..)
            {
                if wasm_fn_type == *existing_fn_type {
                    fn_type_index = existing_type_index;
                }
            }
            if fn_type_index == self.wasm_types.borrow().len() as u32 {
                self.wasm_types.borrow_mut().push(wasm_fn_type.clone());
            }

            match &fn_info.fn_source {
                LoFnSource::Guest { ctx: _, body: _ } => {
                    wasm_module.functions.push(fn_type_index);
                    wasm_module.debug_fn_info.push(WasmDebugFnInfo {
                        fn_index: wasm_fn_index,
                        fn_name: fn_info.fn_name.clone(),
                    });

                    fn_info.wasm_fn_index = wasm_fn_index;

                    wasm_fn_index += 1;
                }
                LoFnSource::Host {
                    module_name,
                    external_fn_name,
                } => {
                    fn_info.wasm_fn_index = wasm_import_fn_index;

                    wasm_module.imports.push(WasmImport {
                        module_name: module_name.clone(),
                        item_name: external_fn_name.clone(),
                        item_desc: WasmImportDesc::Func {
                            type_index: fn_type_index,
                        },
                    });
                    wasm_import_fn_index += 1;
                }
            }

            for export_name in &fn_info.exported_as {
                wasm_module.exports.push(WasmExport {
                    export_type: WasmExportType::Func,
                    export_name: export_name.clone(),
                    exported_item_index: fn_info.wasm_fn_index,
                });
            }
        }

        // build function codes
        for fn_info in &self.functions {
            let LoFnSource::Guest { ctx, body } = &fn_info.fn_source else {
                continue;
            };

            let mut ctx = ctx.clone();
            let mut wasm_expr = WasmExpr { instrs: Vec::new() };

            self.codegen_code_block(&mut ctx, &mut wasm_expr.instrs, body, true);

            let mut wasm_locals_flat = Vec::new();
            for local in &ctx.locals {
                if local.is_fn_param {
                    continue;
                }

                self.lower_type(&local.local_type, &mut wasm_locals_flat);
            }

            let mut wasm_locals = Vec::<WasmLocals>::new();
            for wasm_local_type in wasm_locals_flat {
                if let Some(wasm_locals_of_type) = wasm_locals.last_mut() {
                    if wasm_locals_of_type.value_type == wasm_local_type {
                        wasm_locals_of_type.count += 1;
                        continue;
                    }
                }

                wasm_locals.push(WasmLocals {
                    count: 1,
                    value_type: wasm_local_type,
                });
            }

            wasm_module.codes.push(WasmFn {
                locals: wasm_locals,
                expr: wasm_expr,
            });
        }

        if let Some(memory) = &self.memory {
            let limits = WasmLimits {
                min: memory.min_pages.unwrap_or(0),
                max: None,
            };

            if let Some(module_name) = &self.memory_imported_from {
                wasm_module.imports.push(WasmImport {
                    module_name: module_name.clone(),
                    item_name: String::from("memory"),
                    item_desc: WasmImportDesc::Memory(limits),
                });
            } else {
                wasm_module.memories.push(limits);
            }

            if memory.exported {
                wasm_module.exports.push(WasmExport {
                    export_type: WasmExportType::Mem,
                    export_name: String::from("memory"),
                    exported_item_index: 0,
                });
            }
        }

        for static_data_store in self.datas.borrow().iter() {
            wasm_module.datas.push(static_data_store.clone());
        }

        let mut wasm_types_buf = Vec::with_capacity(1);
        for i in 0..self.globals.len() {
            let global = &self.globals[i];
            self.lower_type(&global.global_type, &mut wasm_types_buf);
            let wasm_value_type = wasm_types_buf.pop().unwrap();

            let mut initial_value = WasmExpr { instrs: Vec::new() };

            match &global.def_expr.global_value {
                GlobalDefValue::Expr(expr) => {
                    let res =
                        self.codegen(global.module_ctx.be_mut(), &mut initial_value.instrs, expr);
                    catch!(res, err, {
                        self.report_error(&err);
                    });
                }
                GlobalDefValue::DataSize => initial_value.instrs.push(WasmInstr::I32Const {
                    value: *self.data_size.borrow() as i32,
                }),
            };

            wasm_module.globals.push(WasmGlobal {
                mutable: true,
                value_type: wasm_value_type,
                initial_value,
            });
        }

        wasm_module.types.append(&mut self.wasm_types.borrow_mut());
    }

    fn codegen_code_block(
        &self,
        ctx: &mut LoExprContext,
        instrs: &mut Vec<WasmInstr>,
        body: &CodeBlockExpr,
        void_only: bool,
    ) -> bool {
        let mut terminated_early = false;
        for expr in &body.exprs {
            let expr_type = catch!(self.get_expr_type(ctx, expr), err, {
                self.report_error(&err);
                continue;
            });

            if terminated_early {
                self.report_warning(&LoError {
                    message: format!("Unreachable expression"),
                    loc: expr.loc().clone(),
                });
            }

            if expr_type == LoType::Never {
                terminated_early = true;
            }

            let mut type_layout = LoTypeLayout::default();
            self.get_type_layout(&expr_type, &mut type_layout);
            if type_layout.primities_count > 0 && void_only {
                self.report_error(&LoError {
                    message: format!(
                        "Non void expression in block. Use `let _ = <expr>` to ignore expression result."
                    ),
                    loc: expr.loc().clone(),
                });
            }

            catch!(self.codegen(ctx, instrs, expr), err, {
                self.report_error(&err);
                continue;
            });
        }

        self.emit_deferred(ctx.current_scope(), instrs);

        terminated_early
    }

    fn define_memory(
        &mut self,
        memory: MemoryDefExpr,
        imported_from: Option<String>,
    ) -> Result<(), LoError> {
        if let Some(existing_memory) = &self.memory {
            return Err(LoError {
                message: format!(
                    "Cannot redefine memory, first defined at {}",
                    existing_memory.loc.to_string(&self.fm)
                ),
                loc: memory.loc,
            });
        }

        if let Some(data_start) = memory.data_start {
            *self.data_size.borrow_mut() = data_start;
        }
        self.memory = Some(memory);
        self.memory_imported_from = imported_from;

        Ok(())
    }

    fn get_fn_param_type(
        &self,
        ctx: &LoExprContext,
        fn_name: &IdentExpr,
        fn_param: &FnParam,
    ) -> Result<LoType, LoError> {
        match &fn_param.param_type {
            FnParamType::Self_ | FnParamType::SelfRef => {
                if fn_name.parts.len() == 1 {
                    return Err(LoError {
                        message: format!("Cannot use self param in non-method function"),
                        loc: fn_param.loc.clone(),
                    });
                }

                let self_type_name = &fn_name.parts[0..&fn_name.parts.len() - 1].join("::");
                let mut self_type_loc = fn_name.loc.clone();
                self_type_loc.end_pos = self_type_loc.pos;
                self_type_loc.end_pos.offset += self_type_name.len();
                self_type_loc.end_pos.col += self_type_name.len();
                let self_type = self.get_type_or_err(self_type_name, &self_type_loc)?;

                if let FnParamType::Self_ = fn_param.param_type {
                    return Ok(self_type);
                }

                return Ok(LoType::Pointer {
                    pointee: Box::new(self_type),
                });
            }
            FnParamType::Type { expr } => self.build_type(ctx, &expr),
            FnParamType::Infer { name: _ } => unreachable!(),
        }
    }

    fn build_type(&self, ctx: &LoExprContext, type_expr: &TypeExpr) -> Result<LoType, LoError> {
        return self.build_type_check_ref(ctx, type_expr, true, &LoLocation::internal());
    }

    // builds a type asserting it doesn't have infinite size
    fn build_type_check_ref(
        &self,
        ctx: &LoExprContext,
        type_expr: &TypeExpr,
        is_referenced: bool,
        loc: &LoLocation,
    ) -> Result<LoType, LoError> {
        match type_expr {
            TypeExpr::Named(TypeExprNamed { name }) => {
                if let Some(macro_type_arg) = ctx.get_macro_type_arg(&name.repr) {
                    return Ok(macro_type_arg.clone());
                }

                let lo_type = self.get_type_or_err(&name.repr, &name.loc)?;
                if let LoType::StructInstance { struct_name } = &lo_type {
                    let struct_def = self.get_struct_def(&struct_name).unwrap();
                    if !is_referenced && !struct_def.fully_defined {
                        return Err(LoError {
                            message: format!(
                                "Cannot use partially defined struct '{}' here",
                                struct_name
                            ),
                            loc: loc.clone(),
                        });
                    }
                }
                Ok(lo_type)
            }
            TypeExpr::Pointer(TypeExprPointer { pointee, loc: _ }) => {
                let pointee = Box::new(self.build_type_check_ref(ctx, &pointee, true, loc)?);

                Ok(LoType::Pointer { pointee })
            }
            TypeExpr::SequencePointer(TypeExprSequencePointer { pointee, loc: _ }) => {
                let pointee = Box::new(self.build_type_check_ref(ctx, &pointee, true, loc)?);

                Ok(LoType::SequencePointer { pointee })
            }
            TypeExpr::Result(TypeExprResult {
                ok_type,
                err_type,
                loc: _,
            }) => {
                let ok = Box::new(self.build_type_check_ref(ctx, &ok_type, false, loc)?);
                let err = Box::new(self.build_type_check_ref(ctx, &err_type, false, loc)?);

                Ok(LoType::Result(LoResultType { ok, err }))
            }
            TypeExpr::Of(TypeExprOf {
                container_type,
                item_type: _,
                loc: _,
            }) => {
                let actual_type = self.build_type_check_ref(ctx, container_type, true, loc)?;

                Ok(actual_type)
            }
        }
    }

    fn codegen(
        &self,
        ctx: &mut LoExprContext,
        instrs: &mut Vec<WasmInstr>,
        expr: &CodeExpr,
    ) -> Result<(), LoError> {
        match expr {
            CodeExpr::BoolLiteral(BoolLiteralExpr { value, loc: _ }) => {
                if *value {
                    instrs.push(WasmInstr::I32Const { value: 1 });
                } else {
                    instrs.push(WasmInstr::I32Const { value: 0 });
                }
            }
            CodeExpr::CharLiteral(CharLiteralExpr {
                repr: _,
                value,
                loc: _,
            }) => {
                instrs.push(WasmInstr::I32Const {
                    value: *value as i32,
                });
            }
            CodeExpr::IntLiteral(IntLiteralExpr {
                repr: _,
                value,
                tag,
                loc: _,
            }) => self.codegen_int_const(ctx, instrs, *value as i32, tag.as_deref()),
            CodeExpr::StringLiteral(StringLiteralExpr {
                repr: _,
                value,
                zero_terminated,
                loc,
            }) => {
                let mut value = value.clone();
                if *zero_terminated {
                    value.push('\0');
                }

                let str = self.process_const_string(value, &loc)?;

                if *zero_terminated {
                    instrs.push(WasmInstr::I32Const {
                        value: str.ptr as i32,
                    });

                    return Ok(());
                }

                // emit str struct values
                instrs.push(WasmInstr::I32Const {
                    value: str.ptr as i32,
                });
                instrs.push(WasmInstr::I32Const {
                    value: str.len as i32,
                });
            }
            CodeExpr::StructLiteral(StructLiteralExpr {
                struct_name,
                fields,
                has_trailing_comma: _,
                loc,
            }) => {
                let Some(struct_def) = self.get_struct_def(&struct_name.repr) else {
                    return Err(LoError {
                        message: format!("Unknown struct: {}", struct_name.repr),
                        loc: loc.clone(),
                    });
                };

                for field_index in 0..fields.len() {
                    let field_literal = &fields[field_index];
                    let Some(struct_field) = struct_def.fields.get(field_index) else {
                        return Err(LoError {
                            message: format!("Excess field values"),
                            loc: field_literal.loc.clone(),
                        });
                    };

                    if &field_literal.field_name != &struct_field.field_name {
                        return Err(LoError {
                            message: format!(
                                "Unexpected struct field name, expecting: `{}`",
                                struct_field.field_name
                            ),
                            loc: field_literal.loc.clone(),
                        });
                    }

                    let field_value_type = self.get_expr_type(ctx, &field_literal.value)?;
                    if field_value_type != struct_field.field_type {
                        return Err(LoError {
                            message: format!(
                                "Invalid type for struct field {}.{}, expected: {}, got: {}",
                                struct_name.repr,
                                struct_field.field_name,
                                struct_field.field_type,
                                field_value_type
                            ),
                            loc: field_literal.value.loc().clone(),
                        });
                    }

                    self.codegen(ctx, instrs, &field_literal.value)?;
                }

                if fields.len() < struct_def.fields.len() {
                    let mut missing_fields = Vec::new();
                    for i in fields.len()..struct_def.fields.len() {
                        missing_fields.push(&struct_def.fields[i].field_name)
                    }

                    return Err(LoError {
                        message: format!("Missing struct fields: {}", ListDisplay(&missing_fields)),
                        loc: loc.clone(),
                    });
                }
            }
            // TODO?: support sequences of any type
            CodeExpr::ArrayLiteral(ArrayLiteralExpr {
                item_type,
                items,
                loc,
            }) => {
                let item_type = self.build_type(ctx, item_type)?;

                let mut bytes = Vec::new();
                let mut tmp_instrs = Vec::new();

                if let LoType::U8 = &item_type {
                    for item in items {
                        let current_item_type = self.get_expr_type(ctx, item)?;
                        if current_item_type != item_type {
                            return Err(LoError {
                                message: format!(
                                    "Unexpected array element type: {}, expected: {}",
                                    current_item_type, item_type,
                                ),
                                loc: item.loc().clone(),
                            });
                        }

                        self.codegen(ctx, &mut tmp_instrs, item)?;
                        let WasmInstr::I32Const { value } = tmp_instrs.pop().unwrap() else {
                            return Err(LoError {
                                message: format!("Unexpected array element value"),
                                loc: item.loc().clone(),
                            });
                        };

                        bytes.push(value as u8);
                    }
                } else if let LoType::StructInstance { struct_name } = &item_type
                    && struct_name == "str"
                {
                    for item in items {
                        let current_item_type = self.get_expr_type(ctx, item)?;
                        if current_item_type != item_type {
                            return Err(LoError {
                                message: format!(
                                    "Unexpected array element type: {}, expected: {}",
                                    current_item_type, item_type,
                                ),
                                loc: item.loc().clone(),
                            });
                        }

                        self.codegen(ctx, &mut tmp_instrs, item)?;
                        let WasmInstr::I32Const { value: len } = tmp_instrs.pop().unwrap() else {
                            return Err(LoError {
                                message: format!("Unexpected array element value"),
                                loc: item.loc().clone(),
                            });
                        };
                        let WasmInstr::I32Const { value: ptr } = tmp_instrs.pop().unwrap() else {
                            return Err(LoError {
                                message: format!("Unexpected array element value"),
                                loc: item.loc().clone(),
                            });
                        };

                        bytes.extend_from_slice(&ptr.to_le_bytes());
                        bytes.extend_from_slice(&len.to_le_bytes());
                    }
                } else {
                    return Err(LoError {
                        message: format!("Unsupported array literal element type: {}", item_type),
                        loc: loc.clone(),
                    });
                }

                let ptr = self.append_data(bytes);
                instrs.push(WasmInstr::I32Const { value: ptr as i32 });

                return Ok(());
            }
            CodeExpr::ResultLiteral(ResultLiteralExpr {
                is_ok,
                result_type,
                value,
                loc,
            }) => {
                let result = self.get_result_literal_type(ctx, result_type, loc)?;

                let mut value_type = LoType::Void;
                if let Some(value) = value {
                    value_type = self.get_expr_type(ctx, value)?;
                }

                if *is_ok {
                    if value_type != *result.ok {
                        return Err(LoError {
                            message: format!(
                                "Cannot create result, Ok type mismatch. Got {}, expected: {}",
                                value_type, result.ok
                            ),
                            loc: loc.clone(),
                        });
                    }

                    if let Some(ok_value) = value {
                        self.codegen(ctx, instrs, ok_value)?;
                    }

                    // error value
                    instrs.push(WasmInstr::I32Const { value: 0 });

                    return Ok(());
                }

                if value_type != *result.err {
                    return Err(LoError {
                        message: format!(
                            "Cannot create result, Err type mismatch. Got {}, expected: {}",
                            value_type, result.err
                        ),
                        loc: loc.clone(),
                    });
                }

                self.codegen_default_value(ctx, instrs, &result.ok);
                self.codegen(ctx, instrs, value.as_ref().unwrap())?;
            }

            CodeExpr::Ident(ident) => {
                if let Some(const_def) = self.get_const(ctx, &ident.repr) {
                    if self.in_inspection_mode {
                        self.print_inspection(&InspectInfo {
                            message: format!("const {}: {}", ident.repr, const_def.code_unit.type_),
                            loc: ident.loc.clone(),
                            linked_loc: Some(const_def.loc.clone()),
                        });
                    }

                    for instr in &const_def.code_unit.instrs {
                        instrs.push(instr.clone());
                    }
                    return Ok(());
                }

                let var = self.var_from_ident(ctx, ident)?;
                if let Some(inspect_info) = var.inspect_info() {
                    self.print_inspection(inspect_info);
                }
                self.codegen_var_get(ctx, instrs, &var)?;
            }
            CodeExpr::Let(LetExpr {
                local_name,
                value,
                loc: _,
            }) => {
                let local_type = self.get_expr_type(ctx, &value)?;

                if local_name.repr == "_" {
                    self.codegen(ctx, instrs, value)?;

                    for _ in 0..self.count_wasm_type_components(&local_type) {
                        instrs.push(WasmInstr::Drop);
                    }
                    return Ok(());
                }

                let local_index = self.define_local(
                    ctx,
                    local_name.loc.clone(),
                    local_name.repr.clone(),
                    &local_type,
                    false,
                )?;
                let var = self.var_local(
                    &local_name.repr,
                    local_type,
                    local_index,
                    local_name.loc.clone(),
                    None,
                );
                if let Some(inspect_info) = var.inspect_info() {
                    self.print_inspection(inspect_info);
                }
                self.codegen_var_set_prepare(ctx, instrs, &var);
                self.codegen(ctx, instrs, value)?;
                self.codegen_var_set(ctx, instrs, &var)?;
            }
            CodeExpr::Cast(CastExpr {
                expr,
                casted_to,
                loc,
            }) => {
                self.codegen(ctx, instrs, expr)?;

                let castee_type = self.get_expr_type(ctx, expr)?;
                let casted_to_type = self.build_type(ctx, casted_to)?;

                if let Some(cast_op) = self.get_cast_instr(&castee_type, &casted_to_type) {
                    instrs.push(cast_op);
                    return Ok(());
                }

                let mut castee_type_components = Vec::new();
                self.lower_type(&castee_type, &mut castee_type_components);

                let mut casted_to_type_components = Vec::new();
                self.lower_type(&casted_to_type, &mut casted_to_type_components);

                if castee_type_components != casted_to_type_components {
                    return Err(LoError {
                        message: format!("Cannot cast from {castee_type} to {casted_to_type}"),
                        loc: loc.clone(),
                    });
                }
            }
            CodeExpr::PrefixOp(PrefixOpExpr {
                op_tag,
                expr,
                op_loc,
                loc,
            }) => match op_tag {
                PrefixOpTag::Dereference => {
                    let var = self.var_from_deref(ctx, expr, op_loc)?;
                    if let Some(inspect_info) = var.inspect_info() {
                        self.print_inspection(inspect_info);
                    }
                    self.codegen_var_get(ctx, instrs, &var)?;
                }
                PrefixOpTag::Not => {
                    let operand_type = self.get_expr_type(ctx, expr)?;
                    let mut wasm_components = Vec::new();
                    self.lower_type(&operand_type, &mut wasm_components);
                    if wasm_components.len() != 1 {
                        return Err(LoError {
                            message: format!(
                                "Cannot apply not operation to expr of type {}",
                                operand_type
                            ),
                            loc: loc.clone(),
                        });
                    }
                    match wasm_components[0] {
                        WasmType::I32 => {
                            instrs.push(WasmInstr::I32Const { value: 0 });
                        }
                        WasmType::I64 => {
                            instrs.push(WasmInstr::I64Const { value: 0 });
                        }
                        WasmType::F32 => {
                            instrs.push(WasmInstr::F32Const { value: 0.0 });
                        }
                        WasmType::F64 => {
                            instrs.push(WasmInstr::F64Const { value: 0.0 });
                        }
                        WasmType::FuncRef => {
                            return Err(LoError {
                                message: format!(
                                    "Cannot apply not operation to expr of type {}",
                                    operand_type
                                ),
                                loc: loc.clone(),
                            });
                        }
                    }

                    self.codegen(ctx, instrs, expr)?;
                    instrs.push(WasmInstr::BinaryOp {
                        kind: self.get_binary_op_kind(&InfixOpTag::Equal, &operand_type, loc)?,
                    });
                }
                PrefixOpTag::Positive => {
                    self.codegen(ctx, instrs, expr)?;
                }
                PrefixOpTag::Negative => {
                    if let CodeExpr::IntLiteral(int_literal) = expr.as_ref() {
                        self.codegen_int_const(
                            ctx,
                            instrs,
                            -(int_literal.value as i32),
                            int_literal.tag.as_deref(),
                        );
                        return Ok(());
                    };

                    let operand_type = self.get_expr_type(ctx, expr)?;
                    let mut wasm_components = Vec::new();
                    self.lower_type(&operand_type, &mut wasm_components);
                    if wasm_components.len() != 1 {
                        return Err(LoError {
                            message: format!("Cannot negate expr of type {}", operand_type),
                            loc: loc.clone(),
                        });
                    }
                    match wasm_components[0] {
                        WasmType::I32 => {
                            instrs.push(WasmInstr::I32Const { value: 0 });
                        }
                        WasmType::I64 => {
                            instrs.push(WasmInstr::I64Const { value: 0 });
                        }
                        WasmType::F32 => {
                            instrs.push(WasmInstr::F32Const { value: 0.0 });
                        }
                        WasmType::F64 => {
                            instrs.push(WasmInstr::F64Const { value: 0.0 });
                        }
                        WasmType::FuncRef => {
                            return Err(LoError {
                                message: format!("Cannot negate expr of type {}", operand_type),
                                loc: loc.clone(),
                            });
                        }
                    }

                    self.codegen(ctx, instrs, expr)?;
                    instrs.push(WasmInstr::BinaryOp {
                        kind: self.get_binary_op_kind(&InfixOpTag::Sub, &operand_type, loc)?,
                    });
                }
            },
            CodeExpr::InfixOp(InfixOpExpr {
                op_tag,
                op_loc,
                lhs,
                rhs,
                loc: _,
            }) => {
                let lhs_type = self.get_expr_type(ctx, lhs)?;
                let rhs_type = self.get_expr_type(ctx, rhs)?;

                if lhs_type != rhs_type {
                    return Err(LoError {
                        message: format!(
                            "Operands are not of the same type: lhs = {}, rhs = {}",
                            lhs_type, rhs_type
                        ),
                        loc: op_loc.clone(),
                    });
                }

                if let Some(base_op) = self.get_compound_assignment_base_op(op_tag) {
                    let Some(var) = self.var_from_expr(ctx, &lhs)? else {
                        return Err(LoError {
                            message: format!("Cannot perform compound assignment: invalid lhs"),
                            loc: op_loc.clone(),
                        });
                    };
                    if let Some(inspect_info) = var.inspect_info() {
                        self.print_inspection(inspect_info);
                    }

                    self.codegen_var_set_prepare(ctx, instrs, &var);
                    self.codegen_var_get(ctx, instrs, &var)?;
                    self.codegen(ctx, instrs, rhs)?;
                    let kind = self.get_binary_op_kind(&base_op, &lhs_type, op_loc)?;
                    instrs.push(WasmInstr::BinaryOp { kind });
                    self.codegen_var_set(ctx, instrs, &var)?;
                    return Ok(());
                }

                self.codegen(ctx, instrs, lhs)?;
                self.codegen(ctx, instrs, rhs)?;

                let kind = self.get_binary_op_kind(op_tag, &lhs_type, op_loc)?;
                instrs.push(WasmInstr::BinaryOp { kind });
            }

            CodeExpr::Assign(AssignExpr {
                op_loc,
                lhs,
                rhs,
                loc: _,
            }) => {
                let Some(var) = self.var_from_expr(ctx, lhs)? else {
                    return Err(LoError {
                        message: format!("Cannot perform assignment: invalid lhs"),
                        loc: op_loc.clone(),
                    });
                };
                if let Some(inspect_info) = var.inspect_info() {
                    self.print_inspection(inspect_info);
                }
                self.codegen_var_set_prepare(ctx, instrs, &var);
                self.codegen(ctx, instrs, rhs)?;
                self.codegen_var_set(ctx, instrs, &var)?;
            }
            CodeExpr::FieldAccess(field_access) => {
                let var = self.var_from_field_access(ctx, field_access)?;
                if let Some(inspect_info) = var.inspect_info() {
                    self.print_inspection(inspect_info);
                }
                self.codegen_var_get(ctx, instrs, &var)?;
            }

            CodeExpr::FnCall(FnCallExpr {
                fn_name,
                args,
                loc: _,
            }) => {
                self.codegen_fn_call(ctx, instrs, &fn_name.repr, None, args, &fn_name.loc)?;
            }
            CodeExpr::MethodCall(MethodCallExpr {
                lhs,
                field_name,
                args,
                loc: _,
            }) => {
                let lhs_type = self.get_expr_type(ctx, lhs)?;
                let fn_name = get_fn_name_from_method(&lhs_type, &field_name.repr);
                self.codegen_fn_call(ctx, instrs, &fn_name, Some(lhs), args, &field_name.loc)?;
            }
            CodeExpr::MacroFnCall(MacroFnCallExpr {
                fn_name,
                type_args,
                args,
                loc: _,
            }) => {
                self.codegen_macro_call(
                    ctx,
                    instrs,
                    &fn_name.repr,
                    type_args,
                    None,
                    args,
                    &fn_name.loc,
                )?;
            }
            CodeExpr::MacroMethodCall(MacroMethodCallExpr {
                lhs,
                field_name,
                type_args,
                args,
                loc: _,
            }) => {
                let lhs_type = self.get_expr_type(ctx, lhs)?;
                let macro_name = get_fn_name_from_method(&lhs_type, &field_name.repr);
                self.codegen_macro_call(
                    ctx,
                    instrs,
                    &macro_name,
                    type_args,
                    Some(lhs),
                    args,
                    &field_name.loc,
                )?;
            }

            CodeExpr::Dbg(DbgExpr { message, loc }) => {
                let debug_message = format!(
                    "{} - {}",
                    loc.to_string(&self.fm),
                    message.unescape(self.fm.get_file_source(loc.file_index))
                );
                let str = self.process_const_string(debug_message, loc)?;

                // emit str struct values
                instrs.push(WasmInstr::I32Const {
                    value: str.ptr as i32,
                });
                instrs.push(WasmInstr::I32Const {
                    value: str.len as i32,
                });
            }
            CodeExpr::Sizeof(SizeofExpr { type_expr, loc: _ }) => {
                let lo_type = self.build_type(ctx, type_expr)?;
                let mut type_layout = LoTypeLayout::default();
                self.get_type_layout(&lo_type, &mut type_layout);

                instrs.push(WasmInstr::I32Const {
                    value: type_layout.byte_size as i32,
                });
            }
            CodeExpr::MemorySize(MemorySizeExpr { loc: _ }) => {
                instrs.push(WasmInstr::MemorySize);
            }
            CodeExpr::MemoryGrow(MemoryGrowExpr { args, loc }) => {
                let mut arg_types = Vec::new();
                for arg in args {
                    arg_types.push(self.get_expr_type(ctx, arg)?);
                }
                let param_types = &[LoType::U32];
                if arg_types != param_types {
                    return Err(LoError {
                        message: format!(
                            "Unexpected arguments for __memory_grow: [{}], expected: [{}]",
                            ListDisplay(&arg_types),
                            ListDisplay(param_types)
                        ),
                        loc: loc.clone(),
                    });
                }

                for arg in args {
                    self.codegen(ctx, instrs, arg)?;
                }

                instrs.push(WasmInstr::MemoryGrow);
            }
            CodeExpr::MemoryCopy(MemoryCopyExpr { args, loc }) => {
                let mut arg_types = Vec::new();
                for arg in args {
                    arg_types.push(self.get_expr_type(ctx, arg)?);
                }
                let param_types = &[LoType::U32, LoType::U32, LoType::U32];
                if arg_types != param_types {
                    return Err(LoError {
                        message: format!(
                            "Unexpected arguments for __memory_copy: [{}], expected: [{}]",
                            ListDisplay(&arg_types),
                            ListDisplay(param_types)
                        ),
                        loc: loc.clone(),
                    });
                }

                for arg in args {
                    self.codegen(ctx, instrs, arg)?;
                }

                instrs.push(WasmInstr::MemoryCopy);
            }

            CodeExpr::Return(ReturnExpr { expr, loc }) => {
                let Some(fn_index) = ctx.fn_index else {
                    return Err(LoError {
                        message: format!("Cannot use `return` in const context"),
                        loc: loc.clone(),
                    });
                };

                let mut return_type = LoType::Void;

                if let Some(return_expr) = expr {
                    self.codegen(ctx, instrs, return_expr)?;
                    return_type = self.get_expr_type(ctx, &return_expr)?;
                };

                let fn_return_type = &self.functions[fn_index].fn_type.output;
                if return_type != *fn_return_type && return_type != LoType::Never {
                    return Err(LoError {
                        message: format!(
                            "Invalid return type: {}, expected: {}",
                            return_type, fn_return_type
                        ),
                        loc: loc.clone(),
                    });
                }

                self.emit_deferred_for_return(ctx, instrs);
                instrs.push(WasmInstr::Return);
            }
            CodeExpr::If(IfExpr {
                cond,
                then_block,
                else_block,
                loc: _,
            }) => {
                self.codegen(ctx, instrs, cond)?;

                instrs.push(WasmInstr::BlockStart {
                    block_kind: WasmBlockKind::If,
                    block_type: WasmBlockType::NoOut,
                });

                ctx.enter_scope(LoScopeType::Block);
                self.codegen_code_block(ctx, instrs, &then_block, true);
                ctx.exit_scope();

                match else_block {
                    ElseBlock::None => {}
                    ElseBlock::Else(code_block_expr) => {
                        instrs.push(WasmInstr::Else);
                        ctx.enter_scope(LoScopeType::Block);
                        self.codegen_code_block(ctx, instrs, &code_block_expr, true);
                        ctx.exit_scope();
                    }
                    ElseBlock::ElseIf(code_expr) => {
                        instrs.push(WasmInstr::Else);
                        ctx.enter_scope(LoScopeType::Block);
                        self.codegen(ctx, instrs, &code_expr)?;
                        ctx.exit_scope();
                    }
                }

                instrs.push(WasmInstr::BlockEnd);
            }
            CodeExpr::Loop(LoopExpr { body, loc: _ }) => {
                instrs.push(WasmInstr::BlockStart {
                    block_kind: WasmBlockKind::Block,
                    block_type: WasmBlockType::NoOut,
                });
                instrs.push(WasmInstr::BlockStart {
                    block_kind: WasmBlockKind::Loop,
                    block_type: WasmBlockType::NoOut,
                });

                ctx.enter_scope(LoScopeType::Loop);
                self.codegen_code_block(ctx, instrs, body, true);
                ctx.exit_scope();

                // implicit continue
                instrs.push(WasmInstr::Branch { label_index: 0 });

                instrs.push(WasmInstr::BlockEnd);
                instrs.push(WasmInstr::BlockEnd);
            }
            CodeExpr::ForLoop(ForLoopExpr {
                counter,
                start,
                end,
                body,
                loc,
            }) => {
                let counter_type = self.get_expr_type(ctx, start)?;
                if self.get_expr_type(ctx, end)? != counter_type {
                    return Err(LoError {
                        message: format!(
                            "Invalid range end type: {}, expected: {counter_type}",
                            self.get_expr_type(ctx, end)?
                        ),
                        loc: loc.clone(),
                    });
                }

                ctx.enter_scope(LoScopeType::ForLoop);

                // define counter and set value to start
                let local_index = self.define_local(
                    ctx,
                    counter.loc.clone(),
                    counter.repr.clone(),
                    &counter_type,
                    false,
                )?;
                let counter_var = self.var_local(
                    &counter.repr,
                    counter_type.clone(),
                    local_index,
                    counter.loc.clone(),
                    None,
                );
                if let Some(inspect_info) = counter_var.inspect_info() {
                    self.print_inspection(inspect_info);
                }
                self.codegen_var_set_prepare(ctx, instrs, &counter_var);
                self.codegen(ctx, instrs, start)?;
                self.codegen_var_set(ctx, instrs, &counter_var)?;

                {
                    instrs.push(WasmInstr::BlockStart {
                        block_kind: WasmBlockKind::Block,
                        block_type: WasmBlockType::NoOut,
                    });

                    {
                        instrs.push(WasmInstr::BlockStart {
                            block_kind: WasmBlockKind::Loop,
                            block_type: WasmBlockType::NoOut,
                        });

                        // break if counter is equal to end
                        self.codegen(ctx, instrs, end)?;
                        self.codegen_var_get(ctx, instrs, &counter_var)?;
                        let cmp_kind =
                            self.get_binary_op_kind(&InfixOpTag::Equal, &counter_type, loc)?;
                        instrs.push(WasmInstr::BinaryOp { kind: cmp_kind });
                        instrs.push(WasmInstr::BranchIf { label_index: 1 });

                        {
                            instrs.push(WasmInstr::BlockStart {
                                block_kind: WasmBlockKind::Block,
                                block_type: WasmBlockType::NoOut,
                            });

                            self.codegen_code_block(ctx, instrs, body, true);

                            instrs.push(WasmInstr::BlockEnd);
                        }

                        // increment counter
                        self.codegen_var_get(ctx, instrs, &counter_var)?;
                        self.codegen_var_set_prepare(ctx, instrs, &counter_var);
                        self.codegen_int_const(
                            ctx,
                            instrs,
                            1,
                            Some(counter_type.to_string().as_str()),
                        );
                        let kind = self.get_binary_op_kind(&InfixOpTag::Add, &counter_type, loc)?;
                        instrs.push(WasmInstr::BinaryOp { kind });
                        self.codegen_var_set(ctx, instrs, &counter_var)?;

                        // implicit continue
                        instrs.push(WasmInstr::Branch { label_index: 0 });

                        instrs.push(WasmInstr::BlockEnd);
                    }

                    instrs.push(WasmInstr::BlockEnd);
                }

                ctx.exit_scope();
            }
            CodeExpr::Break(BreakExpr { loc }) => {
                let mut label_index = 1; // 0 = loop, 1 = loop wrapper block

                for scope in ctx.scopes.iter().rev() {
                    match scope.scope_type {
                        LoScopeType::Block => {
                            label_index += 1;
                        }
                        LoScopeType::Function => {
                            return Err(LoError {
                                message: format!("Cannot break outside of a loop"),
                                loc: loc.clone(),
                            });
                        }
                        LoScopeType::Loop => break,
                        LoScopeType::ForLoop => {
                            label_index += 1;
                            break;
                        }
                        LoScopeType::Macro => continue,
                    }
                }

                instrs.push(WasmInstr::Branch { label_index });
            }
            CodeExpr::Continue(ContinueExpr { loc }) => {
                let mut label_index = 0; // 0 = loop, 1 = loop wrapper block

                for scope in ctx.scopes.iter().rev() {
                    match scope.scope_type {
                        LoScopeType::Block => {
                            label_index += 1;
                        }
                        LoScopeType::Function => {
                            return Err(LoError {
                                message: format!("Cannot continue outside of a loop"),
                                loc: loc.clone(),
                            });
                        }
                        LoScopeType::Loop => break,
                        LoScopeType::ForLoop => break,
                        LoScopeType::Macro => continue,
                    }
                }

                instrs.push(WasmInstr::Branch { label_index });
            }
            CodeExpr::With(WithExpr {
                bind,
                args,
                body,
                loc: _,
            }) => {
                for arg in args {
                    ctx.enter_scope(LoScopeType::Block);
                    let arg_type = self.get_expr_type(ctx, arg)?;
                    let arg_local_index = self.define_local(
                        ctx,
                        bind.loc.clone(),
                        bind.repr.clone(),
                        &arg_type,
                        false,
                    )?;
                    self.codegen(ctx, instrs, arg)?;
                    self.codegen_local_set(instrs, &arg_type, arg_local_index);
                    self.codegen_code_block(ctx, instrs, body, true);
                    ctx.exit_scope();
                }
            }
            CodeExpr::Defer(DeferExpr { expr, loc: _ }) => {
                let code_unit = self.build_code_unit(ctx, expr)?;

                // macros defer into parent scope
                if let LoScopeType::Macro = ctx.current_scope().scope_type {
                    let parent_scope_index = ctx.scopes.len() - 2;
                    ctx.scopes[parent_scope_index].deferred.push(code_unit);
                } else {
                    ctx.current_scope_mut().deferred.push(code_unit);
                }
            }
            CodeExpr::Catch(CatchExpr {
                lhs,
                error_bind,
                catch_body,
                catch_loc,
                loc: _,
            }) => {
                self.codegen_catch(
                    ctx,
                    instrs,
                    lhs,
                    Some(&error_bind),
                    Some(catch_body),
                    catch_loc,
                )?;
            }
            CodeExpr::PropagateError(PropagateErrorExpr { expr, loc }) => {
                self.codegen_catch(ctx, instrs, expr, None, None, loc)?;
            }
            CodeExpr::Paren(ParenExpr { expr, loc: _ }) => {
                self.codegen(ctx, instrs, expr)?;
            }
            CodeExpr::Unreachable(_) => {
                instrs.push(WasmInstr::Unreachable);
            }
        };

        Ok(())
    }

    fn codegen_fn_call(
        &self,
        ctx: &mut LoExprContext,
        instrs: &mut Vec<WasmInstr>,
        fn_name: &str,
        receiver_arg: Option<&CodeExpr>,
        args: &Vec<CodeExpr>,
        loc: &LoLocation,
    ) -> Result<(), LoError> {
        let fn_info = self.get_fn_info_for_call(ctx, fn_name, loc)?;

        let mut arg_types = Vec::new();
        if let Some(receiver_arg) = receiver_arg {
            arg_types.push(self.get_expr_type(ctx, receiver_arg)?);
            self.codegen(ctx, instrs, receiver_arg)?;
        }
        for arg in args {
            arg_types.push(self.get_expr_type(ctx, arg)?);
            self.codegen(ctx, instrs, arg)?;
        }

        if self.in_inspection_mode {
            let params = ListDisplay(&fn_info.fn_params);
            let return_type = &fn_info.fn_type.output;
            self.print_inspection(&InspectInfo {
                message: format!("fn {fn_name}({params}): {return_type}"),
                loc: loc.clone(),
                linked_loc: Some(fn_info.definition_loc.clone()),
            });
        }

        if !self.is_types_compatible(&fn_info.fn_type.inputs, &arg_types) {
            return Err(LoError {
                message: format!(
                    "Invalid function arguments for function {}: [{}], expected [{}]",
                    fn_info.fn_name,
                    ListDisplay(&arg_types),
                    ListDisplay(&fn_info.fn_type.inputs),
                ),
                loc: loc.clone(),
            });
        }

        instrs.push(WasmInstr::Call {
            fn_index: fn_info.wasm_fn_index,
        });

        // TODO: insert this kind of logic into other places
        //   like conditionals where each branch resolves to `never`
        if fn_info.fn_type.output == LoType::Never {
            instrs.push(WasmInstr::Unreachable);
        }

        Ok(())
    }

    fn get_fn_info_for_call(
        &self,
        ctx: &LoExprContext,
        fn_name: &str,
        loc: &LoLocation,
    ) -> Result<&FnInfo, LoError> {
        let Some(item) = self.modules[ctx.module_index].get_item(fn_name) else {
            return Err(LoError {
                message: format!("Unknown function: {}", fn_name),
                loc: loc.clone(),
            });
        };

        let ModuleItemCollection::Function = item.collection else {
            return Err(LoError {
                message: format!(
                    "Trying to call {} which is not a function, defined at: {}",
                    fn_name,
                    item.loc.to_string(&self.fm)
                ),
                loc: loc.clone(),
            });
        };

        Ok(&self.functions[item.collection_index])
    }

    fn get_macro_return_type(
        &self,
        ctx: &mut LoExprContext,
        macro_name: &str,
        type_args: &Vec<TypeExpr>,
        args: &Vec<CodeExpr>,
        receiver_arg: Option<&CodeExpr>,
        loc: &LoLocation,
    ) -> Result<LoType, LoError> {
        ctx.enter_scope(LoScopeType::Macro);

        let macro_def = self.populate_ctx_from_macro_call(
            ctx,
            macro_name,
            type_args,
            receiver_arg,
            args,
            loc,
            None,
        )?;

        let return_type = if let Some(return_type) = &macro_def.return_type {
            self.build_type(ctx, return_type)?
        } else {
            LoType::Void
        };

        Ok(return_type)
    }

    fn populate_ctx_from_macro_call(
        &self,
        ctx: &mut LoExprContext,
        macro_name: &str,
        type_args: &Vec<TypeExpr>,
        receiver_arg: Option<&CodeExpr>,
        args: &Vec<CodeExpr>,
        loc: &LoLocation,
        lo_type_args: Option<&mut Vec<LoType>>,
    ) -> Result<&MacroDefExpr, LoError> {
        let Some(macro_def) = self.get_macro_def(macro_name) else {
            return Err(LoError {
                message: format!("Unknown macro: {}", macro_name),
                loc: loc.clone(),
            });
        };

        let mut all_args = Vec::new();
        if let Some(receiver_arg) = receiver_arg {
            all_args.push(self.build_code_unit(ctx, receiver_arg)?);
        }
        for arg in args {
            all_args.push(self.build_code_unit(ctx, arg)?);
        }

        let lo_type_args = match lo_type_args {
            Some(lo_type_args) => lo_type_args,
            None => &mut Vec::new(),
        };
        for type_arg in type_args {
            lo_type_args.push(self.build_type(ctx, &type_arg)?);
        }
        if lo_type_args.len() != macro_def.macro_type_params.len() {
            return Err(LoError {
                message: format!(
                    "Invalid number of type args, expected {}, got {}",
                    macro_def.macro_type_params.len(),
                    type_args.len()
                ),
                loc: loc.clone(),
            });
        }

        // TODO: check for type shadowing
        for (type_param, type_arg) in macro_def.macro_type_params.iter().zip(lo_type_args.iter()) {
            ctx.current_scope_mut()
                .macro_type_args
                .push(LoMacroTypeArg {
                    name: type_param.clone(),
                    type_: type_arg.clone(),
                });
        }

        if all_args.len() != macro_def.macro_params.len() {
            return Err(LoError {
                message: format!(
                    "Invalid number of macro args, expected {}, got {}",
                    macro_def.macro_params.len(),
                    all_args.len()
                ),
                loc: loc.clone(),
            });
        }

        let mut arg_types = Vec::<LoType>::new();
        for arg in &all_args {
            arg_types.push(arg.type_.clone());
        }

        // TODO: check for const shadowing
        for (macro_param, macro_arg) in macro_def.macro_params.iter().zip(all_args.into_iter()) {
            let const_def = LoConstDef {
                const_name: macro_param.param_name.repr.clone(),
                code_unit: macro_arg,
                loc: macro_param.loc.clone(),
            };

            if let FnParamType::Infer { name } = &macro_param.param_type {
                ctx.current_scope_mut()
                    .macro_type_args
                    .push(LoMacroTypeArg {
                        name: name.clone(),
                        type_: const_def.code_unit.type_.clone(),
                    });
            }

            ctx.current_scope_mut().macro_args.push(const_def);
        }

        let mut macro_types = Vec::<LoType>::new();
        for macro_param in &macro_def.macro_params {
            let macro_type = if let FnParamType::Infer { name } = &macro_param.param_type {
                ctx.get_macro_type_arg(name).unwrap().clone()
            } else {
                self.get_fn_param_type(ctx, &macro_def.macro_name, macro_param)?
            };
            macro_types.push(macro_type);
        }

        if !self.is_types_compatible(&macro_types, &arg_types) {
            return Err(LoError {
                message: format!(
                    "Invalid macro args, expected {}, got {}",
                    ListDisplay(&macro_types),
                    ListDisplay(&arg_types)
                ),
                loc: loc.clone(),
            });
        }

        Ok(macro_def)
    }

    fn codegen_macro_call(
        &self,
        ctx: &mut LoExprContext,
        instrs: &mut Vec<WasmInstr>,
        macro_name: &str,
        type_args: &Vec<TypeExpr>,
        receiver_arg: Option<&CodeExpr>,
        args: &Vec<CodeExpr>,
        loc: &LoLocation,
    ) -> Result<(), LoError> {
        ctx.enter_scope(LoScopeType::Macro);

        let mut lo_type_args = Vec::new();
        let macro_def = self.populate_ctx_from_macro_call(
            ctx,
            macro_name,
            type_args,
            receiver_arg,
            args,
            loc,
            Some(&mut lo_type_args),
        )?;

        if self.in_inspection_mode {
            let lo_type_args = ListDisplay(&lo_type_args);

            let mut macro_args = Vec::new();
            let macro_args_len = ctx.current_scope().macro_args.len();
            for i in macro_args_len - macro_def.macro_params.len()..macro_args_len {
                let const_def = &ctx.current_scope().macro_args[i];
                macro_args.push(LoFnParam {
                    param_name: const_def.const_name.clone(),
                    param_type: const_def.code_unit.type_.clone(),
                });
            }
            let lo_args = ListDisplay(&macro_args);

            let return_type = if let Some(return_type) = &macro_def.return_type {
                self.build_type(ctx, return_type)?
            } else {
                LoType::Void
            };

            self.print_inspection(&InspectInfo {
                message: format!("macro {macro_name}!<{lo_type_args}>({lo_args}): {return_type}"),
                loc: loc.clone(),
                linked_loc: Some(macro_def.macro_name.loc.clone()),
            });
        }

        // TODO: type check that block emits only what's defined by return type
        self.codegen_code_block(ctx, instrs, &macro_def.body, false);

        ctx.exit_scope(); // exit macro scope

        Ok(())
    }

    fn codegen_catch(
        &self,
        ctx: &mut LoExprContext,
        instrs: &mut Vec<WasmInstr>,
        expr: &CodeExpr,
        error_bind: Option<&IdentExpr>,
        catch_body: Option<&CodeBlockExpr>,
        loc: &LoLocation,
    ) -> Result<(), LoError> {
        let expr_type = self.get_expr_type(ctx, expr)?;
        let result = self.assert_catchable_type(&expr_type, loc)?;

        ctx.enter_scope(LoScopeType::Block); // enter catch scope

        // put result on the stack
        self.codegen(ctx, instrs, expr)?;

        // pop error
        let (error_bind, error_bind_loc) = if let Some(error_bind) = error_bind {
            (error_bind.repr.clone(), error_bind.loc.clone())
        } else {
            (String::from("<err>"), LoLocation::internal())
        };
        let err_local_index = self.define_local(
            ctx,
            error_bind_loc.clone(),
            error_bind.clone(),
            &result.err,
            false,
        )?;
        let err_var = self.var_local(
            &error_bind,
            result.err.as_ref().clone(),
            err_local_index,
            error_bind_loc.clone(),
            None,
        );
        if error_bind_loc.file_index != 0 {
            if let Some(inspect_info) = err_var.inspect_info() {
                self.print_inspection(inspect_info);
            }
        }
        self.codegen_var_set_prepare(ctx, instrs, &err_var);
        self.codegen_var_set(ctx, instrs, &err_var)?;

        // pop ok
        let ok_bind = String::from("<ok>");
        let ok_local_index = self.define_local(ctx, loc.clone(), ok_bind, &result.ok, false)?;
        self.codegen_local_set(instrs, &result.ok, ok_local_index);

        // cond: error != 0
        self.codegen_var_get(ctx, instrs, &err_var)?;

        let in_out_type_index = self.get_block_inout_type(&[], &result.ok);
        instrs.push(WasmInstr::BlockStart {
            block_kind: WasmBlockKind::If,
            block_type: WasmBlockType::InOut {
                type_index: in_out_type_index,
            },
        });

        // catch error
        if let Some(catch_body) = catch_body {
            let terminated_early = self.codegen_code_block(ctx, instrs, catch_body, true);
            if !terminated_early {
                self.report_error(&LoError {
                    message: format!("Catch expression must resolve to never, got other type"),
                    loc: loc.clone(),
                });
            }
        } else {
            // return ok_type of function's result + caught error
            let fn_result = self.get_result_literal_type(ctx, &None, loc)?;
            self.codegen_default_value(ctx, instrs, &fn_result.ok);
            self.codegen_var_get(ctx, instrs, &err_var)?;

            self.emit_deferred_for_return(ctx, instrs);
            instrs.push(WasmInstr::Return);
        }

        instrs.push(WasmInstr::Else);

        // no error, push ok value
        let ok_var = LoVariableInfo::Local {
            local_index: ok_local_index,
            local_type: result.ok.as_ref().clone(),
            inspect_info: None,
        };
        self.codegen_var_get(ctx, instrs, &ok_var)?;

        instrs.push(WasmInstr::BlockEnd);

        ctx.exit_scope(); // exit catch scope

        Ok(())
    }

    fn codegen_load_or_store(
        &self,
        instrs: &mut Vec<WasmInstr>,
        pointee_type: &LoType,
        offset: u32,
        is_store: bool,
    ) {
        match pointee_type {
            LoType::Never | LoType::Void => {} // noop
            LoType::Bool | LoType::U8 => {
                if is_store {
                    instrs.push(WasmInstr::Store {
                        kind: WasmStoreKind::I32_8,
                        align: 0,
                        offset,
                    })
                } else {
                    instrs.push(WasmInstr::Load {
                        kind: WasmLoadKind::I32U8,
                        align: 0,
                        offset,
                    })
                }
            }
            LoType::I8 => {
                if is_store {
                    instrs.push(WasmInstr::Store {
                        kind: WasmStoreKind::I32_8,
                        align: 0,
                        offset,
                    })
                } else {
                    instrs.push(WasmInstr::Load {
                        kind: WasmLoadKind::I32I8,
                        align: 0,
                        offset,
                    })
                }
            }
            LoType::U16 => {
                if is_store {
                    instrs.push(WasmInstr::Store {
                        kind: WasmStoreKind::I32_16,
                        align: 0,
                        offset,
                    })
                } else {
                    instrs.push(WasmInstr::Load {
                        kind: WasmLoadKind::I32U16,
                        align: 0,
                        offset,
                    })
                }
            }
            LoType::I16 => {
                if is_store {
                    instrs.push(WasmInstr::Store {
                        kind: WasmStoreKind::I32_16,
                        align: 0,
                        offset,
                    })
                } else {
                    instrs.push(WasmInstr::Load {
                        kind: WasmLoadKind::I32I16,
                        align: 0,
                        offset,
                    })
                }
            }
            LoType::U32
            | LoType::I32
            | LoType::Pointer { pointee: _ }
            | LoType::SequencePointer { pointee: _ } => {
                if is_store {
                    instrs.push(WasmInstr::Store {
                        kind: WasmStoreKind::I32,
                        align: 0,
                        offset,
                    })
                } else {
                    instrs.push(WasmInstr::Load {
                        kind: WasmLoadKind::I32,
                        align: 0,
                        offset,
                    })
                }
            }
            LoType::U64 | LoType::I64 => {
                if is_store {
                    instrs.push(WasmInstr::Store {
                        kind: WasmStoreKind::I64,
                        align: 0,
                        offset,
                    })
                } else {
                    instrs.push(WasmInstr::Load {
                        kind: WasmLoadKind::I64,
                        align: 0,
                        offset,
                    })
                }
            }
            LoType::F32 => {
                if is_store {
                    instrs.push(WasmInstr::Store {
                        kind: WasmStoreKind::F32,
                        align: 0,
                        offset,
                    })
                } else {
                    instrs.push(WasmInstr::Load {
                        kind: WasmLoadKind::F32,
                        align: 0,
                        offset,
                    })
                }
            }
            LoType::F64 => {
                if is_store {
                    instrs.push(WasmInstr::Store {
                        kind: WasmStoreKind::F64,
                        align: 0,
                        offset,
                    })
                } else {
                    instrs.push(WasmInstr::Load {
                        kind: WasmLoadKind::F64,
                        align: 0,
                        offset,
                    })
                }
            }
            LoType::StructInstance { struct_name } => {
                let struct_def = self.get_struct_def(struct_name).unwrap();

                for struct_field in struct_def.fields.iter().rev() {
                    self.codegen_load_or_store(
                        instrs,
                        &struct_field.field_type,
                        offset + struct_field.byte_offset,
                        is_store,
                    );
                }
            }
            LoType::Result(_) => todo!(),
        }
    }

    fn get_result_literal_type(
        &self,
        ctx: &LoExprContext,
        explicit_type: &Option<ResultTypeExpr>,
        loc: &LoLocation,
    ) -> Result<LoResultType, LoError> {
        if let Some(result_type) = explicit_type {
            let ok = Box::new(self.build_type(ctx, &result_type.ok)?);
            let err = Box::new(self.build_type(ctx, &result_type.err)?);
            return Ok(LoResultType { ok, err });
        }

        let Some(fn_index) = ctx.fn_index else {
            return Err(LoError {
                message: format!("Cannot create implicitly typed result in const context"),
                loc: loc.clone(),
            });
        };

        let fn_info = &self.functions[fn_index];
        let LoType::Result(result) = &fn_info.fn_type.output else {
            return Err(LoError {
                message: format!(
                    "Cannot create implicitly typed result: function does not return result"
                ),
                loc: loc.clone(),
            });
        };

        Ok(LoResultType {
            ok: result.ok.clone(),
            err: result.err.clone(),
        })
    }

    fn get_block_inout_type(&self, inputs: &[LoType], output: &LoType) -> u32 {
        let mut inout_fn_type = WasmFnType {
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        for input in inputs {
            self.lower_type(input, &mut inout_fn_type.inputs);
        }
        self.lower_type(output, &mut inout_fn_type.outputs);

        for (fn_type, type_index) in self.wasm_types.borrow().iter().zip(0..) {
            if *fn_type == inout_fn_type {
                return type_index;
            }
        }

        self.wasm_types.borrow_mut().push(inout_fn_type);
        self.wasm_types.borrow().len() as u32 - 1
    }

    fn get_expr_type(&self, ctx: &LoExprContext, expr: &CodeExpr) -> Result<LoType, LoError> {
        match expr {
            CodeExpr::BoolLiteral(_) => Ok(LoType::Bool),
            CodeExpr::CharLiteral(_) => Ok(LoType::U8),
            CodeExpr::IntLiteral(IntLiteralExpr {
                repr: _,
                value: _,
                tag,
                loc,
            }) => match tag.as_deref() {
                None => Ok(LoType::U32),
                Some("u8") => Ok(LoType::U8),
                Some("i8") => Ok(LoType::I8),
                Some("u16") => Ok(LoType::U16),
                Some("i16") => Ok(LoType::I16),
                Some("u32") => Ok(LoType::U32),
                Some("i32") => Ok(LoType::I32),
                Some("f32") => Ok(LoType::F32),
                Some("u64") => Ok(LoType::U64),
                Some("i64") => Ok(LoType::I64),
                Some("f64") => Ok(LoType::F64),
                Some(tag) => {
                    return Err(LoError {
                        message: format!("Unknown int literal tag: {tag}"),
                        loc: loc.clone(),
                    })
                }
            },
            CodeExpr::StringLiteral(StringLiteralExpr {
                repr: _,
                value: _,
                zero_terminated,
                loc: _,
            }) => {
                if *zero_terminated {
                    Ok(LoType::SequencePointer {
                        pointee: Box::new(LoType::U8),
                    })
                } else {
                    Ok(LoType::StructInstance {
                        struct_name: String::from("str"),
                    })
                }
            }
            CodeExpr::StructLiteral(StructLiteralExpr {
                struct_name,
                fields: _,
                has_trailing_comma: _,
                loc,
            }) => {
                let Some(_) = self.get_struct_def(&struct_name.repr) else {
                    return Err(LoError {
                        message: format!("Unknown struct: {}", struct_name.repr),
                        loc: loc.clone(),
                    });
                };

                return Ok(LoType::StructInstance {
                    struct_name: struct_name.repr.clone(),
                });
            }
            CodeExpr::ArrayLiteral(ArrayLiteralExpr {
                item_type,
                items: _,
                loc: _,
            }) => {
                let item_type = self.build_type(ctx, item_type)?;
                return Ok(LoType::SequencePointer {
                    pointee: Box::new(item_type),
                });
            }
            CodeExpr::ResultLiteral(ResultLiteralExpr {
                is_ok: _,
                result_type,
                value: _,
                loc,
            }) => {
                let result = self.get_result_literal_type(ctx, result_type, loc)?;
                return Ok(LoType::Result(result));
            }
            CodeExpr::Ident(ident) => {
                if let Some(const_expr) = self.get_const(ctx, &ident.repr) {
                    return Ok(const_expr.code_unit.type_.clone());
                }

                let var = self.var_from_ident(ctx, ident)?;
                Ok(var.get_type().clone())
            }
            CodeExpr::InfixOp(InfixOpExpr {
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
                | InfixOpTag::Or => Ok(LoType::Bool),

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
                | InfixOpTag::ShiftRightAssign => Ok(LoType::Void),

                // have their own CodeExpr variants
                InfixOpTag::Cast
                | InfixOpTag::Assign
                | InfixOpTag::FieldAccess
                | InfixOpTag::Catch
                | InfixOpTag::ErrorPropagation => unreachable!(),
            },
            CodeExpr::PrefixOp(PrefixOpExpr {
                op_tag,
                expr,
                op_loc: _,
                loc,
            }) => match op_tag {
                PrefixOpTag::Not => Ok(LoType::Bool),
                PrefixOpTag::Dereference => {
                    let expr_type = self.get_expr_type(ctx, expr)?;
                    let (LoType::Pointer { pointee } | LoType::SequencePointer { pointee }) =
                        expr_type
                    else {
                        return Err(LoError {
                            message: format!("Cannot dereference expr of type {}", expr_type),
                            loc: loc.clone(),
                        });
                    };
                    Ok(*pointee)
                }
                PrefixOpTag::Positive | PrefixOpTag::Negative => {
                    let expr_type = self.get_expr_type(ctx, expr)?;

                    match expr_type {
                        LoType::U8 => Ok(LoType::I8),
                        LoType::U16 => Ok(LoType::I16),
                        LoType::U32 => Ok(LoType::I32),
                        LoType::U64 => Ok(LoType::I64),
                        LoType::Never
                        | LoType::Void
                        | LoType::Bool
                        | LoType::I8
                        | LoType::I16
                        | LoType::I32
                        | LoType::F32
                        | LoType::I64
                        | LoType::F64
                        | LoType::Pointer { pointee: _ }
                        | LoType::SequencePointer { pointee: _ }
                        | LoType::StructInstance { struct_name: _ }
                        | LoType::Result(_) => Ok(expr_type),
                    }
                }
            },
            CodeExpr::Cast(CastExpr {
                expr: _,
                casted_to,
                loc: _,
            }) => self.build_type(ctx, casted_to),
            CodeExpr::FieldAccess(field_access) => {
                let lhs_type = self.get_expr_type(ctx, &field_access.lhs)?;
                let field = self.get_struct_or_struct_ref_field(ctx, &lhs_type, field_access)?;
                Ok(field.field_type.clone())
            }
            CodeExpr::FnCall(FnCallExpr {
                fn_name,
                args: _,
                loc: _,
            }) => {
                let fn_info = self.get_fn_info_for_call(ctx, &fn_name.repr, &fn_name.loc)?;

                Ok(fn_info.fn_type.output.clone())
            }
            CodeExpr::MethodCall(MethodCallExpr {
                lhs,
                field_name,
                args: _,
                loc: _,
            }) => {
                let lhs_type = self.get_expr_type(ctx, lhs)?;
                let fn_name = get_fn_name_from_method(&lhs_type, &field_name.repr);

                let fn_info = self.get_fn_info_for_call(ctx, &fn_name, &field_name.loc)?;

                Ok(fn_info.fn_type.output.clone())
            }
            CodeExpr::MacroFnCall(MacroFnCallExpr {
                fn_name,
                type_args,
                args,
                loc,
            }) => {
                let mut ctx = ctx.clone();
                self.get_macro_return_type(&mut ctx, &fn_name.repr, type_args, args, None, loc)
            }
            CodeExpr::MacroMethodCall(MacroMethodCallExpr {
                lhs,
                field_name,
                type_args,
                args,
                loc,
            }) => {
                let lhs_type = self.get_expr_type(ctx, lhs)?;
                let macro_name = get_fn_name_from_method(&lhs_type, &field_name.repr);

                let mut ctx = ctx.clone();
                self.get_macro_return_type(&mut ctx, &macro_name, type_args, args, Some(&lhs), loc)
            }
            CodeExpr::Catch(CatchExpr {
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
            CodeExpr::PropagateError(PropagateErrorExpr { expr, loc }) => {
                let expr_type = self.get_expr_type(ctx, expr)?;
                let result = self.assert_catchable_type(&expr_type, loc)?;
                Ok(result.ok.as_ref().clone())
            }
            CodeExpr::Dbg(_) => Ok(LoType::StructInstance {
                struct_name: String::from("str"),
            }),
            CodeExpr::Sizeof(_) => Ok(LoType::U32),
            CodeExpr::MemorySize(_) => Ok(LoType::I32),
            CodeExpr::MemoryGrow(_) => Ok(LoType::I32),
            CodeExpr::MemoryCopy(_) => Ok(LoType::Void),
            CodeExpr::Let(_) => Ok(LoType::Void),
            CodeExpr::Assign(_) => Ok(LoType::Void),
            CodeExpr::Defer(_) => Ok(LoType::Void),
            CodeExpr::If(_) => Ok(LoType::Void),
            CodeExpr::Loop(_) => Ok(LoType::Void),
            CodeExpr::ForLoop(_) => Ok(LoType::Void),
            CodeExpr::Break(_) => Ok(LoType::Never),
            CodeExpr::Continue(_) => Ok(LoType::Never),
            CodeExpr::With(_) => Ok(LoType::Void),
            CodeExpr::Return(_) => Ok(LoType::Never),
            CodeExpr::Unreachable(_) => Ok(LoType::Never),
            CodeExpr::Paren(ParenExpr { expr, loc: _ }) => self.get_expr_type(ctx, expr),
        }
    }

    fn var_from_expr(
        &self,
        ctx: &mut LoExprContext,
        expr: &CodeExpr,
    ) -> Result<Option<LoVariableInfo>, LoError> {
        Ok(match expr {
            CodeExpr::Ident(ident) => Some(self.var_from_ident(ctx, ident)?),
            CodeExpr::FieldAccess(field_access) => {
                Some(self.var_from_field_access(ctx, field_access)?)
            }
            CodeExpr::Paren(ParenExpr { expr, loc: _ }) => self.var_from_expr(ctx, expr)?,
            CodeExpr::PrefixOp(PrefixOpExpr {
                op_tag,
                expr,
                op_loc,
                loc: _,
            }) => match op_tag {
                PrefixOpTag::Dereference => Some(self.var_from_deref(ctx, expr, op_loc)?),
                _ => None,
            },
            _ => None,
        })
    }

    fn var_local(
        &self,
        local_name: &str,
        local_type: LoType,
        local_index: u32,
        loc: LoLocation,
        linked_loc: Option<LoLocation>,
    ) -> LoVariableInfo {
        let inspect_info = if self.in_inspection_mode {
            Some(InspectInfo {
                message: format!("let {}: {}", local_name, local_type),
                loc: loc.clone(),
                linked_loc,
            })
        } else {
            None
        };

        LoVariableInfo::Local {
            local_index,
            local_type,
            inspect_info,
        }
    }

    fn var_from_ident(
        &self,
        ctx: &LoExprContext,
        ident: &IdentExpr,
    ) -> Result<LoVariableInfo, LoError> {
        if let Some(local) = ctx.get_local(&ident.repr) {
            return Ok(self.var_local(
                &ident.repr,
                local.local_type.clone(),
                local.local_index,
                ident.loc.clone(),
                Some(local.definition_loc.clone()),
            ));
        };

        if let Some(global) = self.get_global(&ident.repr) {
            return Ok(LoVariableInfo::Global {
                global_index: global.global_index,
                global_type: global.global_type.clone(),
                inspect_info: if self.in_inspection_mode {
                    Some(InspectInfo {
                        message: format!("global {}: {}", ident.repr, global.global_type),
                        loc: ident.loc.clone(),
                        linked_loc: Some(global.def_expr.loc.clone()),
                    })
                } else {
                    None
                },
            });
        }

        if let Some(const_) = self.get_const(ctx, &ident.repr) {
            return Ok(LoVariableInfo::Const {
                code_unit: const_.code_unit.relax(),
                loc: const_.loc.clone(),
                inspect_info: if self.in_inspection_mode {
                    Some(InspectInfo {
                        message: format!("const {}: {}", ident.repr, const_.code_unit.type_),
                        loc: ident.loc.clone(),
                        linked_loc: Some(const_.loc.clone()),
                    })
                } else {
                    None
                },
            });
        }

        return Err(LoError {
            message: format!("Unknown variable: {}", ident.repr),
            loc: ident.loc.clone(),
        });
    }

    fn var_from_field_access(
        &self,
        ctx: &mut LoExprContext,
        field_access: &FieldAccessExpr,
    ) -> Result<LoVariableInfo, LoError> {
        let lhs_type = self.get_expr_type(ctx, field_access.lhs.as_ref())?;

        let field = self.get_struct_or_struct_ref_field(ctx, &lhs_type, field_access)?;

        let inspect_info = if self.in_inspection_mode {
            Some(InspectInfo {
                message: format!("{}.{}: {}", lhs_type, field.field_name, field.field_type),
                loc: field_access.field_name.loc.clone(),
                linked_loc: Some(field.loc.clone()),
            })
        } else {
            None
        };

        if let LoType::Pointer { pointee: _ } = lhs_type {
            return Ok(LoVariableInfo::Stored {
                address: self.build_code_unit(ctx, &field_access.lhs)?,
                offset: field.byte_offset,
                value_type: field.field_type.clone(),
                inspect_info,
            });
        }

        if let Some(var) = self.var_from_expr(ctx, &field_access.lhs.as_ref())? {
            match var {
                // struct globals are not supported so these are handled the same way as struct values
                LoVariableInfo::Global {
                    global_index: _,
                    global_type: _,
                    inspect_info: _,
                } => {}
                // consts are handled as struct values as well
                LoVariableInfo::Const {
                    code_unit: _,
                    loc: _,
                    inspect_info: _,
                } => {}
                LoVariableInfo::Local {
                    local_index,
                    local_type: _,
                    inspect_info: parent_inspect_info,
                } => {
                    if let Some(inspect_info) = parent_inspect_info {
                        self.print_inspection(&inspect_info);
                    }

                    return Ok(LoVariableInfo::Local {
                        local_index: local_index + field.field_index,
                        local_type: field.field_type.clone(),
                        inspect_info,
                    });
                }
                LoVariableInfo::Stored {
                    address,
                    offset,
                    value_type: _,
                    inspect_info: parent_inspect_info,
                } => {
                    if let Some(inspect_info) = parent_inspect_info {
                        self.print_inspection(&inspect_info);
                    }

                    return Ok(LoVariableInfo::Stored {
                        address,
                        offset: offset + field.byte_offset,
                        value_type: field.field_type.clone(),
                        inspect_info,
                    });
                }
                LoVariableInfo::StructValueField {
                    struct_value,
                    field_type: _,
                    drops_before,
                    drops_after,
                    loc: _,
                    inspect_info: parent_inspect_info,
                } => {
                    if let Some(inspect_info) = parent_inspect_info {
                        self.print_inspection(&inspect_info);
                    }

                    let struct_components_count = self.count_wasm_type_components(&lhs_type);
                    let field_components_count = self.count_wasm_type_components(&field.field_type);

                    return Ok(LoVariableInfo::StructValueField {
                        struct_value,
                        field_type: field.field_type.clone(),
                        drops_before: drops_before + struct_components_count
                            - field.field_index
                            - field_components_count,
                        drops_after: drops_after + field.field_index,
                        loc: field_access.field_name.loc.clone(),
                        inspect_info,
                    });
                }
            };
        };

        let struct_components_count = self.count_wasm_type_components(&lhs_type);
        let field_components_count = self.count_wasm_type_components(&field.field_type);

        return Ok(LoVariableInfo::StructValueField {
            struct_value: self.build_code_unit(ctx, &field_access.lhs)?,
            field_type: field.field_type.clone(),
            drops_before: struct_components_count - field.field_index - field_components_count,
            drops_after: field.field_index,
            loc: field_access.field_name.loc.clone(),
            inspect_info,
        });
    }

    fn create_or_get_addr_local(&self, ctx: &mut LoExprContext) -> u32 {
        if let Some(addr_local_index) = ctx.addr_local_index {
            return addr_local_index;
        }

        let addr_local_index =
            self.define_unnamed_local(ctx, LoLocation::internal(), &LoType::U32, false);

        return addr_local_index;
    }

    fn get_struct_or_struct_ref_field(
        &self,
        _ctx: &LoExprContext,
        mut lhs_type: &LoType,
        field_access: &FieldAccessExpr,
    ) -> Result<&LoStructField, LoError> {
        if let LoType::Pointer { pointee } = &lhs_type {
            lhs_type = pointee;
        }

        let LoType::StructInstance { struct_name } = lhs_type else {
            return Err(LoError {
                message: format!(
                    "Cannot get field '{}' on non struct: {lhs_type}",
                    field_access.field_name.repr
                ),
                loc: field_access.field_name.loc.clone(),
            });
        };

        let struct_def = self.get_struct_def(&struct_name).unwrap();
        let Some(field) = struct_def
            .fields
            .iter()
            .find(|f| &f.field_name == &field_access.field_name.repr)
        else {
            return Err(LoError {
                message: format!(
                    "Unknown field {} in struct {struct_name}",
                    field_access.field_name.repr
                ),
                loc: field_access.field_name.loc.clone(),
            });
        };

        Ok(field)
    }

    fn var_from_deref(
        &self,
        ctx: &mut LoExprContext,
        addr_expr: &CodeExpr,
        op_loc: &LoLocation,
    ) -> Result<LoVariableInfo, LoError> {
        let addr_type = self.get_expr_type(ctx, addr_expr)?;

        if let LoType::Pointer { pointee } = &addr_type {
            let inspect_info = if self.in_inspection_mode {
                Some(InspectInfo {
                    message: format!("<deref>: {}", pointee),
                    loc: op_loc.clone(),
                    linked_loc: None,
                })
            } else {
                None
            };

            return Ok(LoVariableInfo::Stored {
                address: self.build_code_unit(ctx, addr_expr)?,
                offset: 0,
                value_type: pointee.as_ref().clone(),
                inspect_info,
            });
        };

        return Err(LoError {
            message: format!("Cannot dereference expression of type '{}'", addr_type),
            loc: addr_expr.loc().clone(),
        });
    }

    fn codegen_var_get(
        &self,
        ctx: &mut LoExprContext,
        instrs: &mut Vec<WasmInstr>,
        var: &LoVariableInfo,
    ) -> Result<(), LoError> {
        match var {
            LoVariableInfo::Local {
                local_index,
                local_type,
                inspect_info: _,
            } => {
                for i in 0..self.count_wasm_type_components(local_type) {
                    instrs.push(WasmInstr::LocalGet {
                        local_index: local_index + i,
                    });
                }
            }
            LoVariableInfo::Global {
                global_index,
                global_type,
                inspect_info: _,
            } => {
                for i in 0..self.count_wasm_type_components(global_type) {
                    instrs.push(WasmInstr::GlobalGet {
                        global_index: global_index + i,
                    });
                }
            }
            LoVariableInfo::Const {
                code_unit,
                loc: _,
                inspect_info: _,
            } => {
                instrs.extend_from_slice(&code_unit.instrs);
            }
            LoVariableInfo::Stored {
                address,
                offset,
                value_type,
                inspect_info: _,
            } => {
                let mut loads = Vec::new();
                self.codegen_load_or_store(&mut loads, &value_type, *offset, false);

                if loads.len() == 0 {
                    return Ok(());
                }

                for instr in &address.instrs {
                    instrs.push(instr.clone());
                }

                if loads.len() > 1 {
                    let addr_local_index = self.create_or_get_addr_local(ctx);
                    instrs.push(WasmInstr::LocalSet {
                        local_index: addr_local_index,
                    });

                    for load in loads.into_iter().rev() {
                        instrs.push(WasmInstr::LocalGet {
                            local_index: addr_local_index,
                        });
                        instrs.push(load);
                    }
                } else {
                    instrs.append(&mut loads);
                }
            }
            LoVariableInfo::StructValueField {
                struct_value,
                field_type,
                drops_before,
                drops_after,
                loc,
                inspect_info: _,
            } => {
                for instr in &struct_value.instrs {
                    instrs.push(instr.clone());
                }
                for _ in 0..*drops_before {
                    instrs.push(WasmInstr::Drop);
                }

                if *drops_after > 0 {
                    let local_index =
                        self.define_unnamed_local(ctx, loc.clone(), field_type, false);

                    let var = LoVariableInfo::Local {
                        local_index,
                        local_type: field_type.clone(),
                        inspect_info: None,
                    };
                    self.codegen_var_set_prepare(ctx, instrs, &var);
                    self.codegen_var_set(ctx, instrs, &var)?;

                    for _ in 0..*drops_after {
                        instrs.push(WasmInstr::Drop);
                    }

                    self.codegen_var_get(ctx, instrs, &var)?;
                }
            }
        }

        Ok(())
    }

    // should be called before set's value is pushed to the stack
    fn codegen_var_set_prepare(
        &self,
        _ctx: &mut LoExprContext,
        instrs: &mut Vec<WasmInstr>,
        var: &LoVariableInfo,
    ) {
        match var {
            LoVariableInfo::Stored {
                address,
                offset: _,
                value_type,
                inspect_info: _,
            } => {
                if self.count_wasm_type_components(value_type) == 0 {
                    return;
                }

                for instr in &address.instrs {
                    instrs.push(instr.clone());
                }
            }
            _ => {}
        };
    }

    fn codegen_var_set(
        &self,
        ctx: &mut LoExprContext,
        instrs: &mut Vec<WasmInstr>,
        var: &LoVariableInfo,
    ) -> Result<(), LoError> {
        match var {
            LoVariableInfo::Local {
                local_index,
                local_type,
                inspect_info: _,
            } => {
                self.codegen_local_set(instrs, local_type, *local_index);
            }
            LoVariableInfo::Global {
                global_index,
                global_type,
                inspect_info: _,
            } => {
                for i in (0..self.count_wasm_type_components(global_type)).rev() {
                    instrs.push(WasmInstr::GlobalSet {
                        global_index: global_index + i,
                    });
                }
            }
            LoVariableInfo::Stored {
                address: _,
                offset,
                value_type,
                inspect_info: _,
            } => {
                let mut stores = Vec::new();
                self.codegen_load_or_store(&mut stores, &value_type, *offset, true);

                if stores.len() > 1 {
                    let tmp_value_local_index =
                        self.define_unnamed_local(ctx, LoLocation::internal(), value_type, false);
                    self.codegen_local_set(instrs, value_type, tmp_value_local_index);

                    let addr_local_index = self.create_or_get_addr_local(ctx);
                    instrs.push(WasmInstr::LocalSet {
                        local_index: addr_local_index,
                    });

                    for (store, i) in stores.into_iter().rev().zip(0..) {
                        instrs.push(WasmInstr::LocalGet {
                            local_index: addr_local_index,
                        });
                        instrs.push(WasmInstr::LocalGet {
                            local_index: tmp_value_local_index + i,
                        });
                        instrs.push(store);
                    }
                } else {
                    instrs.append(&mut stores);
                }
            }
            LoVariableInfo::Const {
                code_unit: _,
                loc,
                inspect_info: _,
            }
            | LoVariableInfo::StructValueField {
                struct_value: _,
                field_type: _,
                drops_before: _,
                drops_after: _,
                loc,
                inspect_info: _,
            } => {
                return Err(LoError {
                    message: format!("Cannot mutate a constant"),
                    loc: loc.clone(),
                })
            }
        };

        Ok(())
    }

    fn codegen_local_set(
        &self,
        instrs: &mut Vec<WasmInstr>,
        local_type: &LoType,
        local_index: u32,
    ) {
        for i in (0..self.count_wasm_type_components(local_type)).rev() {
            instrs.push(WasmInstr::LocalSet {
                local_index: local_index + i,
            });
        }
    }

    fn define_local(
        &self,
        ctx: &mut LoExprContext,
        loc: LoLocation,
        local_name: String,
        local_type: &LoType,
        is_fn_param: bool,
    ) -> Result<u32, LoError> {
        for local in ctx.current_scope().locals.iter() {
            if local.local_name == local_name && local.defined_in_this_scope {
                let LoLocal { definition_loc, .. } = &ctx.locals[local.lo_local_index];

                return Err(LoError {
                    message: format!(
                        "Cannot redefine local {}, previously defined at {}",
                        local_name,
                        definition_loc.to_string(&self.fm)
                    ),
                    loc,
                });
            }
        }

        let local_index = self.define_unnamed_local(ctx, loc, local_type, is_fn_param);

        let lo_local_index = ctx.locals.len() - 1;
        ctx.current_scope_mut().locals.push(LoScopedLocal {
            local_name,
            lo_local_index,
            defined_in_this_scope: true,
        });

        Ok(local_index)
    }

    fn define_unnamed_local(
        &self,
        ctx: &mut LoExprContext,
        loc: LoLocation,
        local_type: &LoType,
        is_fn_param: bool,
    ) -> u32 {
        let local_index = ctx.next_local_index;
        ctx.locals.push(LoLocal {
            local_index,
            local_type: local_type.clone(),
            definition_loc: loc,
            is_fn_param,
        });
        ctx.next_local_index += self.count_wasm_type_components(local_type);

        local_index
    }

    fn assert_catchable_type<'a>(
        &self,
        expr_type: &'a LoType,
        loc: &LoLocation,
    ) -> Result<&'a LoResultType, LoError> {
        let LoType::Result(result) = expr_type else {
            return Err(LoError {
                message: format!("Cannot catch error from expr of type {}", expr_type),
                loc: loc.clone(),
            });
        };

        let mut err_type_components = Vec::new();
        self.lower_type(&result.err, &mut err_type_components);
        if err_type_components != [WasmType::I32] {
            return Err(LoError {
                message: format!(
                    "Invalid Result error type: {}, must lower to i32",
                    result.err
                ),
                loc: loc.clone(),
            });
        }

        Ok(result)
    }

    fn emit_deferred(&self, scope: &LoScope, instrs: &mut Vec<WasmInstr>) {
        for expr in scope.deferred.iter().rev() {
            for instr in &expr.instrs {
                instrs.push(instr.clone());
            }
        }
    }

    // TODO!!!: add similar logic for break/continue
    fn emit_deferred_for_return(&self, ctx: &LoExprContext, instrs: &mut Vec<WasmInstr>) {
        for scope in ctx.scopes.iter().rev() {
            self.emit_deferred(scope, instrs);
        }
    }

    fn build_code_unit(
        &self,
        ctx: &mut LoExprContext,
        expr: &CodeExpr,
    ) -> Result<LoCodeUnit, LoError> {
        let mut code_unit = LoCodeUnit {
            type_: self.get_expr_type(ctx, expr)?,
            instrs: Vec::new(),
        };
        self.codegen(ctx, &mut code_unit.instrs, expr)?;

        Ok(code_unit)
    }

    fn process_const_string(&self, value: String, loc: &LoLocation) -> Result<LoStr, LoError> {
        if self.memory.is_none() && !self.in_inspection_mode {
            return Err(LoError {
                message: format!("Cannot use strings with no memory defined"),
                loc: loc.clone(),
            });
        }

        let string_len = value.as_bytes().len() as u32;

        for pooled_str in self.string_pool.borrow().iter() {
            if *pooled_str.value == value {
                return Ok(LoStr {
                    ptr: pooled_str.ptr,
                    len: string_len,
                });
            }
        }

        let ptr = self.append_data(value.clone().into_bytes());

        self.string_pool
            .borrow_mut()
            .push(PooledString { value, ptr });

        return Ok(LoStr {
            ptr,
            len: string_len,
        });
    }

    fn append_data(&self, bytes: Vec<u8>) -> u32 {
        let offset = *self.data_size.borrow();
        let mut instrs = Vec::new();
        instrs.push(WasmInstr::I32Const {
            value: offset as i32,
        });

        *self.data_size.borrow_mut() += bytes.len() as u32;
        self.datas.borrow_mut().push(WasmData::Active {
            offset: WasmExpr { instrs },
            bytes,
        });

        offset
    }

    // TODO: add validation for const expr
    fn ensure_const_expr(&self, _expr: &CodeExpr) -> Result<(), LoError> {
        Ok(())
    }

    fn get_type_or_err(&self, type_name: &str, loc: &LoLocation) -> Result<LoType, LoError> {
        if let Some(typedef) = self.get_typedef(type_name) {
            if self.in_inspection_mode {
                match typedef.kind {
                    LoTypeDefKind::Builtin => {
                        self.print_inspection(&InspectInfo {
                            message: format!("type {type_name} = <builtin>"),
                            loc: loc.clone(),
                            linked_loc: None,
                        });
                    }
                    LoTypeDefKind::Struct => {
                        self.print_inspection(&InspectInfo {
                            message: format!("struct {type_name} {{ ... }}"),
                            loc: loc.clone(),
                            linked_loc: Some(typedef.loc.clone()),
                        });
                    }
                    LoTypeDefKind::Alias => {
                        self.print_inspection(&InspectInfo {
                            message: format!("type {type_name} = {}", typedef.value),
                            loc: loc.clone(),
                            linked_loc: Some(typedef.loc.clone()),
                        });
                    }
                }
            }

            return Ok(typedef.value.clone());
        }

        Err(LoError {
            message: format!("Unknown type: {}", type_name),
            loc: loc.clone(),
        })
    }

    fn get_typedef(&self, type_name: &str) -> Option<&LoTypeDef> {
        for type_def in &self.type_defs {
            if type_def.name == type_name {
                return Some(type_def);
            }
        }

        None
    }

    fn get_const<'a>(&'a self, ctx: &'a LoExprContext, const_name: &str) -> Option<&'a LoConstDef> {
        if let Some(const_def) = self.get_const_def(const_name) {
            return Some(const_def);
        }

        if let Some(macro_arg) = ctx.get_macro_arg(const_name) {
            return Some(macro_arg);
        }

        None
    }

    fn get_const_def(&self, const_name: &str) -> Option<&LoConstDef> {
        for const_def in &self.const_defs {
            if const_def.const_name == const_name {
                return Some(const_def);
            }
        }

        None
    }

    fn get_macro_def(&self, macro_name: &str) -> Option<&MacroDefExpr> {
        for macro_def in &self.macro_defs {
            if macro_def.macro_name.repr == macro_name {
                return Some(macro_def);
            }
        }

        None
    }

    fn get_struct_def(&self, struct_name: &str) -> Option<&LoStructDef> {
        for struct_def in &self.struct_defs {
            if struct_def.struct_name == struct_name {
                return Some(struct_def);
            }
        }

        None
    }

    fn get_struct_def_mut(&mut self, struct_name: &str) -> Option<&mut LoStructDef> {
        for struct_def in &mut self.struct_defs {
            if struct_def.struct_name == struct_name {
                return Some(struct_def);
            }
        }

        None
    }

    fn codegen_default_value(
        &self,
        ctx: &mut LoExprContext,
        instrs: &mut Vec<WasmInstr>,
        value_type: &LoType,
    ) {
        match value_type {
            LoType::Never => {}
            LoType::Void => {}
            LoType::Bool
            | LoType::U8
            | LoType::I8
            | LoType::U16
            | LoType::I16
            | LoType::U32
            | LoType::I32
            | LoType::Pointer { pointee: _ }
            | LoType::SequencePointer { pointee: _ } => {
                instrs.push(WasmInstr::I32Const { value: 0 })
            }
            LoType::U64 | LoType::I64 => instrs.push(WasmInstr::I64Const { value: 0 }),
            LoType::F32 => instrs.push(WasmInstr::F32Const { value: 0.0 }),
            LoType::F64 => instrs.push(WasmInstr::F64Const { value: 0.0 }),
            LoType::StructInstance { struct_name } => {
                let struct_ref = self.get_struct_def(struct_name).unwrap();
                for field in &struct_ref.fields {
                    self.codegen_default_value(ctx, instrs, &field.field_type);
                }
            }
            LoType::Result(result) => {
                self.codegen_default_value(ctx, instrs, &result.ok);
                self.codegen_default_value(ctx, instrs, &result.err);
            }
        }
    }

    fn codegen_int_const(
        &self,
        _ctx: &LoExprContext,
        instrs: &mut Vec<WasmInstr>,
        value: i32,
        tag: Option<&str>,
    ) {
        match tag.as_deref() {
            Some("u32") | Some("i32") | None => instrs.push(WasmInstr::I32Const { value }),
            Some("u64") | Some("i64") => instrs.push(WasmInstr::I64Const {
                value: value as i64,
            }),
            _ => unreachable!(),
        }
    }

    fn is_types_compatible(&self, slots: &Vec<LoType>, values: &Vec<LoType>) -> bool {
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

    fn is_type_compatible(&self, slot: &LoType, value: &LoType) -> bool {
        if let LoType::Pointer { pointee } = slot {
            if **pointee == LoType::Void {
                if let LoType::Pointer { pointee: _ } = value {
                    return true;
                }

                if let LoType::SequencePointer { pointee: _ } = value {
                    return true;
                }
            }
        }

        slot == value
    }

    fn lower_type(&self, lo_type: &LoType, wasm_types: &mut Vec<WasmType>) {
        match lo_type {
            LoType::Never => {}
            LoType::Void => {}
            LoType::Bool => wasm_types.push(WasmType::I32),
            LoType::U8 => wasm_types.push(WasmType::I32),
            LoType::I8 => wasm_types.push(WasmType::I32),
            LoType::U16 => wasm_types.push(WasmType::I32),
            LoType::I16 => wasm_types.push(WasmType::I32),
            LoType::U32 => wasm_types.push(WasmType::I32),
            LoType::I32 => wasm_types.push(WasmType::I32),
            LoType::F32 => wasm_types.push(WasmType::F32),
            LoType::U64 => wasm_types.push(WasmType::I64),
            LoType::I64 => wasm_types.push(WasmType::I64),
            LoType::F64 => wasm_types.push(WasmType::F64),
            LoType::Pointer { pointee: _ } => wasm_types.push(WasmType::I32),
            LoType::SequencePointer { pointee: _ } => wasm_types.push(WasmType::I32),
            LoType::StructInstance { struct_name } => {
                let struct_def = self.get_struct_def(struct_name).unwrap();

                for field in &struct_def.fields {
                    self.lower_type(&field.field_type, wasm_types);
                }
            }
            LoType::Result(result) => {
                self.lower_type(&result.ok, wasm_types);
                self.lower_type(&result.err, wasm_types);
            }
        }
    }

    fn count_wasm_type_components(&self, lo_type: &LoType) -> u32 {
        let layout = &mut LoTypeLayout::default();
        self.get_type_layout(lo_type, layout);
        layout.primities_count
    }

    fn get_type_layout<'a>(&self, lo_type: &LoType, layout: &'a mut LoTypeLayout) {
        match lo_type {
            LoType::Never | LoType::Void => {
                layout.alignment = u32::max(layout.alignment, 1);
            }
            LoType::Bool | LoType::U8 | LoType::I8 => {
                layout.primities_count += 1;
                layout.alignment = u32::max(layout.alignment, 1);
                layout.byte_size = align(layout.byte_size, 1) + 1;
            }
            LoType::U16 | LoType::I16 => {
                layout.primities_count += 1;
                layout.alignment = u32::max(layout.alignment, 2);
                layout.byte_size = align(layout.byte_size, 2) + 2;
            }
            LoType::U32
            | LoType::I32
            | LoType::F32
            | LoType::Pointer { pointee: _ }
            | LoType::SequencePointer { pointee: _ } => {
                layout.primities_count += 1;
                layout.alignment = u32::max(layout.alignment, 4);
                layout.byte_size = align(layout.byte_size, 4) + 4;
            }
            LoType::U64 | LoType::I64 | LoType::F64 => {
                layout.primities_count += 1;
                layout.alignment = u32::max(layout.alignment, 8);
                layout.byte_size = align(layout.byte_size, 8) + 8;
            }
            LoType::StructInstance { struct_name } => {
                let struct_def = self.get_struct_def(struct_name).unwrap();

                // append each field's layout to total struct layout
                for field in &struct_def.fields {
                    self.get_type_layout(&field.field_type, layout);
                }

                layout.alignment = u32::max(layout.alignment, 1);
                layout.byte_size = align(layout.byte_size, layout.alignment);
            }
            LoType::Result(result) => {
                self.get_type_layout(&result.ok, layout);
                self.get_type_layout(&result.err, layout);
            }
        }
    }

    // TODO?: support all numeric types
    fn get_cast_instr(&self, casted_from: &LoType, casted_to: &LoType) -> Option<WasmInstr> {
        if *casted_to == LoType::I64 || *casted_to == LoType::U64 {
            if *casted_from == LoType::I8
                || *casted_from == LoType::I16
                || *casted_from == LoType::I32
            {
                return Some(WasmInstr::I64ExtendI32s);
            }

            if *casted_from == LoType::U8
                || *casted_from == LoType::U16
                || *casted_from == LoType::U32
            {
                return Some(WasmInstr::I64ExtendI32u);
            }
        }

        if *casted_to == LoType::I8
            || *casted_to == LoType::U8
            || *casted_to == LoType::I16
            || *casted_to == LoType::U16
            || *casted_to == LoType::I32
            || *casted_to == LoType::U32
        {
            if *casted_from == LoType::I64 || *casted_from == LoType::U64 {
                return Some(WasmInstr::I32WrapI64);
            }
        }

        None
    }

    fn get_binary_op_kind(
        &self,
        op_tag: &InfixOpTag,
        operand_type: &LoType,
        loc: &LoLocation,
    ) -> Result<WasmBinaryOpKind, LoError> {
        match op_tag {
            InfixOpTag::Equal => match operand_type {
                LoType::Bool
                | LoType::I8
                | LoType::U8
                | LoType::I16
                | LoType::U16
                | LoType::I32
                | LoType::U32 => return Ok(WasmBinaryOpKind::I32_EQ),
                LoType::I64 | LoType::U64 => return Ok(WasmBinaryOpKind::I64_EQ),
                LoType::F32 => return Ok(WasmBinaryOpKind::F32_EQ),
                LoType::F64 => return Ok(WasmBinaryOpKind::F64_EQ),
                _ => {}
            },
            InfixOpTag::NotEqual => match operand_type {
                LoType::Bool
                | LoType::I8
                | LoType::U8
                | LoType::I16
                | LoType::U16
                | LoType::I32
                | LoType::U32 => return Ok(WasmBinaryOpKind::I32_NE),
                LoType::I64 | LoType::U64 => return Ok(WasmBinaryOpKind::I64_NE),
                LoType::F32 => return Ok(WasmBinaryOpKind::F32_NE),
                LoType::F64 => return Ok(WasmBinaryOpKind::F64_NE),
                _ => {}
            },
            InfixOpTag::Less => match operand_type {
                LoType::I8 | LoType::I16 | LoType::I32 => return Ok(WasmBinaryOpKind::I32_LT_S),
                LoType::Bool | LoType::U8 | LoType::U16 | LoType::U32 => {
                    return Ok(WasmBinaryOpKind::I32_LT_U)
                }
                LoType::I64 => return Ok(WasmBinaryOpKind::I64_LT_S),
                LoType::U64 => return Ok(WasmBinaryOpKind::I64_LT_U),
                LoType::F32 => return Ok(WasmBinaryOpKind::F32_LT),
                LoType::F64 => return Ok(WasmBinaryOpKind::F64_LT),
                _ => {}
            },
            InfixOpTag::Greater => match operand_type {
                LoType::I8 | LoType::I16 | LoType::I32 => return Ok(WasmBinaryOpKind::I32_GT_S),
                LoType::Bool | LoType::U8 | LoType::U16 | LoType::U32 => {
                    return Ok(WasmBinaryOpKind::I32_GT_U)
                }
                LoType::I64 => return Ok(WasmBinaryOpKind::I64_GT_S),
                LoType::U64 => return Ok(WasmBinaryOpKind::I64_GT_U),
                LoType::F32 => return Ok(WasmBinaryOpKind::F32_GT),
                LoType::F64 => return Ok(WasmBinaryOpKind::F64_GT),
                _ => {}
            },
            InfixOpTag::LessEqual => match operand_type {
                LoType::I8 | LoType::I16 | LoType::I32 => return Ok(WasmBinaryOpKind::I32_LE_S),
                LoType::Bool | LoType::U8 | LoType::U16 | LoType::U32 => {
                    return Ok(WasmBinaryOpKind::I32_LE_U)
                }
                LoType::I64 => return Ok(WasmBinaryOpKind::I64_LE_S),
                LoType::U64 => return Ok(WasmBinaryOpKind::I64_LE_U),
                LoType::F32 => return Ok(WasmBinaryOpKind::F32_LE),
                LoType::F64 => return Ok(WasmBinaryOpKind::F64_LE),
                _ => {}
            },
            InfixOpTag::GreaterEqual => match operand_type {
                LoType::I8 | LoType::I16 | LoType::I32 => return Ok(WasmBinaryOpKind::I32_GE_S),
                LoType::Bool | LoType::U8 | LoType::U16 | LoType::U32 => {
                    return Ok(WasmBinaryOpKind::I32_GE_U)
                }
                LoType::I64 => return Ok(WasmBinaryOpKind::I64_GE_S),
                LoType::U64 => return Ok(WasmBinaryOpKind::I64_GE_U),
                LoType::F32 => return Ok(WasmBinaryOpKind::F32_GE),
                LoType::F64 => return Ok(WasmBinaryOpKind::F64_GE),
                _ => {}
            },
            InfixOpTag::Add => match operand_type {
                LoType::Bool
                | LoType::I8
                | LoType::U8
                | LoType::I16
                | LoType::U16
                | LoType::I32
                | LoType::U32 => return Ok(WasmBinaryOpKind::I32_ADD),
                LoType::I64 | LoType::U64 => return Ok(WasmBinaryOpKind::I64_ADD),
                LoType::F32 => return Ok(WasmBinaryOpKind::F32_ADD),
                LoType::F64 => return Ok(WasmBinaryOpKind::F64_ADD),
                _ => {}
            },
            InfixOpTag::Sub => match operand_type {
                LoType::Bool
                | LoType::I8
                | LoType::U8
                | LoType::I16
                | LoType::U16
                | LoType::I32
                | LoType::U32 => return Ok(WasmBinaryOpKind::I32_SUB),
                LoType::I64 | LoType::U64 => return Ok(WasmBinaryOpKind::I64_SUB),
                LoType::F32 => return Ok(WasmBinaryOpKind::F32_SUB),
                LoType::F64 => return Ok(WasmBinaryOpKind::F64_SUB),
                _ => {}
            },
            InfixOpTag::Mul => match operand_type {
                LoType::Bool
                | LoType::I8
                | LoType::U8
                | LoType::I16
                | LoType::U16
                | LoType::I32
                | LoType::U32 => return Ok(WasmBinaryOpKind::I32_MUL),
                LoType::I64 | LoType::U64 => return Ok(WasmBinaryOpKind::I64_MUL),
                LoType::F32 => return Ok(WasmBinaryOpKind::F32_MUL),
                LoType::F64 => return Ok(WasmBinaryOpKind::F64_MUL),
                _ => {}
            },
            InfixOpTag::Div => match operand_type {
                LoType::I8 | LoType::I16 | LoType::I32 => return Ok(WasmBinaryOpKind::I32_DIV_S),
                LoType::Bool | LoType::U8 | LoType::U16 | LoType::U32 => {
                    return Ok(WasmBinaryOpKind::I32_DIV_U)
                }
                LoType::I64 => return Ok(WasmBinaryOpKind::I64_DIV_S),
                LoType::U64 => return Ok(WasmBinaryOpKind::I64_DIV_U),
                LoType::F32 => return Ok(WasmBinaryOpKind::F32_DIV),
                LoType::F64 => return Ok(WasmBinaryOpKind::F64_DIV),
                _ => {}
            },
            InfixOpTag::Mod => match operand_type {
                LoType::I8 | LoType::I16 | LoType::I32 => return Ok(WasmBinaryOpKind::I32_REM_S),
                LoType::Bool | LoType::U8 | LoType::U16 | LoType::U32 => {
                    return Ok(WasmBinaryOpKind::I32_REM_U)
                }
                LoType::I64 => return Ok(WasmBinaryOpKind::I64_REM_S),
                LoType::U64 => return Ok(WasmBinaryOpKind::I64_REM_U),
                _ => {}
            },
            InfixOpTag::ShiftLeft => match operand_type {
                LoType::I8 | LoType::I16 | LoType::I32 => return Ok(WasmBinaryOpKind::I32_SHL),
                LoType::Bool | LoType::U8 | LoType::U16 | LoType::U32 => {
                    return Ok(WasmBinaryOpKind::I32_SHL)
                }
                LoType::I64 => return Ok(WasmBinaryOpKind::I64_SHL),
                LoType::U64 => return Ok(WasmBinaryOpKind::I64_SHL),
                _ => {}
            },
            InfixOpTag::ShiftRight => match operand_type {
                LoType::I8 | LoType::I16 | LoType::I32 => return Ok(WasmBinaryOpKind::I32_SHR_S),
                LoType::Bool | LoType::U8 | LoType::U16 | LoType::U32 => {
                    return Ok(WasmBinaryOpKind::I32_SHR_U)
                }
                LoType::I64 => return Ok(WasmBinaryOpKind::I64_SHR_S),
                LoType::U64 => return Ok(WasmBinaryOpKind::I64_SHR_U),
                _ => {}
            },
            InfixOpTag::And => match operand_type {
                LoType::Bool
                | LoType::I8
                | LoType::U8
                | LoType::I16
                | LoType::U16
                | LoType::I32
                | LoType::U32 => return Ok(WasmBinaryOpKind::I32_AND),
                LoType::I64 | LoType::U64 => return Ok(WasmBinaryOpKind::I64_AND),
                _ => {}
            },
            InfixOpTag::Or => match operand_type {
                LoType::Bool
                | LoType::I8
                | LoType::U8
                | LoType::I16
                | LoType::U16
                | LoType::I32
                | LoType::U32 => return Ok(WasmBinaryOpKind::I32_OR),
                LoType::I64 | LoType::U64 => return Ok(WasmBinaryOpKind::I64_OR),
                _ => {}
            },
            InfixOpTag::BitAnd => match operand_type {
                LoType::Bool
                | LoType::I8
                | LoType::U8
                | LoType::I16
                | LoType::U16
                | LoType::I32
                | LoType::U32 => return Ok(WasmBinaryOpKind::I32_AND),
                LoType::I64 | LoType::U64 => return Ok(WasmBinaryOpKind::I64_AND),
                _ => {}
            },
            InfixOpTag::BitOr => match operand_type {
                LoType::Bool
                | LoType::I8
                | LoType::U8
                | LoType::I16
                | LoType::U16
                | LoType::I32
                | LoType::U32 => return Ok(WasmBinaryOpKind::I32_OR),
                LoType::I64 | LoType::U64 => return Ok(WasmBinaryOpKind::I64_OR),
                _ => {}
            },

            // handled in get_compound_assignment_base_op
            InfixOpTag::AddAssign
            | InfixOpTag::SubAssign
            | InfixOpTag::MulAssign
            | InfixOpTag::DivAssign
            | InfixOpTag::ModAssign
            | InfixOpTag::BitAndAssign
            | InfixOpTag::BitOrAssign
            | InfixOpTag::ShiftLeftAssign
            | InfixOpTag::ShiftRightAssign => unreachable!(),

            // have their own CodeExpr variants
            InfixOpTag::Cast
            | InfixOpTag::Assign
            | InfixOpTag::FieldAccess
            | InfixOpTag::Catch
            | InfixOpTag::ErrorPropagation => unreachable!(),
        }

        return Err(LoError {
            message: format!(
                "Operator `{}` is incompatible with operands of type {operand_type}",
                op_tag.to_str(),
            ),
            loc: loc.clone(),
        });
    }

    fn get_compound_assignment_base_op(&self, op_tag: &InfixOpTag) -> Option<InfixOpTag> {
        match op_tag {
            InfixOpTag::AddAssign => Some(InfixOpTag::Add),
            InfixOpTag::SubAssign => Some(InfixOpTag::Sub),
            InfixOpTag::MulAssign => Some(InfixOpTag::Mul),
            InfixOpTag::DivAssign => Some(InfixOpTag::Div),
            InfixOpTag::ModAssign => Some(InfixOpTag::Mod),
            InfixOpTag::BitAndAssign => Some(InfixOpTag::BitAnd),
            InfixOpTag::BitOrAssign => Some(InfixOpTag::BitOr),
            InfixOpTag::ShiftLeftAssign => Some(InfixOpTag::ShiftLeft),
            InfixOpTag::ShiftRightAssign => Some(InfixOpTag::ShiftRight),

            InfixOpTag::Equal
            | InfixOpTag::NotEqual
            | InfixOpTag::Less
            | InfixOpTag::Greater
            | InfixOpTag::LessEqual
            | InfixOpTag::GreaterEqual
            | InfixOpTag::Add
            | InfixOpTag::Sub
            | InfixOpTag::Mul
            | InfixOpTag::Div
            | InfixOpTag::Mod
            | InfixOpTag::And
            | InfixOpTag::BitAnd
            | InfixOpTag::Or
            | InfixOpTag::BitOr
            | InfixOpTag::ShiftLeft
            | InfixOpTag::ShiftRight
            | InfixOpTag::Cast
            | InfixOpTag::Assign
            | InfixOpTag::FieldAccess
            | InfixOpTag::Catch
            | InfixOpTag::ErrorPropagation => None,
        }
    }

    fn get_global(&self, global_name: &str) -> Option<&LoGlobalDef> {
        for global_def in &self.globals {
            if global_def.def_expr.global_name.repr == global_name {
                return Some(global_def);
            }
        }

        None
    }

    fn get_module_by_file_index(&self, file_index: u32) -> Option<&Module> {
        for module in &self.modules {
            if module.file_index == file_index {
                return Some(module);
            }
        }

        None
    }

    fn report_error(&self, err: &LoError) {
        *self.error_count.borrow_mut() += 1;

        if self.in_inspection_mode {
            let source_index = err.loc.file_index;
            let source_range = RangeDisplay(&err.loc);
            let content = json_str_escape(&err.message);
            stdout_writeln(format!(
                "{{ \"type\": \"message\", \
                    \"content\": \"{content}\", \
                    \"severity\": \"error\", \
                    \"loc\": \"{source_index}/{source_range}\" }},",
            ));
            return;
        }

        stderr_write("ERROR: ");
        stderr_write(err.to_string(&self.fm));
        stderr_write("\n");
    }

    fn report_warning(&self, err: &LoError) {
        *self.warning_count.borrow_mut() += 1;

        if self.in_inspection_mode {
            let source_index = err.loc.file_index;
            let source_range = RangeDisplay(&err.loc);
            let content = json_str_escape(&err.message);
            stdout_writeln(format!(
                "{{ \"type\": \"message\", \
                    \"content\": \"{content}\", \
                    \"severity\": \"warning\", \
                    \"loc\": \"{source_index}/{source_range}\" }},",
            ));
            return;
        }

        stderr_write("WARNING: ");
        stderr_write(err.to_string(&self.fm));
        stderr_write("\n");
    }

    fn print_inspection(&self, inspect_info: &InspectInfo) {
        let source_index = inspect_info.loc.file_index;
        let source_range = RangeDisplay(&inspect_info.loc);
        let message = json_str_escape(&inspect_info.message);

        let Some(linked_loc) = &inspect_info.linked_loc else {
            stdout_writeln(format!(
                "{{ \"type\": \"info\", \
                    \"hover\": \"{message}\", \
                    \"loc\": \"{source_index}/{source_range}\" }},",
            ));
            return;
        };

        let target_index = linked_loc.file_index;
        let target_range = RangeDisplay(&linked_loc);

        stdout_writeln(format!(
            "{{ \"type\": \"info\", \
                \"link\": \"{target_index}/{target_range}\", \
                \"hover\": \"{message}\", \
                \"loc\": \"{source_index}/{source_range}\" }},",
        ));
    }
}

fn json_str_escape(value: &str) -> String {
    value
        .replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
}

fn get_fn_name_from_method(receiver_type: &LoType, method_name: &str) -> String {
    let resolved_receiver_type = receiver_type.deref_rec();
    format!("{resolved_receiver_type}::{method_name}")
}

fn align(value: u32, alignment: u32) -> u32 {
    return (value + alignment - 1) / alignment * alignment;
}
