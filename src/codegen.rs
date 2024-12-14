use crate::{ast::*, core::*, lexer::*, lo_todo, parser_v2::*, wasm::*};
use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec::Vec,
};

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
    Pointer {
        pointee: Box<LoType>,
    },
    SequencePointer {
        pointee: Box<LoType>,
    },
    StructInstance {
        struct_name: String,
    },
    Result {
        ok_type: Box<LoType>,
        err_type: Box<LoType>,
    },
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
            LoType::Result { ok_type, err_type } => write!(f, "Result<{ok_type}, {err_type}>"),
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

struct LoFnInfo {
    fn_name: String,
    fn_type: LoFnType,
    fn_source: LoFnSource,
    definition_loc: LoLocation,
}

enum LoFnSource {
    Guest {
        exported_as: Option<String>,
        ctx: LoExprContext,
        body: CodeBlockExpr,
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

#[derive(Default)]
struct LoExprContext {
    locals: Vec<LoLocal>,
    last_local_index: u32,
    scopes: Vec<LoScope>,
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
struct LoScope {
    scope_type: LoScopeType,
    locals: Vec<LoScopedLocal>,
    macro_args: Vec<(String, &'static CodeExpr)>,
    macro_types: Vec<(String, LoType)>,
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
            macro_args: Vec::new(),
            macro_types: Vec::new(),
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

    fn get_macro_type(&self, type_name: &str) -> Option<&LoType> {
        for scope in self.scopes.iter().rev() {
            for macro_type in &scope.macro_types {
                if macro_type.0 == type_name {
                    return Some(&macro_type.1);
                }
            }
        }

        None
    }

    fn get_macro_arg(&self, arg_name: &str) -> Option<&'static CodeExpr> {
        for scope in self.scopes.iter().rev() {
            for macro_arg in &scope.macro_args {
                if macro_arg.0 == arg_name {
                    let value = macro_arg.1;
                    /*
                    Handle the case where macro arg is an identifier with the same name as macro parameter.

                    This happens when macro argument points to an argument from parent macro.

                    Example:
                    ```lo
                    macro i32::mul_by_2!(self): i32 {
                      i32::mul(self, self);
                    };
                    macro i32::mul(self, other: i32): i32 {
                      // here self points to self from `i32::mul_by_2`, not from `i32::mul``
                      self * other;
                    };
                    ```
                    */
                    if let CodeExpr::Ident(ident) = value {
                        if ident.repr == arg_name {
                            continue;
                        }
                    }
                    return Some(value);
                }
            }
        }

        None
    }
}

struct WasmFnInfo {
    fn_name: String,
    lo_fn_index: usize,
    wasm_fn_index: u32,
}

#[derive(Clone)]
struct LoTypeDef {
    name: String,
    value: LoType,
    loc: LoLocation,
}

struct LoStructDef {
    struct_name: String,
    fields: Vec<LoStructField>,
    fully_defined: bool, // used for self-reference checks
}

#[derive(Clone)]
pub struct LoStructField {
    field_name: String,
    field_type: LoType,
    field_index: u32,
    byte_offset: u32,
    loc: LoLocation,
}

struct LoGlobalDef {
    def_expr: GlobalDefExpr,
    global_type: LoType,
    global_index: u32,
}

#[derive(Default)]
struct LoTypeLayout {
    primities_count: u32,
    byte_length: u32,
}

enum VariableInfo {
    Local {
        local_index: u32,
        local_type: LoType,
    },
    Global {
        global_index: u32,
        global_type: LoType,
    },
    Stored {
        address: &'static CodeExpr,
        offset: u32,
        value_type: LoType,
    },
}

impl VariableInfo {
    fn get_type(self) -> LoType {
        match self {
            VariableInfo::Local {
                local_index: _,
                local_type,
            } => local_type,
            VariableInfo::Global {
                global_index: _,
                global_type,
            } => global_type,
            VariableInfo::Stored {
                address: _,
                offset: _,
                value_type,
            } => value_type,
        }
    }
}

#[derive(Default)]
pub struct CodeGen {
    pub errors: LoErrorManager,
    lo_functions: Vec<LoFnInfo>,
    wasm_functions: Vec<WasmFnInfo>,
    type_defs: Vec<LoTypeDef>,
    struct_defs: Vec<LoStructDef>,
    memory: Option<MemoryDefExpr>,
    memory_imported_from: Option<String>,
    static_data_stores: Vec<StaticDataStoreExpr>,
    globals: Vec<LoGlobalDef>,
    const_defs: Vec<ConstDefExpr>,
    macro_defs: Vec<MacroDefExpr>,
}

impl CodeGen {
    pub fn with_default_types() -> Self {
        let mut codegen = Self::default();
        codegen.type_defs.push(LoTypeDef {
            name: String::from("never"),
            value: LoType::Never,
            loc: LoLocation::internal(),
        });
        codegen.type_defs.push(LoTypeDef {
            name: String::from("void"),
            value: LoType::Void,
            loc: LoLocation::internal(),
        });
        codegen.type_defs.push(LoTypeDef {
            name: String::from("bool"),
            value: LoType::Bool,
            loc: LoLocation::internal(),
        });
        codegen.type_defs.push(LoTypeDef {
            name: String::from("u8"),
            value: LoType::U8,
            loc: LoLocation::internal(),
        });
        codegen.type_defs.push(LoTypeDef {
            name: String::from("i8"),
            value: LoType::I8,
            loc: LoLocation::internal(),
        });
        codegen.type_defs.push(LoTypeDef {
            name: String::from("u16"),
            value: LoType::U16,
            loc: LoLocation::internal(),
        });
        codegen.type_defs.push(LoTypeDef {
            name: String::from("i16"),
            value: LoType::I16,
            loc: LoLocation::internal(),
        });
        codegen.type_defs.push(LoTypeDef {
            name: String::from("u32"),
            value: LoType::U32,
            loc: LoLocation::internal(),
        });
        codegen.type_defs.push(LoTypeDef {
            name: String::from("i32"),
            value: LoType::I32,
            loc: LoLocation::internal(),
        });
        codegen.type_defs.push(LoTypeDef {
            name: String::from("f32"),
            value: LoType::F32,
            loc: LoLocation::internal(),
        });
        codegen.type_defs.push(LoTypeDef {
            name: String::from("u64"),
            value: LoType::U64,
            loc: LoLocation::internal(),
        });
        codegen.type_defs.push(LoTypeDef {
            name: String::from("i64"),
            value: LoType::I64,
            loc: LoLocation::internal(),
        });
        codegen.type_defs.push(LoTypeDef {
            name: String::from("f64"),
            value: LoType::F64,
            loc: LoLocation::internal(),
        });
        return codegen;
    }

    pub fn add_file(&mut self, file: FileInfo) -> Result<(), LoError> {
        for expr in file.ast.exprs {
            match expr {
                TopLevelExpr::Include(_) => {} // skip, processed earlier
                TopLevelExpr::FnDef(fn_def) => {
                    let output = match &fn_def.decl.return_type {
                        Some(return_type) => self.build_type(None, return_type)?,
                        _ => LoType::Void,
                    };

                    let mut ctx = LoExprContext::default();
                    ctx.enter_scope(LoScopeType::Function);

                    let mut inputs = Vec::new();
                    'param_loop: for fn_param in &fn_def.decl.fn_params {
                        for var in &ctx.current_scope().locals {
                            if var.local_name == fn_param.param_name {
                                self.errors.report(LoError {
                                    message: format!(
                                        "Duplicate function parameter name: {}",
                                        fn_param.param_name
                                    ),
                                    loc: fn_param.loc.clone(),
                                });
                                continue 'param_loop;
                            }
                        }

                        let param_type = self.get_fn_param_type(&fn_def.decl, fn_param)?;
                        inputs.push(param_type.clone());

                        self.define_local(
                            &mut ctx,
                            fn_param.loc.clone(),
                            fn_param.param_name.clone(),
                            &param_type,
                            true,
                        )?;
                    }

                    let mut exported_as = None;
                    if fn_def.exported {
                        exported_as = Some(fn_def.decl.fn_name.repr.clone())
                    }

                    // TODO: make sure function name does not collide with intrinsics
                    for fn_info in &self.lo_functions {
                        if fn_info.fn_name == fn_def.decl.fn_name.repr {
                            self.errors.report(LoError {
                                message: format!(
                                    "Duplicate function definition: {}, previously defined at {}",
                                    fn_def.decl.fn_name.repr, fn_info.definition_loc
                                ),
                                loc: fn_def.decl.loc.clone(),
                            });
                            break;
                        }
                    }

                    self.lo_functions.push(LoFnInfo {
                        fn_name: fn_def.decl.fn_name.repr,
                        fn_type: LoFnType { inputs, output },
                        fn_source: LoFnSource::Guest {
                            exported_as,
                            ctx,
                            body: fn_def.body,
                        },
                        definition_loc: fn_def.loc.clone(),
                    });
                }
                TopLevelExpr::Import(ImportExpr {
                    module_name,
                    items,
                    loc,
                }) => {
                    let module_name = Lexer::unescape_string(&module_name);

                    for item in items {
                        let fn_decl = match item {
                            ImportItem::FnDecl(fn_decl) => fn_decl,
                            ImportItem::Memory(memory) => {
                                if let Some(existing_memory) = &self.memory {
                                    return Err(LoError {
                                        message: format!(
                                            "Cannot redefine memory, first defined at {}",
                                            existing_memory.loc
                                        ),
                                        loc: memory.loc,
                                    });
                                }

                                self.memory = Some(memory);
                                self.memory_imported_from = Some(module_name.clone());
                                continue;
                            }
                        };

                        let mut fn_type = LoFnType {
                            inputs: Vec::new(),
                            output: LoType::Void,
                        };
                        for fn_param in &fn_decl.fn_params {
                            let param_type = self.get_fn_param_type(&fn_decl, fn_param)?;
                            fn_type.inputs.push(param_type.clone());
                        }
                        if let Some(return_type) = fn_decl.return_type {
                            fn_type.output = self.build_type(None, &return_type)?;
                        }

                        // TODO: make sure function name does not collide with intrinsics
                        for fn_info in &self.lo_functions {
                            if fn_info.fn_name == fn_decl.fn_name.repr {
                                self.errors.report(LoError {
                                    message: format!(
                                        "Duplicate function definition: {}, previously defined at {}",
                                        fn_decl.fn_name.repr, fn_info.definition_loc
                                    ),
                                    loc: fn_decl.loc.clone(),
                                });
                                break;
                            }
                        }

                        self.lo_functions.push(LoFnInfo {
                            fn_name: fn_decl.fn_name.repr.clone(),
                            fn_type,
                            fn_source: LoFnSource::Host {
                                module_name: module_name.clone(),
                                external_fn_name: fn_decl.fn_name.parts.last().unwrap().clone(),
                            },
                            definition_loc: loc.clone(),
                        });
                    }
                }
                TopLevelExpr::GlobalDef(global) => {
                    let existing_global = self.get_global(&global.global_name.repr);
                    if let Some(existing_global) = existing_global {
                        return Err(LoError {
                            message: format!(
                                "Cannot redefine global {}, previously defined at {}",
                                global.global_name.repr, existing_global.def_expr.global_name.loc,
                            ),
                            loc: global.loc,
                        });
                    }

                    self.ensure_const_expr(&global.expr)?;
                    let const_expr_ctx = &mut LoExprContext::default();
                    let value_type = self.get_expr_type(const_expr_ctx, &global.expr)?;
                    let value_comp_count = self.count_wasm_type_components(&value_type);
                    if value_comp_count != 1 {
                        return Err(LoError {
                            message: format!(
                                "Cannot define global with non-primitive type {value_type}",
                            ),
                            loc: global.loc,
                        });
                    }

                    self.globals.push(LoGlobalDef {
                        def_expr: global,
                        global_type: value_type,
                        global_index: self.globals.len() as u32,
                    });
                }
                TopLevelExpr::StructDef(StructDefExpr {
                    struct_name,
                    fields,
                    loc,
                }) => {
                    if let Some(existing_typedef) = self.get_typedef(&struct_name.repr) {
                        return Err(LoError {
                            message: format!(
                                "Cannot redefine type {}, already defined at {}",
                                struct_name.repr, existing_typedef.loc
                            ),
                            loc: struct_name.loc,
                        });
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
                        loc,
                    });

                    let mut struct_layout = LoTypeLayout::default();
                    let mut struct_fields = Vec::<LoStructField>::new();

                    for field in fields {
                        for existing_field in &struct_fields {
                            if existing_field.field_name == field.field_name {
                                return Err(LoError {
                                    message: format!(
                                        "Cannot redefine struct field '{}', already defined at {}",
                                        field.field_name, existing_field.loc,
                                    ),
                                    loc: field.loc,
                                });
                            }
                        }

                        // TODO: add self reference check
                        let field_type = self.build_type(None, &field.field_type)?;

                        struct_fields.push(LoStructField {
                            field_name: field.field_name,
                            field_type: field_type.clone(),
                            field_index: struct_layout.primities_count,
                            byte_offset: struct_layout.byte_length,
                            loc: field.loc,
                        });

                        // append field's layout to total struct layout
                        self.get_type_layout(&field_type, &mut struct_layout);
                    }

                    let struct_def = self.get_struct_def_mut(&struct_name.repr).unwrap();
                    struct_def.fields.append(&mut struct_fields);
                    struct_def.fully_defined = true;
                }
                TopLevelExpr::TypeDef(typedef) => {
                    if let Some(existing_typedef) = self.get_typedef(&typedef.type_name.repr) {
                        return Err(LoError {
                            message: format!(
                                "Cannot redefine type {}, already defined at {}",
                                typedef.type_name.repr, existing_typedef.loc
                            ),
                            loc: typedef.loc,
                        });
                    }

                    let type_value = self.build_type(None, &typedef.type_value)?;

                    self.type_defs.push(LoTypeDef {
                        name: typedef.type_name.repr,
                        value: type_value,
                        loc: typedef.loc,
                    });
                }
                TopLevelExpr::ConstDef(const_def) => {
                    if let Some(existing_const) = self.get_const_def(&const_def.const_name.repr) {
                        return Err(LoError {
                            message: format!(
                                "Cannot redefine constant {}, already defined at {}",
                                const_def.const_name.repr, existing_const.loc
                            ),
                            loc: const_def.loc,
                        });
                    }

                    self.const_defs.push(const_def);
                }
                TopLevelExpr::MemoryDef(memory) => {
                    if let Some(existing_memory) = &self.memory {
                        return Err(LoError {
                            message: format!(
                                "Cannot redefine memory, first defined at {}",
                                existing_memory.loc
                            ),
                            loc: memory.loc,
                        });
                    }

                    self.memory = Some(memory);
                }
                TopLevelExpr::StaticDataStore(static_data_store) => {
                    self.static_data_stores.push(static_data_store);
                }
                TopLevelExpr::ExportExistingFn(existing_fn) => {
                    return Err(crate::lo_todo!(existing_fn.loc.clone()))
                }
                TopLevelExpr::MacroDef(macro_def) => {
                    if let Some(existing_macro) = self.get_macro_def(&macro_def.macro_name.repr) {
                        return Err(LoError {
                            message: format!(
                                "Cannot redefine macro {}, already defined at {}",
                                macro_def.macro_name.repr, existing_macro.loc
                            ),
                            loc: macro_def.loc,
                        });
                    }

                    self.macro_defs.push(macro_def);
                }
            }
        }

        Ok(())
    }

    pub fn generate(&mut self) -> Result<WasmModule, LoError> {
        let mut wasm_module = WasmModule::default();

        let mut fn_imports_count = 0;
        for fn_info in &self.lo_functions {
            if let LoFnSource::Host { .. } = fn_info.fn_source {
                fn_imports_count += 1;
            }
        }

        // resolve wasm fn indicies and populate type, import and export sections
        let mut wasm_import_fn_index = 0;
        let mut wasm_fn_index = fn_imports_count;
        for lo_fn_index in 0..self.lo_functions.len() {
            let fn_info = &self.lo_functions[lo_fn_index];

            let mut wasm_fn_type = WasmFnType {
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            for lo_input_type in &fn_info.fn_type.inputs {
                self.lower_type(lo_input_type, &mut wasm_fn_type.inputs);
            }
            self.lower_type(&fn_info.fn_type.output, &mut wasm_fn_type.outputs);

            let mut fn_type_index = wasm_module.types.len() as u32;
            for (existing_fn_type, existing_type_index) in wasm_module.types.iter().zip(0..) {
                if wasm_fn_type == *existing_fn_type {
                    fn_type_index = existing_type_index;
                }
            }
            if fn_type_index == wasm_module.types.len() as u32 {
                wasm_module.types.push(wasm_fn_type);
            }

            match &fn_info.fn_source {
                LoFnSource::Guest {
                    exported_as,
                    ctx: _,
                    body: _,
                } => {
                    wasm_module.functions.push(fn_type_index);
                    self.wasm_functions.push(WasmFnInfo {
                        fn_name: fn_info.fn_name.clone(),
                        lo_fn_index,
                        wasm_fn_index,
                    });
                    if let Some(export_name) = &exported_as {
                        wasm_module.exports.push(WasmExport {
                            export_type: WasmExportType::Func,
                            export_name: export_name.clone(),
                            exported_item_index: wasm_fn_index,
                        });
                    }

                    wasm_fn_index += 1;
                }
                LoFnSource::Host {
                    module_name,
                    external_fn_name,
                } => {
                    self.wasm_functions.push(WasmFnInfo {
                        fn_name: fn_info.fn_name.clone(),
                        lo_fn_index,
                        wasm_fn_index: wasm_import_fn_index,
                    });
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
        }

        // build function codes
        for i in 0..self.wasm_functions.len() {
            let wasm_fn_info = &self.wasm_functions[i];
            let lo_fn_info = &self.lo_functions[wasm_fn_info.lo_fn_index];

            let LoFnSource::Guest {
                exported_as: _,
                ctx,
                body,
            } = &lo_fn_info.fn_source
            else {
                continue;
            };

            let mut ctx = LoExprContext {
                last_local_index: ctx.last_local_index,
                locals: ctx.locals.clone(),
                scopes: ctx.scopes.clone(),
                ..Default::default()
            };
            let mut wasm_expr = WasmExpr { instrs: Vec::new() };
            for expr in &body.exprs {
                self.codegen(&mut ctx, &mut wasm_expr.instrs, expr)?;
            }

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

        let const_expr_ctx = &mut LoExprContext::default();

        for static_data_store in &self.static_data_stores {
            let mut offset_expr = WasmExpr { instrs: Vec::new() };
            self.ensure_const_expr(&static_data_store.addr)?;
            self.codegen(
                const_expr_ctx,
                &mut offset_expr.instrs,
                &static_data_store.addr,
            )?;
            let bytes = match &static_data_store.data {
                StaticDataStorePayload::String { value } => {
                    Lexer::unescape_string(value).as_bytes().to_vec()
                }
            };

            wasm_module.datas.push(WasmData::Active {
                offset: offset_expr,
                bytes,
            });
        }

        let mut wasm_types_buf = Vec::with_capacity(1);
        for global in &self.globals {
            self.lower_type(&global.global_type, &mut wasm_types_buf);
            let wasm_value_type = wasm_types_buf.pop().unwrap();

            let mut initial_value = WasmExpr { instrs: Vec::new() };
            self.codegen(
                const_expr_ctx,
                &mut initial_value.instrs,
                &global.def_expr.expr,
            )?;

            wasm_module.globals.push(WasmGlobal {
                mutable: true,
                value_type: wasm_value_type,
                initial_value,
            });
        }

        Ok(wasm_module)
    }

    fn get_fn_param_type(
        &mut self,
        fn_decl: &FnDeclExpr,
        fn_param: &FnParam,
    ) -> Result<LoType, LoError> {
        match &fn_param.param_type {
            FnParamType::Self_ | FnParamType::SelfRef => {
                if fn_decl.fn_name.parts.len() == 1 {
                    return Err(LoError {
                        message: format!("Cannot use self param in non-method function"),
                        loc: fn_param.loc.clone(),
                    });
                }

                let self_type_name = &fn_decl.fn_name.parts[0];
                let self_type = self.get_type_or_err(self_type_name, &fn_decl.loc)?;

                if let FnParamType::Self_ = fn_param.param_type {
                    return Ok(self_type);
                }

                return Ok(LoType::Pointer {
                    pointee: Box::new(self_type),
                });
            }
            FnParamType::Type { expr } => self.build_type(None, &expr),
        }
    }

    fn build_type(
        &self,
        ctx: Option<&LoExprContext>,
        type_expr: &TypeExpr,
    ) -> Result<LoType, LoError> {
        match type_expr {
            TypeExpr::Named { name } => {
                if let Some(ctx) = ctx {
                    if let Some(macro_type) = ctx.get_macro_type(&name.repr) {
                        return Ok(macro_type.clone());
                    }
                }

                self.get_type_or_err(&name.repr, &name.loc)
            }
            TypeExpr::Pointer { pointee, loc: _ } => {
                let pointee = Box::new(self.build_type(ctx, &pointee)?);

                Ok(LoType::Pointer { pointee })
            }
            TypeExpr::SequencePointer { pointee, loc: _ } => {
                let pointee = Box::new(self.build_type(ctx, &pointee)?);

                Ok(LoType::SequencePointer { pointee })
            }
            TypeExpr::Result {
                ok_type,
                err_type,
                loc: _,
            } => {
                let ok_type = Box::new(self.build_type(ctx, &ok_type)?);
                let err_type = Box::new(self.build_type(ctx, &err_type)?);

                Ok(LoType::Result { ok_type, err_type })
            }
            TypeExpr::Of {
                container_type,
                item_type: _,
                loc: _,
            } => {
                let actual_type = self.build_type(ctx, container_type)?;

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
            CodeExpr::CharLiteral(_) => todo!(),
            CodeExpr::IntLiteral(IntLiteralExpr {
                repr: _,
                value,
                tag,
                loc: _,
            }) => match tag.as_deref() {
                Some("u32") | Some("i32") | None => instrs.push(WasmInstr::I32Const {
                    value: *value as i32,
                }),
                Some("u64") | Some("i64") => instrs.push(WasmInstr::I64Const {
                    value: *value as i64,
                }),
                _ => todo!(),
            },
            CodeExpr::StringLiteral(_) => todo!(),
            CodeExpr::StructLiteral(StructLiteralExpr {
                struct_name,
                fields,
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
                    for i in (fields.len() - 1)..struct_def.fields.len() {
                        missing_fields.push(&struct_def.fields[i].field_name)
                    }

                    return Err(LoError {
                        message: format!("Missing struct fields: {}", ListDisplay(&missing_fields)),
                        loc: loc.clone(),
                    });
                }
            }
            CodeExpr::ArrayLiteral(_) => todo!(),

            CodeExpr::Ident(IdentExpr {
                repr,
                parts: _,
                loc,
            }) => {
                if let Some(const_expr) = self.get_const(ctx, repr) {
                    return self.codegen(ctx, instrs, &const_expr);
                }

                let var = self.var_from_ident(ctx, repr, loc)?;
                self.codegen_var_get(ctx, instrs, &var)?;
            }
            CodeExpr::Let(LetExpr {
                local_name,
                value,
                loc,
            }) => {
                if local_name == "_" {
                    self.codegen(ctx, instrs, value)?;

                    let local_type = self.get_expr_type(ctx, &value)?;
                    for _ in 0..self.count_wasm_type_components(&local_type) {
                        instrs.push(WasmInstr::Drop);
                    }
                    return Ok(());
                }

                let local_type = self.get_expr_type(ctx, &value)?;
                self.define_local(ctx, loc.clone(), local_name.clone(), &local_type, false)?;

                let var = self.var_from_ident(ctx, local_name, loc)?;
                self.codegen_var_set(ctx, instrs, &var, value)?;
            }
            CodeExpr::Cast(CastExpr {
                expr,
                casted_to,
                loc,
            }) => {
                let castee_type = self.get_expr_type(ctx, expr)?;
                let mut castee_type_components = Vec::new();
                self.lower_type(&castee_type, &mut castee_type_components);

                let casted_to_type = self.build_type(Some(ctx), casted_to)?;
                let mut casted_to_type_components = Vec::new();
                self.lower_type(&casted_to_type, &mut casted_to_type_components);

                if castee_type_components != casted_to_type_components {
                    return Err(LoError {
                        message: format!("Cannot cast from {castee_type} to {casted_to_type}"),
                        loc: loc.clone(),
                    });
                }

                self.codegen(ctx, instrs, expr)?;
            }
            CodeExpr::PrefixOp(PrefixOpExpr { op_tag, expr, loc }) => match op_tag {
                PrefixOpTag::Dereference => {
                    let var = self.var_from_deref(ctx, expr)?;
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
                if let Some(base_op) = self.get_compound_assignment_base_op(op_tag) {
                    return self.codegen_compound_assignment(
                        ctx,
                        instrs,
                        Some(base_op),
                        op_loc,
                        lhs,
                        rhs,
                    );
                }

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
                return self.codegen_compound_assignment(ctx, instrs, None, op_loc, lhs, rhs);
            }
            CodeExpr::FieldAccess(FieldAccessExpr {
                lhs,
                field_name,
                loc: _,
            }) => {
                let var = self.var_from_field_access(ctx, lhs, &field_name)?;
                self.codegen_var_get(ctx, instrs, &var)?;
            }
            CodeExpr::PropagateError(_) => todo!(),

            CodeExpr::FnCall(FnCallExpr { fn_name, args, loc }) => {
                self.codegen_fn_call(ctx, instrs, &fn_name.repr, None, args, loc)?;
            }
            CodeExpr::MethodCall(MethodCallExpr {
                lhs,
                field_name,
                args,
                loc,
            }) => {
                let lhs_type = self.get_expr_type(ctx, lhs)?;
                let fn_name = get_fn_name_from_method(&lhs_type, &field_name.repr);
                self.codegen_fn_call(ctx, instrs, &fn_name, Some(lhs), args, loc)?;
            }
            CodeExpr::MacroFnCall(MacroFnCallExpr {
                fn_name,
                type_args,
                args,
                loc,
            }) => {
                self.codegen_macro_call(ctx, instrs, &fn_name.repr, type_args, None, args, loc)?;
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
                self.codegen_macro_call(ctx, instrs, &macro_name, type_args, Some(lhs), args, loc)?;
            }

            CodeExpr::Dbg(_) => todo!(),
            CodeExpr::Sizeof(SizeofExpr { type_expr, loc: _ }) => {
                let lo_type = self.build_type(Some(ctx), type_expr)?;
                let mut type_layout = LoTypeLayout::default();
                self.get_type_layout(&lo_type, &mut type_layout);

                instrs.push(WasmInstr::I32Const {
                    value: type_layout.byte_length as i32,
                });
            }
            CodeExpr::GetDataSize(GetDataSizeExpr { loc: _ }) => {
                // TODO: implement
                instrs.push(WasmInstr::I32Const { value: 0 });
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

            CodeExpr::Return(ReturnExpr { expr, loc: _ }) => {
                if let Some(return_expr) = expr {
                    self.codegen(ctx, instrs, return_expr)?;
                }

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
                for expr in &then_block.exprs {
                    self.codegen(ctx, instrs, &expr)?;
                }
                ctx.exit_scope();

                match else_block {
                    ElseBlock::None => {}
                    ElseBlock::Else(code_block_expr) => {
                        instrs.push(WasmInstr::Else);
                        ctx.enter_scope(LoScopeType::Block);
                        for expr in &code_block_expr.exprs {
                            self.codegen(ctx, instrs, &expr)?;
                        }
                        ctx.exit_scope();
                    }
                    ElseBlock::ElseIf(code_expr) => {
                        instrs.push(WasmInstr::Else);
                        self.codegen(ctx, instrs, &code_expr)?;
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
                for expr in &body.exprs {
                    self.codegen(ctx, instrs, expr)?;
                }
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
                self.define_local(ctx, loc.clone(), counter.clone(), &counter_type, false)?;
                let counter_var = self.var_from_ident(ctx, counter, loc)?;
                self.codegen_var_set(ctx, instrs, &counter_var, start)?;

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

                            for expr in &body.exprs {
                                self.codegen(ctx, instrs, &expr)?;
                            }

                            instrs.push(WasmInstr::BlockEnd);
                        }

                        // increment counter
                        self.codegen_compound_assignment(
                            ctx,
                            instrs,
                            Some(InfixOpTag::Add),
                            loc,
                            &CodeExpr::Ident(IdentExpr {
                                repr: counter.clone(),
                                parts: Vec::new(),
                                loc: loc.clone(),
                            }),
                            &CodeExpr::IntLiteral(IntLiteralExpr {
                                repr: String::from(""),
                                value: 1,
                                tag: Some(counter_type.to_string()),
                                loc: loc.clone(),
                            }),
                        )?;

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
            CodeExpr::Defer(defer) => return Err(lo_todo!(defer.loc.clone())),
            CodeExpr::Catch(catch) => return Err(lo_todo!(catch.loc.clone())),
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
        let Some((lo_fn_info, wasm_fn_info)) = self.get_fn_info(fn_name) else {
            return Err(LoError {
                message: format!("Unknown function: {}", fn_name),
                loc: loc.clone(),
            });
        };

        let mut arg_types = Vec::new();
        if let Some(receiver_arg) = receiver_arg {
            arg_types.push(self.get_expr_type(ctx, receiver_arg)?);
            self.codegen(ctx, instrs, receiver_arg)?;
        }
        for arg in args {
            arg_types.push(self.get_expr_type(ctx, arg)?);
            self.codegen(ctx, instrs, arg)?;
        }

        if arg_types != lo_fn_info.fn_type.inputs {
            return Err(LoError {
                message: format!(
                    "Invalid function arguments for function {}: [{}], expected [{}]",
                    lo_fn_info.fn_name,
                    ListDisplay(&arg_types),
                    ListDisplay(&lo_fn_info.fn_type.inputs),
                ),
                loc: loc.clone(),
            });
        }

        instrs.push(WasmInstr::Call {
            fn_index: wasm_fn_info.wasm_fn_index,
        });

        Ok(())
    }

    fn get_macro_return_type(
        &self,
        macro_name: &str,
        type_args: &Vec<TypeExpr>,
        loc: &LoLocation,
    ) -> Result<LoType, LoError> {
        let Some(macro_def) = self.get_macro_def(&macro_name) else {
            return Err(LoError {
                message: format!("Unknown macro: {}", macro_name),
                loc: loc.clone(),
            });
        };

        if type_args.len() != macro_def.macro_type_params.len() {
            return Err(LoError {
                message: format!(
                    "Invalid number of type args, expected {}, got {}",
                    macro_def.macro_type_params.len(),
                    type_args.len()
                ),
                loc: macro_def.loc.clone(),
            });
        }

        let mut return_type = LoType::Void;
        if let Some(macro_return_type) = &macro_def.return_type {
            let mut ctx = LoExprContext::default();
            ctx.enter_scope(LoScopeType::Macro);

            for i in 0..macro_def.macro_type_params.len() {
                let type_param = &macro_def.macro_type_params[i];
                let type_arg = &type_args[i];

                let lo_type = self.build_type(Some(&ctx), &type_arg)?;
                ctx.current_scope_mut()
                    .macro_types
                    .push((type_param.clone(), lo_type));
            }

            return_type = self.build_type(Some(&ctx), macro_return_type)?;
        }

        Ok(return_type)
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
        let Some(macro_def) = self.get_macro_def(macro_name) else {
            return Err(LoError {
                message: format!("Unknown macro: {}", macro_name),
                loc: loc.clone(),
            });
        };

        ctx.enter_scope(LoScopeType::Macro);

        if type_args.len() != macro_def.macro_type_params.len() {
            return Err(LoError {
                message: format!(
                    "Invalid number of type args, expected {}, got {}",
                    macro_def.macro_type_params.len(),
                    type_args.len()
                ),
                loc: macro_def.loc.clone(),
            });
        }

        // TODO: check for type shadowing
        for i in 0..macro_def.macro_type_params.len() {
            let type_param = &macro_def.macro_type_params[i];
            let type_arg = &type_args[i];

            let lo_type = self.build_type(Some(ctx), &type_arg)?;
            ctx.current_scope_mut()
                .macro_types
                .push((type_param.clone(), lo_type));
        }

        let mut all_args = Vec::<&CodeExpr>::new();
        if let Some(receiver_arg) = receiver_arg {
            all_args.push(&receiver_arg);
        }
        for arg in args {
            all_args.push(&arg);
        }

        // TODO: type check margo args against param types
        if all_args.len() != macro_def.macro_params.len() {
            return Err(LoError {
                message: format!(
                    "Invalid number of macro args, expected {}, got {}",
                    macro_def.macro_params.len(),
                    all_args.len()
                ),
                loc: macro_def.loc.clone(),
            });
        }

        // TODO: check for const shadowing
        for i in 0..macro_def.macro_params.len() {
            let macro_param = &macro_def.macro_params[i];
            let macro_arg = &all_args[i];

            ctx.current_scope_mut()
                .macro_args
                .push((macro_param.param_name.clone(), unsafe_borrow(macro_arg)));
        }

        for expr in &macro_def.body.exprs {
            self.codegen(ctx, instrs, expr)?;
        }

        ctx.exit_scope(); // exit macro scope

        Ok(())
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
                        local_name, definition_loc
                    ),
                    loc,
                });
            }
        }

        let local_index = ctx.last_local_index;
        ctx.locals.push(LoLocal {
            local_index,
            local_type: local_type.clone(),
            definition_loc: loc,
            is_fn_param,
        });
        let lo_local_index = ctx.locals.len() - 1;
        ctx.current_scope_mut().locals.push(LoScopedLocal {
            local_name,
            lo_local_index,
            defined_in_this_scope: true,
        });
        ctx.last_local_index += self.count_wasm_type_components(local_type);

        Ok(local_index)
    }

    // TODO: merge this with codegen_var_set
    fn codegen_compound_assignment(
        &self,
        ctx: &mut LoExprContext,
        instrs: &mut Vec<WasmInstr>,
        base_op: Option<InfixOpTag>,
        op_loc: &LoLocation,
        lhs: &CodeExpr,
        rhs: &CodeExpr,
    ) -> Result<(), LoError> {
        let lhs_type = self.get_expr_type(ctx, lhs)?;
        let rhs_type = self.get_expr_type(ctx, rhs)?;

        if lhs_type != rhs_type {
            return Err(LoError {
                message: format!(
                    "Unexpected value for assignment: {}, expected {}",
                    rhs_type, lhs_type
                ),
                loc: op_loc.clone(),
            });
        }

        if let CodeExpr::PrefixOp(PrefixOpExpr {
            op_tag: PrefixOpTag::Dereference,
            expr: addr_expr,
            loc: _,
        }) = lhs
        {
            let pointer_type = self.get_expr_type(ctx, addr_expr)?;
            let LoType::Pointer {
                pointee: pointee_type,
            } = pointer_type
            else {
                return Err(LoError {
                    message: format!("Cannot use {pointer_type} as an address, pointer expected"),
                    loc: addr_expr.loc().clone(),
                });
            };

            let component_count = self.count_wasm_type_components(&pointee_type);
            if component_count == 1 {
                self.codegen(ctx, instrs, addr_expr)?;
            }

            if let Some(base_op) = base_op {
                self.codegen(ctx, instrs, lhs)?;
                self.codegen(ctx, instrs, rhs)?;

                let kind = self.get_binary_op_kind(&base_op, &lhs_type, op_loc)?;
                instrs.push(WasmInstr::BinaryOp { kind });
            } else {
                self.codegen(ctx, instrs, rhs)?;
            }

            if component_count != 1 {
                let tmp_local_index = self.define_local(
                    ctx,
                    rhs.loc().clone(),
                    format!("%{}", ctx.last_local_index),
                    &rhs_type,
                    false,
                )?;
                self.codegen_local_set(instrs, &rhs_type, tmp_local_index);

                for i in 0..component_count {
                    self.codegen(ctx, instrs, addr_expr)?;
                    instrs.push(WasmInstr::LocalGet {
                        local_index: tmp_local_index + i,
                    });
                }
            }

            self.codegen_load_or_store(instrs, &pointee_type, 0, true);

            return Ok(());
        }

        if let CodeExpr::Ident(IdentExpr {
            repr,
            parts: _,
            loc,
        }) = lhs
        {
            if let Some(local_index) = ctx.get_local(&repr).map(|l| l.local_index) {
                if let Some(base_op) = base_op {
                    self.codegen(ctx, instrs, lhs)?;
                    self.codegen(ctx, instrs, rhs)?;

                    let kind = self.get_binary_op_kind(&base_op, &lhs_type, op_loc)?;
                    instrs.push(WasmInstr::BinaryOp { kind });
                } else {
                    self.codegen(ctx, instrs, rhs)?;
                }

                for i in (0..self.count_wasm_type_components(&rhs_type)).rev() {
                    instrs.push(WasmInstr::LocalSet {
                        local_index: local_index + i,
                    });
                }

                return Ok(());
            }

            if let Some(global) = self.get_global(&repr) {
                if let Some(base_op) = base_op {
                    self.codegen(ctx, instrs, lhs)?;
                    self.codegen(ctx, instrs, rhs)?;

                    let kind = self.get_binary_op_kind(&base_op, &lhs_type, op_loc)?;
                    instrs.push(WasmInstr::BinaryOp { kind });
                } else {
                    self.codegen(ctx, instrs, rhs)?;
                }

                for i in 0..self.count_wasm_type_components(&rhs_type) {
                    instrs.push(WasmInstr::GlobalSet {
                        global_index: global.global_index + i,
                    });
                }

                return Ok(());
            }

            return Err(LoError {
                message: format!("Unknown variable: {repr}"),
                loc: loc.clone(),
            });
        }

        if let CodeExpr::FieldAccess(FieldAccessExpr {
            lhs,
            field_name,
            loc: _,
        }) = lhs
        {
            let var = self.var_from_field_access(ctx, lhs, field_name)?;
            self.codegen_var_set(ctx, instrs, &var, rhs)?;

            return Ok(());
        }

        todo!();
    }

    fn codegen_load_or_store(
        &self,
        instrs: &mut Vec<WasmInstr>,
        pointee_type: &LoType,
        offset: u32,
        is_store: bool,
    ) {
        match pointee_type {
            LoType::Never | LoType::Void => unreachable!(),
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
            LoType::Result {
                ok_type: _,
                err_type: _,
            } => todo!(),
        }
    }

    fn get_expr_type(&self, ctx: &LoExprContext, expr: &CodeExpr) -> Result<LoType, LoError> {
        match expr {
            CodeExpr::BoolLiteral(_) => Ok(LoType::Bool),
            CodeExpr::CharLiteral(_) => Ok(LoType::U8),
            CodeExpr::IntLiteral(IntLiteralExpr {
                repr: _,
                value: _,
                tag,
                loc: _,
            }) => match tag.as_deref() {
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
                Some(_) => todo!(),
                None => Ok(LoType::U32),
            },
            CodeExpr::StringLiteral(_) => todo!(),
            CodeExpr::StructLiteral(StructLiteralExpr {
                struct_name,
                fields: _,
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
            CodeExpr::ArrayLiteral(_) => todo!(),
            CodeExpr::Ident(IdentExpr {
                repr,
                parts: _,
                loc,
            }) => {
                if let Some(const_expr) = self.get_const(ctx, &repr) {
                    return self.get_expr_type(ctx, const_expr);
                }

                let var = self.var_from_ident(ctx, &repr, loc)?;
                Ok(var.get_type())
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
            CodeExpr::PrefixOp(PrefixOpExpr { op_tag, expr, loc }) => match op_tag {
                PrefixOpTag::Not => Ok(LoType::Bool),
                PrefixOpTag::Dereference => {
                    let expr_type = self.get_expr_type(ctx, expr)?;
                    let LoType::Pointer { pointee } = expr_type else {
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
                        | LoType::Result {
                            ok_type: _,
                            err_type: _,
                        } => Ok(expr_type),
                    }
                }
            },
            CodeExpr::Cast(CastExpr {
                expr: _,
                casted_to,
                loc: _,
            }) => self.build_type(Some(ctx), casted_to),
            CodeExpr::FieldAccess(FieldAccessExpr {
                lhs,
                field_name,
                loc: _,
            }) => {
                let var = self.var_from_field_access(ctx, lhs, field_name)?;
                Ok(var.get_type())
            }
            CodeExpr::FnCall(FnCallExpr {
                fn_name,
                args: _,
                loc,
            }) => {
                let Some((fn_info, _)) = self.get_fn_info(&fn_name.repr) else {
                    return Err(LoError {
                        message: format!("Unknown function: {}", fn_name.repr),
                        loc: loc.clone(),
                    });
                };

                Ok(fn_info.fn_type.output.clone())
            }
            CodeExpr::MethodCall(MethodCallExpr {
                lhs,
                field_name,
                args: _,
                loc,
            }) => {
                let lhs_type = self.get_expr_type(ctx, lhs)?;
                let fn_name = get_fn_name_from_method(&lhs_type, &field_name.repr);

                let Some((fn_info, _)) = self.get_fn_info(&fn_name) else {
                    return Err(LoError {
                        message: format!("Unknown function: {}", fn_name),
                        loc: loc.clone(),
                    });
                };

                Ok(fn_info.fn_type.output.clone())
            }
            CodeExpr::MacroFnCall(MacroFnCallExpr {
                fn_name,
                type_args,
                args: _,
                loc,
            }) => self.get_macro_return_type(&fn_name.repr, type_args, loc),
            CodeExpr::MacroMethodCall(MacroMethodCallExpr {
                lhs,
                field_name,
                type_args,
                args: _,
                loc,
            }) => {
                let lhs_type = self.get_expr_type(ctx, lhs)?;
                let macro_name = get_fn_name_from_method(&lhs_type, &field_name.repr);
                self.get_macro_return_type(&macro_name, type_args, loc)
            }
            CodeExpr::Catch(_) => todo!(),
            CodeExpr::Dbg(_) => todo!(),
            CodeExpr::Sizeof(_) => Ok(LoType::U32),
            CodeExpr::GetDataSize(_) => Ok(LoType::U32),
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
            CodeExpr::Return(_) => Ok(LoType::Never),
            CodeExpr::PropagateError(_) => Ok(LoType::Never),
            CodeExpr::Unreachable(_) => Ok(LoType::Never),
            CodeExpr::Paren(ParenExpr { expr, loc: _ }) => self.get_expr_type(ctx, expr),
        }
    }

    fn var_from_ident(
        &self,
        ctx: &LoExprContext,
        ident: &str,
        loc: &LoLocation,
    ) -> Result<VariableInfo, LoError> {
        if let Some(local) = ctx.get_local(ident) {
            return Ok(VariableInfo::Local {
                local_index: local.local_index,
                local_type: local.local_type.clone(),
            });
        };

        if let Some(global) = self.get_global(ident) {
            return Ok(VariableInfo::Global {
                global_index: global.global_index,
                global_type: global.global_type.clone(),
            });
        }

        return Err(LoError {
            message: format!("Unknown variable: {ident}"),
            loc: loc.clone(),
        });
    }

    fn var_from_field_access(
        &self,
        ctx: &LoExprContext,
        lhs: &CodeExpr,
        field_name: &IdentExpr,
    ) -> Result<VariableInfo, LoError> {
        let lhs_type = self.get_expr_type(ctx, lhs)?;

        if let LoType::StructInstance { struct_name } = &lhs_type {
            let CodeExpr::Ident(ident) = lhs else {
                return Err(LoError {
                    message: format!(
                        "Cannot access struct field '{}' lhs is not a struct local",
                        field_name.repr
                    ),
                    loc: lhs.loc().clone(),
                });
            };
            let Some(struct_local) = ctx.get_local(&ident.repr) else {
                return Err(LoError {
                    message: format!("Unknown local {}", ident.repr),
                    loc: lhs.loc().clone(),
                });
            };

            let struct_def = self.get_struct_def(&struct_name).unwrap();
            let Some(field) = struct_def
                .fields
                .iter()
                .find(|f| &f.field_name == &field_name.repr)
            else {
                return Err(LoError {
                    message: format!("Unknown field {} in struct {struct_name}", field_name.repr),
                    loc: field_name.loc.clone(),
                });
            };

            return Ok(VariableInfo::Local {
                local_index: struct_local.local_index + field.field_index,
                local_type: field.field_type.clone(),
            });
        }

        if let LoType::Pointer { pointee } = &lhs_type {
            if let LoType::StructInstance { struct_name } = pointee.as_ref() {
                let struct_def = self.get_struct_def(&struct_name).unwrap();
                let Some(field) = struct_def
                    .fields
                    .iter()
                    .find(|f| &f.field_name == &field_name.repr)
                else {
                    return Err(LoError {
                        message: format!(
                            "Unknown field {} in struct {struct_name}",
                            field_name.repr
                        ),
                        loc: field_name.loc.clone(),
                    });
                };

                return Ok(VariableInfo::Stored {
                    address: unsafe_borrow(lhs),
                    offset: field.byte_offset,
                    value_type: field.field_type.clone(),
                });
            }
        };

        return Err(LoError {
            message: format!(
                "Cannot get field '{}' on non struct: {lhs_type}",
                field_name.repr
            ),
            loc: field_name.loc.clone(),
        });
    }

    fn var_from_deref(
        &self,
        ctx: &LoExprContext,
        expr: &CodeExpr,
    ) -> Result<VariableInfo, LoError> {
        let expr_type = self.get_expr_type(ctx, expr)?;

        if let LoType::Pointer { pointee } = &expr_type {
            return Ok(VariableInfo::Stored {
                address: unsafe_borrow(expr),
                offset: 0,
                value_type: pointee.as_ref().clone(),
            });
        };

        return Err(LoError {
            message: format!("Cannot dereference expression of type '{}'", expr_type),
            loc: expr.loc().clone(),
        });
    }

    fn codegen_var_get(
        &self,
        ctx: &mut LoExprContext,
        instrs: &mut Vec<WasmInstr>,
        var: &VariableInfo,
    ) -> Result<(), LoError> {
        match var {
            VariableInfo::Local {
                local_index,
                local_type,
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
            } => {
                for i in 0..self.count_wasm_type_components(global_type) {
                    instrs.push(WasmInstr::GlobalGet {
                        global_index: global_index + i,
                    });
                }
            }
            VariableInfo::Stored {
                address,
                offset,
                value_type,
            } => {
                self.codegen(ctx, instrs, address)?;
                self.codegen_load_or_store(instrs, &value_type, *offset, false);
            }
        }

        Ok(())
    }

    fn codegen_var_set(
        &self,
        ctx: &mut LoExprContext,
        instrs: &mut Vec<WasmInstr>,
        var: &VariableInfo,
        value: &CodeExpr,
    ) -> Result<(), LoError> {
        match var {
            VariableInfo::Local {
                local_index,
                local_type,
            } => {
                self.codegen(ctx, instrs, value)?;
                self.codegen_local_set(instrs, local_type, *local_index);
            }
            VariableInfo::Global {
                global_index,
                global_type,
            } => {
                self.codegen(ctx, instrs, value)?;
                for i in (0..self.count_wasm_type_components(global_type)).rev() {
                    instrs.push(WasmInstr::GlobalSet {
                        global_index: global_index + i,
                    });
                }
            }
            VariableInfo::Stored {
                address,
                offset,
                value_type,
            } => {
                self.codegen(ctx, instrs, address)?;
                self.codegen(ctx, instrs, value)?;
                self.codegen_load_or_store(instrs, &value_type, *offset, true);
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

    // TODO: add validation for const expr
    fn ensure_const_expr(&self, _expr: &CodeExpr) -> Result<(), LoError> {
        Ok(())
    }

    fn get_type_or_err(&self, type_name: &str, err_loc: &LoLocation) -> Result<LoType, LoError> {
        if let Some(t) = self.get_typedef(type_name) {
            return Ok(t.value.clone());
        }

        Err(LoError {
            message: format!("Unknown type: {}", type_name),
            loc: err_loc.clone(),
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

    fn get_const(&self, ctx: &LoExprContext, const_name: &str) -> Option<&CodeExpr> {
        if let Some(const_def) = self.get_const_def(const_name) {
            return Some(&const_def.const_value);
        }

        if let Some(macro_arg) = ctx.get_macro_arg(const_name) {
            return Some(macro_arg);
        }

        None
    }

    fn get_const_def(&self, const_name: &str) -> Option<&ConstDefExpr> {
        for const_def in &self.const_defs {
            if const_def.const_name.repr == const_name {
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
            LoType::Result { ok_type, err_type } => {
                self.lower_type(ok_type, wasm_types);
                self.lower_type(err_type, wasm_types);
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
            LoType::Never | LoType::Void => {}
            LoType::Bool | LoType::U8 | LoType::I8 => {
                layout.primities_count += 1;
                layout.byte_length += 1;
            }
            LoType::U16 | LoType::I16 => {
                layout.primities_count += 1;
                layout.byte_length += 2;
            }
            LoType::U32
            | LoType::I32
            | LoType::F32
            | LoType::Pointer { pointee: _ }
            | LoType::SequencePointer { pointee: _ } => {
                layout.primities_count += 1;
                layout.byte_length += 4;
            }
            LoType::U64 | LoType::I64 | LoType::F64 => {
                layout.primities_count += 1;
                layout.byte_length += 8;
            }
            LoType::StructInstance { struct_name } => {
                let struct_def = self.get_struct_def(struct_name).unwrap();

                for field in &struct_def.fields {
                    self.get_type_layout(&field.field_type, layout);
                }
            }
            LoType::Result { ok_type, err_type } => {
                self.get_type_layout(ok_type, layout);
                self.get_type_layout(err_type, layout);
            }
        }
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

    fn get_fn_info(&self, fn_name: &str) -> Option<(&LoFnInfo, &WasmFnInfo)> {
        for wasm_fn_info in &self.wasm_functions {
            if wasm_fn_info.fn_name == fn_name {
                let lo_fn_info = &self.lo_functions[wasm_fn_info.lo_fn_index];
                return Some((lo_fn_info, wasm_fn_info));
            }
        }

        None
    }

    fn get_global(&self, global_name: &str) -> Option<&LoGlobalDef> {
        for global_def in &self.globals {
            if global_def.def_expr.global_name.repr == global_name {
                return Some(global_def);
            }
        }

        None
    }
}

fn get_fn_name_from_method(receiver_type: &LoType, method_name: &str) -> String {
    let resolved_receiver_type = receiver_type.deref_rec();
    format!("{resolved_receiver_type}::{method_name}")
}

fn unsafe_borrow<T>(x: &T) -> &'static T {
    unsafe { &*(x as *const T) }
}
