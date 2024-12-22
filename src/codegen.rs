use crate::{ast::*, core::*, lexer::*, parser_v2::*, wasm::*};
use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::cell::RefCell;

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
    exported_as: Vec<String>,
    definition_loc: LoLocation,
}

enum LoFnSource {
    Guest {
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

#[derive(Default, Clone)]
struct LoExprContext {
    lo_fn_index: Option<usize>,
    locals: Vec<LoLocal>,
    next_local_index: u32,
    addr_local_index: Option<u32>,
    deferred: Vec<LoCodeUnit>,
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
struct LoCodeUnit {
    lo_type: LoType,
    instrs: Vec<WasmInstr>,
}

#[derive(Clone)]
struct LoScope {
    scope_type: LoScopeType,
    locals: Vec<LoScopedLocal>,
    macro_args: Vec<(String, LoCodeUnit)>,
    macro_type_args: Vec<(String, LoType)>,
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
                if macro_type_arg.0 == type_name {
                    return Some(&macro_type_arg.1);
                }
            }
        }

        None
    }

    fn get_macro_arg(&self, arg_name: &str) -> Option<&LoCodeUnit> {
        for scope in self.scopes.iter().rev() {
            for macro_arg in &scope.macro_args {
                if macro_arg.0 == arg_name {
                    return Some(&macro_arg.1);
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
        address: LoCodeUnit,
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

struct LoConstDef {
    const_name: String,
    code_unit: LoCodeUnit,
    loc: LoLocation,
}

#[derive(Default)]
pub struct CodeGen {
    pub errors: LoErrorManager,

    type_defs: Vec<LoTypeDef>,
    struct_defs: Vec<LoStructDef>,
    globals: Vec<LoGlobalDef>,
    const_defs: Vec<LoConstDef>,
    macro_defs: Vec<MacroDefExpr>,

    lo_functions: Vec<LoFnInfo>,
    wasm_functions: Vec<WasmFnInfo>,

    memory: Option<MemoryDefExpr>,
    memory_imported_from: Option<String>,
    datas: RefCell<Vec<WasmData>>,
    string_pool: RefCell<Vec<(String, u32)>>,
    data_size: RefCell<u32>,

    wasm_types: RefCell<Vec<WasmFnType>>,

    const_ctx: LoExprContext,
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
                        Some(return_type) => self.build_type(&self.const_ctx, return_type)?,
                        _ => LoType::Void,
                    };

                    let mut ctx = LoExprContext::default();
                    ctx.lo_fn_index = Some(self.lo_functions.len());
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

                    let mut exported_as = Vec::new();
                    if fn_def.exported {
                        exported_as.push(fn_def.decl.fn_name.repr.clone());
                    }

                    // TODO: make sure function name does not collide with intrinsics
                    if let Some(fn_info) = self.get_fn_def(&fn_def.decl.fn_name.repr) {
                        self.errors.report(LoError {
                            message: format!(
                                "Duplicate function definition: {}, previously defined at {}",
                                fn_def.decl.fn_name.repr, fn_info.definition_loc
                            ),
                            loc: fn_def.decl.loc.clone(),
                        });
                    }

                    self.lo_functions.push(LoFnInfo {
                        fn_name: fn_def.decl.fn_name.repr,
                        fn_type: LoFnType { inputs, output },
                        fn_source: LoFnSource::Guest {
                            ctx,
                            body: fn_def.body,
                        },
                        exported_as,
                        definition_loc: fn_def.loc.clone(),
                    });
                }
                TopLevelExpr::ExportExistingFn(ExportExistingFnExpr {
                    in_fn_name,
                    out_fn_name,
                    loc,
                }) => {
                    let Some(fn_info) = self.get_fn_def_mut(&in_fn_name.repr) else {
                        return Err(LoError {
                            message: format!(
                                "Cannot re-export not existing function: {}",
                                in_fn_name.repr
                            ),
                            loc: loc.clone(),
                        });
                    };

                    fn_info.exported_as.push(out_fn_name.unescape());
                }
                TopLevelExpr::Import(ImportExpr {
                    module_name,
                    items,
                    loc,
                }) => {
                    let module_name = module_name.unescape();

                    for item in items {
                        let fn_decl = match item {
                            ImportItem::FnDecl(fn_decl) => fn_decl,
                            ImportItem::Memory(memory) => {
                                self.define_memory(memory, Some(module_name.clone()))?;
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
                            fn_type.output = self.build_type(&self.const_ctx, &return_type)?;
                        }

                        // TODO: make sure function name does not collide with intrinsics
                        if let Some(fn_info) = self.get_fn_def(&fn_decl.fn_name.repr) {
                            self.errors.report(LoError {
                                message: format!(
                                    "Duplicate function definition: {}, previously defined at {}",
                                    fn_decl.fn_name.repr, fn_info.definition_loc
                                ),
                                loc: fn_decl.loc.clone(),
                            });
                        }

                        self.lo_functions.push(LoFnInfo {
                            fn_name: fn_decl.fn_name.repr.clone(),
                            fn_type,
                            fn_source: LoFnSource::Host {
                                module_name: module_name.clone(),
                                external_fn_name: fn_decl.fn_name.parts.last().unwrap().clone(),
                            },
                            exported_as: Vec::new(),
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

                    let value_type = match &global.global_value {
                        GlobalDefValue::Expr(expr) => {
                            self.ensure_const_expr(expr)?;
                            let value_type = self.get_expr_type(&self.const_ctx, expr)?;
                            let value_comp_count = self.count_wasm_type_components(&value_type);
                            if value_comp_count != 1 {
                                return Err(LoError {
                                    message: format!(
                                        "Cannot define global with non-primitive type {value_type}",
                                    ),
                                    loc: global.loc,
                                });
                            }
                            value_type
                        }
                        GlobalDefValue::DataSize => LoType::U32,
                    };

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
                        let field_type = self.build_type(&self.const_ctx, &field.field_type)?;

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

                    let type_value = self.build_type(&self.const_ctx, &typedef.type_value)?;

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

                    let mut const_ctx = LoExprContext::default();
                    let code_unit = self.build_code_unit(&mut const_ctx, &const_def.const_value)?;
                    self.const_defs.push(LoConstDef {
                        const_name: const_def.const_name.repr.clone(),
                        code_unit,
                        loc: const_def.loc.clone(),
                    });
                }
                TopLevelExpr::MemoryDef(memory) => {
                    self.define_memory(memory, None)?;
                }
                TopLevelExpr::StaticDataStore(static_data_store) => {
                    let mut const_ctx = LoExprContext::default();

                    let mut offset_expr = WasmExpr { instrs: Vec::new() };
                    self.ensure_const_expr(&static_data_store.addr)?;
                    self.codegen(
                        &mut const_ctx,
                        &mut offset_expr.instrs,
                        &static_data_store.addr,
                    )?;
                    let bytes = match &static_data_store.data {
                        StaticDataStorePayload::String { value } => {
                            value.unescape().as_bytes().to_vec()
                        }
                    };

                    let data = WasmData::Active {
                        offset: offset_expr,
                        bytes,
                    };
                    self.datas.borrow_mut().push(data);
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
                    self.wasm_functions.push(WasmFnInfo {
                        fn_name: fn_info.fn_name.clone(),
                        lo_fn_index,
                        wasm_fn_index,
                    });

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

            let exported_item_index = self.wasm_functions.last().unwrap().wasm_fn_index;
            for export_name in &fn_info.exported_as {
                wasm_module.exports.push(WasmExport {
                    export_type: WasmExportType::Func,
                    export_name: export_name.clone(),
                    exported_item_index,
                });
            }
        }

        // build function codes
        for i in 0..self.wasm_functions.len() {
            let wasm_fn_info = &self.wasm_functions[i];
            let lo_fn_info = &self.lo_functions[wasm_fn_info.lo_fn_index];

            let LoFnSource::Guest { ctx, body } = &lo_fn_info.fn_source else {
                continue;
            };

            let mut ctx = ctx.clone();
            let mut wasm_expr = WasmExpr { instrs: Vec::new() };
            let mut had_return = false;
            for expr in &body.exprs {
                if let CodeExpr::Return(_) = expr {
                    had_return = true;
                }
                self.codegen(&mut ctx, &mut wasm_expr.instrs, expr)?;
            }
            if !had_return {
                self.emit_deferred(&mut ctx, &mut wasm_expr.instrs)?;
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

        for static_data_store in self.datas.borrow().iter() {
            wasm_module.datas.push(static_data_store.clone());
        }

        let mut const_ctx = LoExprContext::default();

        let mut wasm_types_buf = Vec::with_capacity(1);
        for global in &self.globals {
            self.lower_type(&global.global_type, &mut wasm_types_buf);
            let wasm_value_type = wasm_types_buf.pop().unwrap();

            let mut initial_value = WasmExpr { instrs: Vec::new() };

            match &global.def_expr.global_value {
                GlobalDefValue::Expr(expr) => {
                    self.codegen(&mut const_ctx, &mut initial_value.instrs, expr)?;
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

        Ok(wasm_module)
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
                    existing_memory.loc
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
            FnParamType::Type { expr } => self.build_type(&self.const_ctx, &expr),
        }
    }

    fn build_type(&self, ctx: &LoExprContext, type_expr: &TypeExpr) -> Result<LoType, LoError> {
        match type_expr {
            TypeExpr::Named { name } => {
                if let Some(macro_type_arg) = ctx.get_macro_type_arg(&name.repr) {
                    return Ok(macro_type_arg.clone());
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

                let (string_ptr, string_len) = self.process_const_string(value, &loc)?;

                if *zero_terminated {
                    instrs.push(WasmInstr::I32Const {
                        value: string_ptr as i32,
                    });

                    return Ok(());
                }

                // emit str struct values
                instrs.push(WasmInstr::I32Const {
                    value: string_ptr as i32,
                });
                instrs.push(WasmInstr::I32Const {
                    value: string_len as i32,
                });
            }
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
                        // TODO: remove U32 check, it is for compat with V1 only
                        if current_item_type != item_type && current_item_type != LoType::U32 {
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
                let (ok_type, err_type) = self.get_result_literal_type(ctx, result_type, loc)?;

                let mut value_type = LoType::Void;
                if let Some(value) = value {
                    value_type = self.get_expr_type(ctx, value)?;
                }

                if *is_ok {
                    if value_type != ok_type {
                        return Err(LoError {
                            message: format!(
                                "Cannot create result, Ok type mismatch. Got {}, expected: {}",
                                value_type, ok_type
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

                if value_type != err_type {
                    return Err(LoError {
                        message: format!(
                            "Cannot create result, Err type mismatch. Got {}, expected: {}",
                            value_type, err_type
                        ),
                        loc: loc.clone(),
                    });
                }

                self.codegen_default_value(ctx, instrs, &ok_type);
                self.codegen(ctx, instrs, value.as_ref().unwrap())?;
            }

            CodeExpr::Ident(ident) => {
                if let Some(const_expr) = self.get_const(ctx, &ident.repr) {
                    for instr in &const_expr.instrs {
                        instrs.push(instr.clone());
                    }
                    return Ok(());
                }

                let var = self.var_from_ident(ctx, ident)?;
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
                let local_index =
                    self.define_local(ctx, loc.clone(), local_name.clone(), &local_type, false)?;
                let var = VariableInfo::Local {
                    local_index,
                    local_type: local_type.clone(),
                };
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

                    self.codegen_var_get(ctx, instrs, &var)?;
                    self.codegen_var_set_prepare(ctx, instrs, &var);
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
                self.codegen_var_set_prepare(ctx, instrs, &var);
                self.codegen(ctx, instrs, rhs)?;
                self.codegen_var_set(ctx, instrs, &var)?;
            }
            CodeExpr::FieldAccess(field_access) => {
                let var = self.var_from_field_access(ctx, field_access)?;
                self.codegen_var_get(ctx, instrs, &var)?;
            }

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

            CodeExpr::Dbg(DbgExpr { message, loc }) => {
                let debug_message = format!("{} - {}", loc, message.unescape());
                let (str_ptr, str_len) = self.process_const_string(debug_message, loc)?;

                // emit str struct values
                instrs.push(WasmInstr::I32Const {
                    value: str_ptr as i32,
                });
                instrs.push(WasmInstr::I32Const {
                    value: str_len as i32,
                });
            }
            CodeExpr::Sizeof(SizeofExpr { type_expr, loc: _ }) => {
                let lo_type = self.build_type(ctx, type_expr)?;
                let mut type_layout = LoTypeLayout::default();
                self.get_type_layout(&lo_type, &mut type_layout);

                instrs.push(WasmInstr::I32Const {
                    value: type_layout.byte_length as i32,
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

            CodeExpr::Return(ReturnExpr { expr, loc: _ }) => {
                if let Some(return_expr) = expr {
                    self.codegen(ctx, instrs, return_expr)?;
                }

                self.emit_deferred(ctx, instrs)?;

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
                let local_index =
                    self.define_local(ctx, loc.clone(), counter.clone(), &counter_type, false)?;
                let counter_var = VariableInfo::Local {
                    local_index,
                    local_type: counter_type.clone(),
                };
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

                            for expr in &body.exprs {
                                self.codegen(ctx, instrs, &expr)?;
                            }

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
            CodeExpr::Defer(DeferExpr { expr, loc: _ }) => {
                let code_unit = self.build_code_unit(ctx, expr)?;
                ctx.deferred.push(code_unit);
            }
            CodeExpr::Catch(CatchExpr {
                lhs,
                error_bind,
                catch_body,
                loc,
            }) => {
                self.codegen_catch(ctx, instrs, lhs, Some((&error_bind, catch_body)), loc)?;
            }
            CodeExpr::PropagateError(PropagateErrorExpr { expr, loc }) => {
                self.codegen_catch(ctx, instrs, expr, None, loc)?;
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
        ctx: &LoExprContext,
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
            let mut ctx = ctx.clone();
            ctx.enter_scope(LoScopeType::Macro);

            for i in 0..macro_def.macro_type_params.len() {
                let type_param = &macro_def.macro_type_params[i];
                let type_arg = &type_args[i];

                let lo_type = self.build_type(&ctx, &type_arg)?;
                ctx.current_scope_mut()
                    .macro_type_args
                    .push((type_param.clone(), lo_type));
            }

            return_type = self.build_type(&ctx, macro_return_type)?;
        }

        Ok(return_type)
    }

    // TODO: typecheck actual macro return with its specified return type
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

        let mut all_args = Vec::new();
        if let Some(receiver_arg) = receiver_arg {
            all_args.push(self.build_code_unit(ctx, receiver_arg)?);
        }
        for arg in args {
            all_args.push(self.build_code_unit(ctx, arg)?);
        }

        let mut all_type_args = Vec::new();
        for type_arg in type_args {
            all_type_args.push(self.build_type(ctx, &type_arg)?);
        }

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
        for (type_param, type_arg) in macro_def
            .macro_type_params
            .iter()
            .zip(all_type_args.into_iter())
        {
            ctx.current_scope_mut()
                .macro_type_args
                .push((type_param.clone(), type_arg));
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
        for (macro_param, macro_arg) in macro_def.macro_params.iter().zip(all_args.into_iter()) {
            ctx.current_scope_mut()
                .macro_args
                .push((macro_param.param_name.clone(), macro_arg));
        }

        for expr in &macro_def.body.exprs {
            self.codegen(ctx, instrs, expr)?;
        }

        ctx.exit_scope(); // exit macro scope

        Ok(())
    }

    fn codegen_catch(
        &self,
        ctx: &mut LoExprContext,
        instrs: &mut Vec<WasmInstr>,
        expr: &CodeExpr,
        catch_details: Option<(&IdentExpr, &CodeBlockExpr)>,
        loc: &LoLocation,
    ) -> Result<(), LoError> {
        let expr_type = self.get_expr_type(ctx, expr)?;
        let (ok_type, err_type) = self.assert_catchable_type(&expr_type, loc)?;

        ctx.enter_scope(LoScopeType::Block); // enter catch scope

        // put result on the stack
        self.codegen(ctx, instrs, expr)?;

        // pop error
        let error_bind = if let Some((error_bind, _)) = catch_details {
            error_bind.repr.clone()
        } else {
            String::from("<err>")
        };
        let err_local_index = self.define_local(ctx, loc.clone(), error_bind, &err_type, false)?;
        self.codegen_local_set(instrs, err_type, err_local_index);

        // pop ok
        let ok_bind = String::from("<ok>");
        let ok_local_index = self.define_local(ctx, loc.clone(), ok_bind, &ok_type, false)?;
        self.codegen_local_set(instrs, ok_type, ok_local_index);

        // cond: error != 0
        instrs.push(WasmInstr::LocalGet {
            local_index: err_local_index,
        });

        let in_out_type_index = self.get_block_inout_type(&[], ok_type);
        instrs.push(WasmInstr::BlockStart {
            block_kind: WasmBlockKind::If,
            block_type: WasmBlockType::InOut {
                type_index: in_out_type_index,
            },
        });

        // catch error
        if let Some((_, catch_body)) = catch_details {
            for expr in &catch_body.exprs {
                self.codegen(ctx, instrs, &expr)?;
            }

            // TODO: push this conditionally
            instrs.push(WasmInstr::Unreachable);
        } else {
            // return ok_type of function's result + caught error
            let (fn_ok_type, _) = self.get_result_literal_type(ctx, &None, loc)?;
            self.codegen_default_value(ctx, instrs, &fn_ok_type);
            instrs.push(WasmInstr::LocalGet {
                local_index: err_local_index,
            });

            self.emit_deferred(ctx, instrs)?;
            instrs.push(WasmInstr::Return);
        }

        instrs.push(WasmInstr::Else);

        // no error, push ok value
        let ok_var = VariableInfo::Local {
            local_index: ok_local_index,
            local_type: ok_type.clone(),
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

    fn get_result_literal_type(
        &self,
        ctx: &LoExprContext,
        explicit_type: &Option<(TypeExpr, TypeExpr)>,
        loc: &LoLocation,
    ) -> Result<(LoType, LoType), LoError> {
        if let Some((ok_type_expr, err_type_expr)) = explicit_type {
            let ok_type = self.build_type(ctx, &ok_type_expr)?;
            let err_type = self.build_type(ctx, &err_type_expr)?;
            return Ok((ok_type, err_type));
        }

        let Some(lo_fn_index) = ctx.lo_fn_index else {
            return Err(LoError {
                message: format!("Cannot create implicitly typed result in const context"),
                loc: loc.clone(),
            });
        };

        let fn_info = &self.lo_functions[lo_fn_index];
        let LoType::Result { ok_type, err_type } = &fn_info.fn_type.output else {
            return Err(LoError {
                message: format!(
                    "Cannot create implicitly typed result: function does not return result"
                ),
                loc: loc.clone(),
            });
        };

        Ok((ok_type.as_ref().clone(), err_type.as_ref().clone()))
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
                let (ok_type, err_type) = self.get_result_literal_type(ctx, result_type, loc)?;

                return Ok(LoType::Result {
                    ok_type: Box::new(ok_type.clone()),
                    err_type: Box::new(err_type.clone()),
                });
            }

            CodeExpr::Ident(ident) => {
                if let Some(const_expr) = self.get_const(ctx, &ident.repr) {
                    return Ok(const_expr.lo_type.clone());
                }

                let var = self.var_from_ident(ctx, ident)?;
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
            }) => self.build_type(ctx, casted_to),
            CodeExpr::FieldAccess(field_access) => {
                let lhs_type = self.get_expr_type(ctx, &field_access.lhs)?;
                let (_, field) =
                    self.get_struct_or_struct_ref_field(ctx, &lhs_type, field_access)?;
                Ok(field.field_type.clone())
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
            }) => self.get_macro_return_type(ctx, &fn_name.repr, type_args, loc),
            CodeExpr::MacroMethodCall(MacroMethodCallExpr {
                lhs,
                field_name,
                type_args,
                args: _,
                loc,
            }) => {
                let lhs_type = self.get_expr_type(ctx, lhs)?;
                let macro_name = get_fn_name_from_method(&lhs_type, &field_name.repr);
                self.get_macro_return_type(ctx, &macro_name, type_args, loc)
            }
            CodeExpr::Catch(CatchExpr {
                lhs,
                error_bind: _,
                catch_body: _,
                loc,
            }) => {
                let expr_type = self.get_expr_type(ctx, lhs)?;
                let (ok_type, _) = self.assert_catchable_type(&expr_type, loc)?;
                Ok(ok_type.clone())
            }
            CodeExpr::PropagateError(PropagateErrorExpr { expr, loc }) => {
                let expr_type = self.get_expr_type(ctx, expr)?;
                let (ok_type, _) = self.assert_catchable_type(&expr_type, loc)?;
                Ok(ok_type.clone())
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
            CodeExpr::Return(_) => Ok(LoType::Never),
            CodeExpr::Unreachable(_) => Ok(LoType::Never),
            CodeExpr::Paren(ParenExpr { expr, loc: _ }) => self.get_expr_type(ctx, expr),
        }
    }

    fn var_from_expr(
        &self,
        ctx: &mut LoExprContext,
        expr: &CodeExpr,
    ) -> Result<Option<VariableInfo>, LoError> {
        Ok(match expr {
            CodeExpr::Ident(ident) => Some(self.var_from_ident(ctx, ident)?),
            CodeExpr::FieldAccess(field_access) => {
                Some(self.var_from_field_access(ctx, field_access)?)
            }
            CodeExpr::Paren(ParenExpr { expr, loc: _ }) => self.var_from_expr(ctx, expr)?,
            CodeExpr::PrefixOp(PrefixOpExpr {
                op_tag,
                expr,
                loc: _,
            }) => match op_tag {
                PrefixOpTag::Dereference => Some(self.var_from_deref(ctx, expr)?),
                _ => None,
            },
            _ => None,
        })
    }

    fn var_from_ident(
        &self,
        ctx: &LoExprContext,
        ident: &IdentExpr,
    ) -> Result<VariableInfo, LoError> {
        if let Some(local) = ctx.get_local(&ident.repr) {
            return Ok(VariableInfo::Local {
                local_index: local.local_index,
                local_type: local.local_type.clone(),
            });
        };

        if let Some(global) = self.get_global(&ident.repr) {
            return Ok(VariableInfo::Global {
                global_index: global.global_index,
                global_type: global.global_type.clone(),
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
    ) -> Result<VariableInfo, LoError> {
        let lhs_type = self.get_expr_type(ctx, field_access.lhs.as_ref())?;

        let (is_ref, field) = self.get_struct_or_struct_ref_field(ctx, &lhs_type, field_access)?;

        if is_ref {
            return Ok(VariableInfo::Stored {
                address: self.build_code_unit(ctx, &field_access.lhs)?,
                offset: field.byte_offset,
                value_type: field.field_type.clone(),
            });
        }

        if let Some(var) = self.var_from_expr(ctx, &field_access.lhs.as_ref())? {
            if let VariableInfo::Local {
                local_index,
                local_type: _,
            } = var
            {
                return Ok(VariableInfo::Local {
                    local_index: local_index + field.field_index,
                    local_type: field.field_type.clone(),
                });
            }

            if let VariableInfo::Stored {
                address,
                offset,
                value_type: _,
            } = var
            {
                return Ok(VariableInfo::Stored {
                    address: address,
                    offset: offset + field.byte_offset,
                    value_type: field.field_type.clone(),
                });
            }
        };

        return Err(LoError {
            message: format!(
                "Cannot access struct field '{}' invalid lhs",
                field_access.field_name.repr
            ),
            loc: field_access.lhs.loc().clone(),
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
    ) -> Result<(bool, &LoStructField), LoError> {
        let mut is_ref = false;

        if let LoType::Pointer { pointee } = &lhs_type {
            is_ref = true;
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

        Ok((is_ref, field))
    }

    fn var_from_deref(
        &self,
        ctx: &mut LoExprContext,
        addr_expr: &CodeExpr,
    ) -> Result<VariableInfo, LoError> {
        let addr_type = self.get_expr_type(ctx, addr_expr)?;

        if let LoType::Pointer { pointee } = &addr_type {
            return Ok(VariableInfo::Stored {
                address: self.build_code_unit(ctx, addr_expr)?,
                offset: 0,
                value_type: pointee.as_ref().clone(),
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
                let mut loads = Vec::new();
                self.codegen_load_or_store(&mut loads, &value_type, *offset, false);

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
        }

        Ok(())
    }

    // should be called before set's value is pushed to the stack
    fn codegen_var_set_prepare(
        &self,
        _ctx: &mut LoExprContext,
        instrs: &mut Vec<WasmInstr>,
        var: &VariableInfo,
    ) {
        match var {
            VariableInfo::Stored {
                address,
                offset: _,
                value_type: _,
            } => {
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
        var: &VariableInfo,
    ) -> Result<(), LoError> {
        match var {
            VariableInfo::Local {
                local_index,
                local_type,
            } => {
                self.codegen_local_set(instrs, local_type, *local_index);
            }
            VariableInfo::Global {
                global_index,
                global_type,
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
                        local_name, definition_loc
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
    ) -> Result<(&'a LoType, &'a LoType), LoError> {
        let LoType::Result { ok_type, err_type } = expr_type else {
            return Err(LoError {
                message: format!("Cannot catch error from expr of type {}", expr_type),
                loc: loc.clone(),
            });
        };

        let mut err_type_components = Vec::new();
        self.lower_type(&err_type, &mut err_type_components);
        if err_type_components != [WasmType::I32] {
            return Err(LoError {
                message: format!("Invalid Result error type: {}, must lower to i32", err_type),
                loc: loc.clone(),
            });
        }

        Ok((ok_type, err_type))
    }

    fn emit_deferred(
        &self,
        ctx: &mut LoExprContext,
        instrs: &mut Vec<WasmInstr>,
    ) -> Result<(), LoError> {
        for expr in ctx.deferred.iter().rev() {
            for instr in &expr.instrs {
                instrs.push(instr.clone());
            }
        }

        Ok(())
    }

    fn build_code_unit(
        &self,
        ctx: &mut LoExprContext,
        expr: &CodeExpr,
    ) -> Result<LoCodeUnit, LoError> {
        let mut code_unit = LoCodeUnit {
            lo_type: self.get_expr_type(ctx, expr)?,
            instrs: Vec::new(),
        };
        self.codegen(ctx, &mut code_unit.instrs, expr)?;

        Ok(code_unit)
    }

    fn process_const_string(&self, value: String, loc: &LoLocation) -> Result<(u32, u32), LoError> {
        if let None = self.memory {
            return Err(LoError {
                message: format!("Cannot use strings with no memory defined"),
                loc: loc.clone(),
            });
        }

        let string_len = value.as_bytes().len() as u32;

        for (string_value, string_ptr) in self.string_pool.borrow().iter() {
            if *string_value == value {
                return Ok((*string_ptr, string_len));
            }
        }

        let string_ptr = self.append_data(value.clone().into_bytes());

        self.string_pool.borrow_mut().push((value, string_ptr));

        return Ok((string_ptr, string_len));
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

    fn get_const<'a>(&'a self, ctx: &'a LoExprContext, const_name: &str) -> Option<&'a LoCodeUnit> {
        if let Some(const_def) = self.get_const_def(const_name) {
            return Some(&const_def.code_unit);
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
            LoType::Result { ok_type, err_type } => {
                self.codegen_default_value(ctx, instrs, &ok_type);
                self.codegen_default_value(ctx, instrs, &err_type);
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
            Some("u32") | Some("i32") | None => instrs.push(WasmInstr::I32Const { value: value }),
            Some("u64") | Some("i64") => instrs.push(WasmInstr::I64Const {
                value: value as i64,
            }),
            _ => todo!(),
        }
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

    fn get_fn_def(&self, fn_name: &str) -> Option<&LoFnInfo> {
        for fn_info in &self.lo_functions {
            if fn_info.fn_name == fn_name {
                return Some(fn_info);
            }
        }

        None
    }

    fn get_fn_def_mut(&mut self, fn_name: &str) -> Option<&mut LoFnInfo> {
        for fn_info in &mut self.lo_functions {
            if fn_info.fn_name == fn_name {
                return Some(fn_info);
            }
        }

        None
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
