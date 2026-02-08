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
    code_unit: &'static CodeUnit,
    loc: Loc,
}

struct VarInfoVoidEnumValue {
    variant_index: usize,
    var_type: Type,
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

impl VarInfo {
    fn get_type(&self) -> &Type {
        match self {
            VarInfo::Local(v) => &v.var_type,
            VarInfo::Global(v) => &v.var_type,
            VarInfo::Const(v) => &v.code_unit.type_,
            VarInfo::VoidEnumValue(v) => &v.var_type,
            VarInfo::Stored(v) => &v.var_type,
            VarInfo::StructValueField(v) => &v.var_type,
        }
    }
}

pub struct CodeGenerator {
    // context
    pub registry: UBRef<Registry>,
    pub reporter: UBRef<Reporter>,

    // state
    wasm_fn_types: UBCell<Vec<WasmFnType>>,
    datas: UBCell<Vec<WasmData>>,
    string_pool: UBCell<Vec<PooledString>>,
    const_slice_lens: Vec<ConstSliceLen>,

    // output
    pub wasm_module: UBCell<WasmModule>,
}

impl CodeGenerator {
    pub fn new(registry: &mut Registry) -> Self {
        Self {
            registry: UBRef::new(&mut *registry),
            reporter: UBRef::new(&mut *registry.reporter),
            wasm_fn_types: Default::default(),
            datas: Default::default(),
            string_pool: Default::default(),
            const_slice_lens: Default::default(),
            wasm_module: Default::default(),
        }
    }

