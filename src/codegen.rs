use crate::{ast::*, common::*, lexer::*, registry::*, typer::*, wasm::*};

struct PooledStr {
    ptr: u32,
    len: u32,
}

struct PooledString {
    value: String,
    ptr: u32,
}

enum VarInfo {
    Local(VarInfoLocal),
    Global(VarInfoGlobal),
    Const(VarInfoConst),
    VoidEnumValue(VarInfoVoidEnumValue),
    Stored(VarInfoStored),
    StructValueField(VarInfoStructValueField),
}
struct VarInfoLocal {
    local_index: u32,
    var_type: Type,
}

struct VarInfoGlobal {
    global_index: u32,
    var_type: Type,
}

struct VarInfoConst {
    const_def: &'static ConstDef,
    loc: Loc,
}

struct VarInfoVoidEnumValue {
    variant_index: usize,
    loc: Loc,
}

struct VarInfoStored {
    address: CodeUnit,
    offset: u32,
    var_type: Type,
}

struct VarInfoStructValueField {
    struct_value: CodeUnit,
    drops_before: u32,
    drops_after: u32,
    var_type: Type,
    loc: Loc,
}

#[derive(Clone)]
struct Symbol {
    name: &'static str,
    col_index: usize,
}

#[derive(Clone, Default)]
struct Scope {
    kind: ScopeKind,
    symbols: Vec<Symbol>,
    deferred_exprs: Vec<CodeUnit>,
    inline_fn_call_loc: Option<Loc>,

    expr_id_offset: usize,
}

impl Scope {
    fn new(scope_type: ScopeKind) -> Self {
        Self {
            kind: scope_type,
            ..Default::default()
        }
    }

    fn get_symbol(&self, symbol_name: &str) -> Option<&Symbol> {
        for symbol in self.symbols.iter().rev() {
            if symbol.name == symbol_name {
                return Some(symbol);
            }
        }

        None
    }
}

struct CodegenModuleInfo {
    scope_stack: Vec<Scope>,
    ctx: ExprContext,
}

#[derive(Default)]
pub struct CodeGenerator {
    // context
    pub registry: UBRef<Registry>,

    // state
    module_info: Vec<CodegenModuleInfo>,
    wasm_fn_types: UBCell<Vec<WasmFnType>>,
    datas: UBCell<Vec<WasmData>>,
    string_pool: UBCell<Vec<PooledString>>,

    // output
    pub wasm_module: UBCell<WasmModule>,
}

impl CodeGenerator {
    pub fn new(registry: &Registry) -> Self {
        let mut it = Self::default();
        it.registry = UBRef::new(&*registry);
        it
    }

