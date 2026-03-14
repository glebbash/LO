use crate::{ast::*, common::*, lexer::*, registry::*, typer::*, wasm::*};

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
}

struct VarInfoVoidEnumValue {
    variant_index: usize,
}

// TODO(optimize): codegen on demand instead of building an extra vec
type CompiledExpr = Vec<WasmInstr>;

struct VarInfoStored {
    address_instrs: CompiledExpr,
    offset: u32,
    var_type: Type,
}

struct VarInfoStructValueField {
    lhs_instrs: CompiledExpr,
    drops_before: u32,
    drops_after: u32,
    var_type: Type,
}

#[derive(Clone, Default)]
struct Scope {
    kind: ScopeKind,
    deferred_exprs: Vec<CompiledExpr>,
    expr_id_offset: usize,
}

impl Scope {
    fn new(scope_type: ScopeKind) -> Self {
        Self {
            kind: scope_type,
            ..Default::default()
        }
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

            self.enter_scope(ctx, ScopeKind::Function);

            self.codegen_code_block(ctx, &mut wasm_expr.instrs, body);

            self.exit_scope(ctx);

            let mut wasm_locals_flat = Vec::new();
            for i in fn_info.params.len()..fn_info.locals_ids.len() {
                self.lower_type(
                    &self.registry.locals[fn_info.locals_ids[i]].local_type,
                    &mut wasm_locals_flat,
                );
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

                for local_id in &fn_info.locals_ids {
                    let local_def = &self.registry.locals[*local_id];

                    if let Some(local_name) = local_def.local_name {
                        self.push_wasm_dbg_name_section_locals(
                            &mut local_names_item.locals,
                            local_def.wasm_local_index,
                            &local_def.local_type,
                            local_name,
                        );
                    }
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

        self.wasm_module.datas.append(self.registry.datas.be_mut());
        self.wasm_module.types.append(self.wasm_fn_types.be_mut());
    }

    fn codegen_code_block(
        &mut self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        block: &CodeBlock,
    ) {
        for expr in &block.exprs {
            self.codegen(ctx, instrs, expr);
        }

        self.emit_deferred(self.current_scope(ctx), instrs);

        let expr_info = self.get_expr_info(ctx, block.block_expr_id, block.loc);
        let block_info = &self.registry.block_info[expr_info];

        if block_info.needs_explicit_trap {
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
                let expr_info = self.get_expr_info(ctx, str_literal.id, str_literal.loc);
                let stored_bytes = &self.registry.stored_bytes[expr_info];

                instrs.push(WasmInstr::I32Const {
                    value: stored_bytes.info.ptr as i32,
                });
                instrs.push(WasmInstr::I32Const {
                    value: stored_bytes.info.len as i32,
                });
            }
            CodeExpr::StructLiteral(struct_literal) => {
                for field in &struct_literal.body.fields {
                    self.codegen(ctx, instrs, &field.value);
                }
            }
            CodeExpr::ArrayLiteral(array_literal) => {
                let expr_info = self.get_expr_info(ctx, array_literal.id, array_literal.loc);
                let stored_bytes = &self.registry.stored_bytes[expr_info];

                instrs.push(WasmInstr::I32Const {
                    value: stored_bytes.info.ptr as i32,
                });
                instrs.push(WasmInstr::I32Const {
                    value: stored_bytes.info.len as i32,
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

            CodeExpr::Ident(_)
            | CodeExpr::FieldAccess(_)
            | CodeExpr::PrefixOp(PrefixOpExpr {
                op_tag: PrefixOpTag::Dereference,
                ..
            }) => {
                let var = self.var_from_expr(ctx, expr).unwrap();
                self.codegen_var_get(ctx, instrs, &var);
            }

            CodeExpr::Let(let_expr) => {
                if let_expr.is_inline {
                    // was handled in typer
                    return;
                }

                let var_type = self.get_expr_type(ctx, &let_expr.value);

                if let_expr.name.repr == "_" {
                    self.codegen(ctx, instrs, &let_expr.value);

                    for _ in 0..count_primitive_components(&self.registry, &var_type) {
                        instrs.push(WasmInstr::Drop);
                    }
                    return;
                }

                let local = self.get_local_info(ctx, let_expr.name.id, let_expr.name.loc);
                self.codegen(ctx, instrs, &let_expr.value);
                self.codegen_local_set(instrs, &local.var_type, local.local_index);
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
            CodeExpr::PrefixOp(prefix_op) => match prefix_op.op_tag {
                // handled separately
                PrefixOpTag::Dereference => unreachable!(),

                PrefixOpTag::Reference => {
                    let VarInfo::Stored(VarInfoStored {
                        mut address_instrs,
                        offset,
                        var_type: _,
                    }) = self.var_from_expr(ctx, &prefix_op.expr).unwrap()
                    else {
                        unreachable!()
                    };

                    instrs.append(&mut address_instrs);
                    instrs.push(WasmInstr::I32Const {
                        value: offset as i32,
                    });
                    instrs.push(WasmInstr::BinaryOp {
                        kind: WasmBinaryOpKind::I32_ADD,
                    });
                }
                PrefixOpTag::Not => {
                    self.codegen(ctx, instrs, &prefix_op.expr);

                    let operand_type = self.get_expr_type(ctx, &prefix_op.expr);
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
                    self.codegen(ctx, instrs, &prefix_op.expr);
                }
                PrefixOpTag::Negative => {
                    let operand_type = self.get_expr_type(ctx, &prefix_op.expr);

                    if let CodeExpr::IntLiteral(int) = &prefix_op.expr.as_ref() {
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
                            self.codegen(ctx, instrs, &prefix_op.expr);
                            instrs.push(WasmInstr::BinaryOp {
                                kind: WasmBinaryOpKind::I32_SUB,
                            });
                        }
                        WasmType::I64 => {
                            instrs.push(WasmInstr::I64Const { value: 0 });
                            self.codegen(ctx, instrs, &prefix_op.expr);
                            instrs.push(WasmInstr::BinaryOp {
                                kind: WasmBinaryOpKind::I64_SUB,
                            });
                        }
                        WasmType::F32 => {
                            self.codegen(ctx, instrs, &prefix_op.expr);
                            instrs.push(WasmInstr::UnaryOp {
                                kind: WasmUnaryOpKind::F32_NEG,
                            });
                        }
                        WasmType::F64 => {
                            self.codegen(ctx, instrs, &prefix_op.expr);
                            instrs.push(WasmInstr::UnaryOp {
                                kind: WasmUnaryOpKind::F64_NEG,
                            });
                        }
                    }
                }
            },
            CodeExpr::InfixOp(infix_op) => {
                let expr_info = self.get_expr_info(ctx, infix_op.id, infix_op.loc);
                let op_info = self.registry.binary_op_info[expr_info].relax();

                if op_info.is_compound_assignment {
                    let var = self.var_from_expr(ctx, &infix_op.lhs).unwrap();

                    self.codegen_var_set_prepare(instrs, &var);
                    {
                        self.codegen_var_get(ctx, instrs, &var);
                        self.codegen(ctx, instrs, &infix_op.rhs);
                        instrs.push(WasmInstr::BinaryOp {
                            kind: op_info.op_kind.clone(),
                        });
                    }
                    self.codegen_var_set(ctx, instrs, &var);
                    return;
                }

                self.codegen(ctx, instrs, &infix_op.lhs);
                self.codegen(ctx, instrs, &infix_op.rhs);
                instrs.push(WasmInstr::BinaryOp {
                    kind: op_info.op_kind.clone(),
                });
            }

            CodeExpr::Assign(assign) => {
                let var = self.var_from_expr(ctx, &assign.lhs).unwrap();

                self.codegen_var_set_prepare(instrs, &var);
                self.codegen(ctx, instrs, &assign.rhs);
                self.codegen_var_set(ctx, instrs, &var);
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
            CodeExpr::MethodCall(method_call) => {
                let call_info_id = self.get_expr_info(ctx, method_call.id, method_call.loc);
                let call_info = &self.registry.call_info[call_info_id];

                match call_info.value {
                    CallInfoValue::EnumCtor(_) => unreachable!(),
                    CallInfoValue::FnCall(fn_index) => {
                        self.codegen_fn_call(
                            ctx,
                            instrs,
                            fn_index,
                            Some(&method_call.lhs),
                            &method_call.args.items,
                        );
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
                    let CodeExpr::StringLiteral(str_literal) = &call.args.items[0] else {
                        unreachable!()
                    };

                    let expr_info = self.get_expr_info(ctx, str_literal.id, str_literal.loc);
                    let stored_bytes = &self.registry.stored_bytes[expr_info];

                    instrs.push(WasmInstr::I32Const {
                        value: stored_bytes.info.ptr as i32,
                    });
                    instrs.push(WasmInstr::I32Const {
                        value: stored_bytes.info.len as i32,
                    });

                    return;
                }

                if call.fn_name.repr == "seg" {
                    for arg in &call.args.items {
                        self.codegen(ctx, instrs, arg);
                    }
                    return;
                }

                if call.fn_name.repr == "inline_fn_call_loc" {
                    let CodeExpr::StringLiteral(str_literal) = &call.args.items[0] else {
                        unreachable!()
                    };

                    let expr_info = self.get_expr_info(ctx, str_literal.id, str_literal.loc);
                    let stored_bytes = &self.registry.stored_bytes[expr_info];

                    instrs.push(WasmInstr::I32Const {
                        value: stored_bytes.info.ptr as i32,
                    });
                    instrs.push(WasmInstr::I32Const {
                        value: stored_bytes.info.len as i32,
                    });

                    return;
                }

                self.registry.reporter.abort_due_to_compiler_bug(
                    &format!("Unknown intrinsic: {}", call.fn_name.repr),
                    call.fn_name.loc,
                );
            }
            CodeExpr::Sizeof(sizeof) => {
                let type_ = self.get_type_expr_value(ctx, &sizeof.type_expr);
                let mut type_layout = TypeLayout::new();
                get_type_layout(&self.registry, &type_, &mut type_layout);

                instrs.push(WasmInstr::I32Const {
                    value: type_layout.byte_size as i32,
                });
            }

            CodeExpr::Paren(paren) => {
                self.codegen(ctx, instrs, &paren.expr);
            }
            CodeExpr::Return(return_expr) => {
                if let Some(return_expr) = &return_expr.expr {
                    self.codegen(ctx, instrs, return_expr);
                };

                self.emit_deferred_for_return(ctx, instrs);
                instrs.push(WasmInstr::Return);
            }
            CodeExpr::If(if_expr) => {
                match &if_expr.cond {
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

                self.codegen_code_block(ctx, instrs, &if_expr.then_block);
                self.exit_scope(ctx);

                match &if_expr.else_block {
                    ElseBlock::None => {}
                    ElseBlock::Else(code_block_expr) => {
                        instrs.push(WasmInstr::Else);
                        self.enter_scope(ctx, ScopeKind::Block);
                        self.codegen_code_block(ctx, instrs, &code_block_expr);
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
            CodeExpr::Match(match_expr) => {
                let enum_ctor = self.codegen_match_header(ctx, instrs, &match_expr.header);

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
                self.codegen_code_block(ctx, instrs, &match_expr.else_branch);
                self.exit_scope(ctx);
                instrs.push(WasmInstr::BlockEnd);
            }
            CodeExpr::While(while_expr) => {
                instrs.push(WasmInstr::BlockStart {
                    block_kind: WasmBlockKind::Block,
                    block_type: WasmBlockType::NoOut,
                });
                instrs.push(WasmInstr::BlockStart {
                    block_kind: WasmBlockKind::Loop,
                    block_type: WasmBlockType::NoOut,
                });

                if let Some(cond) = &while_expr.cond {
                    self.codegen(ctx, instrs, cond);

                    instrs.push(WasmInstr::UnaryOp {
                        kind: WasmUnaryOpKind::I32_EQZ,
                    });
                    instrs.push(WasmInstr::BranchIf { label_index: 1 });
                }

                self.enter_scope(ctx, ScopeKind::Loop);
                self.codegen_code_block(ctx, instrs, &while_expr.body);
                self.exit_scope(ctx);

                // implicit continue
                instrs.push(WasmInstr::Branch { label_index: 0 });

                instrs.push(WasmInstr::BlockEnd);
                instrs.push(WasmInstr::BlockEnd);
            }
            CodeExpr::For(for_expr) => {
                let item_local = self.get_local_info(ctx, for_expr.item.id, for_expr.item.loc);
                let item_type = &item_local.var_type;

                self.enter_scope(ctx, ScopeKind::ForLoop);

                let counter_type;
                let counter_local_index;
                let range_end_local_index;
                let mut item_local_index = None;
                let mut step = 1;

                match &for_expr.iterator {
                    ForExprIterator::Range { start, end } => {
                        counter_type = item_local.var_type.clone();
                        counter_local_index = item_local.local_index;

                        range_end_local_index = self.define_unnamed_local(ctx, &counter_type);

                        self.codegen(ctx, instrs, &start);
                        instrs.push(WasmInstr::LocalSet {
                            local_index: counter_local_index,
                        });

                        self.codegen(ctx, instrs, &end);
                        instrs.push(WasmInstr::LocalSet {
                            local_index: range_end_local_index,
                        });
                    }
                    ForExprIterator::Segment { expr } => {
                        counter_type = Type::U32;
                        range_end_local_index = self.define_unnamed_local(ctx, &counter_type);

                        if for_expr.ref_only {
                            let Type::Pointer(PointerType { pointee, .. }) = item_type else {
                                unreachable!()
                            };

                            let mut item_layout = TypeLayout::new();
                            get_type_layout(
                                &self.registry,
                                self.get_type(*pointee),
                                &mut item_layout,
                            );
                            step = item_layout.byte_size;

                            counter_local_index = item_local.local_index;
                        } else {
                            let mut item_layout = TypeLayout::new();
                            get_type_layout(&self.registry, item_type, &mut item_layout);
                            step = item_layout.byte_size;

                            counter_local_index = self.define_unnamed_local(ctx, &counter_type);

                            item_local_index = Some(item_local.local_index);
                        }

                        self.codegen(ctx, instrs, expr);

                        // needed:
                        // ```
                        // counter = segment.ptr
                        // range_end = segment.ptr + (segment.len * item_size)
                        // ```
                        //
                        // which is done like this to save on locals:
                        // ```
                        // counter, range_end = segment
                        // range_end = counter + (range_end * item_size)
                        // ```
                        instrs.push(WasmInstr::LocalSet {
                            local_index: range_end_local_index,
                        });
                        instrs.push(WasmInstr::LocalSet {
                            local_index: counter_local_index,
                        });
                        instrs.push(WasmInstr::LocalGet {
                            local_index: range_end_local_index,
                        });
                        instrs.push(WasmInstr::I32Const { value: step as i32 });
                        instrs.push(WasmInstr::BinaryOp {
                            kind: WasmBinaryOpKind::I32_MUL,
                        });
                        instrs.push(WasmInstr::LocalGet {
                            local_index: counter_local_index,
                        });
                        instrs.push(WasmInstr::BinaryOp {
                            kind: WasmBinaryOpKind::I32_ADD,
                        });
                        instrs.push(WasmInstr::LocalSet {
                            local_index: range_end_local_index,
                        });
                    }
                }

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
                        instrs.push(WasmInstr::LocalGet {
                            local_index: counter_local_index,
                        });
                        instrs.push(WasmInstr::LocalGet {
                            local_index: range_end_local_index,
                        });
                        if is_wide_int(&counter_type).unwrap() {
                            instrs.push(WasmInstr::BinaryOp {
                                kind: WasmBinaryOpKind::I64_EQ,
                            });
                        } else {
                            instrs.push(WasmInstr::BinaryOp {
                                kind: WasmBinaryOpKind::I32_EQ,
                            });
                        }
                        instrs.push(WasmInstr::BranchIf { label_index: 1 });

                        {
                            instrs.push(WasmInstr::BlockStart {
                                block_kind: WasmBlockKind::Block,
                                block_type: WasmBlockType::NoOut,
                            });

                            // item = load item_type at counter
                            if let Some(item_local_index) = item_local_index {
                                self.codegen_var_get(
                                    ctx,
                                    instrs,
                                    &VarInfo::Stored(VarInfoStored {
                                        address_instrs: vec![WasmInstr::LocalGet {
                                            local_index: counter_local_index,
                                        }],
                                        offset: 0,
                                        var_type: item_type.clone(),
                                    }),
                                );

                                self.codegen_local_set(instrs, item_type, item_local_index);
                            }

                            self.codegen_code_block(ctx, instrs, &for_expr.body);

                            instrs.push(WasmInstr::BlockEnd);
                        }

                        // increment counter
                        instrs.push(WasmInstr::LocalGet {
                            local_index: counter_local_index,
                        });
                        if is_wide_int(&counter_type).unwrap() {
                            instrs.push(WasmInstr::I64Const { value: step as i64 });
                            instrs.push(WasmInstr::BinaryOp {
                                kind: WasmBinaryOpKind::I64_ADD,
                            });
                        } else {
                            instrs.push(WasmInstr::I32Const { value: step as i32 });
                            instrs.push(WasmInstr::BinaryOp {
                                kind: WasmBinaryOpKind::I32_ADD,
                            });
                        }
                        self.codegen_local_set(instrs, &counter_type, counter_local_index);

                        // implicit continue
                        instrs.push(WasmInstr::Branch { label_index: 0 });

                        instrs.push(WasmInstr::BlockEnd);
                    }

                    instrs.push(WasmInstr::BlockEnd);
                }

                self.exit_scope(ctx);
            }
            CodeExpr::Break(break_expr) => {
                let expr_info = self.registry.expr_info[break_expr.id];
                let break_info = &self.registry.breaks[expr_info];

                instrs.push(WasmInstr::Branch {
                    label_index: break_info.label_index,
                });
            }
            CodeExpr::Continue(continue_expr) => {
                let expr_info = self.registry.expr_info[continue_expr.id];
                let break_info = &self.registry.breaks[expr_info];

                instrs.push(WasmInstr::Branch {
                    label_index: break_info.label_index,
                });
            }
            CodeExpr::DoWith(do_with) => {
                let arg_type = self.get_expr_type(ctx, do_with.args.items.first().unwrap());
                let arg_local = self.get_local_info(ctx, do_with.bind_id, do_with.with_loc);

                for arg in &do_with.args.items {
                    self.enter_scope(ctx, ScopeKind::InlineFn);

                    self.codegen(ctx, instrs, arg);

                    self.codegen_local_set(instrs, &arg_type, arg_local.local_index);
                    self.codegen(ctx, instrs, &do_with.body);

                    self.exit_scope(ctx);
                }
            }
            CodeExpr::Pipe(pipe) => {
                self.codegen(ctx, instrs, &pipe.lhs);

                let lhs_var = self.get_local_info(ctx, pipe.bind_id, pipe.op_loc);
                self.codegen_local_set(instrs, &lhs_var.var_type, lhs_var.local_index);
                self.codegen(ctx, instrs, &pipe.rhs);
            }
            CodeExpr::Defer(DeferExpr {
                id: _,
                expr,
                loc: _,
            }) => {
                // find first non-inline-fn scope
                let mut scope_to_defer = self.module_info[ctx.module_id]
                    .scope_stack
                    .relax_mut()
                    .last_mut()
                    .unwrap();
                for scope in self.module_info[ctx.module_id].scope_stack.iter_mut().rev() {
                    if scope.kind != ScopeKind::InlineFn {
                        scope_to_defer = scope.relax_mut();
                        break;
                    }
                }

                let mut expr_instrs = Vec::new();
                self.codegen(ctx, &mut expr_instrs, expr);
                scope_to_defer.deferred_exprs.push(expr_instrs);
            }
            CodeExpr::Catch(catch) => {
                self.codegen_catch(
                    ctx,
                    instrs,
                    &catch.lhs,
                    catch.ok_bind_id,
                    catch.error_bind.id,
                    Some(&catch.catch_body),
                );
            }
            CodeExpr::PropagateError(prop_error) => {
                self.codegen_catch(
                    ctx,
                    instrs,
                    &prop_error.expr,
                    prop_error.ok_bind_id,
                    prop_error.err_bind_id,
                    None,
                );
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
        let local = self.get_local_info(ctx, header.variant_bind.id, header.variant_bind.loc);
        self.codegen(ctx, instrs, &header.expr_to_match);
        self.codegen_local_set(instrs, &local.var_type, local.local_index);

        let expr_info = self.get_expr_info(ctx, header.variant_name.id, header.variant_name.loc);
        let value_info = &self.registry.value_info[expr_info];
        let enum_ctor = &self.registry.enum_ctors[value_info.col_index].relax();

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

        self.current_scope(ctx).be_mut().expr_id_offset = call_info.inner_expr_id_offset;

        self.codegen_code_block(ctx, instrs, body);

        self.exit_scope(ctx);
    }

    fn codegen_catch(
        &mut self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        expr: &CodeExpr,
        ok_bind_id: ExprId,
        err_bind_id: ExprId,
        catch_body: Option<&CodeBlock>,
    ) {
        self.enter_scope(ctx, ScopeKind::Block); // enter catch scope

        // put result on the stack
        self.codegen(ctx, instrs, expr);

        // pop error
        let err_local = self.get_local_info(ctx, err_bind_id, expr.loc());
        self.codegen_local_set(instrs, &err_local.var_type, err_local.local_index);

        // pop ok
        let ok_local = self.get_local_info(ctx, ok_bind_id, expr.loc());
        self.codegen_local_set(instrs, &ok_local.var_type, ok_local.local_index);

        // cond: error != 0
        self.codegen_local_get(instrs, &err_local.var_type, err_local.local_index);

        let in_out_type_index = self.get_block_inout_type(&[], &ok_local.var_type);
        instrs.push(WasmInstr::BlockStart {
            block_kind: WasmBlockKind::If,
            block_type: WasmBlockType::InOut {
                type_index: in_out_type_index,
            },
        });

        // catch error
        if let Some(catch_body) = catch_body {
            self.codegen_code_block(ctx, instrs, catch_body);
        } else {
            // return default ok_type of function's result and caught error
            let fn_def = &self.registry.functions[ctx.fn_index.unwrap()];
            let Type::Result(fn_result) = &fn_def.type_.output else {
                unreachable!()
            };
            self.codegen_default_value(ctx, instrs, self.get_type(fn_result.ok));
            self.codegen_local_get(instrs, &err_local.var_type, err_local.local_index);

            self.emit_deferred_for_return(ctx, instrs);
            instrs.push(WasmInstr::Return);
        }

        instrs.push(WasmInstr::Else);

        // no error, push ok value
        self.codegen_local_get(instrs, &ok_local.var_type, ok_local.local_index);

        instrs.push(WasmInstr::BlockEnd);

        self.exit_scope(ctx); // exit catch scope
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

    fn var_from_expr(&mut self, ctx: &mut ExprContext, expr: &CodeExpr) -> Option<VarInfo> {
        Some(match expr {
            CodeExpr::Paren(paren) => return self.var_from_expr(ctx, &paren.expr),

            CodeExpr::Ident(var_name) => {
                let expr_info = self.get_expr_info(ctx, var_name.id, var_name.loc);
                let value_info = &self.registry.value_info[expr_info];

                match value_info.kind {
                    ValueKind::Global => {
                        let global = &self.registry.globals[value_info.col_index];

                        VarInfo::Global(VarInfoGlobal {
                            global_index: global.wasm_global_index,
                            var_type: self.get_type(global.type_id).clone(),
                        })
                    }
                    ValueKind::Const => {
                        let const_def = &self.registry.constants[value_info.col_index];

                        VarInfo::Const(VarInfoConst {
                            const_def: const_def.relax(),
                        })
                    }
                    ValueKind::EnumConstructor => {
                        let enum_ctor = &self.registry.enum_ctors[value_info.col_index];

                        VarInfo::VoidEnumValue(VarInfoVoidEnumValue {
                            variant_index: enum_ctor.variant_index,
                        })
                    }
                    ValueKind::Local => {
                        let local = &self.registry.locals[value_info.col_index];

                        VarInfo::Local(VarInfoLocal {
                            local_index: local.wasm_local_index,
                            var_type: local.local_type.clone(),
                        })
                    }
                }
            }
            CodeExpr::PrefixOp(prefix_op) if prefix_op.op_tag == PrefixOpTag::Dereference => {
                let addr_type = self.get_expr_type(ctx, &prefix_op.expr);

                let Type::Pointer(PointerType {
                    pointee,
                    is_sequence: false,
                    is_nullable: _,
                }) = &addr_type
                else {
                    unreachable!()
                };

                let mut address_instrs = Vec::new();
                self.codegen(ctx, &mut address_instrs, &prefix_op.expr);

                VarInfo::Stored(VarInfoStored {
                    address_instrs,
                    offset: 0,
                    var_type: self.get_type(*pointee).clone(),
                })
            }
            CodeExpr::FieldAccess(field_access) => {
                let lhs_type = self.get_expr_type(ctx, field_access.lhs.as_ref());

                let field = get_struct_field_info(&self.registry, &lhs_type, field_access)
                    .ok()
                    .unwrap();

                if let Type::Pointer(ptr) = lhs_type
                    && !ptr.is_sequence
                {
                    let mut address_instrs = Vec::new();
                    self.codegen(ctx, &mut address_instrs, &field_access.lhs);

                    return Some(VarInfo::Stored(VarInfoStored {
                        address_instrs,
                        offset: field.byte_offset,
                        var_type: field.field_type.clone(),
                    }));
                }

                if let Some(var) = self.var_from_expr(ctx, &field_access.lhs.as_ref()) {
                    match var {
                        // these are handled as struct values
                        VarInfo::Const(_) | VarInfo::VoidEnumValue(_) => {}
                        VarInfo::Global(VarInfoGlobal {
                            global_index,
                            var_type: _,
                        }) => {
                            return Some(VarInfo::Global(VarInfoGlobal {
                                global_index: global_index + field.field_index,
                                var_type: field.field_type.clone(),
                            }));
                        }
                        VarInfo::Local(VarInfoLocal {
                            local_index,
                            var_type: _,
                        }) => {
                            return Some(VarInfo::Local(VarInfoLocal {
                                local_index: local_index + field.field_index,
                                var_type: field.field_type.clone(),
                            }));
                        }
                        VarInfo::Stored(VarInfoStored {
                            address_instrs: address,
                            offset,
                            var_type: _,
                        }) => {
                            return Some(VarInfo::Stored(VarInfoStored {
                                address_instrs: address,
                                offset: offset + field.byte_offset,
                                var_type: field.field_type.clone(),
                            }));
                        }
                        VarInfo::StructValueField(VarInfoStructValueField {
                            lhs_instrs,
                            drops_before,
                            drops_after,
                            var_type: _,
                        }) => {
                            let struct_components_count =
                                count_primitive_components(&self.registry, &lhs_type);
                            let field_components_count =
                                count_primitive_components(&self.registry, &field.field_type);

                            return Some(VarInfo::StructValueField(VarInfoStructValueField {
                                lhs_instrs,
                                drops_before: drops_before + struct_components_count
                                    - field.field_index
                                    - field_components_count,
                                drops_after: drops_after + field.field_index,
                                var_type: field.field_type.clone(),
                            }));
                        }
                    };
                };

                let struct_components_count = count_primitive_components(&self.registry, &lhs_type);
                let field_components_count =
                    count_primitive_components(&self.registry, &field.field_type);

                let mut lhs_instrs = Vec::new();
                self.codegen(ctx, &mut lhs_instrs, &field_access.lhs);

                VarInfo::StructValueField(VarInfoStructValueField {
                    lhs_instrs,
                    drops_before: struct_components_count
                        - field.field_index
                        - field_components_count,
                    drops_after: field.field_index,
                    var_type: field.field_type.clone(),
                })
            }

            _ => return None,
        })
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
                self.codegen_local_get(instrs, var_type, *local_index);
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
            VarInfo::Const(VarInfoConst { const_def }) => {
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
                address_instrs,
                offset,
                var_type,
            }) => {
                let mut loads = Vec::new();
                self.codegen_load_or_store(&mut loads, &var_type, *offset, false);

                if loads.len() == 0 {
                    return;
                }

                instrs.extend_from_slice(address_instrs);

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
                lhs_instrs,
                drops_before,
                drops_after,
                var_type,
            }) => {
                instrs.extend_from_slice(lhs_instrs);
                for _ in 0..*drops_before {
                    instrs.push(WasmInstr::Drop);
                }

                if *drops_after > 0 {
                    let local_index = self.define_unnamed_local(ctx, var_type);

                    self.codegen_local_set(instrs, &var_type, local_index);

                    for _ in 0..*drops_after {
                        instrs.push(WasmInstr::Drop);
                    }

                    self.codegen_local_get(instrs, &var_type, local_index);
                }
            }
        }
    }

    fn codegen_local_get(&self, instrs: &mut Vec<WasmInstr>, local_type: &Type, local_index: u32) {
        for i in 0..count_primitive_components(&self.registry, local_type) {
            instrs.push(WasmInstr::LocalGet {
                local_index: local_index + i,
            });
        }
    }

    // should be called before set's value is pushed to the stack
    fn codegen_var_set_prepare(&self, instrs: &mut Vec<WasmInstr>, var: &VarInfo) {
        match var {
            VarInfo::Stored(VarInfoStored {
                address_instrs,
                offset: _,
                var_type,
            }) => {
                if count_primitive_components(&self.registry, var_type) == 0 {
                    return;
                }

                instrs.extend_from_slice(address_instrs);
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
                address_instrs: _,
                offset,
                var_type,
            }) => {
                let mut stores = Vec::new();
                self.codegen_load_or_store(&mut stores, &var_type, *offset, true);

                if stores.len() > 1 {
                    let tmp_value_local_index = self.define_unnamed_local(ctx, var_type);
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
            VarInfo::Const(_) | VarInfo::VoidEnumValue(_) | VarInfo::StructValueField(_) => {
                unreachable!()
            }
        };
    }

    fn codegen_local_set(&self, instrs: &mut Vec<WasmInstr>, local_type: &Type, local_index: u32) {
        for i in (0..count_primitive_components(&self.registry, local_type)).rev() {
            instrs.push(WasmInstr::LocalSet {
                local_index: local_index + i,
            });
        }
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

    fn get_local_info(&self, ctx: &ExprContext, bind_id: ExprId, bind_loc: Loc) -> VarInfoLocal {
        let expr_info = self.get_expr_info(ctx, bind_id, bind_loc);
        let value_info = &self.registry.value_info[expr_info];
        let local_info = &self.registry.locals[value_info.col_index];

        VarInfoLocal {
            local_index: local_info.wasm_local_index,
            var_type: local_info.local_type.clone(),
        }
    }

    fn define_unnamed_local(&self, ctx: &mut ExprContext, local_type: &Type) -> u32 {
        self.registry
            .add_local(ctx.fn_index.unwrap(), None, local_type)
    }

    fn create_or_get_addr_local(&self, ctx: &mut ExprContext) -> u32 {
        if let Some(addr_local_index) = ctx.addr_local_index {
            return addr_local_index;
        }

        let addr_local_index = self.define_unnamed_local(ctx, &Type::U32);

        return addr_local_index;
    }

    fn emit_deferred(&self, scope: &Scope, instrs: &mut Vec<WasmInstr>) {
        for expr_instrs in scope.deferred_exprs.iter().rev() {
            for instr in expr_instrs {
                instrs.push(instr.clone());
            }
        }
    }

    fn emit_deferred_for_return(&self, ctx: &ExprContext, instrs: &mut Vec<WasmInstr>) {
        for scope in self.module_info[ctx.module_id].scope_stack.iter().rev() {
            self.emit_deferred(scope, instrs);
        }
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
            new_scope.expr_id_offset = parent.expr_id_offset;
        };
        scope_stack.push(new_scope);
    }

    fn push_wasm_dbg_name_section_locals(
        &self,
        output: &mut Vec<WasmLocalInfo>,
        local_index: u32,
        local_type: &Type,
        local_name: &str,
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
                local_name: String::from(local_name),
            }),
            Type::Struct { struct_index } => {
                let struct_def = &self.registry.structs[*struct_index];
                for field in &struct_def.fields {
                    self.push_wasm_dbg_name_section_locals(
                        output,
                        local_index + field.field_index,
                        &field.field_type,
                        &format!("{}.{}", local_name, field.field_name),
                    );
                }
            }
            Type::Enum { enum_index } => {
                self.push_wasm_dbg_name_section_locals(
                    output,
                    local_index,
                    &Type::U32,
                    &format!("{}#tag", local_name),
                );

                let enum_def = &self.registry.enums[*enum_index];
                self.push_wasm_dbg_name_section_locals(
                    output,
                    local_index,
                    &enum_def.variant_type,
                    &format!("{}#payload", local_name),
                );
            }
            Type::Result(result_type) => {
                self.push_wasm_dbg_name_section_locals(
                    output,
                    local_index,
                    self.get_type(result_type.ok),
                    &format!("{}#ok", local_name),
                );

                self.push_wasm_dbg_name_section_locals(
                    output,
                    local_index,
                    self.get_type(result_type.err),
                    &format!("{}#err", local_name),
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
                    &format!("{}.len", local_name),
                );

                self.push_wasm_dbg_name_section_locals(
                    output,
                    local_index,
                    &ptr_type,
                    &format!("{}.ptr", local_name),
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