    pub fn codegen_all(&mut self) {
        self.codegen_const_values();

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
            for lo_input_type in &fn_info.type_.inputs {
                self.lower_type(lo_input_type, &mut wasm_fn_type.inputs);
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
        let mut global_index = 0;
        for global in self.registry.globals.relax_mut() {
            global.global_index = global_index;

            let mut wasm_types = Vec::new();
            self.lower_type(&global.global_type, &mut wasm_types);

            let mut instrs = Vec::new();
            self.codegen(
                global.module_ctx.be_mut(),
                &mut instrs,
                &global.def_expr.value,
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

            global_index += wasm_types.len() as u32;
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
            let type_aliases_len = self.registry.type_aliases.len();

            self.registry.be_mut().enter_scope(ctx, ScopeKind::Function);

            for fn_param in &fn_info.params {
                self.define_local(ctx, fn_param.loc, fn_param.param_name, &fn_param.param_type);
            }

            self.codegen_code_block(ctx, &mut wasm_expr.instrs, body, true);

            self.registry.be_mut().exit_scope(ctx);

            // remove any constants/types created by inline fn calls
            self.registry.constants.truncate(constants_len);
            self.registry.type_aliases.truncate(type_aliases_len);

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
            let CodeExpr::IntrinsicCall(intrinsic) = &*global.def_expr.value else {
                continue;
            };

            if intrinsic.fn_name.repr != "data_size" {
                continue;
            }

            self.wasm_module.globals[global.global_index as usize]
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

    // TODO: process inline lets similarly to inline fn calls
    fn codegen_const_values(&mut self) {
        for module in self.registry.modules.relax() {
            for expr in &*module.parser.ast {
                match expr {
                    TopLevelExpr::Let(let_expr) if let_expr.is_inline => {
                        let symbol = self
                            .registry
                            .current_scope(&module.ctx)
                            .get_symbol(&let_expr.name.repr)
                            .unwrap()
                            .relax();
                        let const_ = self.registry.constants[symbol.col_index].relax_mut();

                        self.codegen(
                            module.ctx.be_mut(),
                            &mut const_.code_unit.instrs,
                            &let_expr.value,
                        );
                    }
                    _ => {} // skip, not interested
                }
            }
        }
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
            if *self.get_expr_type(ctx, expr) == Type::Never {
                diverges = true;
                diverges_naturally = diverges_naturally || is_naturally_divergent(expr);
            }

            self.codegen(ctx, instrs, expr);
        }

        self.emit_deferred(self.registry.current_scope(ctx), instrs);

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
            CodeExpr::StructLiteral(StructLiteralExpr {
                id: _,
                struct_name,
                body,
                loc: _,
            }) => {
                let Type::StructInstance { struct_index } = self.unwrap(
                    self.registry
                        .get_type_or_err(ctx, &struct_name.repr, &struct_name.loc),
                ) else {
                    unreachable!()
                };

                let struct_def = &self.registry.structs[struct_index].relax();

                for field_index in 0..body.fields.len() {
                    let field_literal = &body.fields[field_index];
                    let Some(struct_field) = struct_def.fields.get(field_index) else {
                        self.report_error(&Error {
                            message: format!("Excess field values"),
                            loc: field_literal.loc,
                        });
                        break;
                    };

                    if &field_literal.key != &struct_field.field_name {
                        self.report_error(&Error {
                            message: format!(
                                "Unexpected struct field name, expecting: `{}`",
                                struct_field.field_name
                            ),
                            loc: field_literal.loc,
                        });
                    }

                    self.codegen(ctx, instrs, &field_literal.value);

                    let field_value_type = self.get_expr_type(ctx, &field_literal.value);
                    if !is_type_compatible(&struct_field.field_type, &field_value_type) {
                        self.report_error(&Error {
                            message: format!(
                                "Invalid type for struct field {}.{}, expected: {}, got: {}",
                                struct_name.repr,
                                struct_field.field_name,
                                TypeFmt(&*self.registry, &struct_field.field_type,),
                                TypeFmt(&*self.registry, &field_value_type),
                            ),
                            loc: field_literal.value.loc(),
                        });
                    }
                }

                if body.fields.len() < struct_def.fields.len() {
                    let mut missing_fields = Vec::new();
                    for i in body.fields.len()..struct_def.fields.len() {
                        missing_fields.push(&struct_def.fields[i].field_name)
                    }

                    self.report_error(&Error {
                        message: format!("Missing struct fields: {}", ListFmt(&missing_fields)),
                        loc: struct_name.loc,
                    });
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
                let item_type = self.unwrap(self.registry.build_type(ctx, item_type));

                let mut bytes = Vec::new();
                let mut tmp_instrs = Vec::new();

                if let Type::U8 = &item_type {
                    for item in items {
                        let current_item_type = self.get_expr_type(ctx, item);
                        if *current_item_type != item_type {
                            // TODO!: move to typer
                            self.reporter.abort_due_to_compiler_bug(
                                &format!(
                                    "Unexpected array element type: {}, expected: {}",
                                    TypeFmt(&*self.registry, &current_item_type),
                                    TypeFmt(&*self.registry, &item_type),
                                ),
                                item.loc(),
                            );
                        }

                        self.codegen(ctx, &mut tmp_instrs, item);
                        let WasmInstr::I32Const { value } = tmp_instrs.pop().unwrap() else {
                            // TODO!: move to typer
                            self.reporter.abort_due_to_compiler_bug(
                                &format!("Unexpected array element value"),
                                item.loc(),
                            );
                        };

                        bytes.push(value as u8);
                    }
                } else if let Type::StructInstance { struct_index } = &item_type
                    && self.registry.structs[*struct_index].struct_name == "str"
                {
                    for item in items {
                        let current_item_type = self.get_expr_type(ctx, item);
                        if *current_item_type != item_type {
                            // TODO!: move to typer
                            self.reporter.abort_due_to_compiler_bug(
                                &format!(
                                    "Unexpected array element type: {}, expected: {}",
                                    TypeFmt(&*self.registry, &current_item_type),
                                    TypeFmt(&*self.registry, &item_type),
                                ),
                                item.loc(),
                            );
                        }

                        self.codegen(ctx, &mut tmp_instrs, item);
                        let WasmInstr::I32Const { value: len } = tmp_instrs.pop().unwrap() else {
                            // TODO!: move to typer
                            self.reporter.abort_due_to_compiler_bug(
                                &format!("Unexpected array element value"),
                                item.loc(),
                            );
                        };
                        let WasmInstr::I32Const { value: ptr } = tmp_instrs.pop().unwrap() else {
                            // TODO!: move to typer
                            self.reporter.abort_due_to_compiler_bug(
                                &format!("Unexpected array element value"),
                                item.loc(),
                            );
                        };

                        bytes.extend_from_slice(&ptr.to_le_bytes());
                        bytes.extend_from_slice(&len.to_le_bytes());
                    }
                } else {
                    // TODO!: move to typer
                    self.reporter.abort_due_to_compiler_bug(
                        &format!(
                            "Unsupported array literal element type: {}",
                            TypeFmt(&*self.registry, &item_type)
                        ),
                        *loc,
                    );
                }

                let ptr = self.append_data(bytes);
                instrs.push(WasmInstr::I32Const { value: ptr as i32 });

                self.const_slice_lens.be_mut().push(ConstSliceLen {
                    slice_ptr: ptr,
                    slice_len: items.len(),
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

                self.codegen_default_value(ctx, instrs, &result.ok);
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
                    let code_unit = self.build_code_unit(ctx, value);
                    self.register_block_const(
                        ctx,
                        ConstDef {
                            const_name: name.repr,
                            code_unit,
                            loc: name.loc,
                        },
                    );
                    return;
                }

                let var_type = self.get_expr_type(ctx, &value).clone();

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
                    var_type,
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
                let casted_to_type = self.unwrap(self.registry.build_type(ctx, casted_to));

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
                        // TODO: port to typer
                        self.reporter.abort_due_to_compiler_bug(
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
                        // TODO!: port to typer
                        self.reporter.abort_due_to_compiler_bug(
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
                    self.reporter.abort_due_to_compiler_bug(
                        &format!("Cannot perform assignment: invalid lhs"),
                        *op_loc,
                    );
                };

                let rhs_type = self.get_expr_type(ctx, rhs);
                if !is_type_compatible(var.get_type(), &rhs_type) {
                    self.report_error(&Error {
                        message: format!(
                            "Cannot assign {} to variable of type {}",
                            TypeFmt(&*self.registry, &rhs_type),
                            TypeFmt(&*self.registry, var.get_type())
                        ),
                        loc: op_loc.clone(),
                    });
                }

                self.codegen_var_set_prepare(instrs, &var);
                self.codegen(ctx, instrs, rhs);
                self.codegen_var_set(ctx, instrs, &var);
            }
            CodeExpr::FieldAccess(field_access) => {
                let var = self.var_from_field_access(ctx, field_access);
                self.codegen_var_get(ctx, instrs, &var);
            }

            CodeExpr::FnCall(FnCallExpr {
                id: _,
                fn_name,
                args,
                loc: _,
            }) => {
                if let Some(symbol) = self.registry.current_scope(ctx).get_symbol(&fn_name.repr) {
                    if let SymbolKind::EnumConstructor = symbol.kind {
                        let ctor = &self.registry.enum_ctors[symbol.col_index];
                        let enum_ = &self.registry.enums[ctor.enum_index];
                        let variant = &enum_.variants[ctor.variant_index];

                        instrs.push(WasmInstr::I32Const {
                            value: ctor.variant_index as i32,
                        });

                        if variant.variant_type == Type::Void && args.items.len() == 0 {
                            return;
                        }

                        self.codegen(ctx, instrs, &args.items[0]);
                        return;
                    }
                }

                self.codegen_fn_call(ctx, instrs, &fn_name.repr, None, &args.items);
            }
            CodeExpr::MethodCall(MethodCallExpr {
                id: _,
                lhs,
                field_name,
                args,
                loc: _,
            }) => {
                let lhs_type = self.get_expr_type(ctx, lhs);
                let fn_name = self
                    .registry
                    .get_fn_name_from_method(&lhs_type, &field_name.repr);
                self.codegen_fn_call(ctx, instrs, &fn_name, Some(lhs), &args.items);
            }
            CodeExpr::InlineFnCall(InlineFnCallExpr {
                id,
                fn_name,
                type_args,
                args,
                loc: _,
            }) => {
                self.codegen_inline_fn_call(
                    ctx,
                    instrs,
                    &fn_name.repr,
                    type_args,
                    None,
                    &args.items,
                    *id,
                    &fn_name.loc,
                );
            }
            CodeExpr::InlineMethodCall(InlineMethodCallExpr {
                id,
                lhs,
                field_name,
                type_args,
                args,
                loc: _,
            }) => {
                let lhs_type = self.get_expr_type(ctx, lhs);
                let inline_fn_name = self
                    .registry
                    .get_fn_name_from_method(&lhs_type, &field_name.repr);
                self.codegen_inline_fn_call(
                    ctx,
                    instrs,
                    &inline_fn_name,
                    type_args,
                    Some(lhs),
                    &args.items,
                    *id,
                    &field_name.loc,
                );
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
                        .reporter
                        .fm
                        .resolve_path(&str_expr.value, &call.fn_name.loc);
                    let bytes = match fs::file_read(&absolute_path) {
                        Ok(value) => value,
                        // TODO!: move to typer?
                        Err(err_message) => self
                            .reporter
                            .abort_due_to_compiler_bug(&err_message, call.args.items[0].loc()),
                    };

                    let bytes_len = bytes.len();
                    let bytes_ptr = self.append_data(bytes);

                    instrs.push(WasmInstr::I32Const {
                        value: bytes_ptr as i32,
                    });

                    self.const_slice_lens.be_mut().push(ConstSliceLen {
                        slice_ptr: bytes_ptr,
                        slice_len: bytes_len,
                    });

                    return;
                }

                // TODO: move the len extraction into typer?
                if call.fn_name.repr == "const_slice_len" {
                    let mut slice_ptr_instrs = Vec::new();
                    self.codegen(ctx, &mut slice_ptr_instrs, &call.args.items[0]);
                    let WasmInstr::I32Const { value: slice_ptr } = &slice_ptr_instrs[0] else {
                        unreachable!()
                    };

                    for const_slice_len in &self.const_slice_lens {
                        if const_slice_len.slice_ptr == *slice_ptr as u32 {
                            instrs.push(WasmInstr::I32Const {
                                value: const_slice_len.slice_len as i32,
                            });
                            return;
                        }
                    }
                    unreachable!();
                }

                if call.fn_name.repr == "inline_fn_call_loc" {
                    let mut inline_fn_call_loc = None;
                    // NOTE: iterating in not-reverse to get the first inline scope
                    for scope in &self.registry.modules[ctx.module_id].scope_stack {
                        if scope.kind == ScopeKind::InlineFn {
                            inline_fn_call_loc = scope.inline_fn_call_loc.clone();
                        }
                    }

                    let loc_str = self.process_const_string(
                        &inline_fn_call_loc.unwrap().to_string(&self.reporter.fm),
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

                if call.fn_name.repr == "get_ok" {
                    let arg_type = self.get_expr_type(ctx, &call.args.items[0]);

                    let Type::Result(ResultType { ok: _, err }) = arg_type else {
                        unreachable!()
                    };

                    self.codegen(ctx, instrs, &call.args.items[0]);

                    // drop `err` leaving only `ok` on the stack
                    for _ in 0..count_primitive_components(&self.registry, &err) {
                        instrs.push(WasmInstr::Drop);
                    }

                    return;
                }

                if call.fn_name.repr == "get_err" {
                    let arg_type = self.get_expr_type(ctx, &call.args.items[0]);

                    let Type::Result(ResultType { ok, err }) = arg_type else {
                        unreachable!()
                    };

                    self.codegen(ctx, instrs, &call.args.items[0]);

                    // push `err` to temp local
                    let tmp_local_index = self.define_unnamed_local(ctx, Loc::internal(), &err);
                    let tmp_local = VarInfo::Local(VarInfoLocal {
                        local_index: tmp_local_index,
                        var_type: *err.clone(),
                    });
                    self.codegen_local_set(instrs, &err, tmp_local_index);

                    // drop `ok`
                    for _ in 0..count_primitive_components(&self.registry, &ok) {
                        instrs.push(WasmInstr::Drop);
                    }

                    // pop `err` back
                    self.codegen_var_get(ctx, instrs, &tmp_local);

                    return;
                }

                self.report_error(&Error {
                    message: format!("Unknown intrinsic: {}", call.fn_name.repr),
                    loc: call.fn_name.loc,
                });
            }
            CodeExpr::Sizeof(SizeofExpr {
                id: _,
                type_expr,
                loc: _,
            }) => {
                let lo_type = self.unwrap(self.registry.build_type(ctx, type_expr));
                let mut type_layout = TypeLayout::new();
                get_type_layout(&self.registry, &lo_type, &mut type_layout);

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
                        self.registry.be_mut().enter_scope(ctx, ScopeKind::Block);
                    }
                    IfCond::Match(match_header) => {
                        // `if match` condition runs inside of then_branch's scope
                        self.registry.be_mut().enter_scope(ctx, ScopeKind::Block);

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
                self.registry.be_mut().exit_scope(ctx);

                match else_block {
                    ElseBlock::None => {}
                    ElseBlock::Else(code_block_expr) => {
                        instrs.push(WasmInstr::Else);
                        self.registry.be_mut().enter_scope(ctx, ScopeKind::Block);
                        self.codegen_code_block(ctx, instrs, &code_block_expr, true);
                        self.registry.be_mut().exit_scope(ctx);
                    }
                    ElseBlock::ElseIf(code_expr) => {
                        instrs.push(WasmInstr::Else);
                        self.registry.be_mut().enter_scope(ctx, ScopeKind::Block);
                        self.codegen(ctx, instrs, &code_expr);
                        self.registry.be_mut().exit_scope(ctx);
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

                self.registry.be_mut().enter_scope(ctx, ScopeKind::Block);
                self.codegen_code_block(ctx, instrs, &else_branch, true);
                self.registry.be_mut().exit_scope(ctx);
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

                self.registry.be_mut().enter_scope(ctx, ScopeKind::Loop);
                self.codegen_code_block(ctx, instrs, body, true);
                self.registry.be_mut().exit_scope(ctx);

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

                self.registry.be_mut().enter_scope(ctx, ScopeKind::ForLoop);

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

                self.registry.be_mut().exit_scope(ctx);
            }
            CodeExpr::Break(BreakExpr { id: _, loc }) => {
                let mut label_index = 1; // 0 = loop, 1 = loop wrapper block

                for scope in self.registry.modules[ctx.module_id]
                    .scope_stack
                    .iter()
                    .rev()
                {
                    match scope.kind {
                        ScopeKind::Block => {
                            label_index += 1;
                        }
                        ScopeKind::Function => {
                            // TODO!: move to typer
                            self.reporter.abort_due_to_compiler_bug(
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

                for scope in self.registry.modules[ctx.module_id]
                    .scope_stack
                    .iter()
                    .rev()
                {
                    match scope.kind {
                        ScopeKind::Block => {
                            label_index += 1;
                        }
                        ScopeKind::Function => {
                            // TODO!: move to typer
                            self.reporter.abort_due_to_compiler_bug(
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
                    self.registry.be_mut().enter_scope(ctx, ScopeKind::InlineFn);

                    self.codegen(ctx, instrs, arg);

                    let arg_local_index = self.define_local(ctx, with_loc.clone(), "it", &arg_type);

                    self.codegen_local_set(instrs, &arg_type, arg_local_index);
                    self.codegen(ctx, instrs, body);

                    self.registry.be_mut().exit_scope(ctx);
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

                self.registry.be_mut().enter_scope(ctx, ScopeKind::InlineFn);

                let lhs_local_index = self.define_local(ctx, op_loc.clone(), "it", &lhs_type);

                self.codegen_local_set(instrs, &lhs_type, lhs_local_index);
                self.codegen(ctx, instrs, rhs);
                self.registry.be_mut().exit_scope(ctx);
            }
            CodeExpr::Defer(DeferExpr {
                id: _,
                expr,
                loc: _,
            }) => {
                let code_unit = self.build_code_unit(ctx, expr);

                // find first non-inline-fn scope
                let mut scope_to_defer = self.registry.modules[ctx.module_id]
                    .scope_stack
                    .relax_mut()
                    .last_mut()
                    .unwrap();
                for scope in self.registry.modules[ctx.module_id]
                    .scope_stack
                    .iter_mut()
                    .rev()
                {
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
        let Some(symbol) = self
            .registry
            .current_scope(ctx)
            .get_symbol(&header.variant_name.repr)
        else {
            unreachable!()
        };
        let SymbolKind::EnumConstructor = symbol.kind else {
            unreachable!()
        };

        let enum_ctor = &self.registry.enum_ctors[symbol.col_index].relax();
        let enum_def = &self.registry.enums[enum_ctor.enum_index].relax();
        let enum_variant = &enum_def.variants[enum_ctor.variant_index].relax();

        let local_index = self.define_local(
            ctx,
            header.variant_bind.loc,
            header.variant_bind.repr,
            &enum_variant.variant_type,
        );
        let local = VarInfo::Local(VarInfoLocal {
            local_index,
            var_type: enum_variant.variant_type.clone(),
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
        fn_name: &str,
        receiver_arg: Option<&CodeExpr>,
        args: &Vec<CodeExpr>,
    ) {
        // TODO: extract fn_index from call_info that's to be stored by typer
        let Some(symbol) = self.registry.current_scope(ctx).get_symbol(fn_name) else {
            unreachable!()
        };
        let SymbolKind::Function = symbol.kind else {
            unreachable!()
        };
        let fn_info = self.registry.functions[symbol.col_index].relax();

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

    fn populate_ctx_from_inline_fn_call(
        &mut self,
        ctx: &mut ExprContext,
        inline_fn_name: &str,
        type_args: &Vec<TypeExpr>,
        receiver_arg: Option<&CodeExpr>,
        args: &Vec<CodeExpr>,
    ) -> &FnExpr {
        let Some(symbol) = self.registry.current_scope(ctx).get_symbol(inline_fn_name) else {
            unreachable!();
        };

        let inline_fn_def = self.registry.inline_fns[symbol.col_index];

        let mut all_args = Vec::new();
        if let Some(receiver_arg) = receiver_arg {
            all_args.push(self.build_code_unit(ctx, receiver_arg));
        }
        for arg in args {
            all_args.push(self.build_code_unit(ctx, arg));
        }

        let mut lo_type_args = Vec::new();
        for type_arg in type_args {
            lo_type_args.push(self.unwrap(self.registry.build_type(ctx, &type_arg)));
        }

        for (i, (type_param, type_arg)) in inline_fn_def
            .decl
            .type_params
            .iter()
            .zip(lo_type_args.iter())
            .enumerate()
        {
            self.register_block_type(ctx, type_param, type_arg.clone(), *type_args[i].loc());
        }

        for (inline_fn_param, inline_fn_arg) in
            inline_fn_def.decl.params.iter().zip(all_args.into_iter())
        {
            let const_def = ConstDef {
                const_name: inline_fn_param.param_name.repr,
                code_unit: inline_fn_arg,
                loc: inline_fn_param.loc,
            };

            if let Some(type_name) = self.unwrap(get_infer_type_name(inline_fn_param)) {
                self.register_block_type(
                    ctx,
                    type_name,
                    const_def.code_unit.type_.clone(),
                    inline_fn_param.loc,
                );
            }

            self.register_block_const(ctx, const_def);
        }

        inline_fn_def
    }

    fn codegen_inline_fn_call(
        &mut self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        inline_fn_name: &str,
        type_args: &Vec<TypeExpr>,
        receiver_arg: Option<&CodeExpr>,
        args: &Vec<CodeExpr>,
        call_expr_id: ExprId,
        loc: &Loc,
    ) {
        self.registry.be_mut().enter_scope(ctx, ScopeKind::InlineFn);
        self.registry.current_scope(ctx).be_mut().inline_fn_call_loc = Some(*loc);

        let inline_fn_def = self.relax_mut().populate_ctx_from_inline_fn_call(
            ctx,
            inline_fn_name,
            type_args,
            receiver_arg,
            args,
        );

        let FnExprValue::Body(body) = &inline_fn_def.value else {
            unreachable!()
        };

        if let Some(expr_info_id) = self.get_expr_info(ctx, call_expr_id) {
            let call_info = &self.registry.inline_fn_call_info[expr_info_id];
            self.registry.current_scope(ctx).be_mut().expr_info_offset =
                call_info.inner_expr_offset;
        }

        self.codegen_code_block(ctx, instrs, body, false);

        self.registry.be_mut().exit_scope(ctx);
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

        self.registry.be_mut().enter_scope(ctx, ScopeKind::Block); // enter catch scope

        // put result on the stack
        self.codegen(ctx, instrs, expr);

        // pop error
        let (error_bind, error_bind_loc) = if let Some(error_bind) = error_bind {
            (error_bind.repr, error_bind.loc)
        } else {
            ("<err>", Loc::internal())
        };
        let err_local_index =
            self.define_local(ctx, error_bind_loc.clone(), error_bind, &result.err);
        let err_var = VarInfo::Local(VarInfoLocal {
            local_index: err_local_index,
            var_type: result.err.as_ref().clone(),
        });
        self.codegen_var_set_prepare(instrs, &err_var);
        self.codegen_var_set(ctx, instrs, &err_var);

        // pop ok
        let ok_local_index = self.define_local(ctx, *loc, "<ok>", &result.ok);
        self.codegen_local_set(instrs, &result.ok, ok_local_index);

        // cond: error != 0
        self.codegen_var_get(ctx, instrs, &err_var);

        let in_out_type_index = self.get_block_inout_type(&[], &result.ok);
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
            self.codegen_default_value(ctx, instrs, &fn_result.ok);
            self.codegen_var_get(ctx, instrs, &err_var);

            self.emit_deferred_for_return(ctx, instrs);
            instrs.push(WasmInstr::Return);
        }

        instrs.push(WasmInstr::Else);

        // no error, push ok value
        let ok_var = VarInfo::Local(VarInfoLocal {
            local_index: ok_local_index,
            var_type: result.ok.as_ref().clone(),
        });
        self.codegen_var_get(ctx, instrs, &ok_var);

        instrs.push(WasmInstr::BlockEnd);

        self.registry.be_mut().exit_scope(ctx); // exit catch scope
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
            Type::EnumInstance { enum_index } => {
                let enum_def = &self.registry.enums[*enum_index];

                let mut tag_layout = TypeLayout::new();
                get_type_layout(&self.registry, &Type::U32, &mut tag_layout);

                self.codegen_load_or_store(
                    instrs,
                    &enum_def.variant_type,
                    offset + tag_layout.byte_size,
                    is_store,
                );
                self.codegen_load_or_store(instrs, &Type::U32, offset, is_store);
            }
            Type::Result(ResultType { ok, err }) => {
                let mut ok_layout = TypeLayout::new();
                get_type_layout(&self.registry, &ok, &mut ok_layout);

                self.codegen_load_or_store(instrs, &err, offset + ok_layout.byte_size, is_store);
                self.codegen_load_or_store(instrs, &ok, offset, is_store);
            }
            Type::Container(ContainerType {
                container,
                items: _,
            }) => {
                self.codegen_load_or_store(instrs, container, offset, is_store);
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

    fn codegen_var_get(&self, ctx: &mut ExprContext, instrs: &mut Vec<WasmInstr>, var: &VarInfo) {
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
            VarInfo::Const(VarInfoConst { code_unit, loc: _ }) => {
                instrs.extend_from_slice(&code_unit.instrs);
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
                self.reporter
                    .abort_due_to_compiler_bug(&format!("Cannot mutate a constant"), *loc);
            }
        };
    }

    fn define_local(
        &mut self,
        ctx: &mut ExprContext,
        loc: Loc,
        local_name: &'static str,
        local_type: &Type,
    ) -> u32 {
        let res = self
            .registry
            .define_symbol(ctx, local_name, SymbolKind::Local, loc);

        if let Err(existing) = res
            && existing.kind == SymbolKind::Local
        {
            return ctx.locals[existing.col_index].local_index;
        }

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

    fn register_block_const(&mut self, ctx: &ExprContext, const_def: ConstDef) {
        if const_def.const_name == "_" {
            return;
        }

        let _ = self.registry.define_symbol(
            ctx,
            const_def.const_name,
            SymbolKind::Const,
            const_def.loc,
        );
        self.registry.constants.push(const_def);
    }

    fn register_block_type(
        &mut self,
        ctx: &ExprContext,
        name: &'static str,
        type_: Type,
        loc: Loc,
    ) {
        let _ = self
            .registry
            .define_symbol(ctx, name, SymbolKind::TypeAlias, loc);
        self.registry.type_aliases.push(type_);
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
        for scope in self.registry.modules[ctx.module_id]
            .scope_stack
            .iter()
            .rev()
        {
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
        let Some(symbol) = self.registry.current_scope(ctx).get_symbol(var_name.repr) else {
            unreachable!()
        };

        match symbol.kind {
            SymbolKind::Local => {
                let local = &ctx.locals[symbol.col_index];

                VarInfo::Local(VarInfoLocal {
                    local_index: local.local_index,
                    var_type: local.local_type.clone(),
                })
            }
            SymbolKind::Global => {
                let global = &self.registry.globals[symbol.col_index];

                VarInfo::Global(VarInfoGlobal {
                    global_index: global.global_index,
                    var_type: global.global_type.clone(),
                })
            }
            SymbolKind::Const => {
                let const_def = &self.registry.constants[symbol.col_index];

                VarInfo::Const(VarInfoConst {
                    code_unit: const_def.code_unit.relax(),
                    loc: var_name.loc,
                })
            }
            SymbolKind::EnumConstructor => {
                let enum_ctor = &self.registry.enum_ctors[symbol.col_index];

                let var_type = Type::EnumInstance {
                    enum_index: enum_ctor.enum_index,
                };

                VarInfo::VoidEnumValue(VarInfoVoidEnumValue {
                    variant_index: enum_ctor.variant_index,
                    var_type,
                    loc: var_name.loc,
                })
            }
            SymbolKind::TypeAlias
            | SymbolKind::Struct
            | SymbolKind::Enum
            | SymbolKind::InlineFn
            | SymbolKind::Function => unreachable!(),
        }
    }

    fn var_from_field_access(
        &mut self,
        ctx: &mut ExprContext,
        field_access: &FieldAccessExpr,
    ) -> VarInfo {
        let lhs_type = self.get_expr_type(ctx, field_access.lhs.as_ref());

        let field = self
            .get_struct_or_struct_ref_field(&lhs_type, field_access)
            .relax();

        if let Type::Pointer { pointee: _ } = lhs_type {
            return VarInfo::Stored(VarInfoStored {
                address: self.build_code_unit(ctx, &field_access.lhs),
                offset: field.byte_offset,
                var_type: field.field_type.clone(),
            });
        }

        if let Some(var) = self.var_from_expr(ctx, &field_access.lhs.as_ref()) {
            match var {
                // TODO: update this since struct globals are now supported
                // struct globals are not supported so these are handled the same way as struct values
                VarInfo::Global(_) => {}
                // consts are handled as struct values as well
                VarInfo::Const(_) => {}
                // void enums are handled as struct values as well
                VarInfo::VoidEnumValue(_) => {}
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

    fn get_struct_or_struct_ref_field(
        &self,
        mut lhs_type: &Type,
        field_access: &FieldAccessExpr,
    ) -> &StructField {
        if let Type::Pointer { pointee } = &lhs_type {
            lhs_type = pointee;
        }

        let struct_index: usize;
        if let Type::StructInstance { struct_index: si } = lhs_type {
            struct_index = *si;
        } else if let Type::Container(ContainerType {
            container,
            items: _,
        }) = lhs_type
            && let Type::StructInstance { struct_index: si } = &**container
        {
            struct_index = *si;
        } else {
            unreachable!()
        };

        let struct_def = &self.registry.structs[struct_index];
        let Some(field) = struct_def
            .fields
            .iter()
            .find(|f| &f.field_name == &field_access.field_name.repr)
        else {
            unreachable!()
        };

        field
    }

    fn var_from_deref(&mut self, ctx: &mut ExprContext, addr_expr: &CodeExpr) -> VarInfo {
        let addr_type = self.get_expr_type(ctx, addr_expr);

        let Type::Pointer { pointee } = &addr_type else {
            unreachable!()
        };

        return VarInfo::Stored(VarInfoStored {
            address: self.build_code_unit(ctx, addr_expr),
            offset: 0,
            var_type: pointee.as_ref().clone(),
        });
    }

    fn build_code_unit(&mut self, ctx: &mut ExprContext, expr: &CodeExpr) -> CodeUnit {
        let mut code_unit = CodeUnit {
            type_: self.get_expr_type(ctx, expr).clone(),
            instrs: Vec::new(),
        };
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
            | Type::Pointer { pointee: _ }
            | Type::SequencePointer { pointee: _ } => instrs.push(WasmInstr::I32Const { value: 0 }),
            Type::U64 | Type::I64 => instrs.push(WasmInstr::I64Const { value: 0 }),
            Type::F32 => instrs.push(WasmInstr::F32Const { value: 0.0 }),
            Type::F64 => instrs.push(WasmInstr::F64Const { value: 0.0 }),
            Type::StructInstance { struct_index } => {
                let struct_ref = &self.registry.structs[*struct_index];
                for field in &struct_ref.fields {
                    self.codegen_default_value(ctx, instrs, &field.field_type);
                }
            }
            Type::EnumInstance { enum_index } => {
                let enum_def = &self.registry.enums[*enum_index];

                self.codegen_default_value(ctx, instrs, &Type::U32);
                self.codegen_default_value(ctx, instrs, &enum_def.variant_type);
            }
            Type::Result(result) => {
                self.codegen_default_value(ctx, instrs, &result.ok);
                self.codegen_default_value(ctx, instrs, &result.err);
            }
            Type::Container(ContainerType {
                container,
                items: _,
            }) => {
                self.codegen_default_value(ctx, instrs, container);
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
            Type::Null
            | Type::Bool
            | Type::U8
            | Type::U16
            | Type::U32
            | Type::Pointer { pointee: _ }
            | Type::SequencePointer { pointee: _ } => {}
            Type::EnumInstance { enum_index }
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
            | Type::EnumInstance { enum_index: _ }
            | Type::StructInstance { struct_index: _ }
            | Type::Result(_)
            | Type::Container(_) => {
                // TODO!: move to typer
                self.reporter.abort_due_to_compiler_bug(
                    &format!(
                        "Operator `{}` is incompatible with operands of type {}",
                        op_loc.read_span(&self.registry.modules[ctx.module_id].source),
                        TypeFmt(&*self.registry, operand_type)
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
            self_.reporter.abort_due_to_compiler_bug(
                &format!(
                    "Operator `{}` is incompatible with operands of type {}",
                    op_loc.read_span(&self_.registry.modules[ctx.module_id].source),
                    TypeFmt(&*self_.registry, op_type)
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
                let struct_def = &self.registry.structs[*struct_index];

                for field in &struct_def.fields {
                    self.lower_type(&field.field_type, wasm_types);
                }
            }
            Type::EnumInstance { enum_index } => {
                let enum_def = &self.registry.enums[*enum_index];

                self.lower_type(&Type::U32, wasm_types);
                self.lower_type(&enum_def.variant_type, wasm_types);
            }
            Type::Result(result) => {
                self.lower_type(&result.ok, wasm_types);
                self.lower_type(&result.err, wasm_types);
            }
            Type::Container(ContainerType {
                container,
                items: _,
            }) => {
                self.lower_type(container, wasm_types);
            }
        }
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
            | Type::Pointer { pointee: _ }
            | Type::SequencePointer { pointee: _ } => output.push(WasmLocalInfo {
                local_index,
                local_name,
            }),
            Type::StructInstance { struct_index } => {
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
            Type::EnumInstance { enum_index } => {
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
                    &result_type.ok,
                    alloc::format!("{}#ok", local_name),
                );

                self.push_wasm_dbg_name_section_locals(
                    output,
                    local_index,
                    &&result_type.err,
                    alloc::format!("{}#err", local_name),
                );
            }
            Type::Container(ContainerType {
                container,
                items: _,
            }) => {
                self.push_wasm_dbg_name_section_locals(output, local_index, container, local_name);
            }
        }
    }

    fn get_expr_type(&self, ctx: &ExprContext, expr: &CodeExpr) -> &'static Type {
        let Some(expr_info_id) = self.get_expr_info(ctx, expr.id()) else {
            self.reporter
                .abort_due_to_compiler_bug("Untyped expression", expr.loc());
        };

        match expr {
            CodeExpr::InlineFnCall(_) | CodeExpr::InlineMethodCall(_) => {
                let call_info = &self.registry.inline_fn_call_info[expr_info_id];
                return &self.registry.types[call_info.return_type_id].relax();
            }
            _ => {
                return &self.registry.types[expr_info_id].relax();
            }
        }
    }

    fn get_expr_info(&self, ctx: &ExprContext, expr_id: ExprId) -> Option<ExprInfo> {
        let scope = self.registry.current_scope(ctx);

        let expr_info = self.registry.expr_info[scope.expr_info_offset + expr_id];
        if expr_info == EXPR_INFO_INVALID {
            return None;
        }

        Some(expr_info)
    }

    fn unwrap<T>(&self, result: Result<T, Error>) -> T {
        match result {
            Ok(value) => value,
            Err(err) => self
                .reporter
                .abort_due_to_compiler_bug(&err.message, err.loc),
        }
    }

    // TODO: remove tag after migration
    fn report_error(&self, err: &Error) {
        let marked_error = Error {
            message: format!("(codegen) {}", err.message),
            loc: err.loc.clone(),
        };
        self.reporter.error(&marked_error);
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