    pub fn codegen_all(&mut self) {
        for module in &mut self.registry.modules {
            self.module_info.push(CodegenModuleInfo {
                scope_stack: vec![Scope::new(ScopeKind::Global)],
                ctx: ExprContext::new(module.id, None),
            });
        }

        let mut fn_imports_count = 0;
        for fn_info in &self.registry.functions {
            if let FnSource::Host { .. } = fn_info.source {
                fn_imports_count += 1;
            }
        }

        // resolve wasm fn indicies and populate type, import and export sections
        let mut wasm_import_fn_index = 0;
        let mut wasm_fn_index = fn_imports_count;
        for fn_index in 0..self.registry.functions.len() {
            let fn_info = self.registry.functions[fn_index].relax_mut();

            let mut wasm_fn_type = WasmFnType {
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            for input_type in &fn_info.type_.inputs {
                self.lower_type(self.get_type(*input_type), &mut wasm_fn_type.inputs);
            }
            self.lower_type(&fn_info.type_.output, &mut wasm_fn_type.outputs);

            let mut fn_type_index = self.wasm_fn_types.len() as u32;
            for (existing_fn_type, existing_type_index) in self.wasm_fn_types.iter().zip(0..) {
                if wasm_fn_type == *existing_fn_type {
                    fn_type_index = existing_type_index;
                }
            }
            if fn_type_index == self.wasm_fn_types.len() as u32 {
                self.wasm_fn_types.be_mut().push(wasm_fn_type.clone());
            }

            match &fn_info.source {
                FnSource::Guest { .. } => {
                    self.wasm_module.functions.push(fn_type_index);
                    self.wasm_module.function_names.push(WasmFnNameItem {
                        fn_index: wasm_fn_index,
                        fn_name: String::from(fn_info.name),
                    });

                    fn_info.wasm_fn_index = wasm_fn_index;

                    wasm_fn_index += 1;
                }
                FnSource::Host {
                    module_name,
                    external_fn_name,
                } => {
                    fn_info.wasm_fn_index = wasm_import_fn_index;

                    self.wasm_module.imports.push(WasmImport {
                        module_name: module_name.clone(),
                        item_name: String::from(*external_fn_name),
                        item_desc: WasmImportDesc::Func {
                            type_index: fn_type_index,
                        },
                    });
                    wasm_import_fn_index += 1;
                }
            }

            for export_name in &fn_info.exported_as {
                self.wasm_module.exports.push(WasmExport {
                    export_type: WasmExportType::Func,
                    export_name: export_name.clone(),
                    exported_item_index: fn_info.wasm_fn_index,
                });
            }
        }

        // build global initializers and update global indices
        let mut wasm_global_index = 0;
        for global in self.registry.globals.relax_mut() {
            global.wasm_global_index = wasm_global_index;

            let mut wasm_types = Vec::new();
            self.lower_type(self.get_type(global.type_id), &mut wasm_types);

            let mut instrs = Vec::new();
            self.relax_mut().codegen(
                &mut self.module_info[global.module_id].ctx,
                &mut instrs,
                &global.value,
            );

            for i in 0..wasm_types.len() {
                let wasm_type = &wasm_types[i];
                let instr = &instrs[i];

                let mut initial_value = WasmExpr { instrs: Vec::new() };
                initial_value.instrs.push(instr.clone());

                self.wasm_module.globals.push(WasmGlobal {
                    mutable: true,
                    value_type: wasm_type.clone(),
                    initial_value,
                });
            }

            wasm_global_index += wasm_types.len() as u32;
        }

        // build function codes
        for fn_info in self.registry.functions.relax_mut().iter() {
            let FnSource::Guest {
                module_id,
                lo_fn_index,
                body,
            } = &fn_info.source
            else {
                continue;
            };

            let ctx = &mut ExprContext::new(*module_id, Some(*lo_fn_index));
            let mut wasm_expr = WasmExpr { instrs: Vec::new() };

            let constants_len = self.registry.constants.len();

            self.enter_scope(ctx, ScopeKind::Function);

            for fn_param in &fn_info.params {
                self.define_local(ctx, fn_param.loc, fn_param.param_name, &fn_param.param_type);
            }

            self.codegen_code_block(ctx, &mut wasm_expr.instrs, body, true);

            self.exit_scope(ctx);

            // remove any constants/types created by inline fn calls
            self.registry.constants.truncate(constants_len);

            let mut wasm_locals_flat = Vec::new();
            for (i, local) in ctx.locals.iter().enumerate() {
                let is_fn_param = i < fn_info.params.len();
                if is_fn_param {
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

            self.wasm_module.codes.push(WasmFn {
                locals: wasm_locals,
                expr: wasm_expr,
            });

            // emit debug info for local names
            {
                let mut local_names_item = WasmLocalNameItem {
                    fn_index: fn_info.wasm_fn_index,
                    locals: Vec::new(),
                };

                for local in &ctx.locals {
                    let local_file_id = local.definition_loc.file_id;
                    if local_file_id == 0 {
                        continue;
                    }

                    let mut local_module = &self.registry.modules[ctx.module_id];

                    // local defined by inline fn from another module
                    if local_file_id != local_module.parser.lexer.file_id {
                        let module_id = self.registry.get_module_id_by_file_id(local_file_id);
                        local_module = &self.registry.modules[module_id];
                    }

                    self.push_wasm_dbg_name_section_locals(
                        &mut local_names_item.locals,
                        local.local_index,
                        &local.local_type,
                        String::from(local.definition_loc.read_span(local_module.source)),
                    );
                }

                self.wasm_module.local_names.push(local_names_item);
            }
        }

        // patch @data_size values in globals
        let data_size_instr = WasmInstr::I32Const {
            value: *self.registry.data_size as i32,
        };
        for global in self.registry.globals.relax_mut() {
            let CodeExpr::IntrinsicCall(intrinsic) = &*global.value else {
                continue;
            };

            if intrinsic.fn_name.repr != "data_size" {
                continue;
            }

            self.wasm_module.globals[global.wasm_global_index as usize]
                .initial_value
                .instrs[0] = data_size_instr.clone()
        }

        if let Some(memory) = &self.registry.memory {
            let limits = WasmLimits {
                min: memory.min_pages.unwrap_or(0),
                max: None,
            };

            if let Some(module_name) = &memory.imported_from {
                self.wasm_module.imports.push(WasmImport {
                    module_name: module_name.clone(),
                    item_name: String::from("memory"),
                    item_desc: WasmImportDesc::Memory(limits),
                });
            } else {
                self.wasm_module.memories.push(limits);
            }

            if memory.exported {
                self.wasm_module.exports.push(WasmExport {
                    export_type: WasmExportType::Mem,
                    export_name: String::from("memory"),
                    exported_item_index: 0,
                });
            }
        }

        for static_data_store in &*self.datas {
            self.wasm_module.datas.push(static_data_store.clone());
        }

        self.wasm_module.types.append(self.wasm_fn_types.be_mut());
    }

    fn codegen_code_block(
        &mut self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        body: &CodeBlock,
        void_only: bool,
    ) {
        let mut diverges = false;
        let mut diverges_naturally = false;

        for expr in &body.exprs {
            if let Type::Never = self.get_expr_type(ctx, expr) {
                diverges = true;
                diverges_naturally = diverges_naturally || is_naturally_divergent(expr);
            }

            self.codegen(ctx, instrs, expr);
        }

        self.emit_deferred(self.current_scope(ctx), instrs);

        // TODO: move this decision to Typer, only emit if needed here
        if void_only && diverges && !diverges_naturally {
            instrs.push(WasmInstr::Unreachable);
        }
    }

    fn codegen(&mut self, ctx: &mut ExprContext, instrs: &mut Vec<WasmInstr>, expr: &CodeExpr) {
        match expr {
            CodeExpr::BoolLiteral(BoolLiteralExpr {
                id: _,
                value,
                loc: _,
            }) => {
                if *value {
                    instrs.push(WasmInstr::I32Const { value: 1 });
                } else {
                    instrs.push(WasmInstr::I32Const { value: 0 });
                }
            }
            CodeExpr::CharLiteral(CharLiteralExpr {
                id: _,
                repr: _,
                value,
                loc: _,
            }) => {
                instrs.push(WasmInstr::I32Const {
                    value: *value as i32,
                });
            }
            CodeExpr::NullLiteral(NullLiteralExpr { id: _, loc: _ }) => {
                instrs.push(WasmInstr::I32Const { value: 0 });
            }
            CodeExpr::IntLiteral(int) => {
                if is_wide_int(self.get_expr_type(ctx, expr)).unwrap() {
                    instrs.push(WasmInstr::I64Const {
                        value: int.value as i64,
                    });
                } else {
                    instrs.push(WasmInstr::I32Const {
                        value: int.value as i32,
                    });
                }
            }
            CodeExpr::StringLiteral(str_literal) => {
                let str = self.process_const_string(&str_literal.value);

                // emit str struct values
                instrs.push(WasmInstr::I32Const {
                    value: str.ptr as i32,
                });
                instrs.push(WasmInstr::I32Const {
                    value: str.len as i32,
                });
            }
            CodeExpr::StructLiteral(struct_literal) => {
                for field in &struct_literal.body.fields {
                    self.codegen(ctx, instrs, &field.value);
                }
            }
            // TODO?: support sequences of any type
            CodeExpr::ArrayLiteral(ArrayLiteralExpr {
                id: _,
                item_type,
                items,
                has_trailing_comma: _,
                loc,
            }) => {
                let item_type = self.get_type_expr_value(ctx, item_type);

                let mut bytes = Vec::new();
                let mut tmp_instrs = Vec::new();

                if let Type::U8 = item_type {
                    for item in items {
                        let current_item_type = self.get_expr_type(ctx, item);
                        if current_item_type != item_type {
                            // TODO!: move to typer
                            self.registry.reporter.abort_due_to_compiler_bug(
                                &format!(
                                    "Unexpected array element type: {}, expected: {}",
                                    self.registry.fmt(current_item_type),
                                    self.registry.fmt(item_type),
                                ),
                                item.loc(),
                            );
                        }

                        self.codegen(ctx, &mut tmp_instrs, item);
                        let WasmInstr::I32Const { value } = tmp_instrs.pop().unwrap() else {
                            // TODO!: move to typer
                            self.registry.reporter.abort_due_to_compiler_bug(
                                &format!("Unexpected array element value"),
                                item.loc(),
                            );
                        };

                        bytes.push(value as u8);
                    }
                } else if item_type == self.registry.str_literal_type.as_ref().unwrap() {
                    for item in items {
                        let current_item_type = self.get_expr_type(ctx, item);
                        if current_item_type != item_type {
                            // TODO!: move to typer
                            self.registry.reporter.abort_due_to_compiler_bug(
                                &format!(
                                    "Unexpected array element type: {}, expected: {}",
                                    self.registry.fmt(current_item_type),
                                    self.registry.fmt(item_type),
                                ),
                                item.loc(),
                            );
                        }

                        self.codegen(ctx, &mut tmp_instrs, item);
                        let WasmInstr::I32Const { value: len } = tmp_instrs.pop().unwrap() else {
                            // TODO!: move to typer
                            self.registry.reporter.abort_due_to_compiler_bug(
                                &format!("Unexpected array element value"),
                                item.loc(),
                            );
                        };
                        let WasmInstr::I32Const { value: ptr } = tmp_instrs.pop().unwrap() else {
                            // TODO!: move to typer
                            self.registry.reporter.abort_due_to_compiler_bug(
                                &format!("Unexpected array element value"),
                                item.loc(),
                            );
                        };

                        bytes.extend_from_slice(&ptr.to_le_bytes());
                        bytes.extend_from_slice(&len.to_le_bytes());
                    }
                } else {
                    // TODO!: move to typer
                    self.registry.reporter.abort_due_to_compiler_bug(
                        &format!(
                            "Unsupported array literal element type: {}",
                            self.registry.fmt(item_type)
                        ),
                        *loc,
                    );
                }

                let ptr = self.append_data(bytes);
                instrs.push(WasmInstr::I32Const { value: ptr as i32 });
                instrs.push(WasmInstr::I32Const {
                    value: items.len() as i32,
                });
            }
            CodeExpr::ResultLiteral(ResultLiteralExpr {
                id: _,
                is_ok,
                result_type: _,
                value,
                loc: _,
            }) => {
                let Type::Result(result) = self.get_expr_type(ctx, expr) else {
                    unreachable!()
                };

                if *is_ok {
                    if let Some(ok_value) = value {
                        self.codegen(ctx, instrs, ok_value);
                    }

                    // error value
                    instrs.push(WasmInstr::I32Const { value: 0 });

                    return;
                }

                self.codegen_default_value(ctx, instrs, self.get_type(result.ok));
                self.codegen(ctx, instrs, value.as_ref().unwrap());
            }

            CodeExpr::Ident(ident) => {
                let var = self.var_from_ident(ctx, ident);
                self.codegen_var_get(ctx, instrs, &var);
            }
            CodeExpr::Let(LetExpr {
                id: _,
                is_inline,
                name,
                value,
                loc: _,
            }) => {
                if *is_inline {
                    // already handled in typer
                    return;
                }

                let var_type = self.get_expr_type(ctx, &value);

                if name.repr == "_" {
                    self.codegen(ctx, instrs, value);

                    for _ in 0..count_primitive_components(&self.registry, &var_type) {
                        instrs.push(WasmInstr::Drop);
                    }
                    return;
                }

                let local_index = self.define_local(ctx, name.loc, name.repr, &var_type);
                let var = VarInfo::Local(VarInfoLocal {
                    local_index,
                    var_type: var_type.clone(),
                });
                self.codegen_var_set_prepare(instrs, &var);
                self.codegen(ctx, instrs, value);
                self.codegen_var_set(ctx, instrs, &var);
            }
            CodeExpr::Cast(CastExpr {
                id: _,
                expr,
                casted_to,
                loc: _,
            }) => {
                self.codegen(ctx, instrs, expr);

                let castee_type = self.get_expr_type(ctx, expr);
                let casted_to_type = self.get_type_expr_value(ctx, casted_to);

                if let Some(cast_op) = get_cast_instr(castee_type, &casted_to_type) {
                    instrs.push(cast_op);
                }
            }
            CodeExpr::PrefixOp(PrefixOpExpr {
                id: _,
                op_tag,
                expr,
                op_loc: _,
                loc,
            }) => match op_tag {
                PrefixOpTag::Reference => {
                    let Some(VarInfo::Stored(VarInfoStored {
                        mut address,
                        offset,
                        var_type: _,
                    })) = self.var_from_expr(ctx, expr)
                    else {
                        // TODO!: move to typer
                        self.registry.reporter.abort_due_to_compiler_bug(
                            &format!(
                                "Invalid reference expression. Only struct reference fields allowed.",
                            ),
                            *loc,
                        );
                    };

                    instrs.append(&mut address.instrs);
                    instrs.push(WasmInstr::I32Const {
                        value: offset as i32,
                    });
                    instrs.push(WasmInstr::BinaryOp {
                        kind: WasmBinaryOpKind::I32_ADD,
                    });
                }
                PrefixOpTag::Dereference => {
                    let var = self.var_from_deref(ctx, expr);
                    self.codegen_var_get(ctx, instrs, &var);
                }
                PrefixOpTag::Not => {
                    self.codegen(ctx, instrs, expr);

                    let operand_type = self.get_expr_type(ctx, expr);
                    let mut wasm_components = Vec::new();
                    self.lower_type(&operand_type, &mut wasm_components);

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
                    }
                }
                PrefixOpTag::Positive => {
                    self.codegen(ctx, instrs, expr);
                }
                PrefixOpTag::Negative => {
                    let operand_type = self.get_expr_type(ctx, expr);

                    if let CodeExpr::IntLiteral(int) = expr.as_ref() {
                        if is_wide_int(operand_type).unwrap() {
                            instrs.push(WasmInstr::I64Const {
                                value: -(int.value as i64),
                            });
                        } else {
                            instrs.push(WasmInstr::I32Const {
                                value: -(int.value as i32),
                            });
                        }
                        return;
                    };

                    let mut wasm_components = Vec::new();
                    self.lower_type(&operand_type, &mut wasm_components);
                    match wasm_components[0] {
                        WasmType::I32 => {
                            instrs.push(WasmInstr::I32Const { value: 0 });
                            self.codegen(ctx, instrs, expr);
                            instrs.push(WasmInstr::BinaryOp {
                                kind: WasmBinaryOpKind::I32_SUB,
                            });
                        }
                        WasmType::I64 => {
                            instrs.push(WasmInstr::I64Const { value: 0 });
                            self.codegen(ctx, instrs, expr);
                            instrs.push(WasmInstr::BinaryOp {
                                kind: WasmBinaryOpKind::I64_SUB,
                            });
                        }
                        WasmType::F32 => {
                            self.codegen(ctx, instrs, expr);
                            instrs.push(WasmInstr::UnaryOp {
                                kind: WasmUnaryOpKind::F32_NEG,
                            });
                        }
                        WasmType::F64 => {
                            self.codegen(ctx, instrs, expr);
                            instrs.push(WasmInstr::UnaryOp {
                                kind: WasmUnaryOpKind::F64_NEG,
                            });
                        }
                    }
                }
            },
            CodeExpr::InfixOp(InfixOpExpr {
                id: _,
                op_tag,
                op_loc,
                lhs,
                rhs,
                loc: _,
            }) => {
                let operand_type = self.get_expr_type(ctx, lhs);

                if let Some(base_op) = self.get_compound_assignment_base_op(op_tag) {
                    let Some(var) = self.var_from_expr(ctx, &lhs) else {
                        // TODO!: move to typer
                        self.registry.reporter.abort_due_to_compiler_bug(
                            &format!("Cannot perform compound assignment: invalid lhs"),
                            *op_loc,
                        );
                    };

                    self.codegen_var_set_prepare(instrs, &var);
                    self.codegen_var_get(ctx, instrs, &var);
                    self.codegen(ctx, instrs, rhs);
                    self.codegen_binary_op(ctx, instrs, &base_op, &operand_type, op_loc);

                    self.codegen_var_set(ctx, instrs, &var);
                    return;
                }

                self.codegen(ctx, instrs, lhs);
                self.codegen(ctx, instrs, rhs);
                self.codegen_binary_op(ctx, instrs, &op_tag, &operand_type, op_loc);
            }

            CodeExpr::Assign(AssignExpr {
                id: _,
                op_loc,
                lhs,
                rhs,
                loc: _,
            }) => {
                let Some(var) = self.var_from_expr(ctx, lhs) else {
                    // TODO!: move to typer
                    self.registry.reporter.abort_due_to_compiler_bug(
                        &format!("Cannot perform assignment: invalid lhs"),
                        *op_loc,
                    );
                };

                self.codegen_var_set_prepare(instrs, &var);
                self.codegen(ctx, instrs, rhs);
                self.codegen_var_set(ctx, instrs, &var);
            }
            CodeExpr::FieldAccess(field_access) => {
                let var = self.var_from_field_access(ctx, field_access);
                self.codegen_var_get(ctx, instrs, &var);
            }

            CodeExpr::FnCall(FnCallExpr {
                id: call_expr_id,
                fn_name: _,
                args,
                loc,
            }) => {
                let call_info_id = self.get_expr_info(ctx, *call_expr_id, *loc);
                let call_info = &self.registry.call_info[call_info_id];

                match call_info.value {
                    CallInfoValue::EnumCtor(variant_index) => {
                        instrs.push(WasmInstr::I32Const {
                            value: variant_index,
                        });

                        self.codegen(ctx, instrs, &args.items[0]);
                    }
                    CallInfoValue::FnCall(fn_index) => {
                        self.codegen_fn_call(ctx, instrs, fn_index, None, &args.items);
                    }
                }
            }
            CodeExpr::MethodCall(MethodCallExpr {
                id: call_expr_id,
                lhs,
                field_name: _,
                args,
                loc,
            }) => {
                let call_info_id = self.get_expr_info(ctx, *call_expr_id, *loc);
                let call_info = &self.registry.call_info[call_info_id];

                match call_info.value {
                    CallInfoValue::EnumCtor(_) => unreachable!(),
                    CallInfoValue::FnCall(fn_index) => {
                        self.codegen_fn_call(ctx, instrs, fn_index, Some(lhs), &args.items);
                    }
                }
            }
            CodeExpr::InlineFnCall(call) => {
                self.codegen_inline_fn_call(ctx, instrs, call.id, &call.fn_name.loc);
            }
            CodeExpr::InlineMethodCall(call) => {
                self.codegen_inline_fn_call(ctx, instrs, call.id, &call.field_name.loc);
            }
            CodeExpr::IntrinsicCall(call) => {
                if call.fn_name.repr == "inspect_symbols" {
                    // processed in Typer
                    return;
                }

                if call.fn_name.repr == "unreachable" {
                    instrs.push(WasmInstr::Unreachable);
                    return;
                }

                if call.fn_name.repr == "memory_size" {
                    instrs.push(WasmInstr::MemorySize);
                    return;
                }

                if call.fn_name.repr == "memory_grow" {
                    let mut arg_types = Vec::new();
                    for arg in &call.args.items {
                        arg_types.push(self.get_expr_type(ctx, arg).clone());
                    }

                    for arg in &call.args.items {
                        self.codegen(ctx, instrs, arg);
                    }

                    instrs.push(WasmInstr::MemoryGrow);
                    return;
                }

                if call.fn_name.repr == "memory_copy" {
                    for arg in &call.args.items {
                        self.codegen(ctx, instrs, arg);
                    }

                    instrs.push(WasmInstr::MemoryCopy);
                    return;
                }

                if call.fn_name.repr == "data_size" {
                    // placeholder, filled in in `generate`
                    instrs.push(WasmInstr::I32Const { value: 0 });
                    return;
                }

                if call.fn_name.repr == "embed_file" {
                    let CodeExpr::StringLiteral(str_expr) = &call.args.items[0] else {
                        unreachable!()
                    };

                    let absolute_path = self
                        .registry
                        .resolve_path(&str_expr.value, &call.fn_name.loc);
                    let bytes = match fs::file_read(&absolute_path) {
                        Ok(value) => value,
                        // TODO!: move to typer?
                        Err(err_message) => self
                            .registry
                            .reporter
                            .abort_due_to_compiler_bug(&err_message, call.args.items[0].loc()),
                    };

                    let bytes_len = bytes.len();
                    let bytes_ptr = self.append_data(bytes);

                    instrs.push(WasmInstr::I32Const {
                        value: bytes_ptr as i32,
                    });
                    instrs.push(WasmInstr::I32Const {
                        value: bytes_len as i32,
                    });

                    return;
                }

                if call.fn_name.repr == "inline_fn_call_loc" {
                    let mut inline_fn_call_loc = None;
                    // NOTE: iterating in not-reverse to get the first inline scope
                    for scope in &self.module_info[ctx.module_id].scope_stack {
                        if scope.kind == ScopeKind::InlineFn {
                            inline_fn_call_loc = scope.inline_fn_call_loc.clone();
                        }
                    }

                    let loc_str = self.process_const_string(
                        &inline_fn_call_loc.unwrap().to_string(&self.registry),
                    );
                    // emit str struct values
                    instrs.push(WasmInstr::I32Const {
                        value: loc_str.ptr as i32,
                    });
                    instrs.push(WasmInstr::I32Const {
                        value: loc_str.len as i32,
                    });

                    return;
                }

                self.registry.reporter.abort_due_to_compiler_bug(
                    &format!("Unknown intrinsic: {}", call.fn_name.repr),
                    call.fn_name.loc,
                );
            }
            CodeExpr::Sizeof(SizeofExpr {
                id: _,
                type_expr,
                loc: _,
            }) => {
                let type_ = self.get_type_expr_value(ctx, type_expr);
                let mut type_layout = TypeLayout::new();
                get_type_layout(&self.registry, &type_, &mut type_layout);

                instrs.push(WasmInstr::I32Const {
                    value: type_layout.byte_size as i32,
                });
            }

            CodeExpr::Paren(ParenExpr {
                id: _,
                expr,
                has_trailing_comma: _,
                loc: _,
            }) => {
                self.codegen(ctx, instrs, expr);
            }
            CodeExpr::Return(ReturnExpr {
                id: _,
                expr,
                loc: _,
            }) => {
                if let Some(return_expr) = expr {
                    self.codegen(ctx, instrs, return_expr);
                };

                self.emit_deferred_for_return(ctx, instrs);
                instrs.push(WasmInstr::Return);
            }
            CodeExpr::If(IfExpr {
                id: _,
                cond,
                then_block,
                else_block,
                loc: _,
            }) => {
                match cond {
                    IfCond::Expr(expr) => {
                        self.codegen(ctx, instrs, expr);

                        // `if` condition runs outside of then_branch's scope
                        self.enter_scope(ctx, ScopeKind::Block);
                    }
                    IfCond::Match(match_header) => {
                        // `if match` condition runs inside of then_branch's scope
                        self.enter_scope(ctx, ScopeKind::Block);

                        let enum_ctor = self.codegen_match_header(ctx, instrs, match_header);

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
                self.exit_scope(ctx);

                match else_block {
                    ElseBlock::None => {}
                    ElseBlock::Else(code_block_expr) => {
                        instrs.push(WasmInstr::Else);
                        self.enter_scope(ctx, ScopeKind::Block);
                        self.codegen_code_block(ctx, instrs, &code_block_expr, true);
                        self.exit_scope(ctx);
                    }
                    ElseBlock::ElseIf(code_expr) => {
                        instrs.push(WasmInstr::Else);
                        self.enter_scope(ctx, ScopeKind::Block);
                        self.codegen(ctx, instrs, &code_expr);
                        self.exit_scope(ctx);
                    }
                }

                instrs.push(WasmInstr::BlockEnd);
            }
            CodeExpr::Match(MatchExpr {
                id: _,
                header,
                else_branch,
                loc: _,
            }) => {
                let enum_ctor = self.codegen_match_header(ctx, instrs, header);

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

                self.enter_scope(ctx, ScopeKind::Block);
                self.codegen_code_block(ctx, instrs, &else_branch, true);
                self.exit_scope(ctx);
                instrs.push(WasmInstr::BlockEnd);
            }
            CodeExpr::While(WhileExpr {
                id: _,
                cond,
                body,
                loc: _,
            }) => {
                instrs.push(WasmInstr::BlockStart {
                    block_kind: WasmBlockKind::Block,
                    block_type: WasmBlockType::NoOut,
                });
                instrs.push(WasmInstr::BlockStart {
                    block_kind: WasmBlockKind::Loop,
                    block_type: WasmBlockType::NoOut,
                });

                if let Some(cond) = cond {
                    self.codegen(ctx, instrs, cond);

                    instrs.push(WasmInstr::UnaryOp {
                        kind: WasmUnaryOpKind::I32_EQZ,
                    });
                    instrs.push(WasmInstr::BranchIf { label_index: 1 });
                }

                self.enter_scope(ctx, ScopeKind::Loop);
                self.codegen_code_block(ctx, instrs, body, true);
                self.exit_scope(ctx);

                // implicit continue
                instrs.push(WasmInstr::Branch { label_index: 0 });

                instrs.push(WasmInstr::BlockEnd);
                instrs.push(WasmInstr::BlockEnd);
            }
            CodeExpr::For(ForExpr {
                id: _,
                counter,
                start,
                end,
                body,
                op_loc,
                loc: _,
            }) => {
                let counter_type = self.get_expr_type(ctx, start);

                self.enter_scope(ctx, ScopeKind::ForLoop);

                // define counter and set value to start
                let local_index = self.define_local(ctx, counter.loc, counter.repr, &counter_type);
                let counter_var = VarInfo::Local(VarInfoLocal {
                    local_index,
                    var_type: counter_type.clone(),
                });
                self.codegen_var_set_prepare(instrs, &counter_var);
                self.codegen(ctx, instrs, start);
                self.codegen_var_set(ctx, instrs, &counter_var);

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
                        self.codegen(ctx, instrs, end);
                        self.codegen_var_get(ctx, instrs, &counter_var);
                        self.codegen_binary_op(
                            ctx,
                            instrs,
                            &InfixOpTag::Equal,
                            &counter_type,
                            op_loc,
                        );
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
                        self.codegen_var_get(ctx, instrs, &counter_var);
                        self.codegen_var_set_prepare(instrs, &counter_var);
                        if is_wide_int(&counter_type).unwrap() {
                            instrs.push(WasmInstr::I64Const { value: 1 });
                        } else {
                            instrs.push(WasmInstr::I32Const { value: 1 });
                        }
                        self.codegen_binary_op(
                            ctx,
                            instrs,
                            &InfixOpTag::Add,
                            &counter_type,
                            op_loc,
                        );
                        self.codegen_var_set(ctx, instrs, &counter_var);

                        // implicit continue
                        instrs.push(WasmInstr::Branch { label_index: 0 });

                        instrs.push(WasmInstr::BlockEnd);
                    }

                    instrs.push(WasmInstr::BlockEnd);
                }

                self.exit_scope(ctx);
            }
            CodeExpr::Break(BreakExpr { id: _, loc }) => {
                let mut label_index = 1; // 0 = loop, 1 = loop wrapper block

                for scope in self.module_info[ctx.module_id].scope_stack.iter().rev() {
                    match scope.kind {
                        ScopeKind::Block => {
                            label_index += 1;
                        }
                        ScopeKind::Function => {
                            // TODO!: move to typer
                            self.registry.reporter.abort_due_to_compiler_bug(
                                &format!("Cannot break outside of a loop"),
                                *loc,
                            );
                        }
                        ScopeKind::Loop => break,
                        ScopeKind::ForLoop => {
                            label_index += 1;
                            break;
                        }
                        ScopeKind::InlineFn => continue,
                        ScopeKind::Global => unreachable!(),
                    }
                }

                instrs.push(WasmInstr::Branch { label_index });
            }
            CodeExpr::Continue(ContinueExpr { id: _, loc }) => {
                let mut label_index = 0; // 0 = loop, 1 = loop wrapper block

                for scope in self.module_info[ctx.module_id].scope_stack.iter().rev() {
                    match scope.kind {
                        ScopeKind::Block => {
                            label_index += 1;
                        }
                        ScopeKind::Function => {
                            // TODO!: move to typer
                            self.registry.reporter.abort_due_to_compiler_bug(
                                &format!("Cannot continue outside of a loop"),
                                *loc,
                            );
                        }
                        ScopeKind::Loop => break,
                        ScopeKind::ForLoop => break,
                        ScopeKind::InlineFn => continue,
                        ScopeKind::Global => unreachable!(),
                    }
                }

                instrs.push(WasmInstr::Branch { label_index });
            }
            CodeExpr::DoWith(DoWithExpr {
                id: _,
                args,
                body,
                with_loc,
                loc: _,
            }) => {
                let arg_type = self.get_expr_type(ctx, args.items.first().unwrap());

                for arg in &args.items {
                    self.enter_scope(ctx, ScopeKind::InlineFn);

                    self.codegen(ctx, instrs, arg);

                    let arg_local_index = self.define_local(ctx, with_loc.clone(), "it", &arg_type);

                    self.codegen_local_set(instrs, &arg_type, arg_local_index);
                    self.codegen(ctx, instrs, body);

                    self.exit_scope(ctx);
                }
            }
            CodeExpr::Pipe(PipeExpr {
                id: _,
                lhs,
                rhs,
                op_loc,
                loc: _,
            }) => {
                let lhs_type = self.get_expr_type(ctx, lhs);
                self.codegen(ctx, instrs, lhs);

                self.enter_scope(ctx, ScopeKind::InlineFn);

                let lhs_local_index = self.define_local(ctx, op_loc.clone(), "it", &lhs_type);

                self.codegen_local_set(instrs, &lhs_type, lhs_local_index);
                self.codegen(ctx, instrs, rhs);
                self.exit_scope(ctx);
            }
            CodeExpr::Defer(DeferExpr {
                id: _,
                expr,
                loc: _,
            }) => {
                let code_unit = self.build_code_unit(ctx, expr);

                // find first non-inline-fn scope
                let mut scope_to_defer = self.module_info[ctx.module_id]
                    .scope_stack
                    .relax_mut()
                    .last_mut()
                    .unwrap();
                for scope in self.module_info[ctx.module_id].scope_stack.iter_mut().rev() {
                    if scope.kind != ScopeKind::InlineFn {
                        scope_to_defer = scope;
                        break;
                    }
                }

                scope_to_defer.deferred_exprs.push(code_unit);
            }
            CodeExpr::Catch(CatchExpr {
                id: _,
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
                );
            }
            CodeExpr::PropagateError(PropagateErrorExpr { id: _, expr, loc }) => {
                self.codegen_catch(ctx, instrs, expr, None, None, loc);
            }
        };
    }

    /// defines a local with match bind's name and pushes enum's variant to the stack
    fn codegen_match_header(
        &mut self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        header: &Box<MatchHeader>,
    ) -> &EnumConstructor {
        let expr_info = self.get_expr_info(ctx, header.variant_bind.id, header.variant_bind.loc);
        let value_info = &self.registry.value_info[expr_info];
        let enum_ctor = &self.registry.enum_ctors[value_info.col_index].relax();
        let enum_def = &self.registry.enums[enum_ctor.enum_index].relax();
        let enum_variant = &enum_def.variants[enum_ctor.variant_index].relax();

        let local_index = self.define_local(
            ctx,
            header.variant_bind.loc,
            header.variant_bind.repr,
            self.get_type(enum_variant.variant_type_id),
        );
        let local = VarInfo::Local(VarInfoLocal {
            local_index,
            var_type: self.get_type(enum_variant.variant_type_id).clone(),
        });

        self.codegen_var_set_prepare(instrs, &local);
        self.codegen(ctx, instrs, &header.expr_to_match);
        self.codegen_var_set(ctx, instrs, &local);

        enum_ctor
    }

    fn codegen_fn_call(
        &mut self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        fn_index: usize,
        receiver_arg: Option<&CodeExpr>,
        args: &Vec<CodeExpr>,
    ) {
        let fn_info = self.registry.functions[fn_index].relax();

        if let Some(receiver_arg) = receiver_arg {
            self.codegen(ctx, instrs, receiver_arg);
        }
        for arg in args {
            self.codegen(ctx, instrs, arg);
        }

        instrs.push(WasmInstr::Call {
            fn_index: fn_info.wasm_fn_index,
        });
    }

    fn codegen_inline_fn_call(
        &mut self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        call_expr_id: ExprId,
        loc: &Loc,
    ) {
        let call_info_id = self.get_expr_info(ctx, call_expr_id, *loc);
        let call_info = self.registry.inline_call_info[call_info_id].relax();
        let inline_fn_def = self.registry.inline_fns[call_info.inline_fn_index];
        let FnExprValue::Body(body) = &inline_fn_def.value else {
            unreachable!()
        };

        self.enter_scope(ctx, ScopeKind::InlineFn);
        self.current_scope(ctx).be_mut().inline_fn_call_loc = Some(*loc);

        self.current_scope(ctx).be_mut().expr_id_offset = call_info.inner_expr_id_offset;

        self.codegen_code_block(ctx, instrs, body, false);

        self.exit_scope(ctx);
    }

    fn codegen_catch(
        &mut self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        expr: &CodeExpr,
        error_bind: Option<&IdentExpr>,
        catch_body: Option<&CodeBlock>,
        loc: &Loc,
    ) {
        let expr_type = self.get_expr_type(ctx, expr);
        let Type::Result(result) = expr_type else {
            unreachable!()
        };
        let ok = self.get_type(result.ok);
        let err = self.get_type(result.err);

        self.enter_scope(ctx, ScopeKind::Block); // enter catch scope

        // put result on the stack
        self.codegen(ctx, instrs, expr);

        // pop error
        let (error_bind, error_bind_loc) = if let Some(error_bind) = error_bind {
            (error_bind.repr, error_bind.loc)
        } else {
            ("<err>", Loc::internal())
        };
        let err_local_index = self.define_local(ctx, error_bind_loc.clone(), error_bind, err);
        let err_var = VarInfo::Local(VarInfoLocal {
            local_index: err_local_index,
            var_type: err.clone(),
        });
        self.codegen_var_set_prepare(instrs, &err_var);
        self.codegen_var_set(ctx, instrs, &err_var);

        // pop ok
        let ok_local_index = self.define_local(ctx, *loc, "<ok>", ok);
        self.codegen_local_set(instrs, ok, ok_local_index);

        // cond: error != 0
        self.codegen_var_get(ctx, instrs, &err_var);

        let in_out_type_index = self.get_block_inout_type(&[], ok);
        instrs.push(WasmInstr::BlockStart {
            block_kind: WasmBlockKind::If,
            block_type: WasmBlockType::InOut {
                type_index: in_out_type_index,
            },
        });

        // catch error
        if let Some(catch_body) = catch_body {
            self.codegen_code_block(ctx, instrs, catch_body, true);
        } else {
            // return default ok_type of function's result and caught error
            let fn_def = &self.registry.functions[ctx.fn_index.unwrap()];
            let Type::Result(fn_result) = &fn_def.type_.output else {
                unreachable!()
            };
            self.codegen_default_value(ctx, instrs, self.get_type(fn_result.ok));
            self.codegen_var_get(ctx, instrs, &err_var);

            self.emit_deferred_for_return(ctx, instrs);
            instrs.push(WasmInstr::Return);
        }

        instrs.push(WasmInstr::Else);

        // no error, push ok value
        let ok_var = VarInfo::Local(VarInfoLocal {
            local_index: ok_local_index,
            var_type: ok.clone(),
        });
        self.codegen_var_get(ctx, instrs, &ok_var);

        instrs.push(WasmInstr::BlockEnd);

        self.exit_scope(ctx); // exit catch scope
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
            Type::U32 | Type::I32 | Type::Null | Type::Pointer { .. } => {
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
            Type::Struct { struct_index } => {
                let struct_def = &self.registry.structs[*struct_index];

                for struct_field in struct_def.fields.iter().rev() {
                    self.codegen_load_or_store(
                        instrs,
                        &struct_field.field_type,
                        offset + struct_field.byte_offset,
                        is_store,
                    );
                }
            }
            Type::Enum { enum_index } => {
                let enum_def = &self.registry.enums[*enum_index];

                self.codegen_load_or_store(instrs, &enum_def.variant_type, offset + 4, is_store);
                self.codegen_load_or_store(instrs, &Type::U32, offset, is_store);
            }
            Type::Seg(seg) => {
                let ptr_type = Type::Pointer(PointerType {
                    pointee: seg.item,
                    is_sequence: true,
                    is_nullable: false,
                });
                self.codegen_load_or_store(instrs, &Type::U32, offset + 4, is_store);
                self.codegen_load_or_store(instrs, &ptr_type, offset, is_store);
            }
            Type::Result(result) => {
                let ok = self.get_type(result.ok);
                let err = self.get_type(result.err);

                let mut ok_layout = TypeLayout::new();
                get_type_layout(&self.registry, ok, &mut ok_layout);

                self.codegen_load_or_store(instrs, err, offset + ok_layout.byte_size, is_store);
                self.codegen_load_or_store(instrs, ok, offset, is_store);
            }
            Type::Container(ctr) => {
                self.codegen_load_or_store(instrs, self.get_type(ctr.container), offset, is_store);
            }
        }
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

        for (fn_type, type_index) in self.wasm_fn_types.iter().zip(0..) {
            if *fn_type == inout_fn_type {
                return type_index;
            }
        }

        self.wasm_fn_types.be_mut().push(inout_fn_type);
        self.wasm_fn_types.len() as u32 - 1
    }

    fn codegen_var_get(
        &mut self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        var: &VarInfo,
    ) {
        match var {
            VarInfo::Local(VarInfoLocal {
                local_index,
                var_type,
            }) => {
                for i in 0..count_primitive_components(&self.registry, var_type) {
                    instrs.push(WasmInstr::LocalGet {
                        local_index: local_index + i,
                    });
                }
            }
            VarInfo::Global(VarInfoGlobal {
                global_index,
                var_type,
            }) => {
                for i in 0..count_primitive_components(&self.registry, var_type) {
                    instrs.push(WasmInstr::GlobalGet {
                        global_index: global_index + i,
                    });
                }
            }
            VarInfo::Const(VarInfoConst { const_def, loc: _ }) => {
                self.enter_scope(ctx, ScopeKind::InlineFn);
                self.current_scope(ctx).be_mut().expr_id_offset = const_def.inner_expr_id_offset;

                self.codegen(ctx, instrs, &const_def.expr);

                self.exit_scope(ctx);
            }
            VarInfo::VoidEnumValue(VarInfoVoidEnumValue { variant_index, .. }) => {
                instrs.push(WasmInstr::I32Const {
                    value: *variant_index as i32,
                })
            }
            VarInfo::Stored(VarInfoStored {
                address,
                offset,
                var_type,
            }) => {
                let mut loads = Vec::new();
                self.codegen_load_or_store(&mut loads, &var_type, *offset, false);

                if loads.len() == 0 {
                    return;
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
            VarInfo::StructValueField(VarInfoStructValueField {
                struct_value,
                drops_before,
                drops_after,
                var_type,
                loc,
            }) => {
                for instr in &struct_value.instrs {
                    instrs.push(instr.clone());
                }
                for _ in 0..*drops_before {
                    instrs.push(WasmInstr::Drop);
                }

                if *drops_after > 0 {
                    let local_index = self.define_unnamed_local(ctx, *loc, var_type);

                    let var = VarInfo::Local(VarInfoLocal {
                        local_index,
                        var_type: var_type.clone(),
                    });
                    self.codegen_var_set_prepare(instrs, &var);
                    self.codegen_var_set(ctx, instrs, &var);

                    for _ in 0..*drops_after {
                        instrs.push(WasmInstr::Drop);
                    }

                    self.codegen_var_get(ctx, instrs, &var);
                }
            }
        }
    }

    // should be called before set's value is pushed to the stack
    fn codegen_var_set_prepare(&self, instrs: &mut Vec<WasmInstr>, var: &VarInfo) {
        match var {
            VarInfo::Stored(VarInfoStored {
                address,
                offset: _,
                var_type,
            }) => {
                if count_primitive_components(&self.registry, var_type) == 0 {
                    return;
                }

                for instr in &address.instrs {
                    instrs.push(instr.clone());
                }
            }
            _ => {}
        };
    }

    fn codegen_var_set(&self, ctx: &mut ExprContext, instrs: &mut Vec<WasmInstr>, var: &VarInfo) {
        match var {
            VarInfo::Local(VarInfoLocal {
                local_index,
                var_type,
            }) => {
                self.codegen_local_set(instrs, var_type, *local_index);
            }
            VarInfo::Global(VarInfoGlobal {
                global_index,
                var_type,
            }) => {
                for i in (0..count_primitive_components(&self.registry, var_type)).rev() {
                    instrs.push(WasmInstr::GlobalSet {
                        global_index: global_index + i,
                    });
                }
            }
            VarInfo::Stored(VarInfoStored {
                address: _,
                offset,
                var_type,
            }) => {
                let mut stores = Vec::new();
                self.codegen_load_or_store(&mut stores, &var_type, *offset, true);

                if stores.len() > 1 {
                    let tmp_value_local_index =
                        self.define_unnamed_local(ctx, Loc::internal(), var_type);
                    self.codegen_local_set(instrs, var_type, tmp_value_local_index);

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
            VarInfo::Const(VarInfoConst { loc, .. })
            | VarInfo::VoidEnumValue(VarInfoVoidEnumValue { loc, .. })
            | VarInfo::StructValueField(VarInfoStructValueField { loc, .. }) => {
                // TODO!: move to typer
                self.registry
                    .reporter
                    .abort_due_to_compiler_bug(&format!("Cannot mutate a constant"), *loc);
            }
        };
    }

    fn define_local(
        &self,
        ctx: &mut ExprContext,
        loc: Loc,
        local_name: &'static str,
        local_type: &Type,
    ) -> u32 {
        let current_scope = self.current_scope(ctx).relax().be_mut();

        current_scope.symbols.push(Symbol {
            name: local_name,
            col_index: ctx.locals.len(),
        });

        let local_index = self.define_unnamed_local(ctx, loc, local_type);
        local_index
    }

    fn define_unnamed_local(&self, ctx: &mut ExprContext, loc: Loc, local_type: &Type) -> u32 {
        let local_index = ctx.next_local_index;
        ctx.locals.push(Local {
            local_index,
            local_type: local_type.clone(),
            definition_loc: loc,
        });
        ctx.next_local_index += count_primitive_components(&self.registry, local_type);

        local_index
    }

    fn create_or_get_addr_local(&self, ctx: &mut ExprContext) -> u32 {
        if let Some(addr_local_index) = ctx.addr_local_index {
            return addr_local_index;
        }

        let addr_local_index = self.define_unnamed_local(ctx, Loc::internal(), &Type::U32);

        return addr_local_index;
    }

    fn codegen_local_set(&self, instrs: &mut Vec<WasmInstr>, local_type: &Type, local_index: u32) {
        for i in (0..count_primitive_components(&self.registry, local_type)).rev() {
            instrs.push(WasmInstr::LocalSet {
                local_index: local_index + i,
            });
        }
    }

    fn emit_deferred(&self, scope: &Scope, instrs: &mut Vec<WasmInstr>) {
        for expr in scope.deferred_exprs.iter().rev() {
            for instr in &expr.instrs {
                instrs.push(instr.clone());
            }
        }
    }

    fn emit_deferred_for_return(&self, ctx: &ExprContext, instrs: &mut Vec<WasmInstr>) {
        for scope in self.module_info[ctx.module_id].scope_stack.iter().rev() {
            self.emit_deferred(scope, instrs);
        }
    }

    fn var_from_expr(&mut self, ctx: &mut ExprContext, expr: &CodeExpr) -> Option<VarInfo> {
        match expr {
            CodeExpr::Ident(ident) => Some(self.var_from_ident(ctx, ident)),
            CodeExpr::FieldAccess(field_access) => {
                Some(self.var_from_field_access(ctx, field_access))
            }
            CodeExpr::Paren(ParenExpr {
                id: _,
                expr,
                has_trailing_comma: _,
                loc: _,
            }) => self.var_from_expr(ctx, expr),
            CodeExpr::PrefixOp(PrefixOpExpr {
                id: _,
                op_tag,
                expr,
                op_loc: _,
                loc: _,
            }) => match op_tag {
                PrefixOpTag::Dereference => Some(self.var_from_deref(ctx, expr)),
                _ => None,
            },
            _ => None,
        }
    }

    fn var_from_ident(&self, ctx: &ExprContext, var_name: &IdentExpr) -> VarInfo {
        let expr_info = self.get_expr_info(ctx, var_name.id, var_name.loc);
        let value_info = &self.registry.value_info[expr_info];
        match value_info.kind {
            ValueKind::Global => {
                let global = &self.registry.globals[value_info.col_index];

                return VarInfo::Global(VarInfoGlobal {
                    global_index: global.wasm_global_index,
                    var_type: self.get_type(global.type_id).clone(),
                });
            }
            ValueKind::Const => {
                let const_def = &self.registry.constants[value_info.col_index];

                return VarInfo::Const(VarInfoConst {
                    const_def: const_def.relax(),
                    loc: var_name.loc,
                });
            }
            ValueKind::EnumConstructor => {
                let enum_ctor = &self.registry.enum_ctors[value_info.col_index];

                return VarInfo::VoidEnumValue(VarInfoVoidEnumValue {
                    variant_index: enum_ctor.variant_index,
                    loc: var_name.loc,
                });
            }
            ValueKind::Local => { /* processed separately */ }
        }

        let Some(symbol) = self.current_scope(ctx).get_symbol(var_name.repr) else {
            unreachable!()
        };

        let local = &ctx.locals[symbol.col_index];

        VarInfo::Local(VarInfoLocal {
            local_index: local.local_index,
            var_type: local.local_type.clone(),
        })
    }

    fn var_from_field_access(
        &mut self,
        ctx: &mut ExprContext,
        field_access: &FieldAccessExpr,
    ) -> VarInfo {
        let lhs_type = self.get_expr_type(ctx, field_access.lhs.as_ref());

        let field = get_struct_field_info(&self.registry, &lhs_type, field_access)
            .ok()
            .unwrap();

        if let Type::Pointer(ptr) = lhs_type
            && !ptr.is_sequence
        {
            return VarInfo::Stored(VarInfoStored {
                address: self.build_code_unit(ctx, &field_access.lhs),
                offset: field.byte_offset,
                var_type: field.field_type.clone(),
            });
        }

        if let Some(var) = self.var_from_expr(ctx, &field_access.lhs.as_ref()) {
            match var {
                // these are handled as struct values
                VarInfo::Const(_) | VarInfo::VoidEnumValue(_) => {}
                VarInfo::Global(VarInfoGlobal {
                    global_index,
                    var_type: _,
                }) => {
                    return VarInfo::Global(VarInfoGlobal {
                        global_index: global_index + field.field_index,
                        var_type: field.field_type.clone(),
                    });
                }
                VarInfo::Local(VarInfoLocal {
                    local_index,
                    var_type: _,
                }) => {
                    return VarInfo::Local(VarInfoLocal {
                        local_index: local_index + field.field_index,
                        var_type: field.field_type.clone(),
                    });
                }
                VarInfo::Stored(VarInfoStored {
                    address,
                    offset,
                    var_type: _,
                }) => {
                    return VarInfo::Stored(VarInfoStored {
                        address,
                        offset: offset + field.byte_offset,
                        var_type: field.field_type.clone(),
                    });
                }
                VarInfo::StructValueField(VarInfoStructValueField {
                    struct_value,
                    drops_before,
                    drops_after,
                    var_type: _,
                    loc: _,
                }) => {
                    let struct_components_count =
                        count_primitive_components(&self.registry, &lhs_type);
                    let field_components_count =
                        count_primitive_components(&self.registry, &field.field_type);

                    return VarInfo::StructValueField(VarInfoStructValueField {
                        struct_value,
                        drops_before: drops_before + struct_components_count
                            - field.field_index
                            - field_components_count,
                        drops_after: drops_after + field.field_index,
                        var_type: field.field_type.clone(),
                        loc: field_access.field_name.loc,
                    });
                }
            };
        };

        let struct_components_count = count_primitive_components(&self.registry, &lhs_type);
        let field_components_count = count_primitive_components(&self.registry, &field.field_type);

        return VarInfo::StructValueField(VarInfoStructValueField {
            struct_value: self.build_code_unit(ctx, &field_access.lhs),
            drops_before: struct_components_count - field.field_index - field_components_count,
            drops_after: field.field_index,
            var_type: field.field_type.clone(),
            loc: field_access.field_name.loc,
        });
    }

    fn var_from_deref(&mut self, ctx: &mut ExprContext, addr_expr: &CodeExpr) -> VarInfo {
        let addr_type = self.get_expr_type(ctx, addr_expr);

        let Type::Pointer(PointerType {
            pointee,
            is_sequence: false,
            is_nullable: _,
        }) = &addr_type
        else {
            unreachable!()
        };

        return VarInfo::Stored(VarInfoStored {
            address: self.build_code_unit(ctx, addr_expr),
            offset: 0,
            var_type: self.get_type(*pointee).clone(),
        });
    }

    fn build_code_unit(&mut self, ctx: &mut ExprContext, expr: &CodeExpr) -> CodeUnit {
        let mut code_unit = CodeUnit { instrs: Vec::new() };
        self.codegen(ctx, &mut code_unit.instrs, expr);
        code_unit
    }

    fn process_const_string(&self, value: &str) -> PooledStr {
        let string_len = value.as_bytes().len() as u32;

        for pooled_str in self.string_pool.iter() {
            if pooled_str.value == value {
                return PooledStr {
                    ptr: pooled_str.ptr,
                    len: string_len,
                };
            }
        }

        let ptr = self.append_data(String::from(value).into_bytes());

        self.string_pool.be_mut().push(PooledString {
            value: String::from(value),
            ptr,
        });

        return PooledStr {
            ptr,
            len: string_len,
        };
    }

    fn append_data(&self, bytes: Vec<u8>) -> u32 {
        let offset = *self.registry.data_size;
        let mut instrs = Vec::new();
        instrs.push(WasmInstr::I32Const {
            value: offset as i32,
        });

        *self.registry.data_size.be_mut() += bytes.len() as u32;
        self.datas.be_mut().push(WasmData::Active {
            offset: WasmExpr { instrs },
            bytes,
        });

        offset
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
            | Type::Pointer { .. } => instrs.push(WasmInstr::I32Const { value: 0 }),
            Type::U64 | Type::I64 => instrs.push(WasmInstr::I64Const { value: 0 }),
            Type::F32 => instrs.push(WasmInstr::F32Const { value: 0.0 }),
            Type::F64 => instrs.push(WasmInstr::F64Const { value: 0.0 }),
            Type::Struct { struct_index } => {
                let struct_ref = &self.registry.structs[*struct_index];
                for field in &struct_ref.fields {
                    self.codegen_default_value(ctx, instrs, &field.field_type);
                }
            }
            Type::Enum { enum_index } => {
                let enum_def = &self.registry.enums[*enum_index];

                self.codegen_default_value(ctx, instrs, &Type::U32);
                self.codegen_default_value(ctx, instrs, &enum_def.variant_type);
            }
            Type::Result(result) => {
                self.codegen_default_value(ctx, instrs, self.get_type(result.ok));
                self.codegen_default_value(ctx, instrs, self.get_type(result.err));
            }
            Type::Seg(seg) => {
                let ptr_type = Type::Pointer(PointerType {
                    pointee: seg.item,
                    is_sequence: false,
                    is_nullable: false,
                });
                self.codegen_default_value(ctx, instrs, &ptr_type);
                self.codegen_default_value(ctx, instrs, &Type::U32);
            }
            Type::Container(ctr) => {
                self.codegen_default_value(ctx, instrs, self.get_type(ctr.container));
            }
        }
    }

    fn codegen_binary_op(
        &self,
        ctx: &ExprContext,
        instrs: &mut Vec<WasmInstr>,
        op_tag: &InfixOpTag,
        operand_type: &Type,
        op_loc: &Loc,
    ) {
        let kind = self.get_binary_op_kind(ctx, op_tag, operand_type, op_loc);
        instrs.push(WasmInstr::BinaryOp { kind });
    }

    fn get_binary_op_kind(
        &self,
        ctx: &ExprContext,
        op_tag: &InfixOpTag,
        operand_type: &Type,
        op_loc: &Loc,
    ) -> WasmBinaryOpKind {
        let mut signed = false;
        let mut wasm_op_type = WasmType::I32;

        match operand_type {
            Type::Null | Type::Bool | Type::U8 | Type::U16 | Type::U32 | Type::Pointer { .. } => {}
            Type::Enum { enum_index }
                if self.registry.enums[*enum_index].variant_type == Type::Void => {}

            Type::I8 | Type::I16 | Type::I32 => signed = true,

            Type::I64 => {
                wasm_op_type = WasmType::I64;
                signed = true;
            }
            Type::U64 => wasm_op_type = WasmType::I64,

            Type::F32 => wasm_op_type = WasmType::F32,
            Type::F64 => wasm_op_type = WasmType::F64,

            Type::Never
            | Type::Void
            | Type::Enum { enum_index: _ }
            | Type::Struct { struct_index: _ }
            | Type::Result(_)
            | Type::Seg(_)
            | Type::Container(_) => {
                // TODO!: move to typer
                self.registry.reporter.abort_due_to_compiler_bug(
                    &format!(
                        "Operator `{}` is incompatible with operands of type {}",
                        op_loc.read_span(&self.registry.modules[ctx.module_id].source),
                        self.registry.fmt(operand_type)
                    ),
                    op_loc.clone(),
                );
            }
        }

        use InfixOpTag::*;
        use WasmBinaryOpKind::*;

        return match wasm_op_type {
            WasmType::I32 => match op_tag {
                Equal => I32_EQ,
                NotEqual => I32_NE,
                Less if signed => I32_LT_S,
                Less => I32_LT_U,
                Greater if signed => I32_GT_S,
                Greater => I32_GT_U,
                LessEqual if signed => I32_LE_S,
                LessEqual => I32_LE_U,
                GreaterEqual if signed => I32_GE_S,
                GreaterEqual => I32_GE_U,
                Add => I32_ADD,
                Sub => I32_SUB,
                Mul => I32_MUL,
                Div if signed => I32_DIV_S,
                Div => I32_DIV_U,
                Mod if signed => I32_REM_S,
                Mod => I32_REM_U,
                And => I32_AND,
                BitAnd => I32_AND,
                Or => I32_OR,
                BitOr => I32_OR,
                ShiftLeft => I32_SHL,
                ShiftRight if signed => I32_SHR_S,
                ShiftRight => I32_SHR_U,
                _ => unreachable!(),
            },
            WasmType::I64 => match op_tag {
                Equal => I64_EQ,
                NotEqual => I64_NE,
                Less if signed => I64_LT_S,
                Less => I64_LT_U,
                Greater if signed => I64_GT_S,
                Greater => I64_GT_U,
                LessEqual if signed => I64_LE_S,
                LessEqual => I64_LE_U,
                GreaterEqual if signed => I64_GE_S,
                GreaterEqual => I64_GE_U,
                Add => I64_ADD,
                Sub => I64_SUB,
                Mul => I64_MUL,
                Div if signed => I64_DIV_S,
                Div => I64_DIV_U,
                Mod if signed => I64_REM_S,
                Mod => I64_REM_U,
                And => I64_AND,
                BitAnd => I64_AND,
                Or => I64_OR,
                BitOr => I64_OR,
                ShiftLeft => I64_SHL,
                ShiftRight if signed => I64_SHR_S,
                ShiftRight => I64_SHR_U,
                _ => unreachable!(),
            },
            WasmType::F32 => match op_tag {
                Equal => F32_EQ,
                NotEqual => F32_NE,
                Less => F32_LT,
                Greater => F32_GT,
                LessEqual => F32_LE,
                GreaterEqual => F32_GE,
                Add => F32_ADD,
                Sub => F32_SUB,
                Mul => F32_MUL,
                Div => F32_DIV,
                Mod | And | BitAnd | Or | BitOr | ShiftLeft | ShiftRight => {
                    incompatible_op_err(self, ctx, operand_type, op_loc);
                }
                _ => unreachable!(),
            },
            WasmType::F64 => match op_tag {
                Equal => F64_EQ,
                NotEqual => F64_NE,
                Less => F64_LT,
                Greater => F64_GT,
                LessEqual => F64_LE,
                GreaterEqual => F64_GE,
                Add => F64_ADD,
                Sub => F64_SUB,
                Mul => F64_MUL,
                Div => F64_DIV,
                Mod | And | BitAnd | Or | BitOr | ShiftLeft | ShiftRight => {
                    incompatible_op_err(self, ctx, operand_type, op_loc);
                }
                _ => unreachable!(),
            },
        };

        fn incompatible_op_err(
            self_: &CodeGenerator,
            ctx: &ExprContext,
            op_type: &Type,
            op_loc: &Loc,
        ) -> ! {
            // TODO!: move to typer
            self_.registry.reporter.abort_due_to_compiler_bug(
                &format!(
                    "Operator `{}` is incompatible with operands of type {}",
                    op_loc.read_span(&self_.registry.modules[ctx.module_id].source),
                    self_.registry.fmt(op_type)
                ),
                op_loc.clone(),
            );
        }
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
            | InfixOpTag::Pipe => None,
        }
    }

    fn lower_type(&self, type_: &Type, wasm_types: &mut Vec<WasmType>) {
        match type_ {
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
            Type::Pointer { .. } => wasm_types.push(WasmType::I32),
            Type::Struct { struct_index } => {
                let struct_def = &self.registry.structs[*struct_index];

                for field in &struct_def.fields {
                    self.lower_type(&field.field_type, wasm_types);
                }
            }
            Type::Enum { enum_index } => {
                let enum_def = &self.registry.enums[*enum_index];

                self.lower_type(&Type::U32, wasm_types);
                self.lower_type(&enum_def.variant_type, wasm_types);
            }
            Type::Result(result) => {
                self.lower_type(self.get_type(result.ok), wasm_types);
                self.lower_type(self.get_type(result.err), wasm_types);
            }
            Type::Seg(seg) => {
                let ptr_type = Type::Pointer(PointerType {
                    pointee: seg.item,
                    is_sequence: false,
                    is_nullable: false,
                });
                self.lower_type(&Type::U32, wasm_types);
                self.lower_type(&ptr_type, wasm_types);
            }
            Type::Container(ctr) => {
                self.lower_type(self.get_type(ctr.container), wasm_types);
            }
        }
    }

    fn enter_scope(&mut self, ctx: &ExprContext, scope_type: ScopeKind) {
        let module = &mut self.relax_mut().module_info[ctx.module_id];

        self.init_scope_from_parent_and_push(&mut module.scope_stack, scope_type);
    }

    fn exit_scope(&mut self, ctx: &ExprContext) -> Scope {
        let module = &mut self.module_info[ctx.module_id];

        module.scope_stack.pop().unwrap()
    }

    fn current_scope(&self, ctx: &ExprContext) -> &Scope {
        let module = &self.module_info[ctx.module_id];

        module.scope_stack.last().unwrap()
    }

    fn init_scope_from_parent_and_push(
        &mut self,
        scope_stack: &mut Vec<Scope>,
        scope_type: ScopeKind,
    ) {
        let mut new_scope = Scope::new(scope_type);
        if let Some(parent) = scope_stack.last() {
            new_scope.symbols.extend_from_slice(&parent.symbols);
            new_scope.expr_id_offset = parent.expr_id_offset;
        };
        scope_stack.push(new_scope);
    }

    fn push_wasm_dbg_name_section_locals(
        &self,
        output: &mut Vec<WasmLocalInfo>,
        local_index: u32,
        local_type: &Type,
        local_name: String,
    ) {
        // TODO: fix unnamed locals breaking wasm2wat
        if !self.registry.should_emit_dbg_local_names {
            return;
        }

        match local_type {
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
            | Type::F64
            | Type::Pointer { .. } => output.push(WasmLocalInfo {
                local_index,
                local_name,
            }),
            Type::Struct { struct_index } => {
                let struct_def = &self.registry.structs[*struct_index];
                for field in &struct_def.fields {
                    self.push_wasm_dbg_name_section_locals(
                        output,
                        local_index + field.field_index,
                        &field.field_type,
                        alloc::format!("{}.{}", local_name, field.field_name),
                    );
                }
            }
            Type::Enum { enum_index } => {
                self.push_wasm_dbg_name_section_locals(
                    output,
                    local_index,
                    &Type::U32,
                    alloc::format!("{}#tag", local_name),
                );

                let enum_def = &self.registry.enums[*enum_index];
                self.push_wasm_dbg_name_section_locals(
                    output,
                    local_index,
                    &enum_def.variant_type,
                    alloc::format!("{}#payload", local_name),
                );
            }
            Type::Result(result_type) => {
                self.push_wasm_dbg_name_section_locals(
                    output,
                    local_index,
                    self.get_type(result_type.ok),
                    alloc::format!("{}#ok", local_name),
                );

                self.push_wasm_dbg_name_section_locals(
                    output,
                    local_index,
                    self.get_type(result_type.err),
                    alloc::format!("{}#err", local_name),
                );
            }
            Type::Seg(seg) => {
                let ptr_type = Type::Pointer(PointerType {
                    pointee: seg.item,
                    is_sequence: false,
                    is_nullable: false,
                });

                self.push_wasm_dbg_name_section_locals(
                    output,
                    local_index,
                    &Type::U32,
                    alloc::format!("{}.len", local_name),
                );

                self.push_wasm_dbg_name_section_locals(
                    output,
                    local_index,
                    &ptr_type,
                    alloc::format!("{}.ptr", local_name),
                );
            }
            Type::Container(ctr) => {
                self.push_wasm_dbg_name_section_locals(
                    output,
                    local_index,
                    self.get_type(ctr.container),
                    local_name,
                );
            }
        }
    }

    fn get_expr_type(&self, ctx: &ExprContext, expr: &CodeExpr) -> &'static Type {
        let scope = self.current_scope(ctx);
        let Some(type_id) = self.registry.get_expr_type(scope.expr_id_offset, expr) else {
            self.registry
                .reporter
                .abort_due_to_compiler_bug("Untyped code expression", expr.loc());
        };
        self.get_type(type_id)
    }

    fn get_type_expr_value(&self, ctx: &ExprContext, expr: &TypeExpr) -> &'static Type {
        let type_id = self.get_expr_info(ctx, expr.id(), expr.loc());
        self.get_type(type_id)
    }

    fn get_expr_info(&self, ctx: &ExprContext, expr_id: ExprId, expr_loc: Loc) -> ExprInfo {
        let scope = self.current_scope(ctx);
        let Some(expr_info) = self.registry.get_expr_info(scope.expr_id_offset, expr_id) else {
            self.registry
                .reporter
                .abort_due_to_compiler_bug("Untyped expression", expr_loc);
        };
        expr_info
    }

    fn get_type(&self, type_id: TypeId) -> &'static Type {
        self.registry.get_type(type_id)
    }
}

fn get_cast_instr(casted_from: &Type, casted_to: &Type) -> Option<WasmInstr> {
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
