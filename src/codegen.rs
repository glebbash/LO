use crate::{ast::*, core::*, lexer::*, parser_v2::*, wasm::*};
use alloc::{boxed::Box, format, string::String, vec::Vec};

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
            LoType::Result { ok_type, err_type } => write!(f, "Result<{ok_type}, {err_type}>"),
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

#[derive(Default, Clone)]
struct LoExprContext {
    locals: Vec<LoLocal>,
    scopes: Vec<LoScope>,
}

#[derive(Clone)]
struct LoLocal {
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
}

#[derive(Clone)]
struct LoScope {
    scope_type: LoScopeType,
    locals: Vec<LoScopedLocal>,
}

#[derive(Clone)]
struct LoScopedLocal {
    local_name: String,
    local_index: usize,
    defined_in_this_scope: bool,
}

impl LoExprContext {
    fn enter_scope(&mut self, scope_type: LoScopeType) {
        let mut new_scope = LoScope {
            scope_type,
            locals: Vec::new(),
        };

        if let Some(parent_scope) = self.scopes.last() {
            for local in &parent_scope.locals {
                new_scope.locals.push(LoScopedLocal {
                    local_name: local.local_name.clone(),
                    local_index: local.local_index,
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

    fn define_local(
        &mut self,
        loc: LoLocation,
        local_name: String,
        local_type: &LoType,
        is_fn_param: bool,
    ) -> Result<u32, LoError> {
        for local in self.current_scope().locals.iter() {
            if local.local_name == local_name && local.defined_in_this_scope {
                let LoLocal { definition_loc, .. } = &self.locals[local.local_index];

                return Err(LoError {
                    message: format!(
                        "Cannot redefine local {}, previously defined at {}",
                        local_name, definition_loc
                    ),
                    loc,
                });
            }
        }

        let local_index = self.locals.len();
        self.locals.push(LoLocal {
            local_type: local_type.clone(),
            definition_loc: loc,
            is_fn_param,
        });
        self.current_scope_mut().locals.push(LoScopedLocal {
            local_name,
            local_index,
            defined_in_this_scope: true,
        });

        Ok(local_index as u32)
    }

    fn get_local_index(&self, local_name: &str) -> Option<usize> {
        for local in &self.current_scope().locals {
            if local.local_name == local_name {
                return Some(local.local_index);
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

struct LoTypeDef {
    name: String,
    value: LoType,
}

struct LoGlobalDef {
    def_expr: GlobalDefExpr,
    global_type: LoType,
    global_index: u32,
}

#[derive(Default)]
pub struct CodeGen {
    pub errors: LoErrorManager,
    lo_functions: Vec<LoFnInfo>,
    wasm_functions: Vec<WasmFnInfo>,
    type_defs: Vec<LoTypeDef>,
    memory: Option<MemoryDefExpr>,
    memory_imported_from: Option<String>,
    static_data_stores: Vec<StaticDataStoreExpr>,
    globals: Vec<LoGlobalDef>,
}

impl CodeGen {
    pub fn add_file(&mut self, file: FileInfo) -> Result<(), LoError> {
        for expr in file.ast.exprs {
            match expr {
                TopLevelExpr::Include(_) => {} // skip, processed earlier
                TopLevelExpr::FnDef(fn_def) => {
                    let output = match &fn_def.decl.return_type {
                        Some(return_type) => self.build_type(return_type)?,
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

                        ctx.define_local(
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
                // TODO: handle method imports names properly
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
                            fn_type.output = self.build_type(&return_type)?;
                        }

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
                                external_fn_name: fn_decl.fn_name.repr.clone(),
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
                    let value_comp_count = self.count_wasm_type_components(&value_type) as u32;
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
                TopLevelExpr::StructDef(_) => return Err(LoError::todo(file!(), line!())),
                TopLevelExpr::TypeDef(_) => return Err(LoError::todo(file!(), line!())),
                TopLevelExpr::ConstDef(_) => return Err(LoError::todo(file!(), line!())),
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
                TopLevelExpr::ExportExistingFn(_) => return Err(LoError::todo(file!(), line!())),
                TopLevelExpr::MacroDef(_) => return Err(LoError::todo(file!(), line!())),
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

            let mut ctx = ctx.clone();
            let mut wasm_expr = WasmExpr { instrs: Vec::new() };
            for expr in &body.exprs {
                self.codegen(&mut ctx, expr, &mut wasm_expr.instrs)?;
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
                &static_data_store.addr,
                &mut offset_expr.instrs,
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
                &global.def_expr.expr,
                &mut initial_value.instrs,
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
                for type_def in &self.type_defs {
                    if type_def.name == *self_type_name {
                        if let FnParamType::Self_ = fn_param.param_type {
                            return Ok(type_def.value.clone());
                        } else {
                            return Ok(LoType::Pointer {
                                pointee: Box::new(type_def.value.clone()),
                            });
                        }
                    }
                }

                return Err(LoError {
                    message: format!("Unknown type: {self_type_name}"),
                    loc: fn_param.loc.clone(),
                });
            }
            FnParamType::Type { expr } => self.build_type(&expr),
        }
    }

    fn build_type(&self, type_expr: &TypeExpr) -> Result<LoType, LoError> {
        match type_expr {
            TypeExpr::Named { name, loc } => match &name.repr[..] {
                "never" => Ok(LoType::Never),
                "void" => Ok(LoType::Void),
                "bool" => Ok(LoType::Bool),
                "u32" => Ok(LoType::U32),
                _ => {
                    for type_def in &self.type_defs {
                        if type_def.name == name.repr {
                            return Ok(type_def.value.clone());
                        }
                    }

                    Err(LoError {
                        message: format!("Unknown type: {}", name.repr),
                        loc: loc.clone(),
                    })
                }
            },
            TypeExpr::Pointer { pointee, loc: _ } => {
                let pointee = Box::new(self.build_type(&pointee)?);

                Ok(LoType::Pointer { pointee })
            }
            TypeExpr::SequencePointer { pointee, loc: _ } => {
                let pointee = Box::new(self.build_type(&pointee)?);

                Ok(LoType::SequencePointer { pointee })
            }
            TypeExpr::Result {
                ok_type,
                err_type,
                loc: _,
            } => {
                let ok_type = Box::new(self.build_type(&ok_type)?);
                let err_type = Box::new(self.build_type(&err_type)?);

                Ok(LoType::Result { ok_type, err_type })
            }
            TypeExpr::Of {
                container_type,
                item_type: _,
                loc: _,
            } => {
                let actual_type = self.build_type(container_type)?;

                Ok(actual_type)
            }
        }
    }

    fn codegen(
        &self,
        ctx: &mut LoExprContext,
        expr: &CodeExpr,
        instrs: &mut Vec<WasmInstr>,
    ) -> Result<(), LoError> {
        match expr {
            CodeExpr::BoolLiteral(_) => todo!(),
            CodeExpr::CharLiteral(_) => todo!(),
            CodeExpr::IntLiteral(IntLiteralExpr {
                repr: _,
                value,
                tag,
                loc: _,
            }) => {
                let None = tag else { todo!() };

                instrs.push(WasmInstr::I32Const {
                    value: *value as i32,
                });
            }
            CodeExpr::StringLiteral(_) => todo!(),
            CodeExpr::StructLiteral(_) => todo!(),
            CodeExpr::ArrayLiteral(_) => todo!(),

            CodeExpr::Ident(IdentExpr {
                repr,
                parts: _,
                loc,
            }) => {
                if let Some(local_index) = ctx.get_local_index(repr) {
                    let local = &ctx.locals[local_index];

                    for i in 0..self.count_wasm_type_components(&local.local_type) {
                        instrs.push(WasmInstr::LocalGet {
                            local_index: (local_index + i) as u32,
                        });
                    }

                    return Ok(());
                };

                if let Some(global) = self.get_global(repr) {
                    for i in 0..self.count_wasm_type_components(&global.global_type) {
                        instrs.push(WasmInstr::GlobalGet {
                            global_index: global.global_index + i as u32,
                        });
                    }

                    return Ok(());
                }

                return Err(LoError {
                    message: format!("Unknown variable: {repr}"),
                    loc: loc.clone(),
                });
            }
            CodeExpr::Let(LetExpr {
                local_name,
                value,
                loc,
            }) => {
                if local_name == "_" {
                    self.codegen(ctx, value, instrs)?;

                    let local_type = self.get_expr_type(ctx, &value)?;
                    for _ in 0..self.count_wasm_type_components(&local_type) {
                        instrs.push(WasmInstr::Drop);
                    }
                    return Ok(());
                }

                let local_type = self.get_expr_type(ctx, &value)?;
                let local_index =
                    ctx.define_local(loc.clone(), local_name.clone(), &local_type, false)?;
                self.codegen_local_set(ctx, value, instrs, local_type, local_index)?;
            }
            CodeExpr::Cast(CastExpr {
                expr,
                casted_to,
                loc,
            }) => {
                let castee_type = self.get_expr_type(ctx, expr)?;
                let casted_to = self.build_type(casted_to)?;

                self.codegen(ctx, expr, instrs)?;

                match (&castee_type, &casted_to) {
                    (LoType::U32, LoType::Pointer { .. }) => {}
                    _ => {
                        return Err(LoError {
                            message: format!("Cannot cast from {castee_type} to {casted_to}"),
                            loc: loc.clone(),
                        })
                    }
                };
            }
            CodeExpr::PrefixOp(_) => todo!(),
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
                        Some(base_op),
                        op_loc,
                        lhs,
                        rhs,
                        instrs,
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

                self.codegen(ctx, lhs, instrs)?;
                self.codegen(ctx, rhs, instrs)?;

                let kind = self.get_binary_op_kind(op_tag, &lhs_type, op_loc)?;
                instrs.push(WasmInstr::BinaryOp { kind });
            }

            CodeExpr::Assign(AssignExpr {
                op_loc,
                lhs,
                rhs,
                loc: _,
            }) => {
                return self.codegen_compound_assignment(ctx, None, op_loc, lhs, rhs, instrs);
            }
            CodeExpr::FieldAccess(_) => todo!(),
            CodeExpr::PropagateError(_) => todo!(),

            CodeExpr::FnCall(FnCallExpr { fn_name, args, loc }) => {
                let Some((lo_fn_info, wasm_fn_info)) = self.get_fn_info(&fn_name.repr) else {
                    return Err(LoError {
                        message: format!("Unknown function: {}", fn_name.repr),
                        loc: loc.clone(),
                    });
                };

                let mut arg_types = Vec::new();
                for arg in args {
                    arg_types.push(self.get_expr_type(ctx, arg)?);
                    self.codegen(ctx, arg, instrs)?;
                }

                if arg_types != lo_fn_info.fn_type.inputs {
                    return Err(LoError {
                        message: format!(
                            "Invalid function arguments for function {}: {}, expected {}",
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
            }
            CodeExpr::MethodCall(_) => todo!(),
            CodeExpr::MacroFnCall(_) => todo!(),
            CodeExpr::MacroMethodCall(_) => todo!(),

            CodeExpr::Dbg(_) => todo!(),
            CodeExpr::Sizeof(_) => todo!(),
            CodeExpr::GetDataSize(_) => todo!(),

            CodeExpr::Return(ReturnExpr { expr, loc: _ }) => {
                if let Some(return_expr) = expr {
                    self.codegen(ctx, return_expr, instrs)?;
                }

                instrs.push(WasmInstr::Return);
            }
            CodeExpr::If(IfExpr {
                cond,
                then_block,
                else_block,
                loc: _,
            }) => {
                self.codegen(ctx, cond, instrs)?;

                instrs.push(WasmInstr::BlockStart {
                    block_kind: WasmBlockKind::If,
                    block_type: WasmBlockType::NoOut,
                });

                ctx.enter_scope(LoScopeType::Block);
                for expr in &then_block.exprs {
                    self.codegen(ctx, &expr, instrs)?;
                }
                ctx.exit_scope();

                match else_block {
                    ElseBlock::None => {}
                    ElseBlock::Else(code_block_expr) => {
                        instrs.push(WasmInstr::Else);
                        ctx.enter_scope(LoScopeType::Block);
                        for expr in &code_block_expr.exprs {
                            self.codegen(ctx, &expr, instrs)?;
                        }
                        ctx.exit_scope();
                    }
                    ElseBlock::ElseIf(code_expr) => {
                        instrs.push(WasmInstr::Else);
                        self.codegen(ctx, &code_expr, instrs)?;
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
                    self.codegen(ctx, expr, instrs)?;
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

                let cmp_instr;
                let add_instr;
                let inc_amount_instr;
                match counter_type {
                    LoType::Bool | LoType::I8 | LoType::U8 | LoType::I32 | LoType::U32 => {
                        cmp_instr = WasmInstr::BinaryOp {
                            kind: WasmBinaryOpKind::I32_EQ,
                        };
                        add_instr = WasmInstr::BinaryOp {
                            kind: WasmBinaryOpKind::I32_ADD,
                        };
                        inc_amount_instr = WasmInstr::I32Const { value: 1 };
                    }
                    LoType::I64 | LoType::U64 => {
                        cmp_instr = WasmInstr::BinaryOp {
                            kind: WasmBinaryOpKind::I64_EQ,
                        };
                        add_instr = WasmInstr::BinaryOp {
                            kind: WasmBinaryOpKind::I64_ADD,
                        };
                        inc_amount_instr = WasmInstr::I64Const { value: 1 };
                    }
                    _ => {
                        return Err(LoError {
                            message: format!("Invalid counter type: {counter_type}",),
                            loc: loc.clone(),
                        })
                    }
                };

                ctx.enter_scope(LoScopeType::ForLoop);

                // define counter and set value to start
                let counter_local_index =
                    ctx.define_local(loc.clone(), counter.clone(), &counter_type, false)?;
                self.codegen_local_set(ctx, &start, instrs, counter_type, counter_local_index)?;

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
                        self.codegen(ctx, end, instrs)?;
                        instrs.push(WasmInstr::LocalGet {
                            local_index: counter_local_index,
                        });
                        instrs.push(cmp_instr);
                        instrs.push(WasmInstr::BranchIf { label_index: 1 });

                        {
                            instrs.push(WasmInstr::BlockStart {
                                block_kind: WasmBlockKind::Block,
                                block_type: WasmBlockType::NoOut,
                            });

                            for expr in &body.exprs {
                                self.codegen(ctx, &expr, instrs)?;
                            }

                            instrs.push(WasmInstr::BlockEnd);
                        }

                        // increment counter
                        instrs.push(WasmInstr::LocalGet {
                            local_index: counter_local_index,
                        });
                        instrs.push(inc_amount_instr);
                        instrs.push(add_instr);
                        instrs.push(WasmInstr::LocalSet {
                            local_index: counter_local_index,
                        });

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
                    }
                }

                instrs.push(WasmInstr::Branch { label_index });
            }
            CodeExpr::Defer(_) => todo!(),
            CodeExpr::Catch(_) => todo!(),
            CodeExpr::Paren(ParenExpr { expr, loc: _ }) => {
                self.codegen(ctx, expr, instrs)?;
            }
            CodeExpr::Unreachable(_) => {
                instrs.push(WasmInstr::Unreachable);
            }
        };

        Ok(())
    }

    fn codegen_local_set(
        &self,
        ctx: &mut LoExprContext,
        value: &CodeExpr,
        instrs: &mut Vec<WasmInstr>,
        local_type: LoType,
        local_index: u32,
    ) -> Result<(), LoError> {
        self.codegen(ctx, value, instrs)?;

        for i in 0..self.count_wasm_type_components(&local_type) as u32 {
            instrs.push(WasmInstr::LocalSet {
                local_index: local_index + i,
            });
        }

        Ok(())
    }

    fn codegen_compound_assignment(
        &self,
        ctx: &mut LoExprContext,
        base_op: Option<InfixOpTag>,
        op_loc: &LoLocation,
        lhs: &CodeExpr,
        rhs: &CodeExpr,
        instrs: &mut Vec<WasmInstr>,
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
            let LoType::Pointer { pointee } = pointer_type else {
                return Err(LoError {
                    message: format!("Cannot use {pointer_type} as an address, pointer expected"),
                    loc: addr_expr.loc().clone(),
                });
            };

            let LoType::U32 = pointee.as_ref() else {
                todo!()
            };

            self.codegen(ctx, addr_expr, instrs)?;

            if let Some(base_op) = base_op {
                self.codegen(ctx, lhs, instrs)?;
                self.codegen(ctx, rhs, instrs)?;

                let kind = self.get_binary_op_kind(&base_op, &lhs_type, op_loc)?;
                instrs.push(WasmInstr::BinaryOp { kind });
            } else {
                self.codegen(ctx, rhs, instrs)?;
            }

            instrs.push(WasmInstr::Store {
                kind: WasmStoreKind::I32,
                align: 0,
                offset: 0,
            });

            return Ok(());
        }

        if let CodeExpr::Ident(IdentExpr {
            repr,
            parts: _,
            loc,
        }) = lhs
        {
            if let Some(local_index) = ctx.get_local_index(&repr) {
                if let Some(base_op) = base_op {
                    self.codegen(ctx, lhs, instrs)?;
                    self.codegen(ctx, rhs, instrs)?;

                    let kind = self.get_binary_op_kind(&base_op, &lhs_type, op_loc)?;
                    instrs.push(WasmInstr::BinaryOp { kind });
                } else {
                    self.codegen(ctx, rhs, instrs)?;
                }

                for i in 0..self.count_wasm_type_components(&rhs_type) {
                    instrs.push(WasmInstr::LocalSet {
                        local_index: (local_index + i) as u32,
                    });
                }

                return Ok(());
            }

            if let Some(global) = self.get_global(&repr) {
                if let Some(base_op) = base_op {
                    self.codegen(ctx, lhs, instrs)?;
                    self.codegen(ctx, rhs, instrs)?;

                    let kind = self.get_binary_op_kind(&base_op, &lhs_type, op_loc)?;
                    instrs.push(WasmInstr::BinaryOp { kind });
                } else {
                    self.codegen(ctx, rhs, instrs)?;
                }

                for i in 0..self.count_wasm_type_components(&rhs_type) {
                    instrs.push(WasmInstr::GlobalSet {
                        global_index: global.global_index + i as u32,
                    });
                }

                return Ok(());
            }

            return Err(LoError {
                message: format!("Unknown variable: {repr}"),
                loc: loc.clone(),
            });
        }

        todo!();
    }

    // TODO: place ctx before expr
    fn get_expr_type(&self, ctx: &LoExprContext, expr: &CodeExpr) -> Result<LoType, LoError> {
        match expr {
            CodeExpr::BoolLiteral(_) => Ok(LoType::Bool),
            CodeExpr::CharLiteral(_) => todo!(),
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
            CodeExpr::StructLiteral(_) => todo!(),
            CodeExpr::ArrayLiteral(_) => todo!(),
            CodeExpr::Ident(IdentExpr {
                repr,
                parts: _,
                loc,
            }) => {
                if let Some(local_index) = ctx.get_local_index(repr) {
                    let local = &ctx.locals[local_index];
                    return Ok(local.local_type.clone());
                };

                if let Some(global) = self.get_global(repr) {
                    return Ok(global.global_type.clone());
                }

                Err(LoError {
                    message: format!("Unknown variable: {repr}"),
                    loc: loc.clone(),
                })
            }
            CodeExpr::Let(_) => todo!(),
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
                PrefixOpTag::Positive => self.get_expr_type(ctx, expr),
                PrefixOpTag::Negative => self.get_expr_type(ctx, expr),
            },
            CodeExpr::Cast(CastExpr {
                expr: _,
                casted_to,
                loc: _,
            }) => self.build_type(casted_to),
            CodeExpr::Assign(_) => todo!(),
            CodeExpr::FieldAccess(_) => todo!(),
            CodeExpr::PropagateError(_) => todo!(),
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
            CodeExpr::MethodCall(_) => todo!(),
            CodeExpr::MacroFnCall(_) => todo!(),
            CodeExpr::MacroMethodCall(_) => todo!(),
            CodeExpr::Dbg(_) => todo!(),
            CodeExpr::Sizeof(_) => todo!(),
            CodeExpr::GetDataSize(_) => todo!(),
            CodeExpr::Return(_) => todo!(),
            CodeExpr::If(_) => todo!(),
            CodeExpr::Loop(_) => todo!(),
            CodeExpr::Break(_) => todo!(),
            CodeExpr::Unreachable(_) => todo!(),
            CodeExpr::ForLoop(_) => todo!(),
            CodeExpr::Continue(_) => todo!(),
            CodeExpr::Defer(_) => todo!(),
            CodeExpr::Catch(_) => todo!(),
            CodeExpr::Paren(ParenExpr { expr, loc: _ }) => self.get_expr_type(ctx, expr),
        }
    }

    // TODO: add validation for const expr
    fn ensure_const_expr(&self, _expr: &CodeExpr) -> Result<(), LoError> {
        Ok(())
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
            LoType::Result { ok_type, err_type } => {
                self.lower_type(&ok_type, wasm_types);
                self.lower_type(&err_type, wasm_types);
            }
        }
    }

    fn count_wasm_type_components(&self, lo_type: &LoType) -> usize {
        let mut wasm_types = Vec::new();
        self.lower_type(lo_type, &mut wasm_types);
        wasm_types.len()
    }

    fn get_binary_op_kind(
        &self,
        op_tag: &InfixOpTag,
        operand_type: &LoType,
        loc: &LoLocation,
    ) -> Result<WasmBinaryOpKind, LoError> {
        match op_tag {
            InfixOpTag::Equal => match operand_type {
                LoType::U32 => return Ok(WasmBinaryOpKind::I32_EQ),
                _ => {}
            },
            InfixOpTag::NotEqual => todo!(),
            InfixOpTag::Less => match operand_type {
                LoType::U32 => return Ok(WasmBinaryOpKind::I32_LT_U),
                _ => {}
            },
            InfixOpTag::Greater => todo!(),
            InfixOpTag::LessEqual => todo!(),
            InfixOpTag::GreaterEqual => todo!(),
            InfixOpTag::Add => match operand_type {
                LoType::U32 => return Ok(WasmBinaryOpKind::I32_ADD),
                _ => {}
            },
            InfixOpTag::Sub => match operand_type {
                LoType::U32 => return Ok(WasmBinaryOpKind::I32_SUB),
                _ => {}
            },
            InfixOpTag::Mul => match operand_type {
                LoType::U32 => return Ok(WasmBinaryOpKind::I32_MUL),
                _ => {}
            },
            InfixOpTag::Div => todo!(),
            InfixOpTag::Mod => todo!(),
            InfixOpTag::And => todo!(),
            InfixOpTag::BitAnd => todo!(),
            InfixOpTag::Or => todo!(),
            InfixOpTag::BitOr => todo!(),
            InfixOpTag::ShiftLeft => todo!(),
            InfixOpTag::ShiftRight => match operand_type {
                LoType::U32 => return Ok(WasmBinaryOpKind::I32_SHR_U),
                _ => {}
            },
            InfixOpTag::AddAssign => todo!(),
            InfixOpTag::SubAssign => todo!(),
            InfixOpTag::MulAssign => todo!(),
            InfixOpTag::DivAssign => todo!(),
            InfixOpTag::ModAssign => todo!(),
            InfixOpTag::BitAndAssign => todo!(),
            InfixOpTag::BitOrAssign => todo!(),
            InfixOpTag::ShiftLeftAssign => todo!(),
            InfixOpTag::ShiftRightAssign => todo!(),

            // have their own CodeExpr variants
            InfixOpTag::Cast
            | InfixOpTag::Assign
            | InfixOpTag::FieldAccess
            | InfixOpTag::Catch
            | InfixOpTag::ErrorPropagation => unreachable!(),
        }

        return Err(LoError {
            message: format!(
                "Operator {} is not applicable to operands of type {operand_type}",
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
