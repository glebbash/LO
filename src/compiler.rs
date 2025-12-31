use crate::{ast::*, core::*, lexer::*, parser::*, wasm::*};
use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::{cell::RefCell, fmt::Write};

#[derive(Clone, PartialEq)]
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
}

pub struct TypeFmt<'a>(pub &'a Compiler, pub &'a Type);

impl<'a> core::fmt::Display for TypeFmt<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.1.format(f, self.0)
    }
}

pub struct TypeListFmt<'a>(pub &'a Compiler, pub &'a [Type]);

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

#[derive(Clone, PartialEq)]
pub struct ResultType {
    ok: Box<Type>,
    err: Box<Type>,
}

impl Type {
    fn format(&self, f: &mut core::fmt::Formatter<'_>, compiler: &Compiler) -> core::fmt::Result {
        match self {
            Type::Never => f.write_str("never"),
            Type::Null => f.write_str("null"),
            Type::Void => f.write_str("void"),
            Type::Bool => f.write_str("bool"),
            Type::U8 => f.write_str("u8"),
            Type::I8 => f.write_str("i8"),
            Type::U16 => f.write_str("u16"),
            Type::I16 => f.write_str("i16"),
            Type::U32 => f.write_str("u32"),
            Type::I32 => f.write_str("i32"),
            Type::F32 => f.write_str("f32"),
            Type::U64 => f.write_str("u64"),
            Type::I64 => f.write_str("i64"),
            Type::F64 => f.write_str("f64"),
            Type::Pointer { pointee } => write!(f, "&{}", TypeFmt(compiler, pointee)),
            Type::SequencePointer { pointee } => {
                write!(f, "*&{}", TypeFmt(compiler, pointee))
            }
            Type::StructInstance { struct_index } => {
                f.write_str(&compiler.struct_defs[*struct_index].struct_name)
            }
            Type::EnumInstance { enum_index } => {
                f.write_str(&compiler.enum_defs[*enum_index].enum_name)
            }
            Type::Result(result) => {
                write!(
                    f,
                    "Result<{}, {}>",
                    TypeFmt(compiler, &result.ok),
                    TypeFmt(compiler, &result.err)
                )
            }
        }
    }
}

impl Type {
    fn deref_rec(&self) -> &Type {
        match self {
            Type::Pointer { pointee } => pointee.deref_rec(),
            Type::SequencePointer { pointee } => pointee.deref_rec(),
            other => other,
        }
    }
}

struct FnInfo {
    fn_name: String,
    fn_type: FnType,
    fn_params: Vec<FnParameter>,
    fn_source: FnSource,
    exported_as: Vec<String>,
    wasm_fn_index: u32,
    definition_loc: Loc,
}

struct FnParameter {
    param_name: String,
    param_type: Type,
}

enum FnSource {
    Guest {
        ctx: ExprContext,
        body: &'static CodeBlock,
    },
    Host {
        module_name: String,
        external_fn_name: String,
    },
}

struct FnType {
    inputs: Vec<Type>,
    output: Type,
}

#[derive(Clone)]
struct ExprContext {
    module_index: usize,
    fn_index: Option<usize>,
    locals: Vec<Local>,
    next_local_index: u32,
    addr_local_index: Option<u32>,
    scopes: Vec<Scope>,
}

impl ExprContext {
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
struct Local {
    local_index: u32,
    local_type: Type,
    definition_loc: Loc,
    is_fn_param: bool,
}

#[derive(Clone)]
enum ScopeType {
    Function,
    Block,
    Loop,
    ForLoop,
    Macro,
}

#[derive(Clone)]
struct CodeUnit {
    type_: Type,
    instrs: Vec<WasmInstr>,
}

struct ConstSliceLen {
    slice_offset: u32,
    slice_len: usize,
}

#[derive(Clone)]
struct MacroTypeArg {
    name: String,
    type_: Type,
}

#[derive(Clone)]
struct Scope {
    scope_type: ScopeType,
    locals: Vec<ScopedLocal>,
    deferred: Vec<CodeUnit>,
    macro_args: Vec<ConstDef>,
    macro_type_args: Vec<MacroTypeArg>,
}

#[derive(Clone)]
struct ScopedLocal {
    local_name: String,
    lo_local_index: usize,
    defined_in_this_scope: bool,
}

impl ExprContext {
    fn enter_scope(&mut self, scope_type: ScopeType) {
        let mut new_scope = Scope {
            scope_type,
            locals: Vec::new(),
            deferred: Vec::new(),
            macro_args: Vec::new(),
            macro_type_args: Vec::new(),
        };

        if let Some(parent_scope) = self.scopes.last() {
            for local in &parent_scope.locals {
                new_scope.locals.push(ScopedLocal {
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

    fn current_scope(&self) -> &Scope {
        self.scopes.last().unwrap()
    }

    fn current_scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    fn get_local(&self, local_name: &str) -> Option<&Local> {
        for scope in self.scopes.iter().rev() {
            for local in &scope.locals {
                if local.local_name == local_name {
                    return Some(&self.locals[local.lo_local_index]);
                }
            }
        }

        None
    }

    fn get_macro_type_arg(&self, type_name: &str) -> Option<&Type> {
        for scope in self.scopes.iter().rev() {
            for macro_type_arg in &scope.macro_type_args {
                if macro_type_arg.name == type_name {
                    return Some(&macro_type_arg.type_);
                }
            }
        }

        None
    }

    fn get_macro_arg(&self, arg_name: &str) -> Option<&ConstDef> {
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

struct StructDef {
    struct_name: String,
    fields: Vec<StructField>,
    fully_defined: bool, // used for self-reference checks
}

pub struct StructField {
    field_name: String,
    field_type: Type,
    field_layout: TypeLayout,
    field_index: u32,
    byte_offset: u32,
    loc: Loc,
}

struct EnumDef {
    enum_name: String,
    variant_type: Type,
    variants: Vec<EnumVariant>,
}

pub struct EnumVariant {
    variant_name: String,
    variant_type: Type,
    loc: Loc,
}

pub struct EnumConstructor {
    enum_index: usize,
    variant_index: usize,
}

struct GlobalDef {
    module_ctx: &'static ExprContext,
    def_expr: &'static GlobalDefExpr,
    global_type: Type,
    global_index: u32,
}

struct TypeLayout {
    primities_count: u32,
    byte_size: u32,
    alignment: u32,
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

enum VariableInfo {
    Local {
        local_index: u32,
        local_type: Type,
        inspect_info: Option<InspectInfo>,
    },
    Global {
        global_index: u32,
        global_type: Type,
        inspect_info: Option<InspectInfo>,
    },
    Const {
        code_unit: &'static CodeUnit,
        loc: Loc,
        inspect_info: Option<InspectInfo>,
    },
    Stored {
        address: CodeUnit,
        offset: u32,
        value_type: Type,
        inspect_info: Option<InspectInfo>,
    },
    StructValueField {
        struct_value: CodeUnit,
        field_type: Type,
        drops_before: u32,
        drops_after: u32,
        loc: Loc,
        inspect_info: Option<InspectInfo>,
    },
}

impl VariableInfo {
    fn get_type(&self) -> &Type {
        match self {
            VariableInfo::Local {
                local_index: _,
                local_type,
                inspect_info: _,
            } => local_type,
            VariableInfo::Global {
                global_index: _,
                global_type,
                inspect_info: _,
            } => global_type,
            VariableInfo::Const {
                code_unit,
                loc: _,
                inspect_info: _,
            } => &code_unit.type_,
            VariableInfo::Stored {
                address: _,
                offset: _,
                value_type,
                inspect_info: _,
            } => value_type,
            VariableInfo::StructValueField {
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
            VariableInfo::Local {
                local_index: _,
                local_type: _,
                inspect_info,
            }
            | VariableInfo::Global {
                global_index: _,
                global_type: _,
                inspect_info,
            }
            | VariableInfo::Const {
                code_unit: _,
                loc: _,
                inspect_info,
            }
            | VariableInfo::Stored {
                address: _,
                offset: _,
                value_type: _,
                inspect_info,
            }
            | VariableInfo::StructValueField {
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
    loc: Loc,
    linked_loc: Option<Loc>,
}

#[derive(Clone)]
struct ConstDef {
    const_name: String,
    code_unit: CodeUnit,
    loc: Loc,
}

#[derive(Clone)]
struct PooledString {
    value: String,
    ptr: u32,
}

#[derive(Clone)]
struct Str {
    ptr: u32,
    len: u32,
}

pub struct Module {
    pub index: usize,
    pub parser: Parser,
    includes: Vec<ModuleInclude>,
    own_items: Vec<ModuleItem>,
    all_items: Vec<ModuleItem>,
    ctx: ExprContext,
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
    loc: Loc,
}

#[derive(Clone)]
enum ModuleItemCollection {
    TypeAlias,
    Function,
    Macro,
    Global,
    Const,
    Struct,
    Enum,
    EnumConstructor,
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

pub struct Compiler {
    pub in_inspection_mode: bool,
    pub in_single_file_mode: bool,

    pub fm: FileManager,
    pub modules: Vec<Module>,
    pub error_count: RefCell<u32>,
    pub warning_count: RefCell<u32>,

    global_items: Vec<ModuleItem>,

    type_aliases: Vec<Type>,
    struct_defs: Vec<StructDef>,
    enum_defs: Vec<EnumDef>,
    enum_ctors: Vec<EnumConstructor>,
    globals: Vec<GlobalDef>,
    const_defs: Vec<ConstDef>,
    macro_defs: Vec<&'static MacroDefExpr>,
    functions: Vec<FnInfo>,
    const_slice_lens: Vec<ConstSliceLen>,

    memory: Option<&'static MemoryDefExpr>,
    memory_imported_from: Option<String>,
    datas: RefCell<Vec<WasmData>>,
    string_pool: RefCell<Vec<PooledString>>,
    data_size: RefCell<u32>,

    wasm_types: RefCell<Vec<WasmFnType>>,
}

impl Compiler {
    pub fn new() -> Self {
        let mut self_ = Self {
            in_inspection_mode: false,
            in_single_file_mode: false,

            fm: FileManager::new(),
            modules: Vec::new(),
            error_count: RefCell::new(0),
            warning_count: RefCell::new(0),

            global_items: Vec::new(),

            type_aliases: Vec::new(),
            struct_defs: Vec::new(),
            enum_defs: Vec::new(),
            enum_ctors: Vec::new(),
            globals: Vec::new(),
            const_defs: Vec::new(),
            macro_defs: Vec::new(),
            functions: Vec::new(),
            const_slice_lens: Vec::new(),

            memory: None,
            memory_imported_from: None,
            datas: RefCell::new(Vec::new()),
            string_pool: RefCell::new(Vec::new()),
            data_size: RefCell::new(0),

            wasm_types: RefCell::new(Vec::new()),
        };

        self_.add_builtin_type("never", Type::Never);
        self_.add_builtin_type("void", Type::Void);
        self_.add_builtin_type("bool", Type::Bool);
        self_.add_builtin_type("u8", Type::U8);
        self_.add_builtin_type("i8", Type::I8);
        self_.add_builtin_type("u16", Type::U16);
        self_.add_builtin_type("i16", Type::I16);
        self_.add_builtin_type("u32", Type::U32);
        self_.add_builtin_type("i32", Type::I32);
        self_.add_builtin_type("f32", Type::F32);
        self_.add_builtin_type("u64", Type::U64);
        self_.add_builtin_type("i64", Type::I64);
        self_.add_builtin_type("f64", Type::F64);

        return self_;
    }

    #[inline]
    fn add_builtin_type(&mut self, name: &str, type_: Type) {
        self.global_items.push(ModuleItem {
            name: String::from(name),
            collection: ModuleItemCollection::TypeAlias,
            collection_index: self.type_aliases.len(),
            loc: Loc::internal(),
        });
        self.type_aliases.push(type_);
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

    pub fn include(&mut self, relative_path: &str, loc: &Loc) -> Option<&Module> {
        let file_index = catch!(self.fm.include_file(relative_path, loc), err, {
            self.report_error(&err);
            return None;
        });

        let file_is_newly_added = self.fm.files[file_index].included_times == 1;

        if self.in_inspection_mode {
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
            self.report_error(&err);
            return None;
        });

        let parser = Parser::new(lexer);
        catch!(parser.parse_file(), err, {
            self.report_error(&err);
            return None;
        });

        let mut includes = Vec::new();

        if !self.in_single_file_mode {
            for expr in &parser.ast {
                let TopLevelExpr::Include(include) = expr else {
                    continue;
                };

                let Some(module) = self.include(&include.file_path.unescape(source), &include.loc)
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
        let ctx = ExprContext::new(module_index, None);

        self.modules.push(Module {
            index: module_index,
            parser,
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
            self.pass_assemble_complex_types(module);
        }

        for module in self.modules.relax_mut() {
            self.pass_define_memories(module);
        }

        for module in self.modules.relax_mut() {
            self.pass_main(module);
        }
    }

    fn pass_collect_own_items(&mut self, module: &mut Module) {
        for expr in &module.parser.ast {
            match expr {
                TopLevelExpr::StructDef(struct_def) => {
                    self.define_item(ModuleItem {
                        name: struct_def.struct_name.repr.clone(),
                        collection: ModuleItemCollection::Struct,
                        collection_index: self.struct_defs.len(),
                        loc: struct_def.struct_name.loc.clone(),
                    });

                    self.struct_defs.push(StructDef {
                        struct_name: struct_def.struct_name.repr.clone(),
                        fields: Vec::new(),
                        fully_defined: false,
                    });
                }
                TopLevelExpr::EnumDef(enum_def) => {
                    self.define_item(ModuleItem {
                        name: enum_def.enum_name.repr.clone(),
                        collection: ModuleItemCollection::Enum,
                        collection_index: self.enum_defs.len(),
                        loc: enum_def.enum_name.loc.clone(),
                    });

                    self.enum_defs.push(EnumDef {
                        enum_name: enum_def.enum_name.repr.clone(),
                        variant_type: Type::Void, // placeholder
                        variants: Vec::new(),     // placeholder
                    });

                    for (variant, variant_index) in enum_def.variants.iter().zip(0..) {
                        let constructor_name =
                            format!("{}::{}", enum_def.enum_name.repr, variant.variant_name.repr);

                        self.define_item(ModuleItem {
                            name: constructor_name,
                            collection: ModuleItemCollection::EnumConstructor,
                            collection_index: self.enum_ctors.len(),
                            loc: enum_def.enum_name.loc.clone(),
                        });

                        self.enum_ctors.push(EnumConstructor {
                            enum_index: self.enum_defs.len() - 1,
                            variant_index,
                        });
                    }
                }
                TopLevelExpr::TypeDef(type_def) => {
                    self.define_item(ModuleItem {
                        name: type_def.type_name.repr.clone(),
                        collection: ModuleItemCollection::TypeAlias,
                        collection_index: self.type_aliases.len(),
                        loc: type_def.type_name.loc.clone(),
                    });

                    self.type_aliases.push(Type::Never); // placeholder
                }
                TopLevelExpr::FnDef(fn_def) => {
                    self.define_item(ModuleItem {
                        name: fn_def.decl.fn_name.repr.clone(),
                        collection: ModuleItemCollection::Function,
                        collection_index: self.functions.len(),
                        loc: fn_def.decl.fn_name.loc.clone(),
                    });

                    let mut ctx = ExprContext::new(module.index, Some(self.functions.len()));
                    ctx.enter_scope(ScopeType::Function);

                    let mut exported_as = Vec::new();
                    if fn_def.exported {
                        exported_as.push(fn_def.decl.fn_name.repr.clone());
                    }

                    self.functions.push(FnInfo {
                        fn_name: fn_def.decl.fn_name.repr.clone(),
                        fn_type: FnType {
                            inputs: Vec::new(),
                            output: Type::Void,
                        },
                        fn_params: Vec::new(),
                        fn_source: FnSource::Guest {
                            ctx,
                            body: fn_def.body.relax(),
                        },
                        exported_as,
                        wasm_fn_index: u32::MAX, // placeholder
                        definition_loc: fn_def.decl.fn_name.loc.clone(),
                    });
                }
                TopLevelExpr::Import(import_expr) => {
                    let module_name = import_expr.module_name.unescape(module.parser.lexer.source);

                    for item in &import_expr.items {
                        let ImportItem::FnDecl(fn_decl) = item else {
                            continue;
                        };

                        self.define_item(ModuleItem {
                            name: fn_decl.fn_name.repr.clone(),
                            collection: ModuleItemCollection::Function,
                            collection_index: self.functions.len(),
                            loc: fn_decl.fn_name.loc.clone(),
                        });

                        let method_name = fn_decl.fn_name.parts.last().unwrap();
                        let external_fn_name = method_name
                            .read_span(module.parser.lexer.source)
                            .to_string();

                        self.functions.push(FnInfo {
                            fn_name: fn_decl.fn_name.repr.clone(),
                            fn_type: FnType {
                                inputs: Vec::new(),
                                output: Type::Void,
                            },
                            fn_params: Vec::new(),
                            fn_source: FnSource::Host {
                                module_name: module_name.clone(),
                                external_fn_name,
                            },
                            exported_as: Vec::new(),
                            wasm_fn_index: u32::MAX, // not known at this point
                            definition_loc: fn_decl.fn_name.loc.clone(),
                        });
                    }
                }
                TopLevelExpr::MacroDef(macro_def) => {
                    let mut macro_item_name = macro_def.macro_name.repr.clone();
                    macro_item_name.push('!');

                    self.define_item(ModuleItem {
                        name: macro_item_name,
                        collection: ModuleItemCollection::Macro,
                        collection_index: self.macro_defs.len(),
                        loc: macro_def.macro_name.loc.clone(),
                    });

                    self.macro_defs.push(macro_def.relax());
                }
                TopLevelExpr::GlobalDef(global_def) => {
                    self.define_item(ModuleItem {
                        name: global_def.global_name.repr.clone(),
                        collection: ModuleItemCollection::Global,
                        collection_index: self.globals.len(),
                        loc: global_def.global_name.loc.clone(),
                    });

                    self.globals.push(GlobalDef {
                        module_ctx: module.ctx.relax(),
                        def_expr: global_def.relax(),
                        global_type: Type::Never, // placeholder
                        global_index: self.globals.len() as u32,
                    });
                }
                TopLevelExpr::ConstDef(const_def) => {
                    self.define_item(ModuleItem {
                        name: const_def.const_name.repr.clone(),
                        collection: ModuleItemCollection::Const,
                        collection_index: self.const_defs.len(),
                        loc: const_def.const_name.loc.clone(),
                    });

                    self.const_defs.push(ConstDef {
                        const_name: const_def.const_name.repr.clone(),
                        code_unit: CodeUnit {
                            type_: Type::Never, // placeholder
                            instrs: Vec::new(), // placeholder
                        },
                        loc: const_def.const_name.loc.clone(),
                    });
                }
                _ => {} // skip, not interested
            }
        }
    }

    fn define_item(&mut self, item: ModuleItem) {
        let module = self
            .get_module_by_file_index(item.loc.file_index)
            .unwrap()
            .be_mut();

        if let Some(existing_item) = module.get_own_item(&item.name) {
            self.report_error(&Error {
                message: format!(
                    "Cannot redefine {}, already defined at {}",
                    item.name,
                    existing_item.loc.to_string(&self.fm)
                ),
                loc: item.loc.clone(),
            });
        }

        module.own_items.push(item);
    }

    fn pass_collect_all_items(&mut self, module: &mut Module) {
        for item in &self.global_items {
            module.all_items.push(item.clone());
        }

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

    fn pass_assemble_complex_types(&mut self, module: &Module) {
        'exprs: for expr in &module.parser.ast {
            match expr {
                TopLevelExpr::StructDef(struct_def) => {
                    let mut struct_fields = Vec::<StructField>::new();
                    let mut struct_primitives_count = 0;
                    let mut struct_aligment = 1;

                    'fields: for field in &struct_def.fields {
                        for existing_field in &struct_fields {
                            if existing_field.field_name == field.field_name.repr {
                                self.report_error(&Error {
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
                        let mut field_layout = TypeLayout::new();
                        self.get_type_layout(&field_type, &mut field_layout);

                        struct_aligment = u32::max(struct_aligment, field_layout.alignment);
                        struct_primitives_count += field_layout.primities_count;

                        struct_fields.push(StructField {
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

                    let item = module.get_own_item(&struct_def.struct_name.repr).unwrap();
                    let ModuleItemCollection::Struct = item.collection else {
                        continue;
                    };
                    let struct_ = &mut self.struct_defs[item.collection_index];

                    struct_.fields.append(&mut struct_fields);
                    struct_.fully_defined = true;
                }
                TopLevelExpr::EnumDef(enum_def) => {
                    let item = module.get_own_item(&enum_def.enum_name.repr).unwrap();
                    let ModuleItemCollection::Enum = item.collection else {
                        continue;
                    };
                    let enum_ = self.enum_defs[item.collection_index].relax_mut();

                    let mut enum_variant_wasm_types = Vec::new();

                    'variants: for (variant, i) in enum_def.variants.iter().zip(0..) {
                        for existing_variant in &enum_.variants {
                            if existing_variant.variant_name == variant.variant_name.repr {
                                self.report_error(&Error {
                                    message: format!(
                                        "Cannot redefine enum variant '{}', already defined at {}",
                                        variant.variant_name.repr,
                                        existing_variant.loc.to_string(&self.fm),
                                    ),
                                    loc: variant.variant_name.loc.clone(),
                                });
                                continue 'variants;
                            }
                        }

                        let mut variant_type = Type::Void;
                        if let Some(variant_type_expr) = &variant.variant_type {
                            variant_type =
                                catch!(self.build_type(&module.ctx, variant_type_expr), err, {
                                    self.report_error(&err);
                                    continue 'variants;
                                });
                        }

                        if i == 0 {
                            enum_.variant_type = variant_type.clone();
                            self.lower_type(&enum_.variant_type, &mut enum_variant_wasm_types);
                        } else {
                            let mut this_variant_wasm_types = Vec::new();
                            self.lower_type(&variant_type, &mut this_variant_wasm_types);

                            if enum_variant_wasm_types != this_variant_wasm_types {
                                self.report_error(&Error {
                                    message: format!("Enum variants don't lower to the same types"),
                                    loc: variant.variant_name.loc.clone(),
                                });
                            }
                        }

                        enum_.variants.push(EnumVariant {
                            variant_name: variant.variant_name.repr.clone(),
                            variant_type,
                            loc: variant.variant_name.loc.clone(),
                        });
                    }
                }
                TopLevelExpr::TypeDef(type_def) => {
                    let type_value = self.build_type(&module.ctx, &type_def.type_value);
                    let type_value = catch!(type_value, err, {
                        self.report_error(&err);
                        continue;
                    });

                    let item = module.get_own_item(&type_def.type_name.repr).unwrap();
                    let ModuleItemCollection::TypeAlias = item.collection else {
                        continue;
                    };
                    self.type_aliases[item.collection_index] = type_value;
                }
                _ => {} // skip, not interested
            }
        }
    }

    fn pass_define_memories(&mut self, module: &mut Module) {
        for expr in &module.parser.ast {
            match expr {
                TopLevelExpr::MemoryDef(memory_def) => {
                    catch!(self.define_memory(memory_def.relax(), None), err, {
                        self.report_error(&err);
                        continue;
                    });
                }
                TopLevelExpr::Import(import_expr) => {
                    let module_name = import_expr.module_name.unescape(module.parser.lexer.source);

                    for item in &import_expr.items {
                        let ImportItem::Memory(memory) = item else {
                            continue;
                        };

                        let res = self.define_memory(memory.relax(), Some(module_name.clone()));
                        catch!(res, err, {
                            self.report_error(&err);
                        });
                    }
                }
                _ => {} // skip, not interested
            }
        }
    }

    fn pass_main(&mut self, module: &mut Module) {
        for expr in &module.parser.ast {
            match expr {
                TopLevelExpr::Include(_) => {} // skip, processed in pass_collect_all_items
                TopLevelExpr::TypeDef(_) => {} // skip, processed in pass_collect_all_items
                TopLevelExpr::MacroDef(_) => {} // skip, processed in pass_collect_all_items
                TopLevelExpr::StructDef(_) => {} // skip, processed in pass_assemble_complex_types
                TopLevelExpr::EnumDef(_) => {} // skip, processed in pass_assemble_complex_types
                TopLevelExpr::MemoryDef(_) => {} // skip, processed in pass_define_memories

                TopLevelExpr::FnDef(fn_def) => {
                    let item = module.get_own_item(&fn_def.decl.fn_name.repr).unwrap();
                    let ModuleItemCollection::Function = item.collection else {
                        continue;
                    };
                    let fn_info = self.functions[item.collection_index].relax_mut();

                    fn_info.fn_type.output = match &fn_def.decl.return_type {
                        Some(return_type) => {
                            catch!(self.build_type(&module.ctx, return_type), err, {
                                self.report_error(&err);
                                continue;
                            })
                        }
                        _ => Type::Void,
                    };

                    let FnSource::Guest { ctx, body: _ } = &mut fn_info.fn_source else {
                        unreachable!()
                    };

                    let self_type =
                        self.get_fn_self_type(&fn_def.decl.fn_name, &fn_def.decl.fn_params);

                    'param_loop: for fn_param in &fn_def.decl.fn_params {
                        for var in &ctx.current_scope().locals {
                            if var.local_name == fn_param.param_name.repr {
                                self.report_error(&Error {
                                    message: format!(
                                        "Duplicate function parameter name: {}",
                                        fn_param.param_name.repr
                                    ),
                                    loc: fn_param.loc.clone(),
                                });
                                continue 'param_loop;
                            }
                        }

                        let param_type = self.get_fn_param_type(&module.ctx, &self_type, fn_param);
                        let param_type = catch!(param_type, err, {
                            self.report_error(&err);
                            continue 'param_loop;
                        });
                        fn_info.fn_type.inputs.push(param_type.clone());

                        fn_info.fn_params.push(FnParameter {
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
                TopLevelExpr::TryExport(try_export_expr) => {
                    let mut target_module = module as &Module;
                    if try_export_expr.from_root {
                        target_module = &self.modules.last().unwrap();
                    }

                    // TODO: same syntax should probably also be able to export named memories
                    let Ok(fn_info) = self.get_fn_info_for_call(
                        &target_module.ctx,
                        &try_export_expr.in_name.repr,
                        &try_export_expr.loc,
                    ) else {
                        return; // ignore if it doesn't exist or is not a function
                    };

                    if self.in_inspection_mode {
                        self.print_inspection(&InspectInfo {
                            message: try_export_expr.in_name.repr.clone(),
                            loc: try_export_expr.loc.clone(),
                            linked_loc: Some(fn_info.definition_loc.clone()),
                        });
                    }

                    let exported_as = try_export_expr
                        .out_name
                        .unescape(module.parser.lexer.source);
                    fn_info.be_mut().exported_as.push(exported_as);
                }
                TopLevelExpr::Import(import_expr) => {
                    'items: for item in &import_expr.items {
                        let fn_decl = match item {
                            ImportItem::FnDecl(fn_decl) => fn_decl,
                            ImportItem::Memory(_) => {
                                continue; // skip, already processed
                            }
                        };

                        let item = module.get_own_item(&fn_decl.fn_name.repr).unwrap();
                        let ModuleItemCollection::Function = item.collection else {
                            continue;
                        };
                        let fn_info = self.functions[item.collection_index].relax_mut();

                        let self_type = self.get_fn_self_type(&fn_decl.fn_name, &fn_decl.fn_params);

                        for fn_param in &fn_decl.fn_params {
                            let param_type =
                                self.get_fn_param_type(&module.ctx, &self_type, fn_param);
                            let param_type = catch!(param_type, err, {
                                self.report_error(&err);
                                continue 'items;
                            });
                            fn_info.fn_type.inputs.push(param_type.clone());
                            fn_info.fn_params.push(FnParameter {
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
                TopLevelExpr::GlobalDef(global_def) => {
                    let item = module.get_own_item(&global_def.global_name.repr).unwrap();
                    let ModuleItemCollection::Global = item.collection else {
                        continue;
                    };
                    let global = self.globals[item.collection_index].relax_mut();

                    catch!(self.ensure_const_expr(&global_def.global_value), err, {
                        self.report_error(&err);
                    });

                    let value_type = self.get_expr_type(&module.ctx, &global_def.global_value);
                    let value_type = catch!(value_type, err, {
                        self.report_error(&err);
                        continue;
                    });
                    let value_comp_count = self.count_wasm_type_components(&value_type);
                    if value_comp_count != 1 {
                        self.report_error(&Error {
                            message: format!(
                                "Cannot define global with non-primitive type {}",
                                TypeFmt(self, &value_type)
                            ),
                            loc: global_def.loc.clone(),
                        });
                        continue;
                    }

                    if self.in_inspection_mode {
                        let global_name = &global_def.global_name.repr;

                        self.print_inspection(&InspectInfo {
                            message: format!(
                                "global {global_name}: {}",
                                TypeFmt(self, &value_type)
                            ),
                            loc: global_def.global_name.loc.clone(),
                            linked_loc: None,
                        });
                    }

                    global.global_type = value_type;
                }
                TopLevelExpr::ConstDef(const_def) => {
                    let item = module.get_own_item(&const_def.const_name.repr).unwrap();

                    let const_ = self.const_defs[item.collection_index].relax_mut();

                    let const_type = self.get_expr_type(&module.ctx, &const_def.const_value);
                    let const_type = catch!(const_type, err, {
                        self.report_error(&err);
                        continue;
                    });
                    const_.code_unit.type_ = const_type;

                    let res = self.codegen(
                        module.ctx.be_mut(),
                        &mut const_.code_unit.instrs,
                        &const_def.const_value,
                    );
                    catch!(res, err, {
                        self.report_error(&err);
                        continue;
                    });

                    if self.in_inspection_mode {
                        let const_name = &const_def.const_name.repr;

                        self.print_inspection(&InspectInfo {
                            message: format!(
                                "const {const_name}: {}",
                                TypeFmt(self, &const_.code_unit.type_)
                            ),
                            loc: const_def.const_name.loc.clone(),
                            linked_loc: None,
                        });
                    }
                }
            }
        }
    }

    // TODO: add local names to debug info
    pub fn generate(&mut self, wasm_module: &mut WasmModule) {
        let mut fn_imports_count = 0;
        for fn_info in &self.functions {
            if let FnSource::Host { .. } = fn_info.fn_source {
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
                FnSource::Guest { ctx: _, body: _ } => {
                    wasm_module.functions.push(fn_type_index);
                    wasm_module.debug_fn_info.push(WasmDebugFnInfo {
                        fn_index: wasm_fn_index,
                        fn_name: fn_info.fn_name.clone(),
                    });

                    fn_info.wasm_fn_index = wasm_fn_index;

                    wasm_fn_index += 1;
                }
                FnSource::Host {
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
            let FnSource::Guest { ctx, body } = &fn_info.fn_source else {
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
            let wasm_value_type = wasm_types_buf.pop().unwrap_or(WasmType::I32);

            let mut initial_value = WasmExpr { instrs: Vec::new() };

            let res = self.codegen(
                global.module_ctx.be_mut(),
                &mut initial_value.instrs,
                &global.def_expr.global_value,
            );
            catch!(res, err, {
                self.report_error(&err);
            });

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
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        body: &CodeBlock,
        void_only: bool,
    ) -> bool {
        let mut naturally_diverges = false;
        let mut terminates_early = false;

        for expr in &body.exprs {
            let expr_type = catch!(self.get_expr_type(ctx, expr), err, {
                self.report_error(&err);
                continue;
            });

            if terminates_early {
                self.report_warning(&Error {
                    message: format!("Unreachable expression"),
                    loc: expr.loc().clone(),
                });
            }

            if expr_type == Type::Never {
                terminates_early = true;

                naturally_diverges = naturally_diverges || self.is_naturally_divergent(expr);
            }

            let mut type_layout = TypeLayout::new();
            self.get_type_layout(&expr_type, &mut type_layout);
            if type_layout.primities_count > 0 && void_only {
                self.report_error(&Error {
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

        if void_only && terminates_early && !naturally_diverges {
            instrs.push(WasmInstr::Unreachable);
        }

        terminates_early
    }

    fn define_memory(
        &mut self,
        memory: &'static MemoryDefExpr,
        imported_from: Option<String>,
    ) -> Result<(), Error> {
        if let Some(existing_memory) = &self.memory {
            return Err(Error {
                message: format!(
                    "Cannot redefine memory, first defined at {}",
                    existing_memory.loc.to_string(&self.fm)
                ),
                loc: memory.loc.clone(),
            });
        }

        if let Some(data_start) = memory.data_start {
            *self.data_size.borrow_mut() = data_start;
        }
        self.memory = Some(memory);
        self.memory_imported_from = imported_from;

        Ok(())
    }

    fn get_fn_self_type(&self, fn_name: &IdentExpr, fn_params: &Vec<FnParam>) -> Option<Type> {
        let mut has_self_param = false;
        for fn_param in fn_params {
            let (FnParamType::Self_ | FnParamType::SelfRef) = fn_param.param_type else {
                continue;
            };

            has_self_param = true;

            if fn_name.parts.len() == 1 {
                self.report_error(&Error {
                    message: format!("Cannot use self param in non-method function"),
                    loc: fn_param.loc.clone(),
                });
                return Some(Type::Never);
            }
        }
        if !has_self_param {
            return None;
        }

        let fn_module = self
            .get_module_by_file_index(fn_name.loc.file_index)
            .unwrap();
        let fn_source = fn_module.parser.lexer.source;

        let mut self_type_name = String::from(fn_name.parts[0].read_span(fn_source));
        for i in 1..fn_name.parts.len() - 1 {
            self_type_name += "::";
            self_type_name += fn_name.parts[i].read_span(fn_source);
        }

        let mut self_type_loc = fn_name.loc.clone();
        self_type_loc.end_pos = self_type_loc.pos;
        self_type_loc.end_pos.offset += self_type_name.len();
        self_type_loc.end_pos.col += self_type_name.len();

        let self_type = catch!(
            self.get_type_or_err(&self_type_name, &self_type_loc),
            err,
            {
                self.report_error(&err);
                return Some(Type::Never);
            }
        );

        Some(self_type)
    }

    fn get_fn_param_type(
        &self,
        ctx: &ExprContext,
        self_type: &Option<Type>,
        fn_param: &FnParam,
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
            FnParamType::Type { expr } => self.build_type(ctx, &expr),
            FnParamType::Infer { name: _ } => unreachable!(),
        }
    }

    fn build_type(&self, ctx: &ExprContext, type_expr: &TypeExpr) -> Result<Type, Error> {
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
                if let Some(macro_type_arg) = ctx.get_macro_type_arg(&name.repr) {
                    return Ok(macro_type_arg.clone());
                }

                let lo_type = self.get_type_or_err(&name.repr, &name.loc)?;
                if let Type::StructInstance { struct_index } = &lo_type {
                    let struct_def = &self.struct_defs[*struct_index];
                    if !is_referenced && !struct_def.fully_defined {
                        return Err(Error {
                            message: format!(
                                "Cannot use partially defined struct '{}' here",
                                struct_def.struct_name
                            ),
                            loc: loc.clone(),
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
            TypeExpr::Result(TypeExprResult {
                ok_type,
                err_type,
                loc: _,
            }) => {
                let ok = Box::new(self.build_type_check_ref(ctx, &ok_type, false, loc)?);
                let err = Box::new(self.build_type_check_ref(ctx, &err_type, false, loc)?);

                Ok(Type::Result(ResultType { ok, err }))
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

    // TODO: this should report errors more (case-by-case decision)
    fn codegen(
        &self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        expr: &CodeExpr,
    ) -> Result<(), Error> {
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
            CodeExpr::NullLiteral(NullLiteralExpr { loc: _ }) => {
                instrs.push(WasmInstr::I32Const { value: 0 });
            }
            CodeExpr::IntLiteral(IntLiteralExpr {
                repr: _,
                value,
                tag,
                loc: _,
            }) => self.codegen_int_const(instrs, *value as i32, tag.as_deref()),
            CodeExpr::StringLiteral(StringLiteralExpr {
                repr: _,
                value,
                loc,
            }) => {
                let str = self.process_const_string(value.clone(), &loc)?;

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
                let Some(item) = self.modules[ctx.module_index].get_item(&struct_name.repr) else {
                    return Err(Error {
                        message: format!("Unknown struct: {}", struct_name.repr),
                        loc: loc.clone(),
                    });
                };

                let struct_def = &self.struct_defs[item.collection_index];

                for field_index in 0..fields.len() {
                    let field_literal = &fields[field_index];
                    let Some(struct_field) = struct_def.fields.get(field_index) else {
                        return Err(Error {
                            message: format!("Excess field values"),
                            loc: field_literal.loc.clone(),
                        });
                    };

                    if &field_literal.field_name != &struct_field.field_name {
                        return Err(Error {
                            message: format!(
                                "Unexpected struct field name, expecting: `{}`",
                                struct_field.field_name
                            ),
                            loc: field_literal.loc.clone(),
                        });
                    }

                    let field_value_type = self.get_expr_type(ctx, &field_literal.value)?;
                    if !self.is_type_compatible(&struct_field.field_type, &field_value_type) {
                        return Err(Error {
                            message: format!(
                                "Invalid type for struct field {}.{}, expected: {}, got: {}",
                                struct_name.repr,
                                struct_field.field_name,
                                TypeFmt(self, &struct_field.field_type,),
                                TypeFmt(self, &field_value_type),
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

                    return Err(Error {
                        message: format!("Missing struct fields: {}", ListFmt(&missing_fields)),
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

                if let Type::U8 = &item_type {
                    for item in items {
                        let current_item_type = self.get_expr_type(ctx, item)?;
                        if current_item_type != item_type {
                            return Err(Error {
                                message: format!(
                                    "Unexpected array element type: {}, expected: {}",
                                    TypeFmt(self, &current_item_type),
                                    TypeFmt(self, &item_type),
                                ),
                                loc: item.loc().clone(),
                            });
                        }

                        self.codegen(ctx, &mut tmp_instrs, item)?;
                        let WasmInstr::I32Const { value } = tmp_instrs.pop().unwrap() else {
                            return Err(Error {
                                message: format!("Unexpected array element value"),
                                loc: item.loc().clone(),
                            });
                        };

                        bytes.push(value as u8);
                    }
                } else if let Type::StructInstance { struct_index } = &item_type
                    && self.struct_defs[*struct_index].struct_name == "str"
                {
                    for item in items {
                        let current_item_type = self.get_expr_type(ctx, item)?;
                        if current_item_type != item_type {
                            return Err(Error {
                                message: format!(
                                    "Unexpected array element type: {}, expected: {}",
                                    TypeFmt(self, &current_item_type),
                                    TypeFmt(self, &item_type),
                                ),
                                loc: item.loc().clone(),
                            });
                        }

                        self.codegen(ctx, &mut tmp_instrs, item)?;
                        let WasmInstr::I32Const { value: len } = tmp_instrs.pop().unwrap() else {
                            return Err(Error {
                                message: format!("Unexpected array element value"),
                                loc: item.loc().clone(),
                            });
                        };
                        let WasmInstr::I32Const { value: ptr } = tmp_instrs.pop().unwrap() else {
                            return Err(Error {
                                message: format!("Unexpected array element value"),
                                loc: item.loc().clone(),
                            });
                        };

                        bytes.extend_from_slice(&ptr.to_le_bytes());
                        bytes.extend_from_slice(&len.to_le_bytes());
                    }
                } else {
                    return Err(Error {
                        message: format!(
                            "Unsupported array literal element type: {}",
                            TypeFmt(self, &item_type)
                        ),
                        loc: loc.clone(),
                    });
                }

                let ptr = self.append_data(bytes);
                instrs.push(WasmInstr::I32Const { value: ptr as i32 });

                self.const_slice_lens.be_mut().push(ConstSliceLen {
                    slice_offset: ptr,
                    slice_len: items.len(),
                });

                return Ok(());
            }
            CodeExpr::ResultLiteral(ResultLiteralExpr {
                is_ok,
                result_type,
                value,
                loc,
            }) => {
                let result = self.get_result_literal_type(ctx, result_type, loc)?;

                let mut value_type = Type::Void;
                if let Some(value) = value {
                    value_type = self.get_expr_type(ctx, value)?;
                }

                if *is_ok {
                    if value_type != *result.ok {
                        return Err(Error {
                            message: format!(
                                "Cannot create result, Ok type mismatch. Got {}, expected: {}",
                                TypeFmt(self, &value_type),
                                TypeFmt(self, &result.ok),
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
                    return Err(Error {
                        message: format!(
                            "Cannot create result, Err type mismatch. Got {}, expected: {}",
                            TypeFmt(self, &value_type),
                            TypeFmt(self, &result.err),
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
                            message: format!(
                                "const {}: {}",
                                ident.repr,
                                TypeFmt(self, &const_def.code_unit.type_)
                            ),
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
                self.codegen_var_set_prepare(instrs, &var);
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
                    return Err(Error {
                        message: format!(
                            "Cannot cast from {} to {}",
                            TypeFmt(self, &castee_type),
                            TypeFmt(self, &casted_to_type)
                        ),
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
                PrefixOpTag::Reference => {
                    let Some(VariableInfo::Stored {
                        mut address,
                        offset,
                        value_type: _,
                        inspect_info,
                    }) = self.var_from_expr(ctx, expr)?
                    else {
                        return Err(Error {
                            message: format!(
                                "Invalid reference expression. Only struct reference fields allowed.",
                            ),
                            loc: loc.clone(),
                        });
                    };
                    if let Some(inspect_info) = inspect_info {
                        self.print_inspection(&inspect_info);
                    }

                    instrs.append(&mut address.instrs);
                    instrs.push(WasmInstr::I32Const {
                        value: offset as i32,
                    });
                    instrs.push(WasmInstr::BinaryOp {
                        kind: WasmBinaryOpKind::I32_ADD,
                    });
                }
                PrefixOpTag::Dereference => {
                    let var = self.var_from_deref(ctx, expr, op_loc)?;
                    if let Some(inspect_info) = var.inspect_info() {
                        self.print_inspection(inspect_info);
                    }
                    self.codegen_var_get(ctx, instrs, &var)?;
                }
                PrefixOpTag::Not => {
                    self.codegen(ctx, instrs, expr)?;

                    let operand_type = self.get_expr_type(ctx, expr)?;
                    let mut wasm_components = Vec::new();
                    self.lower_type(&operand_type, &mut wasm_components);
                    if wasm_components.len() != 1 {
                        return Err(Error {
                            message: format!(
                                "Cannot apply not operation to expr of type {}",
                                TypeFmt(self, &operand_type)
                            ),
                            loc: loc.clone(),
                        });
                    }
                    match wasm_components[0] {
                        WasmType::I32 => {
                            instrs.push(WasmInstr::UnaryOp {
                                kind: WasmUnaryOpKind::I32_EQZ,
                            });
                        }
                        WasmType::I64 => {
                            instrs.push(WasmInstr::UnaryOp {
                                kind: WasmUnaryOpKind::I64_EQZ,
                            });
                        }
                        WasmType::F32 => {
                            instrs.push(WasmInstr::F32Const { value: 0.0 });
                            instrs.push(WasmInstr::BinaryOp {
                                kind: WasmBinaryOpKind::F32_EQ,
                            });
                        }
                        WasmType::F64 => {
                            instrs.push(WasmInstr::F64Const { value: 0.0 });
                            instrs.push(WasmInstr::BinaryOp {
                                kind: WasmBinaryOpKind::F64_EQ,
                            });
                        }
                        WasmType::FuncRef => {
                            return Err(Error {
                                message: format!(
                                    "Cannot apply not operation to expr of type {}",
                                    TypeFmt(self, &operand_type)
                                ),
                                loc: loc.clone(),
                            });
                        }
                    }
                }
                PrefixOpTag::Positive => {
                    self.codegen(ctx, instrs, expr)?;
                }
                PrefixOpTag::Negative => {
                    if let CodeExpr::IntLiteral(int_literal) = expr.as_ref() {
                        self.codegen_int_const(
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
                        return Err(Error {
                            message: format!(
                                "Cannot negate expr of type {}",
                                TypeFmt(self, &operand_type)
                            ),
                            loc: loc.clone(),
                        });
                    }
                    match wasm_components[0] {
                        WasmType::I32 => {
                            instrs.push(WasmInstr::I32Const { value: 0 });
                            self.codegen(ctx, instrs, expr)?;
                            instrs.push(WasmInstr::BinaryOp {
                                kind: WasmBinaryOpKind::I32_SUB,
                            });
                        }
                        WasmType::I64 => {
                            instrs.push(WasmInstr::I64Const { value: 0 });
                            self.codegen(ctx, instrs, expr)?;
                            instrs.push(WasmInstr::BinaryOp {
                                kind: WasmBinaryOpKind::I64_SUB,
                            });
                        }
                        WasmType::F32 => {
                            self.codegen(ctx, instrs, expr)?;
                            instrs.push(WasmInstr::UnaryOp {
                                kind: WasmUnaryOpKind::F32_NEG,
                            });
                        }
                        WasmType::F64 => {
                            self.codegen(ctx, instrs, expr)?;
                            instrs.push(WasmInstr::UnaryOp {
                                kind: WasmUnaryOpKind::F64_NEG,
                            });
                        }
                        WasmType::FuncRef => {
                            return Err(Error {
                                message: format!(
                                    "Cannot negate expr of type {}",
                                    TypeFmt(self, &operand_type)
                                ),
                                loc: loc.clone(),
                            });
                        }
                    }
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

                if !self.is_type_compatible(&lhs_type, &rhs_type) {
                    return Err(Error {
                        message: format!(
                            "Operands are not of the same type: lhs = {}, rhs = {}",
                            TypeFmt(self, &lhs_type),
                            TypeFmt(self, &rhs_type),
                        ),
                        loc: op_loc.clone(),
                    });
                }

                if let Some(base_op) = self.get_compound_assignment_base_op(op_tag) {
                    let Some(var) = self.var_from_expr(ctx, &lhs)? else {
                        return Err(Error {
                            message: format!("Cannot perform compound assignment: invalid lhs"),
                            loc: op_loc.clone(),
                        });
                    };
                    if let Some(inspect_info) = var.inspect_info() {
                        self.print_inspection(inspect_info);
                    }

                    self.codegen_var_set_prepare(instrs, &var);
                    self.codegen_var_get(ctx, instrs, &var)?;
                    self.codegen(ctx, instrs, rhs)?;
                    self.codegen_binary_op(instrs, &base_op, &lhs_type, op_loc)?;

                    self.codegen_var_set(ctx, instrs, &var)?;
                    return Ok(());
                }

                self.codegen(ctx, instrs, lhs)?;
                self.codegen(ctx, instrs, rhs)?;
                self.codegen_binary_op(instrs, &op_tag, &lhs_type, op_loc)?;
            }

            CodeExpr::Assign(AssignExpr {
                op_loc,
                lhs,
                rhs,
                loc: _,
            }) => {
                let Some(var) = self.var_from_expr(ctx, lhs)? else {
                    return Err(Error {
                        message: format!("Cannot perform assignment: invalid lhs"),
                        loc: op_loc.clone(),
                    });
                };
                if let Some(inspect_info) = var.inspect_info() {
                    self.print_inspection(inspect_info);
                }
                self.codegen_var_set_prepare(instrs, &var);
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
                // TODO: add inspections
                if let Some(item) = self.modules[ctx.module_index].get_item(&fn_name.repr) {
                    if let ModuleItemCollection::EnumConstructor = item.collection {
                        let ctor = &self.enum_ctors[item.collection_index];
                        let enum_ = &self.enum_defs[ctor.enum_index];
                        let variant = &enum_.variants[ctor.variant_index];

                        if self.in_inspection_mode {
                            self.print_inspection(&InspectInfo {
                                message: format!("{} // {}", fn_name.repr, ctor.variant_index),
                                loc: fn_name.loc.clone(),
                                linked_loc: Some(variant.loc.clone()),
                            });
                        }

                        self.codegen_int_const(instrs, ctor.variant_index as i32, None);

                        if variant.variant_type == Type::Void && args.items.len() == 0 {
                            return Ok(());
                        }

                        if args.items.len() != 1 {
                            return Err(Error {
                                message: format!(
                                    "Non-void enum constructors require exactly one argument"
                                ),
                                loc: fn_name.loc.clone(),
                            });
                        }

                        let expr_type = self.get_expr_type(ctx, &args.items[0])?;
                        if !self.is_type_compatible(&variant.variant_type, &expr_type) {
                            return Err(Error {
                                message: format!(
                                    "Invalid enum payload: {}, expected: {}",
                                    TypeFmt(self, &expr_type),
                                    TypeFmt(self, &variant.variant_type),
                                ),
                                loc: fn_name.loc.clone(),
                            });
                        }

                        self.codegen(ctx, instrs, &args.items[0])?;
                        return Ok(());
                    }
                }

                self.codegen_fn_call(ctx, instrs, &fn_name.repr, None, &args.items, &fn_name.loc)?;
            }
            CodeExpr::MethodCall(MethodCallExpr {
                lhs,
                field_name,
                args,
                loc: _,
            }) => {
                let lhs_type = self.get_expr_type(ctx, lhs)?;
                let fn_name = self.get_fn_name_from_method(&lhs_type, &field_name.repr);
                self.codegen_fn_call(
                    ctx,
                    instrs,
                    &fn_name,
                    Some(lhs),
                    &args.items,
                    &field_name.loc,
                )?;
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
                    &args.items,
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
                let macro_name = self.get_fn_name_from_method(&lhs_type, &field_name.repr);
                self.codegen_macro_call(
                    ctx,
                    instrs,
                    &macro_name,
                    type_args,
                    Some(lhs),
                    &args.items,
                    &field_name.loc,
                )?;
            }
            CodeExpr::IntrinsicCall(MacroFnCallExpr {
                fn_name,
                type_args,
                args,
                loc: _,
            }) => {
                if fn_name.repr == "unreachable" {
                    if args.items.len() != 0 || type_args.len() != 0 {
                        return Err(Error {
                            message: format!("@{}() accepts no arguments", fn_name.repr),
                            loc: fn_name.loc.clone(),
                        });
                    }

                    instrs.push(WasmInstr::Unreachable);
                    return Ok(());
                }

                if fn_name.repr == "memory_size" {
                    if args.items.len() != 0 || type_args.len() != 0 {
                        return Err(Error {
                            message: format!("@{}() accepts no arguments", fn_name.repr),
                            loc: fn_name.loc.clone(),
                        });
                    }

                    instrs.push(WasmInstr::MemorySize);
                    return Ok(());
                }

                if fn_name.repr == "memory_grow" {
                    if type_args.len() != 0 {
                        return Err(Error {
                            message: format!("@{}() accepts no type arguments", fn_name.repr),
                            loc: fn_name.loc.clone(),
                        });
                    }

                    let mut arg_types = Vec::new();
                    for arg in &args.items {
                        arg_types.push(self.get_expr_type(ctx, arg)?);
                    }
                    let param_types = &[Type::U32];
                    if arg_types != param_types {
                        return Err(Error {
                            message: format!(
                                "Unexpected arguments [{}] for @{}(num_pages: u32): i32",
                                TypeListFmt(self, &arg_types),
                                fn_name.repr,
                            ),
                            loc: fn_name.loc.clone(),
                        });
                    }

                    for arg in &args.items {
                        self.codegen(ctx, instrs, arg)?;
                    }

                    instrs.push(WasmInstr::MemoryGrow);
                    return Ok(());
                }

                if fn_name.repr == "memory_copy" {
                    if type_args.len() != 0 {
                        return Err(Error {
                            message: format!("@{}() accepts no type arguments", fn_name.repr),
                            loc: fn_name.loc.clone(),
                        });
                    }

                    let mut arg_types = Vec::new();
                    for arg in &args.items {
                        arg_types.push(self.get_expr_type(ctx, arg)?);
                    }
                    let param_types = &[Type::U32, Type::U32, Type::U32];
                    if arg_types != param_types {
                        return Err(Error {
                            message: format!(
                                "Unexpected arguments [{}] for @{}(dest: u32, source: u32: num_bytes: u32)",
                                TypeListFmt(self, &arg_types),
                                fn_name.repr,
                            ),
                            loc: fn_name.loc.clone(),
                        });
                    }

                    for arg in &args.items {
                        self.codegen(ctx, instrs, arg)?;
                    }

                    instrs.push(WasmInstr::MemoryCopy);
                    return Ok(());
                }

                if fn_name.repr == "data_size" {
                    if args.items.len() != 0 || type_args.len() != 0 {
                        return Err(Error {
                            message: format!("@{}() accepts no arguments", fn_name.repr),
                            loc: fn_name.loc.clone(),
                        });
                    }

                    if let Some(_) = ctx.fn_index {
                        return Err(Error {
                            message: format!("@{}() can only be used in globals", fn_name.repr),
                            loc: fn_name.loc.clone(),
                        });
                    }

                    instrs.push(WasmInstr::I32Const {
                        value: *self.data_size.borrow() as i32,
                    });
                    return Ok(());
                }

                if fn_name.repr == "const_slice_len" {
                    if args.items.len() != 1 {
                        return report_error(self, fn_name);
                    }

                    let mut slice_ptr_instrs = Vec::new();
                    if let Err(err) = self.codegen(ctx, &mut slice_ptr_instrs, &args.items[0]) {
                        self.report_error(&err);
                        return Ok(());
                    };

                    if slice_ptr_instrs.len() != 1 {
                        return report_error(self, fn_name);
                    }

                    let WasmInstr::I32Const { value: slice_ptr } = &slice_ptr_instrs[0] else {
                        return report_error(self, fn_name);
                    };

                    for const_slice_len in &self.const_slice_lens {
                        if const_slice_len.slice_offset == *slice_ptr as u32 {
                            instrs.push(WasmInstr::I32Const {
                                value: const_slice_len.slice_len as i32,
                            });
                            return Ok(());
                        }
                    }

                    return report_error(self, fn_name);

                    fn report_error(compiler: &Compiler, fn_name: &IdentExpr) -> Result<(), Error> {
                        compiler.report_error(&Error {
                            message: format!(
                                "Invalid arguments for @{}(items: const T[])",
                                fn_name.repr,
                            ),
                            loc: fn_name.loc.clone(),
                        });

                        Ok(())
                    }
                }

                if fn_name.repr == "inspect_items" {
                    if !self.in_inspection_mode {
                        return Ok(());
                    }

                    let module = self
                        .get_module_by_file_index(fn_name.loc.file_index)
                        .unwrap();

                    let mut message = String::new();
                    for item in &module.all_items {
                        write!(message, "{}\n", item.name).unwrap();
                    }

                    self.print_inspection(&InspectInfo {
                        message,
                        loc: fn_name.loc.clone(),
                        linked_loc: None,
                    });
                    return Ok(());
                }

                self.report_error(&Error {
                    message: format!("Unknown intrinsic: {}", fn_name.repr),
                    loc: fn_name.loc.clone(),
                });
            }
            CodeExpr::Dbg(DbgExpr { message, loc }) => {
                let debug_message = format!(
                    "{} - {}",
                    loc.to_string(&self.fm),
                    message.unescape(self.fm.files[loc.file_index].source.as_bytes().relax())
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
                let mut type_layout = TypeLayout::new();
                self.get_type_layout(&lo_type, &mut type_layout);

                instrs.push(WasmInstr::I32Const {
                    value: type_layout.byte_size as i32,
                });
            }

            CodeExpr::Paren(ParenExpr {
                expr,
                has_trailing_comma: _,
                loc: _,
            }) => {
                self.codegen(ctx, instrs, expr)?;
            }
            CodeExpr::Return(ReturnExpr { expr, loc }) => {
                let Some(fn_index) = ctx.fn_index else {
                    return Err(Error {
                        message: format!("Cannot use `return` in const context"),
                        loc: loc.clone(),
                    });
                };

                let mut return_type = Type::Void;

                if let Some(return_expr) = expr {
                    self.codegen(ctx, instrs, return_expr)?;
                    return_type = self.get_expr_type(ctx, &return_expr)?;
                };

                let fn_return_type = &self.functions[fn_index].fn_type.output;
                if !self.is_type_compatible(fn_return_type, &return_type) {
                    return Err(Error {
                        message: format!(
                            "Invalid return type: {}, expected: {}",
                            TypeFmt(self, &return_type),
                            TypeFmt(self, &fn_return_type),
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
                match cond {
                    IfCond::Expr(expr) => {
                        self.codegen(ctx, instrs, expr)?;

                        // `if` condition runs outside of then_branch's scope
                        ctx.enter_scope(ScopeType::Block);
                    }
                    IfCond::Match(match_header) => {
                        // `if match` condition runs inside of then_branch's scope
                        ctx.enter_scope(ScopeType::Block);

                        let enum_ctor = self.codegen_match_header(ctx, instrs, match_header)?;

                        // pop enum's variant from the stack and compare it to the expected variant
                        instrs.push(WasmInstr::I32Const {
                            value: enum_ctor.variant_index as i32,
                        });
                        instrs.push(WasmInstr::BinaryOp {
                            kind: WasmBinaryOpKind::I32_EQ,
                        });
                    }
                }

                instrs.push(WasmInstr::BlockStart {
                    block_kind: WasmBlockKind::If,
                    block_type: WasmBlockType::NoOut,
                });

                self.codegen_code_block(ctx, instrs, &then_block, true);
                ctx.exit_scope();

                match else_block {
                    ElseBlock::None => {}
                    ElseBlock::Else(code_block_expr) => {
                        instrs.push(WasmInstr::Else);
                        ctx.enter_scope(ScopeType::Block);
                        self.codegen_code_block(ctx, instrs, &code_block_expr, true);
                        ctx.exit_scope();
                    }
                    ElseBlock::ElseIf(code_expr) => {
                        instrs.push(WasmInstr::Else);
                        ctx.enter_scope(ScopeType::Block);
                        self.codegen(ctx, instrs, &code_expr)?;
                        ctx.exit_scope();
                    }
                }

                instrs.push(WasmInstr::BlockEnd);
            }
            CodeExpr::Match(MatchExpr {
                header,
                else_branch,
                loc: _,
            }) => {
                let enum_ctor = self.codegen_match_header(ctx, instrs, header)?;

                // pop enum's variant from the stack and compare it to the expected variant
                // if it's not equal then `else_branch`` must run
                instrs.push(WasmInstr::I32Const {
                    value: enum_ctor.variant_index as i32,
                });
                instrs.push(WasmInstr::BinaryOp {
                    kind: WasmBinaryOpKind::I32_NE,
                });

                instrs.push(WasmInstr::BlockStart {
                    block_kind: WasmBlockKind::If,
                    block_type: WasmBlockType::NoOut,
                });

                ctx.enter_scope(ScopeType::Block);
                let terminates_early = self.codegen_code_block(ctx, instrs, &else_branch, true);
                if !terminates_early {
                    self.report_error(&Error {
                        message: format!(
                            "Match's else block must resolve to never, got other type"
                        ),
                        loc: else_branch.loc.clone(),
                    });
                }
                ctx.exit_scope();
                instrs.push(WasmInstr::BlockEnd);
            }
            CodeExpr::WhileLoop(WhileLoopExpr { cond, body, loc: _ }) => {
                instrs.push(WasmInstr::BlockStart {
                    block_kind: WasmBlockKind::Block,
                    block_type: WasmBlockType::NoOut,
                });
                instrs.push(WasmInstr::BlockStart {
                    block_kind: WasmBlockKind::Loop,
                    block_type: WasmBlockType::NoOut,
                });

                if let Some(cond) = cond {
                    catch!(self.codegen(ctx, instrs, cond), err, {
                        self.report_error(&err);
                    });

                    instrs.push(WasmInstr::UnaryOp {
                        kind: WasmUnaryOpKind::I32_EQZ,
                    });
                    // instrs.push(WasmInstr::BranchIf { label_index: 1 });
                    instrs.push(WasmInstr::BlockStart {
                        block_kind: WasmBlockKind::If,
                        block_type: WasmBlockType::NoOut,
                    });
                    instrs.push(WasmInstr::Branch { label_index: 2 });
                    instrs.push(WasmInstr::BlockEnd);
                }

                ctx.enter_scope(ScopeType::Loop);
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
                op_loc,
                loc,
            }) => {
                let counter_type = self.get_expr_type(ctx, start)?;
                if self.get_expr_type(ctx, end)? != counter_type {
                    return Err(Error {
                        message: format!(
                            "Invalid range end type: {}, expected: {}",
                            TypeFmt(self, &self.get_expr_type(ctx, end)?),
                            TypeFmt(self, &counter_type),
                        ),
                        loc: loc.clone(),
                    });
                }

                ctx.enter_scope(ScopeType::ForLoop);

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
                self.codegen_var_set_prepare(instrs, &counter_var);
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
                        self.codegen_binary_op(instrs, &InfixOpTag::Equal, &counter_type, op_loc)?;
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
                        self.codegen_var_set_prepare(instrs, &counter_var);
                        self.codegen_int_const(
                            instrs,
                            1,
                            Some(TypeFmt(self, &counter_type).to_string().as_str()),
                        );
                        self.codegen_binary_op(instrs, &InfixOpTag::Add, &counter_type, op_loc)?;
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
                        ScopeType::Block => {
                            label_index += 1;
                        }
                        ScopeType::Function => {
                            return Err(Error {
                                message: format!("Cannot break outside of a loop"),
                                loc: loc.clone(),
                            });
                        }
                        ScopeType::Loop => break,
                        ScopeType::ForLoop => {
                            label_index += 1;
                            break;
                        }
                        ScopeType::Macro => continue,
                    }
                }

                instrs.push(WasmInstr::Branch { label_index });
            }
            CodeExpr::Continue(ContinueExpr { loc }) => {
                let mut label_index = 0; // 0 = loop, 1 = loop wrapper block

                for scope in ctx.scopes.iter().rev() {
                    match scope.scope_type {
                        ScopeType::Block => {
                            label_index += 1;
                        }
                        ScopeType::Function => {
                            return Err(Error {
                                message: format!("Cannot continue outside of a loop"),
                                loc: loc.clone(),
                            });
                        }
                        ScopeType::Loop => break,
                        ScopeType::ForLoop => break,
                        ScopeType::Macro => continue,
                    }
                }

                instrs.push(WasmInstr::Branch { label_index });
            }
            CodeExpr::DoWith(DoWithExpr {
                args,
                body,
                with_loc,
                loc: _,
            }) => {
                let Some(first_arg) = args.items.first() else {
                    self.report_error(&Error {
                        message: format!("do-with expressions must have at least one argument"),
                        loc: with_loc.clone(),
                    });
                    return Ok(());
                };

                let arg_type = self.get_expr_type(ctx, first_arg)?;

                for arg in &args.items {
                    let current_arg_type = self.get_expr_type(ctx, arg)?;
                    if current_arg_type != arg_type {
                        self.report_error(&Error {
                            message: format!(
                                "do-with argument type mismatch. expected: {}, got: {}",
                                TypeFmt(self, &arg_type),
                                TypeFmt(self, &current_arg_type),
                            ),
                            loc: arg.loc().clone(),
                        });
                        continue;
                    }

                    ctx.enter_scope(ScopeType::Block);

                    self.codegen(ctx, instrs, arg)?;

                    let arg_local_index = self.define_local(
                        ctx,
                        with_loc.clone(),
                        String::from("it"),
                        &arg_type,
                        false,
                    )?;

                    self.codegen_local_set(instrs, &arg_type, arg_local_index);
                    self.codegen(ctx, instrs, body)?;

                    ctx.exit_scope();
                }
            }
            CodeExpr::ExprPipe(ExprPipeExpr {
                lhs,
                rhs,
                op_loc,
                loc: _,
            }) => {
                let lhs_type = self.get_expr_type(ctx, lhs)?;
                catch!(self.codegen(ctx, instrs, lhs), err, {
                    self.report_error(&err);
                    return Ok(());
                });

                ctx.enter_scope(ScopeType::Block);

                let lhs_local_index =
                    self.define_local(ctx, op_loc.clone(), String::from("it"), &lhs_type, false)?;

                self.codegen_local_set(instrs, &lhs_type, lhs_local_index);
                catch!(self.codegen(ctx, instrs, rhs), err, {
                    self.report_error(&err);
                    return Ok(());
                });
                ctx.exit_scope();
            }
            CodeExpr::Defer(DeferExpr { expr, loc: _ }) => {
                let code_unit = self.build_code_unit(ctx, expr)?;

                // macros defer into parent scope
                if let ScopeType::Macro = ctx.current_scope().scope_type {
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
        };

        Ok(())
    }

    /// defines a local with match bind's name and pushes enum's variant to the stack
    fn codegen_match_header(
        &self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        header: &Box<MatchHeader>,
    ) -> Result<&EnumConstructor, Error> {
        let Some(item) = self.modules[ctx.module_index].get_item(&header.variant_name.repr) else {
            return Err(Error {
                message: format!("Unkown enum constructor: {}", header.variant_name.repr),
                loc: header.variant_name.loc.clone(),
            });
        };
        let ModuleItemCollection::EnumConstructor = item.collection else {
            return Err(Error {
                message: format!("Not an enum constructor: {}", header.variant_name.repr),
                loc: header.variant_name.loc.clone(),
            });
        };

        let enum_ctor = &self.enum_ctors[item.collection_index];
        let enum_variant = &self.enum_defs[enum_ctor.enum_index].variants[enum_ctor.variant_index];
        let local_index = self.define_local(
            ctx,
            header.variant_bind.loc.clone(),
            header.variant_bind.repr.clone(),
            &enum_variant.variant_type,
            false,
        )?;
        let local = self.var_local(
            &header.variant_bind.repr,
            enum_variant.variant_type.clone(),
            local_index,
            header.variant_bind.loc.clone(),
            None,
        );
        if let Some(inspect_info) = local.inspect_info() {
            self.print_inspection(inspect_info);
        }
        self.codegen_var_set_prepare(instrs, &local);
        self.codegen(ctx, instrs, &header.expr_to_match)?;
        self.codegen_var_set(ctx, instrs, &local)?;

        Ok(enum_ctor)
    }

    fn codegen_fn_call(
        &self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        fn_name: &str,
        receiver_arg: Option<&CodeExpr>,
        args: &Vec<CodeExpr>,
        loc: &Loc,
    ) -> Result<(), Error> {
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
            let mut message = String::new();

            write!(&mut message, "fn {fn_name}(").unwrap();

            for (param, i) in fn_info.fn_params.iter().zip(0..) {
                if i != 0 {
                    message.push_str(", ");
                }

                message.push_str(&param.param_name);
                message.push_str(": ");
                write!(&mut message, "{}", TypeFmt(self, &param.param_type)).unwrap();
            }

            let return_type = TypeFmt(self, &fn_info.fn_type.output);
            write!(&mut message, "): {}", return_type).unwrap();

            self.print_inspection(&InspectInfo {
                message,
                loc: loc.clone(),
                linked_loc: Some(fn_info.definition_loc.clone()),
            });
        }

        if !self.is_types_compatible(&fn_info.fn_type.inputs, &arg_types) {
            return Err(Error {
                message: format!(
                    "Invalid function arguments for function {}: [{}], expected [{}]",
                    fn_info.fn_name,
                    TypeListFmt(self, &arg_types),
                    TypeListFmt(self, &fn_info.fn_type.inputs),
                ),
                loc: loc.clone(),
            });
        }

        instrs.push(WasmInstr::Call {
            fn_index: fn_info.wasm_fn_index,
        });

        Ok(())
    }

    fn get_fn_info_for_call(
        &self,
        ctx: &ExprContext,
        fn_name: &str,
        loc: &Loc,
    ) -> Result<&FnInfo, Error> {
        let Some(item) = self.modules[ctx.module_index].get_item(fn_name) else {
            return Err(Error {
                message: format!("Unknown function: {}", fn_name),
                loc: loc.clone(),
            });
        };

        let ModuleItemCollection::Function = item.collection else {
            return Err(Error {
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
        ctx: &mut ExprContext,
        macro_name: &str,
        type_args: &Vec<TypeExpr>,
        args: &Vec<CodeExpr>,
        receiver_arg: Option<&CodeExpr>,
        loc: &Loc,
    ) -> Result<Type, Error> {
        ctx.enter_scope(ScopeType::Macro);

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
            Type::Void
        };

        Ok(return_type)
    }

    fn populate_ctx_from_macro_call(
        &self,
        ctx: &mut ExprContext,
        macro_name: &str,
        type_args: &Vec<TypeExpr>,
        receiver_arg: Option<&CodeExpr>,
        args: &Vec<CodeExpr>,
        loc: &Loc,
        lo_type_args: Option<&mut Vec<Type>>,
    ) -> Result<&MacroDefExpr, Error> {
        // TODO: find a way to not allocate
        let Some(item) = self.modules[ctx.module_index].get_item(&(String::from(macro_name) + "!"))
        else {
            return Err(Error {
                message: format!("Unknown macro: {}", macro_name),
                loc: loc.clone(),
            });
        };

        let macro_def = self.macro_defs[item.collection_index];

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
            return Err(Error {
                message: format!(
                    "Invalid number of type args, expected {}, got {}",
                    macro_def.macro_type_params.len(),
                    type_args.len()
                ),
                loc: loc.clone(),
            });
        }

        for (type_param, type_arg) in macro_def.macro_type_params.iter().zip(lo_type_args.iter()) {
            ctx.current_scope_mut().macro_type_args.push(MacroTypeArg {
                name: type_param.clone(),
                type_: type_arg.clone(),
            });
        }

        if all_args.len() != macro_def.macro_params.len() {
            return Err(Error {
                message: format!(
                    "Invalid number of macro args, expected {}, got {}",
                    macro_def.macro_params.len(),
                    all_args.len()
                ),
                loc: loc.clone(),
            });
        }

        let mut arg_types = Vec::<Type>::new();
        for arg in &all_args {
            arg_types.push(arg.type_.clone());
        }

        for (macro_param, macro_arg) in macro_def.macro_params.iter().zip(all_args.into_iter()) {
            let const_def = ConstDef {
                const_name: macro_param.param_name.repr.clone(),
                code_unit: macro_arg,
                loc: macro_param.loc.clone(),
            };

            if let FnParamType::Infer { name } = &macro_param.param_type {
                ctx.current_scope_mut().macro_type_args.push(MacroTypeArg {
                    name: name.clone(),
                    type_: const_def.code_unit.type_.clone(),
                });
            }

            ctx.current_scope_mut().macro_args.push(const_def);
        }

        let self_type = self.get_fn_self_type(&macro_def.macro_name, &macro_def.macro_params);

        let mut macro_types = Vec::<Type>::new();
        for macro_param in &macro_def.macro_params {
            let macro_type = if let FnParamType::Infer { name } = &macro_param.param_type {
                ctx.get_macro_type_arg(name).unwrap().clone()
            } else {
                self.get_fn_param_type(ctx, &self_type, macro_param)?
            };
            macro_types.push(macro_type);
        }

        if !self.is_types_compatible(&macro_types, &arg_types) {
            return Err(Error {
                message: format!(
                    "Invalid macro args, expected {}, got {}",
                    TypeListFmt(self, &macro_types),
                    TypeListFmt(self, &arg_types)
                ),
                loc: loc.clone(),
            });
        }

        Ok(macro_def)
    }

    fn codegen_macro_call(
        &self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        macro_name: &str,
        type_args: &Vec<TypeExpr>,
        receiver_arg: Option<&CodeExpr>,
        args: &Vec<CodeExpr>,
        loc: &Loc,
    ) -> Result<(), Error> {
        ctx.enter_scope(ScopeType::Macro);

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
            let mut message = String::new();

            let lo_type_args = TypeListFmt(self, &lo_type_args);
            write!(&mut message, "macro {macro_name}!<{lo_type_args}>(").unwrap();

            let macro_args_len = ctx.current_scope().macro_args.len();
            for i in macro_args_len - macro_def.macro_params.len()..macro_args_len {
                if i != 0 {
                    message.push_str(", ");
                }

                let macro_arg = &ctx.current_scope().macro_args[i];
                message.push_str(&macro_arg.const_name);
                message.push_str(": ");
                let arg_type = TypeFmt(self, &macro_arg.code_unit.type_);
                write!(&mut message, "{arg_type}",).unwrap();
            }

            let return_type = if let Some(return_type) = &macro_def.return_type {
                self.build_type(ctx, return_type)?
            } else {
                Type::Void
            };
            write!(&mut message, "): {}", TypeFmt(self, &return_type)).unwrap();

            self.print_inspection(&InspectInfo {
                message,
                loc: loc.clone(),
                linked_loc: Some(macro_def.macro_name.loc.clone()),
            });
        }

        self.codegen_code_block(ctx, instrs, &macro_def.body, false);

        ctx.exit_scope(); // exit macro scope

        Ok(())
    }

    fn codegen_catch(
        &self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        expr: &CodeExpr,
        error_bind: Option<&IdentExpr>,
        catch_body: Option<&CodeBlock>,
        loc: &Loc,
    ) -> Result<(), Error> {
        let expr_type = self.get_expr_type(ctx, expr)?;
        let result = self.assert_catchable_type(&expr_type, loc)?;

        ctx.enter_scope(ScopeType::Block); // enter catch scope

        // put result on the stack
        self.codegen(ctx, instrs, expr)?;

        // pop error
        let (error_bind, error_bind_loc) = if let Some(error_bind) = error_bind {
            (error_bind.repr.clone(), error_bind.loc.clone())
        } else {
            (String::from("<err>"), Loc::internal())
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
        self.codegen_var_set_prepare(instrs, &err_var);
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
            let terminates_early = self.codegen_code_block(ctx, instrs, catch_body, true);
            if !terminates_early {
                self.report_error(&Error {
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
        let ok_var = VariableInfo::Local {
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
        pointee_type: &Type,
        offset: u32,
        is_store: bool,
    ) {
        match pointee_type {
            Type::Never | Type::Void => {} // noop
            Type::Bool | Type::U8 => {
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
            Type::I8 => {
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
            Type::U16 => {
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
            Type::I16 => {
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
            Type::U32
            | Type::I32
            | Type::Null
            | Type::Pointer { pointee: _ }
            | Type::SequencePointer { pointee: _ } => {
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
            Type::U64 | Type::I64 => {
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
            Type::F32 => {
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
            Type::F64 => {
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
            Type::StructInstance { struct_index } => {
                let struct_def = &self.struct_defs[*struct_index];

                for struct_field in struct_def.fields.iter().rev() {
                    self.codegen_load_or_store(
                        instrs,
                        &struct_field.field_type,
                        offset + struct_field.byte_offset,
                        is_store,
                    );
                }
            }
            Type::EnumInstance { enum_index } => {
                let enum_def = &self.enum_defs[*enum_index];

                let mut tag_layout = TypeLayout::new();
                self.get_type_layout(&Type::U32, &mut tag_layout);

                // TODO: figure out alignment

                self.codegen_load_or_store(instrs, &Type::U32, offset, is_store);
                self.codegen_load_or_store(
                    instrs,
                    &enum_def.variant_type,
                    offset + tag_layout.byte_size,
                    is_store,
                );
            }
            Type::Result(ResultType { ok, err }) => {
                let mut ok_layout = TypeLayout::new();
                self.get_type_layout(&ok, &mut ok_layout);

                // TODO: figure out alignment

                self.codegen_load_or_store(instrs, &ok, offset, is_store);
                self.codegen_load_or_store(instrs, &err, offset + ok_layout.byte_size, is_store);
            }
        }
    }

    fn get_result_literal_type(
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
                loc: loc.clone(),
            });
        };

        let fn_info = &self.functions[fn_index];
        let Type::Result(result) = &fn_info.fn_type.output else {
            return Err(Error {
                message: format!(
                    "Cannot create implicitly typed result: function does not return result"
                ),
                loc: loc.clone(),
            });
        };

        Ok(ResultType {
            ok: result.ok.clone(),
            err: result.err.clone(),
        })
    }

    fn get_block_inout_type(&self, inputs: &[Type], output: &Type) -> u32 {
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

    fn get_expr_type(&self, ctx: &ExprContext, expr: &CodeExpr) -> Result<Type, Error> {
        match expr {
            CodeExpr::BoolLiteral(_) => Ok(Type::Bool),
            CodeExpr::CharLiteral(_) => Ok(Type::U8),
            CodeExpr::NullLiteral(_) => Ok(Type::Null),
            CodeExpr::IntLiteral(IntLiteralExpr {
                repr: _,
                value: _,
                tag,
                loc,
            }) => match tag.as_deref() {
                None => Ok(Type::U32),
                Some("u8") => Ok(Type::U8),
                Some("i8") => Ok(Type::I8),
                Some("u16") => Ok(Type::U16),
                Some("i16") => Ok(Type::I16),
                Some("u32") => Ok(Type::U32),
                Some("i32") => Ok(Type::I32),
                Some("f32") => Ok(Type::F32),
                Some("u64") => Ok(Type::U64),
                Some("i64") => Ok(Type::I64),
                Some("f64") => Ok(Type::F64),
                Some(tag) => {
                    return Err(Error {
                        message: format!("Unknown int literal tag: {tag}"),
                        loc: loc.clone(),
                    });
                }
            },
            CodeExpr::StringLiteral(StringLiteralExpr {
                repr: _,
                value: _,
                loc,
            }) => {
                let Some(item) = self.modules[ctx.module_index].get_item("str") else {
                    return Err(Error {
                        message: format!("Cannot use strings with no `str` struct defined"),
                        loc: loc.clone(),
                    });
                };

                Ok(Type::StructInstance {
                    struct_index: item.collection_index,
                })
            }
            CodeExpr::StructLiteral(StructLiteralExpr {
                struct_name,
                fields: _,
                has_trailing_comma: _,
                loc,
            }) => {
                let Some(item) = self.modules[ctx.module_index].get_item(&struct_name.repr) else {
                    return Err(Error {
                        message: format!("Unknown struct: {}", struct_name.repr),
                        loc: loc.clone(),
                    });
                };

                return Ok(Type::StructInstance {
                    struct_index: item.collection_index,
                });
            }
            CodeExpr::ArrayLiteral(ArrayLiteralExpr {
                item_type,
                items: _,
                loc: _,
            }) => {
                let item_type = self.build_type(ctx, item_type)?;
                return Ok(Type::SequencePointer {
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
                return Ok(Type::Result(result));
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
                | InfixOpTag::ExprPipe => unreachable!(),
            },
            CodeExpr::PrefixOp(PrefixOpExpr {
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
                            loc: loc.clone(),
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
                let field = self.get_struct_or_struct_ref_field(&lhs_type, field_access)?;
                Ok(field.field_type.clone())
            }
            CodeExpr::FnCall(FnCallExpr {
                fn_name,
                args: _,
                loc: _,
            }) => {
                if let Some(item) = self.modules[ctx.module_index].get_item(&fn_name.repr) {
                    if let ModuleItemCollection::EnumConstructor = item.collection {
                        let ctor = &self.enum_ctors[item.collection_index];

                        return Ok(Type::EnumInstance {
                            enum_index: ctor.enum_index,
                        });
                    }
                }

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
                let fn_name = self.get_fn_name_from_method(&lhs_type, &field_name.repr);

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
                self.get_macro_return_type(
                    &mut ctx,
                    &fn_name.repr,
                    type_args,
                    &args.items,
                    None,
                    loc,
                )
            }
            CodeExpr::IntrinsicCall(MacroFnCallExpr {
                fn_name,
                type_args: _,
                args: _,
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

                if fn_name.repr == "const_slice_len" {
                    return Ok(Type::U32);
                }

                if fn_name.repr.starts_with("inspect_") {
                    return Ok(Type::Void);
                }

                Err(Error {
                    message: format!("Unknown intrinsic macro: {}", fn_name.repr),
                    loc: fn_name.loc.clone(),
                })
            }
            CodeExpr::MacroMethodCall(MacroMethodCallExpr {
                lhs,
                field_name,
                type_args,
                args,
                loc,
            }) => {
                let lhs_type = self.get_expr_type(ctx, lhs)?;
                let macro_name = self.get_fn_name_from_method(&lhs_type, &field_name.repr);

                let mut ctx = ctx.clone();
                self.get_macro_return_type(
                    &mut ctx,
                    &macro_name,
                    type_args,
                    &args.items,
                    Some(&lhs),
                    loc,
                )
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
            CodeExpr::ExprPipe(ExprPipeExpr {
                lhs,
                rhs,
                op_loc,
                loc: _,
            }) => {
                let ctx = ctx.be_mut();

                let lhs_type = catch!(self.get_expr_type(ctx, &lhs), err, {
                    self.report_error(&err);
                    return Ok(Type::Never);
                });

                ctx.enter_scope(ScopeType::Block);

                ctx.current_scope_mut().macro_args.push(ConstDef {
                    const_name: String::from("it"),
                    code_unit: CodeUnit {
                        type_: lhs_type,
                        instrs: Vec::new(),
                    },
                    loc: op_loc.clone(),
                });

                let rhs_type = catch!(self.get_expr_type(ctx, &rhs), err, {
                    self.report_error(&err);
                    return Ok(Type::Never);
                });

                ctx.exit_scope();

                return Ok(rhs_type);
            }
            CodeExpr::Dbg(_) => Ok(Type::StructInstance {
                struct_index: self.modules[ctx.module_index]
                    .get_item("str")
                    .unwrap()
                    .collection_index,
            }),
            CodeExpr::Sizeof(_) => Ok(Type::U32),
            CodeExpr::Let(let_) => {
                if self.get_expr_type(ctx, &let_.value)? == Type::Never {
                    return Ok(Type::Never);
                }
                Ok(Type::Void)
            }
            CodeExpr::Assign(_) => Ok(Type::Void),
            CodeExpr::Defer(_) => Ok(Type::Void),
            CodeExpr::If(IfExpr {
                cond,
                then_block,
                else_block,
                loc: _,
            }) => {
                match cond {
                    IfCond::Expr(e) => {
                        if self.get_expr_type(ctx, e)? == Type::Never {
                            return Ok(Type::Never);
                        }
                    }
                    IfCond::Match(_) => {}
                };

                let then_diverges =
                    self.get_code_block_type(ctx, &then_block.exprs)? == Type::Never;

                let mut else_diverges = false;
                match else_block {
                    ElseBlock::None => {}
                    ElseBlock::Else(else_) => {
                        else_diverges = self.get_code_block_type(ctx, &else_.exprs)? == Type::Never;
                    }
                    ElseBlock::ElseIf(e) => {
                        else_diverges = self.get_expr_type(ctx, e)? == Type::Never;
                    }
                }

                if then_diverges && else_diverges {
                    return Ok(Type::Never);
                }

                Ok(Type::Void)
            }
            CodeExpr::Match(_) => Ok(Type::Void),
            CodeExpr::WhileLoop(_) => Ok(Type::Void),
            CodeExpr::ForLoop(_) => Ok(Type::Void),
            CodeExpr::Break(_) => Ok(Type::Never),
            CodeExpr::Continue(_) => Ok(Type::Never),
            CodeExpr::DoWith(_) => Ok(Type::Void),
            CodeExpr::Return(_) => Ok(Type::Never),
            CodeExpr::Paren(ParenExpr {
                expr,
                has_trailing_comma: _,
                loc: _,
            }) => self.get_expr_type(ctx, expr),
        }
    }

    fn get_code_block_type(&self, ctx: &ExprContext, exprs: &Vec<CodeExpr>) -> Result<Type, Error> {
        let ctx = ctx.be_mut();

        let mut diverges = false;

        ctx.enter_scope(ScopeType::Block);

        for expr in exprs {
            if let CodeExpr::Let(LetExpr {
                local_name,
                value,
                loc,
            }) = expr
            {
                let value_type = self.get_expr_type(ctx, value)?;
                diverges = diverges || value_type == Type::Never;

                ctx.current_scope_mut().macro_args.push(ConstDef {
                    const_name: local_name.repr.clone(),
                    code_unit: CodeUnit {
                        type_: value_type,
                        instrs: Vec::new(),
                    },
                    loc: loc.clone(),
                });

                continue;
            }

            diverges = diverges || self.get_expr_type(ctx, expr)? == Type::Never;
        }

        ctx.exit_scope();

        if diverges {
            return Ok(Type::Never);
        }

        Ok(Type::Void)
    }

    fn is_naturally_divergent(&self, expr: &CodeExpr) -> bool {
        match expr {
            CodeExpr::BoolLiteral(_)
            | CodeExpr::CharLiteral(_)
            | CodeExpr::NullLiteral(_)
            | CodeExpr::IntLiteral(_)
            | CodeExpr::StringLiteral(_)
            | CodeExpr::StructLiteral(_)
            | CodeExpr::ArrayLiteral(_)
            | CodeExpr::ResultLiteral(_)
            | CodeExpr::Ident(_)
            | CodeExpr::Let(_)
            | CodeExpr::Cast(_)
            | CodeExpr::Assign(_)
            | CodeExpr::FieldAccess(_)
            | CodeExpr::PropagateError(_)
            | CodeExpr::FnCall(_)
            | CodeExpr::MethodCall(_)
            | CodeExpr::MacroFnCall(_)
            | CodeExpr::If(_)
            | CodeExpr::Dbg(_)
            | CodeExpr::Sizeof(_)
            | CodeExpr::WhileLoop(_)
            | CodeExpr::ForLoop(_)
            | CodeExpr::Defer(_)
            | CodeExpr::MacroMethodCall(_) => false,

            CodeExpr::Break(_) | CodeExpr::Continue(_) | CodeExpr::Return(_) => true,

            CodeExpr::Paren(paren_expr) => self.is_naturally_divergent(&paren_expr.expr),
            CodeExpr::IntrinsicCall(intrinsic) => intrinsic.fn_name.repr == "unreachable",

            CodeExpr::Catch(catch_) => self.is_naturally_divergent(&catch_.lhs),
            CodeExpr::InfixOp(infix) => {
                self.is_naturally_divergent(&infix.lhs) || self.is_naturally_divergent(&infix.rhs)
            }
            CodeExpr::PrefixOp(prefix) => self.is_naturally_divergent(&prefix.expr),
            CodeExpr::Match(match_) => self.is_naturally_divergent(&match_.header.expr_to_match),
            CodeExpr::ExprPipe(pipe_) => {
                self.is_naturally_divergent(&pipe_.lhs) || self.is_naturally_divergent(&pipe_.rhs)
            }

            CodeExpr::DoWith(do_with_) => {
                let mut divergent = self.is_naturally_divergent(&do_with_.body);
                for expr in &do_with_.args.items {
                    divergent = divergent || self.is_naturally_divergent(expr);
                }
                divergent
            }
        }
    }

    fn var_from_expr(
        &self,
        ctx: &mut ExprContext,
        expr: &CodeExpr,
    ) -> Result<Option<VariableInfo>, Error> {
        Ok(match expr {
            CodeExpr::Ident(ident) => Some(self.var_from_ident(ctx, ident)?),
            CodeExpr::FieldAccess(field_access) => {
                Some(self.var_from_field_access(ctx, field_access)?)
            }
            CodeExpr::Paren(ParenExpr {
                expr,
                has_trailing_comma: _,
                loc: _,
            }) => self.var_from_expr(ctx, expr)?,
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
        local_type: Type,
        local_index: u32,
        loc: Loc,
        linked_loc: Option<Loc>,
    ) -> VariableInfo {
        let inspect_info = if self.in_inspection_mode {
            Some(InspectInfo {
                message: format!("let {}: {}", local_name, TypeFmt(self, &local_type)),
                loc: loc.clone(),
                linked_loc,
            })
        } else {
            None
        };

        VariableInfo::Local {
            local_index,
            local_type,
            inspect_info,
        }
    }

    fn var_from_ident(&self, ctx: &ExprContext, ident: &IdentExpr) -> Result<VariableInfo, Error> {
        if let Some(local) = ctx.get_local(&ident.repr) {
            return Ok(self.var_local(
                &ident.repr,
                local.local_type.clone(),
                local.local_index,
                ident.loc.clone(),
                Some(local.definition_loc.clone()),
            ));
        };

        if let Some(item) = self.modules[ctx.module_index].get_item(&ident.repr) {
            if let ModuleItemCollection::Global = item.collection {
                let global = &self.globals[item.collection_index];

                let mut inspect_info = None;
                if self.in_inspection_mode {
                    inspect_info = Some(InspectInfo {
                        message: format!(
                            "global {}: {}",
                            ident.repr,
                            TypeFmt(self, &global.global_type)
                        ),
                        loc: ident.loc.clone(),
                        linked_loc: Some(item.loc.clone()),
                    });
                }

                return Ok(VariableInfo::Global {
                    global_index: global.global_index,
                    global_type: global.global_type.clone(),
                    inspect_info,
                });
            }
        }

        if let Some(const_) = self.get_const(ctx, &ident.repr) {
            let mut inspect_info = None;
            if self.in_inspection_mode {
                inspect_info = Some(InspectInfo {
                    message: format!(
                        "const {}: {}",
                        ident.repr,
                        TypeFmt(self, &const_.code_unit.type_)
                    ),
                    loc: ident.loc.clone(),
                    linked_loc: Some(const_.loc.clone()),
                })
            }

            return Ok(VariableInfo::Const {
                code_unit: const_.code_unit.relax(),
                loc: const_.loc.clone(),
                inspect_info,
            });
        }

        return Err(Error {
            message: format!("Unknown variable: {}", ident.repr),
            loc: ident.loc.clone(),
        });
    }

    fn var_from_field_access(
        &self,
        ctx: &mut ExprContext,
        field_access: &FieldAccessExpr,
    ) -> Result<VariableInfo, Error> {
        let lhs_type = self.get_expr_type(ctx, field_access.lhs.as_ref())?;

        let field = self.get_struct_or_struct_ref_field(&lhs_type, field_access)?;

        let inspect_info = if self.in_inspection_mode {
            Some(InspectInfo {
                message: format!(
                    "{}.{}: {}",
                    TypeFmt(self, &lhs_type),
                    field.field_name,
                    TypeFmt(self, &field.field_type),
                ),
                loc: field_access.field_name.loc.clone(),
                linked_loc: Some(field.loc.clone()),
            })
        } else {
            None
        };

        if let Type::Pointer { pointee: _ } = lhs_type {
            return Ok(VariableInfo::Stored {
                address: self.build_code_unit(ctx, &field_access.lhs)?,
                offset: field.byte_offset,
                value_type: field.field_type.clone(),
                inspect_info,
            });
        }

        if let Some(var) = self.var_from_expr(ctx, &field_access.lhs.as_ref())? {
            match var {
                // struct globals are not supported so these are handled the same way as struct values
                VariableInfo::Global {
                    global_index: _,
                    global_type: _,
                    inspect_info: _,
                } => {}
                // consts are handled as struct values as well
                VariableInfo::Const {
                    code_unit: _,
                    loc: _,
                    inspect_info: _,
                } => {}
                VariableInfo::Local {
                    local_index,
                    local_type: _,
                    inspect_info: parent_inspect_info,
                } => {
                    if let Some(inspect_info) = parent_inspect_info {
                        self.print_inspection(&inspect_info);
                    }

                    return Ok(VariableInfo::Local {
                        local_index: local_index + field.field_index,
                        local_type: field.field_type.clone(),
                        inspect_info,
                    });
                }
                VariableInfo::Stored {
                    address,
                    offset,
                    value_type: _,
                    inspect_info: parent_inspect_info,
                } => {
                    if let Some(inspect_info) = parent_inspect_info {
                        self.print_inspection(&inspect_info);
                    }

                    return Ok(VariableInfo::Stored {
                        address,
                        offset: offset + field.byte_offset,
                        value_type: field.field_type.clone(),
                        inspect_info,
                    });
                }
                VariableInfo::StructValueField {
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

                    return Ok(VariableInfo::StructValueField {
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

        return Ok(VariableInfo::StructValueField {
            struct_value: self.build_code_unit(ctx, &field_access.lhs)?,
            field_type: field.field_type.clone(),
            drops_before: struct_components_count - field.field_index - field_components_count,
            drops_after: field.field_index,
            loc: field_access.field_name.loc.clone(),
            inspect_info,
        });
    }

    fn create_or_get_addr_local(&self, ctx: &mut ExprContext) -> u32 {
        if let Some(addr_local_index) = ctx.addr_local_index {
            return addr_local_index;
        }

        let addr_local_index = self.define_unnamed_local(ctx, Loc::internal(), &Type::U32, false);

        return addr_local_index;
    }

    fn get_struct_or_struct_ref_field(
        &self,
        mut lhs_type: &Type,
        field_access: &FieldAccessExpr,
    ) -> Result<&StructField, Error> {
        if let Type::Pointer { pointee } = &lhs_type {
            lhs_type = pointee;
        }

        let Type::StructInstance { struct_index } = lhs_type else {
            return Err(Error {
                message: format!(
                    "Cannot get field '{}' on non struct: {}",
                    field_access.field_name.repr,
                    TypeFmt(self, lhs_type),
                ),
                loc: field_access.field_name.loc.clone(),
            });
        };

        let struct_def = &self.struct_defs[*struct_index];
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
                loc: field_access.field_name.loc.clone(),
            });
        };

        Ok(field)
    }

    fn var_from_deref(
        &self,
        ctx: &mut ExprContext,
        addr_expr: &CodeExpr,
        op_loc: &Loc,
    ) -> Result<VariableInfo, Error> {
        let addr_type = self.get_expr_type(ctx, addr_expr)?;

        if let Type::Pointer { pointee } = &addr_type {
            let inspect_info = if self.in_inspection_mode {
                Some(InspectInfo {
                    message: format!("<deref>: {}", TypeFmt(self, &pointee)),
                    loc: op_loc.clone(),
                    linked_loc: None,
                })
            } else {
                None
            };

            return Ok(VariableInfo::Stored {
                address: self.build_code_unit(ctx, addr_expr)?,
                offset: 0,
                value_type: pointee.as_ref().clone(),
                inspect_info,
            });
        };

        return Err(Error {
            message: format!(
                "Cannot dereference expression of type '{}'",
                TypeFmt(self, &addr_type)
            ),
            loc: addr_expr.loc().clone(),
        });
    }

    fn codegen_var_get(
        &self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        var: &VariableInfo,
    ) -> Result<(), Error> {
        match var {
            VariableInfo::Local {
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
            VariableInfo::Global {
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
            VariableInfo::Const {
                code_unit,
                loc: _,
                inspect_info: _,
            } => {
                instrs.extend_from_slice(&code_unit.instrs);
            }
            VariableInfo::Stored {
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
            VariableInfo::StructValueField {
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

                    let var = VariableInfo::Local {
                        local_index,
                        local_type: field_type.clone(),
                        inspect_info: None,
                    };
                    self.codegen_var_set_prepare(instrs, &var);
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
    fn codegen_var_set_prepare(&self, instrs: &mut Vec<WasmInstr>, var: &VariableInfo) {
        match var {
            VariableInfo::Stored {
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
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        var: &VariableInfo,
    ) -> Result<(), Error> {
        match var {
            VariableInfo::Local {
                local_index,
                local_type,
                inspect_info: _,
            } => {
                self.codegen_local_set(instrs, local_type, *local_index);
            }
            VariableInfo::Global {
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
            VariableInfo::Stored {
                address: _,
                offset,
                value_type,
                inspect_info: _,
            } => {
                let mut stores = Vec::new();
                self.codegen_load_or_store(&mut stores, &value_type, *offset, true);

                if stores.len() > 1 {
                    let tmp_value_local_index =
                        self.define_unnamed_local(ctx, Loc::internal(), value_type, false);
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
            VariableInfo::Const {
                code_unit: _,
                loc,
                inspect_info: _,
            }
            | VariableInfo::StructValueField {
                struct_value: _,
                field_type: _,
                drops_before: _,
                drops_after: _,
                loc,
                inspect_info: _,
            } => {
                return Err(Error {
                    message: format!("Cannot mutate a constant"),
                    loc: loc.clone(),
                });
            }
        };

        Ok(())
    }

    fn codegen_local_set(&self, instrs: &mut Vec<WasmInstr>, local_type: &Type, local_index: u32) {
        for i in (0..self.count_wasm_type_components(local_type)).rev() {
            instrs.push(WasmInstr::LocalSet {
                local_index: local_index + i,
            });
        }
    }

    fn define_local(
        &self,
        ctx: &mut ExprContext,
        loc: Loc,
        local_name: String,
        local_type: &Type,
        is_fn_param: bool,
    ) -> Result<u32, Error> {
        for local in ctx.current_scope().locals.iter() {
            if local.local_name == local_name && local.defined_in_this_scope {
                let Local { definition_loc, .. } = &ctx.locals[local.lo_local_index];

                return Err(Error {
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
        ctx.current_scope_mut().locals.push(ScopedLocal {
            local_name,
            lo_local_index,
            defined_in_this_scope: true,
        });

        Ok(local_index)
    }

    fn define_unnamed_local(
        &self,
        ctx: &mut ExprContext,
        loc: Loc,
        local_type: &Type,
        is_fn_param: bool,
    ) -> u32 {
        let local_index = ctx.next_local_index;
        ctx.locals.push(Local {
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
        expr_type: &'a Type,
        loc: &Loc,
    ) -> Result<&'a ResultType, Error> {
        let Type::Result(result) = expr_type else {
            return Err(Error {
                message: format!(
                    "Cannot catch error from expr of type {}",
                    TypeFmt(self, &expr_type)
                ),
                loc: loc.clone(),
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
                loc: loc.clone(),
            });
        }

        Ok(result)
    }

    fn emit_deferred(&self, scope: &Scope, instrs: &mut Vec<WasmInstr>) {
        for expr in scope.deferred.iter().rev() {
            for instr in &expr.instrs {
                instrs.push(instr.clone());
            }
        }
    }

    // TODO!!!: add similar logic for break/continue
    fn emit_deferred_for_return(&self, ctx: &ExprContext, instrs: &mut Vec<WasmInstr>) {
        for scope in ctx.scopes.iter().rev() {
            self.emit_deferred(scope, instrs);
        }
    }

    fn build_code_unit(&self, ctx: &mut ExprContext, expr: &CodeExpr) -> Result<CodeUnit, Error> {
        let mut code_unit = CodeUnit {
            type_: self.get_expr_type(ctx, expr)?,
            instrs: Vec::new(),
        };
        self.codegen(ctx, &mut code_unit.instrs, expr)?;

        Ok(code_unit)
    }

    fn process_const_string(&self, value: String, loc: &Loc) -> Result<Str, Error> {
        if self.memory.is_none() && !self.in_inspection_mode {
            return Err(Error {
                message: format!("Cannot use strings with no memory defined"),
                loc: loc.clone(),
            });
        }

        let string_len = value.as_bytes().len() as u32;

        for pooled_str in self.string_pool.borrow().iter() {
            if *pooled_str.value == value {
                return Ok(Str {
                    ptr: pooled_str.ptr,
                    len: string_len,
                });
            }
        }

        let ptr = self.append_data(value.clone().into_bytes());

        self.string_pool
            .borrow_mut()
            .push(PooledString { value, ptr });

        return Ok(Str {
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
    fn ensure_const_expr(&self, _expr: &CodeExpr) -> Result<(), Error> {
        Ok(())
    }

    fn get_type_or_err(&self, type_name: &str, loc: &Loc) -> Result<Type, Error> {
        let Some(item) = self
            .get_module_by_file_index(loc.file_index)
            .unwrap()
            .get_item(type_name)
        else {
            return Err(Error {
                message: format!("Unknown type: {}", type_name),
                loc: loc.clone(),
            });
        };

        match item.collection {
            ModuleItemCollection::Struct => {
                if self.in_inspection_mode {
                    self.print_inspection(&InspectInfo {
                        message: format!("struct {type_name} {{ ... }}"),
                        loc: loc.clone(),
                        linked_loc: Some(item.loc.clone()),
                    });
                }

                Ok(Type::StructInstance {
                    struct_index: item.collection_index,
                })
            }
            ModuleItemCollection::Enum => {
                if self.in_inspection_mode {
                    self.print_inspection(&InspectInfo {
                        message: format!("enum {type_name} {{ ... }}"),
                        loc: loc.clone(),
                        linked_loc: Some(item.loc.clone()),
                    });
                }

                Ok(Type::EnumInstance {
                    enum_index: item.collection_index,
                })
            }
            ModuleItemCollection::TypeAlias => {
                let type_ = &self.type_aliases[item.collection_index];

                // don't print inspection for built-ins
                if self.in_inspection_mode && item.loc.file_index != 0 {
                    self.print_inspection(&InspectInfo {
                        message: format!("type {type_name} = {}", TypeFmt(self, &type_)),
                        loc: loc.clone(),
                        linked_loc: Some(item.loc.clone()),
                    });
                }

                Ok(type_.clone())
            }
            ModuleItemCollection::Function
            | ModuleItemCollection::Macro
            | ModuleItemCollection::Global
            | ModuleItemCollection::Const
            | ModuleItemCollection::EnumConstructor => Err(Error {
                message: format!("Item is not a type: {}", type_name),
                loc: loc.clone(),
            }),
        }
    }

    fn get_const<'a>(&'a self, ctx: &'a ExprContext, const_name: &str) -> Option<&'a ConstDef> {
        if let Some(item) = self.modules[ctx.module_index].get_item(const_name) {
            let ModuleItemCollection::Const = item.collection else {
                return None;
            };

            let const_def = &self.const_defs[item.collection_index];
            return Some(const_def);
        }

        if let Some(macro_arg) = ctx.get_macro_arg(const_name) {
            return Some(macro_arg);
        }

        None
    }

    fn codegen_default_value(
        &self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        value_type: &Type,
    ) {
        match value_type {
            Type::Never | Type::Void => {}
            Type::Bool
            | Type::U8
            | Type::I8
            | Type::U16
            | Type::I16
            | Type::U32
            | Type::I32
            | Type::Null
            | Type::Pointer { pointee: _ }
            | Type::SequencePointer { pointee: _ } => instrs.push(WasmInstr::I32Const { value: 0 }),
            Type::U64 | Type::I64 => instrs.push(WasmInstr::I64Const { value: 0 }),
            Type::F32 => instrs.push(WasmInstr::F32Const { value: 0.0 }),
            Type::F64 => instrs.push(WasmInstr::F64Const { value: 0.0 }),
            Type::StructInstance { struct_index } => {
                let struct_ref = &self.struct_defs[*struct_index];
                for field in &struct_ref.fields {
                    self.codegen_default_value(ctx, instrs, &field.field_type);
                }
            }
            Type::EnumInstance { enum_index } => {
                let enum_def = &self.enum_defs[*enum_index];

                self.codegen_default_value(ctx, instrs, &Type::U32);
                self.codegen_default_value(ctx, instrs, &enum_def.variant_type);
            }
            Type::Result(result) => {
                self.codegen_default_value(ctx, instrs, &result.ok);
                self.codegen_default_value(ctx, instrs, &result.err);
            }
        }
    }

    fn codegen_int_const(&self, instrs: &mut Vec<WasmInstr>, value: i32, tag: Option<&str>) {
        match tag.as_deref() {
            Some("u32") | Some("i32") | None => instrs.push(WasmInstr::I32Const { value }),
            Some("u64") | Some("i64") => instrs.push(WasmInstr::I64Const {
                value: value as i64,
            }),
            _ => unreachable!(),
        }
    }

    fn is_types_compatible(&self, slots: &Vec<Type>, values: &Vec<Type>) -> bool {
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

    fn is_type_compatible(&self, slot: &Type, value: &Type) -> bool {
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
        }

        if *value == Type::Never {
            return true;
        }

        slot == value
    }

    fn lower_type(&self, lo_type: &Type, wasm_types: &mut Vec<WasmType>) {
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
                let struct_def = &self.struct_defs[*struct_index];

                for field in &struct_def.fields {
                    self.lower_type(&field.field_type, wasm_types);
                }
            }
            Type::EnumInstance { enum_index } => {
                let enum_def = &self.enum_defs[*enum_index];

                self.lower_type(&Type::U32, wasm_types);
                self.lower_type(&enum_def.variant_type, wasm_types);
            }
            Type::Result(result) => {
                self.lower_type(&result.ok, wasm_types);
                self.lower_type(&result.err, wasm_types);
            }
        }
    }

    fn count_wasm_type_components(&self, lo_type: &Type) -> u32 {
        let layout = &mut TypeLayout::new();
        self.get_type_layout(lo_type, layout);
        layout.primities_count
    }

    fn get_type_layout(&self, lo_type: &Type, layout: &mut TypeLayout) {
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
                let struct_def = &self.struct_defs[*struct_index];

                // append each field's layout to total struct layout
                for field in &struct_def.fields {
                    self.get_type_layout(&field.field_type, layout);
                }

                layout.alignment = u32::max(layout.alignment, 1);
                layout.byte_size = align(layout.byte_size, layout.alignment);
            }
            Type::EnumInstance { enum_index } => {
                let enum_def = &self.enum_defs[*enum_index];

                self.get_type_layout(&Type::U32, layout);
                self.get_type_layout(&enum_def.variant_type, layout);

                // TODO!!!: figure out the alignment and byte_size
            }
            Type::Result(result) => {
                self.get_type_layout(&result.ok, layout);
                self.get_type_layout(&result.err, layout);

                // TODO!!!: figure out the alignment and byte_size
            }
        }
    }

    fn get_cast_instr(&self, casted_from: &Type, casted_to: &Type) -> Option<WasmInstr> {
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

    fn codegen_binary_op(
        &self,
        instrs: &mut Vec<WasmInstr>,
        op_tag: &InfixOpTag,
        operand_type: &Type,
        op_loc: &Loc,
    ) -> Result<(), Error> {
        let kind = self.get_binary_op_kind(op_tag, operand_type, op_loc)?;
        instrs.push(WasmInstr::BinaryOp { kind });
        Ok(())
    }

    fn get_binary_op_kind(
        &self,
        op_tag: &InfixOpTag,
        operand_type: &Type,
        op_loc: &Loc,
    ) -> Result<WasmBinaryOpKind, Error> {
        match op_tag {
            InfixOpTag::Equal => match operand_type {
                Type::Bool
                | Type::I8
                | Type::U8
                | Type::I16
                | Type::U16
                | Type::I32
                | Type::U32
                | Type::Pointer { pointee: _ }
                | Type::SequencePointer { pointee: _ } => return Ok(WasmBinaryOpKind::I32_EQ),
                Type::EnumInstance { enum_index }
                    if self.enum_defs[*enum_index].variant_type == Type::Void =>
                {
                    return Ok(WasmBinaryOpKind::I32_EQ);
                }
                Type::I64 | Type::U64 => return Ok(WasmBinaryOpKind::I64_EQ),
                Type::F32 => return Ok(WasmBinaryOpKind::F32_EQ),
                Type::F64 => return Ok(WasmBinaryOpKind::F64_EQ),
                _ => {}
            },
            InfixOpTag::NotEqual => match operand_type {
                Type::Bool
                | Type::I8
                | Type::U8
                | Type::I16
                | Type::U16
                | Type::I32
                | Type::U32
                | Type::Pointer { pointee: _ }
                | Type::SequencePointer { pointee: _ } => return Ok(WasmBinaryOpKind::I32_NE),
                Type::EnumInstance { enum_index }
                    if self.enum_defs[*enum_index].variant_type == Type::Void =>
                {
                    return Ok(WasmBinaryOpKind::I32_NE);
                }
                Type::I64 | Type::U64 => return Ok(WasmBinaryOpKind::I64_NE),
                Type::F32 => return Ok(WasmBinaryOpKind::F32_NE),
                Type::F64 => return Ok(WasmBinaryOpKind::F64_NE),
                _ => {}
            },
            InfixOpTag::Less => match operand_type {
                Type::I8 | Type::I16 | Type::I32 => return Ok(WasmBinaryOpKind::I32_LT_S),
                Type::Bool | Type::U8 | Type::U16 | Type::U32 => {
                    return Ok(WasmBinaryOpKind::I32_LT_U);
                }
                Type::I64 => return Ok(WasmBinaryOpKind::I64_LT_S),
                Type::U64 => return Ok(WasmBinaryOpKind::I64_LT_U),
                Type::F32 => return Ok(WasmBinaryOpKind::F32_LT),
                Type::F64 => return Ok(WasmBinaryOpKind::F64_LT),
                _ => {}
            },
            InfixOpTag::Greater => match operand_type {
                Type::I8 | Type::I16 | Type::I32 => return Ok(WasmBinaryOpKind::I32_GT_S),
                Type::Bool | Type::U8 | Type::U16 | Type::U32 => {
                    return Ok(WasmBinaryOpKind::I32_GT_U);
                }
                Type::I64 => return Ok(WasmBinaryOpKind::I64_GT_S),
                Type::U64 => return Ok(WasmBinaryOpKind::I64_GT_U),
                Type::F32 => return Ok(WasmBinaryOpKind::F32_GT),
                Type::F64 => return Ok(WasmBinaryOpKind::F64_GT),
                _ => {}
            },
            InfixOpTag::LessEqual => match operand_type {
                Type::I8 | Type::I16 | Type::I32 => return Ok(WasmBinaryOpKind::I32_LE_S),
                Type::Bool | Type::U8 | Type::U16 | Type::U32 => {
                    return Ok(WasmBinaryOpKind::I32_LE_U);
                }
                Type::I64 => return Ok(WasmBinaryOpKind::I64_LE_S),
                Type::U64 => return Ok(WasmBinaryOpKind::I64_LE_U),
                Type::F32 => return Ok(WasmBinaryOpKind::F32_LE),
                Type::F64 => return Ok(WasmBinaryOpKind::F64_LE),
                _ => {}
            },
            InfixOpTag::GreaterEqual => match operand_type {
                Type::I8 | Type::I16 | Type::I32 => return Ok(WasmBinaryOpKind::I32_GE_S),
                Type::Bool | Type::U8 | Type::U16 | Type::U32 => {
                    return Ok(WasmBinaryOpKind::I32_GE_U);
                }
                Type::I64 => return Ok(WasmBinaryOpKind::I64_GE_S),
                Type::U64 => return Ok(WasmBinaryOpKind::I64_GE_U),
                Type::F32 => return Ok(WasmBinaryOpKind::F32_GE),
                Type::F64 => return Ok(WasmBinaryOpKind::F64_GE),
                _ => {}
            },
            InfixOpTag::Add => match operand_type {
                Type::Bool
                | Type::I8
                | Type::U8
                | Type::I16
                | Type::U16
                | Type::I32
                | Type::U32 => return Ok(WasmBinaryOpKind::I32_ADD),
                Type::I64 | Type::U64 => return Ok(WasmBinaryOpKind::I64_ADD),
                Type::F32 => return Ok(WasmBinaryOpKind::F32_ADD),
                Type::F64 => return Ok(WasmBinaryOpKind::F64_ADD),
                _ => {}
            },
            InfixOpTag::Sub => match operand_type {
                Type::Bool
                | Type::I8
                | Type::U8
                | Type::I16
                | Type::U16
                | Type::I32
                | Type::U32 => return Ok(WasmBinaryOpKind::I32_SUB),
                Type::I64 | Type::U64 => return Ok(WasmBinaryOpKind::I64_SUB),
                Type::F32 => return Ok(WasmBinaryOpKind::F32_SUB),
                Type::F64 => return Ok(WasmBinaryOpKind::F64_SUB),
                _ => {}
            },
            InfixOpTag::Mul => match operand_type {
                Type::Bool
                | Type::I8
                | Type::U8
                | Type::I16
                | Type::U16
                | Type::I32
                | Type::U32 => return Ok(WasmBinaryOpKind::I32_MUL),
                Type::I64 | Type::U64 => return Ok(WasmBinaryOpKind::I64_MUL),
                Type::F32 => return Ok(WasmBinaryOpKind::F32_MUL),
                Type::F64 => return Ok(WasmBinaryOpKind::F64_MUL),
                _ => {}
            },
            InfixOpTag::Div => match operand_type {
                Type::I8 | Type::I16 | Type::I32 => return Ok(WasmBinaryOpKind::I32_DIV_S),
                Type::Bool | Type::U8 | Type::U16 | Type::U32 => {
                    return Ok(WasmBinaryOpKind::I32_DIV_U);
                }
                Type::I64 => return Ok(WasmBinaryOpKind::I64_DIV_S),
                Type::U64 => return Ok(WasmBinaryOpKind::I64_DIV_U),
                Type::F32 => return Ok(WasmBinaryOpKind::F32_DIV),
                Type::F64 => return Ok(WasmBinaryOpKind::F64_DIV),
                _ => {}
            },
            InfixOpTag::Mod => match operand_type {
                Type::I8 | Type::I16 | Type::I32 => return Ok(WasmBinaryOpKind::I32_REM_S),
                Type::Bool | Type::U8 | Type::U16 | Type::U32 => {
                    return Ok(WasmBinaryOpKind::I32_REM_U);
                }
                Type::I64 => return Ok(WasmBinaryOpKind::I64_REM_S),
                Type::U64 => return Ok(WasmBinaryOpKind::I64_REM_U),
                _ => {}
            },
            InfixOpTag::ShiftLeft => match operand_type {
                Type::I8 | Type::I16 | Type::I32 => return Ok(WasmBinaryOpKind::I32_SHL),
                Type::Bool | Type::U8 | Type::U16 | Type::U32 => {
                    return Ok(WasmBinaryOpKind::I32_SHL);
                }
                Type::I64 => return Ok(WasmBinaryOpKind::I64_SHL),
                Type::U64 => return Ok(WasmBinaryOpKind::I64_SHL),
                _ => {}
            },
            InfixOpTag::ShiftRight => match operand_type {
                Type::I8 | Type::I16 | Type::I32 => return Ok(WasmBinaryOpKind::I32_SHR_S),
                Type::Bool | Type::U8 | Type::U16 | Type::U32 => {
                    return Ok(WasmBinaryOpKind::I32_SHR_U);
                }
                Type::I64 => return Ok(WasmBinaryOpKind::I64_SHR_S),
                Type::U64 => return Ok(WasmBinaryOpKind::I64_SHR_U),
                _ => {}
            },
            InfixOpTag::And => match operand_type {
                Type::Bool
                | Type::I8
                | Type::U8
                | Type::I16
                | Type::U16
                | Type::I32
                | Type::U32 => return Ok(WasmBinaryOpKind::I32_AND),
                Type::I64 | Type::U64 => return Ok(WasmBinaryOpKind::I64_AND),
                _ => {}
            },
            InfixOpTag::Or => match operand_type {
                Type::Bool
                | Type::I8
                | Type::U8
                | Type::I16
                | Type::U16
                | Type::I32
                | Type::U32 => return Ok(WasmBinaryOpKind::I32_OR),
                Type::I64 | Type::U64 => return Ok(WasmBinaryOpKind::I64_OR),
                _ => {}
            },
            InfixOpTag::BitAnd => match operand_type {
                Type::Bool
                | Type::I8
                | Type::U8
                | Type::I16
                | Type::U16
                | Type::I32
                | Type::U32 => return Ok(WasmBinaryOpKind::I32_AND),
                Type::I64 | Type::U64 => return Ok(WasmBinaryOpKind::I64_AND),
                _ => {}
            },
            InfixOpTag::BitOr => match operand_type {
                Type::Bool
                | Type::I8
                | Type::U8
                | Type::I16
                | Type::U16
                | Type::I32
                | Type::U32 => return Ok(WasmBinaryOpKind::I32_OR),
                Type::I64 | Type::U64 => return Ok(WasmBinaryOpKind::I64_OR),
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
            | InfixOpTag::ErrorPropagation
            | InfixOpTag::ExprPipe => unreachable!(),
        }

        let module = self.get_module_by_file_index(op_loc.file_index).unwrap();
        return Err(Error {
            message: format!(
                "Operator `{}` is incompatible with operands of type {}",
                op_loc.read_span(module.parser.lexer.source),
                TypeFmt(self, operand_type)
            ),
            loc: op_loc.clone(),
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
            | InfixOpTag::ErrorPropagation
            | InfixOpTag::ExprPipe => None,
        }
    }

    fn get_fn_name_from_method(&self, receiver_type: &Type, method_name: &str) -> String {
        let receiver_type = TypeFmt(self, receiver_type.deref_rec());
        format!("{receiver_type}::{method_name}")
    }

    fn get_module_by_file_index(&self, file_index: usize) -> Option<&Module> {
        for module in &self.modules {
            if module.parser.lexer.file_index == file_index {
                return Some(module);
            }
        }

        None
    }

    fn report_error(&self, err: &Error) {
        *self.error_count.borrow_mut() += 1;

        if self.in_inspection_mode {
            let source_index = err.loc.file_index;
            let source_range = RangeFmt(&err.loc);
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

    fn report_warning(&self, err: &Error) {
        *self.warning_count.borrow_mut() += 1;

        if self.in_inspection_mode {
            let source_index = err.loc.file_index;
            let source_range = RangeFmt(&err.loc);
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
        let source_range = RangeFmt(&inspect_info.loc);
        let message = json_str_escape(&inspect_info.message);

        if let Some(linked_loc) = &inspect_info.linked_loc {
            if linked_loc.file_index != 0 {
                let target_index = linked_loc.file_index;
                let target_range = RangeFmt(&linked_loc);
                stdout_writeln(format!(
                    "{{ \"type\": \"info\", \
                        \"link\": \"{target_index}/{target_range}\", \
                        \"hover\": \"{message}\", \
                        \"loc\": \"{source_index}/{source_range}\" }},",
                ));
                return;
            }
        };

        stdout_writeln(format!(
            "{{ \"type\": \"info\", \
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

fn align(value: u32, alignment: u32) -> u32 {
    return (value + alignment - 1) / alignment * alignment;
}
