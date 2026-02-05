use crate::{ast::*, common::*, lexer::*, registry::*, typer::*, wasm::*};

pub struct CodeGenerator {
    // context
    pub registry: UBRef<Registry>,
    pub reporter: UBRef<Reporter>,

    // state
    wasm_fn_types: UBCell<Vec<WasmFnType>>,

    // output
    pub wasm_module: UBCell<WasmModule>,
}

impl CodeGenerator {
    pub fn new(registry: &mut Registry) -> Self {
        Self {
            registry: UBRef::new(&mut *registry),
            reporter: UBRef::new(&mut *registry.reporter),
            wasm_fn_types: Default::default(),
            wasm_module: Default::default(),
        }
    }

    pub fn codegen_all(&mut self) {
        self.codegen_top_level_leftover();

        let mut fn_imports_count = 0;
        for fn_info in &self.registry.functions {
            if let FnSource::Host { .. } = fn_info.fn_source {
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
            for lo_input_type in &fn_info.fn_type.inputs {
                self.registry
                    .lower_type(lo_input_type, &mut wasm_fn_type.inputs);
            }
            self.registry
                .lower_type(&fn_info.fn_type.output, &mut wasm_fn_type.outputs);

            let mut fn_type_index = self.wasm_fn_types.len() as u32;
            for (existing_fn_type, existing_type_index) in self.wasm_fn_types.iter().zip(0..) {
                if wasm_fn_type == *existing_fn_type {
                    fn_type_index = existing_type_index;
                }
            }
            if fn_type_index == self.wasm_fn_types.len() as u32 {
                self.wasm_fn_types.be_mut().push(wasm_fn_type.clone());
            }

            match &fn_info.fn_source {
                FnSource::Guest { .. } => {
                    self.wasm_module.functions.push(fn_type_index);
                    self.wasm_module.function_names.push(WasmFnNameItem {
                        fn_index: wasm_fn_index,
                        fn_name: String::from(fn_info.fn_name),
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
            self.registry
                .lower_type(&global.global_type, &mut wasm_types);

            let mut instrs = Vec::new();
            let res = self.codegen(
                global.module_ctx.be_mut(),
                &mut instrs,
                &global.def_expr.value,
            );
            catch!(res, err, {
                self.report_error(&err);
            });

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
            } = &fn_info.fn_source
            else {
                continue;
            };

            let ctx = &mut ExprContext::new(*module_id, Some(*lo_fn_index));
            let mut wasm_expr = WasmExpr { instrs: Vec::new() };

            let constants_len = self.registry.constants.len();
            let type_aliases_len = self.registry.type_aliases.len();

            self.registry.be_mut().enter_scope(ctx, ScopeKind::Function);

            for fn_param in &fn_info.fn_params {
                self.registry.define_local(
                    ctx,
                    fn_param.loc,
                    fn_param.param_name,
                    &fn_param.param_type,
                );
            }

            self.codegen_code_block(ctx, &mut wasm_expr.instrs, body, true);

            self.registry.be_mut().exit_scope(ctx);

            // remove any constants/types created by inline fn calls
            self.registry.constants.truncate(constants_len);
            self.registry.type_aliases.truncate(type_aliases_len);

            let mut wasm_locals_flat = Vec::new();
            for (i, local) in ctx.locals.iter().enumerate() {
                let is_fn_param = i < fn_info.fn_params.len();
                if is_fn_param {
                    continue;
                }

                self.registry
                    .lower_type(&local.local_type, &mut wasm_locals_flat);
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
                    let file_index = local.definition_loc.file_index;
                    if file_index == 0 {
                        continue;
                    }

                    let mut module = &self.registry.modules[ctx.module_id];
                    if file_index != module.parser.lexer.file_index {
                        // local defined by inline fn from another module
                        module = &self.registry.get_module_by_file_index(file_index).unwrap();
                    }

                    self.push_wasm_dbg_name_section_locals(
                        &mut local_names_item.locals,
                        local.local_index,
                        &local.local_type,
                        String::from(local.definition_loc.read_span(module.source)),
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

        for static_data_store in self.registry.datas.iter() {
            self.wasm_module.datas.push(static_data_store.clone());
        }

        self.wasm_module.types.append(self.wasm_fn_types.be_mut());

        if let Some(string_usage_loc) = *self.registry.first_string_usage
            && self.registry.memory.is_none()
            && !self.reporter.in_inspection_mode
        {
            self.report_error(&Error {
                message: format!("Cannot use strings with no memory defined"),
                loc: string_usage_loc,
            });
        }
    }

    // TODO: cleanup
    fn codegen_top_level_leftover(&mut self) {
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

                        let const_type =
                            match self.registry.get_expr_type(&module.ctx, &let_expr.value) {
                                Ok(x) => x,
                                Err(err) => {
                                    self.report_error(&err);
                                    continue;
                                }
                            };
                        const_.code_unit.type_ = const_type;

                        if let Err(err) = self.codegen(
                            module.ctx.be_mut(),
                            &mut const_.code_unit.instrs,
                            &let_expr.value,
                        ) {
                            self.report_error(&err);
                            continue;
                        };

                        if self.reporter.in_inspection_mode {
                            let const_name = &let_expr.name.repr;

                            self.reporter.print_inspection(&InspectInfo {
                                message: format!(
                                    "inline let {const_name}: {}",
                                    TypeFmt(&*self.registry, &const_.code_unit.type_)
                                ),
                                loc: let_expr.name.loc,
                                linked_loc: None,
                            });
                        }
                    }
                    TopLevelExpr::Intrinsic(InlineFnCallExpr {
                        id,
                        fn_name: intrinsic,
                        type_args,
                        args,
                        loc,
                    }) if intrinsic.repr == "use_memory" => {
                        if let Some(existing_memory) = &self.registry.memory {
                            self.report_error(&Error {
                                message: format!(
                                    "Cannot redefine memory, first defined at {}",
                                    existing_memory.loc.to_string(&self.reporter.fm)
                                ),
                                loc: intrinsic.loc,
                            });
                            continue;
                        }

                        if type_args.len() != 0 {
                            self.report_error(&bad_signature(intrinsic));
                        }

                        let mut memory = MemoryInfo {
                            min_pages: None,
                            data_start: None,
                            exported: false,
                            imported_from: None,
                            loc: intrinsic.loc,
                        };

                        let ctx = &mut module.be_mut().ctx;
                        let tmp_instrs = &mut Vec::new();

                        for arg in &args.items {
                            if let CodeExpr::Ident(ident) = arg
                                && ident.repr == "exported"
                            {
                                memory.exported = true;
                                continue;
                            }

                            let CodeExpr::Assign(AssignExpr {
                                lhs, rhs: value, ..
                            }) = arg
                            else {
                                self.report_error(&bad_signature(intrinsic));
                                continue;
                            };

                            let CodeExpr::Ident(key) = &**lhs else {
                                self.report_error(&bad_signature(intrinsic));
                                continue;
                            };

                            if key.repr == "data_start" {
                                tmp_instrs.clear();
                                if let Err(err) = self.codegen(ctx, tmp_instrs, &value) {
                                    self.report_error(&err);
                                };

                                if tmp_instrs.len() == 1 {
                                    if let WasmInstr::I32Const { value } = &tmp_instrs[0] {
                                        memory.data_start = Some(*value as u32);
                                        *self.registry.data_size.be_mut() = *value as u32;
                                        continue;
                                    }
                                }

                                self.report_error(&bad_signature(intrinsic));
                                continue;
                            }

                            if key.repr == "min_pages" {
                                tmp_instrs.clear();
                                if let Err(err) = self.codegen(ctx, tmp_instrs, &value) {
                                    self.report_error(&err);
                                };

                                if tmp_instrs.len() == 1 {
                                    if let WasmInstr::I32Const { value } = &tmp_instrs[0] {
                                        memory.min_pages = Some(*value as u32);
                                        continue;
                                    }
                                }

                                self.report_error(&bad_signature(intrinsic));
                                continue;
                            }

                            if key.repr == "import_from" {
                                let CodeExpr::StringLiteral(str) = &**value else {
                                    self.report_error(&bad_signature(intrinsic));
                                    continue;
                                };

                                memory.imported_from = Some(str.value.clone());
                                continue;
                            }

                            self.report_error(&bad_signature(intrinsic));
                        }

                        self.registry.memory = Some(memory);

                        continue;

                        fn bad_signature(fn_name: &IdentExpr) -> Error {
                            Error {
                                message: format!(
                                    "Invalid call, expected signature: @{}([min_pages = <u32>, data_start = <u32>, exported, import_from = <str-literal>])",
                                    fn_name.repr
                                ),
                                loc: fn_name.loc,
                            }
                        }
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
    ) -> bool {
        let mut naturally_diverges = false;
        let mut terminates_early = false;

        for expr in &body.exprs {
            let expr_type = catch!(self.registry.get_expr_type(ctx, expr), err, {
                self.report_error(&err);
                continue;
            });

            if terminates_early {
                self.reporter.warning(&Error {
                    message: format!("Unreachable expression"),
                    loc: expr.loc(),
                });
            }

            if expr_type == Type::Never {
                terminates_early = true;

                naturally_diverges = naturally_diverges || self.is_naturally_divergent(expr);
            }

            let mut type_layout = TypeLayout::new();
            self.registry.get_type_layout(&expr_type, &mut type_layout);
            if type_layout.primities_count > 0 && void_only {
                self.report_error(&Error {
                    message: format!(
                        "Non void expression in block. Use `let _ = <expr>` to ignore expression result."
                    ),
                    loc: expr.loc(),
                });
            }

            catch!(self.codegen(ctx, instrs, expr), err, {
                self.report_error(&err);
                continue;
            });
        }

        self.emit_deferred(self.registry.current_scope(ctx), instrs);

        if void_only && terminates_early && !naturally_diverges {
            instrs.push(WasmInstr::Unreachable);
        }

        // TODO: move this check to typer
        if !naturally_diverges
            && !terminates_early
            && self.registry.current_scope(ctx).kind == ScopeKind::Function
        {
            let fn_info = &self.registry.functions[ctx.fn_index.unwrap()];
            if fn_info.fn_type.output != Type::Void {
                self.report_error(&Error {
                    // error message stolen from clang
                    message: format!("Control reaches end of non-void function"),
                    loc: fn_info.definition_loc,
                });
            }
        }

        terminates_early
    }

    // TODO: this should report errors more (case-by-case decision)
    fn codegen(
        &mut self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        expr: &CodeExpr,
    ) -> Result<(), Error> {
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
            CodeExpr::IntLiteral(int_literal) => {
                self.codegen_int_literal(ctx, instrs, int_literal);
            }
            CodeExpr::StringLiteral(StringLiteralExpr {
                id: _,
                repr: _,
                value,
                loc,
            }) => {
                let str = self.process_const_string(value.clone(), loc);

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
                let Type::StructInstance { struct_index } =
                    self.registry
                        .get_type_or_err(ctx, &struct_name.repr, &struct_name.loc)?
                else {
                    return Err(Error {
                        message: format!("Unknown struct: {}", struct_name.repr),
                        loc: struct_name.loc,
                    });
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

                    catch!(self.codegen(ctx, instrs, &field_literal.value), err, {
                        self.report_error(&err);
                        continue;
                    });

                    let field_value_type =
                        self.registry.get_expr_type(ctx, &field_literal.value)?;
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
                let item_type = self.registry.build_type(ctx, item_type)?;

                let mut bytes = Vec::new();
                let mut tmp_instrs = Vec::new();

                if let Type::U8 = &item_type {
                    for item in items {
                        let current_item_type = self.registry.get_expr_type(ctx, item)?;
                        if current_item_type != item_type {
                            return Err(Error {
                                message: format!(
                                    "Unexpected array element type: {}, expected: {}",
                                    TypeFmt(&*self.registry, &current_item_type),
                                    TypeFmt(&*self.registry, &item_type),
                                ),
                                loc: item.loc(),
                            });
                        }

                        self.codegen(ctx, &mut tmp_instrs, item)?;
                        let WasmInstr::I32Const { value } = tmp_instrs.pop().unwrap() else {
                            return Err(Error {
                                message: format!("Unexpected array element value"),
                                loc: item.loc(),
                            });
                        };

                        bytes.push(value as u8);
                    }
                } else if let Type::StructInstance { struct_index } = &item_type
                    && self.registry.structs[*struct_index].struct_name == "str"
                {
                    for item in items {
                        let current_item_type = self.registry.get_expr_type(ctx, item)?;
                        if current_item_type != item_type {
                            return Err(Error {
                                message: format!(
                                    "Unexpected array element type: {}, expected: {}",
                                    TypeFmt(&*self.registry, &current_item_type),
                                    TypeFmt(&*self.registry, &item_type),
                                ),
                                loc: item.loc(),
                            });
                        }

                        self.codegen(ctx, &mut tmp_instrs, item)?;
                        let WasmInstr::I32Const { value: len } = tmp_instrs.pop().unwrap() else {
                            return Err(Error {
                                message: format!("Unexpected array element value"),
                                loc: item.loc(),
                            });
                        };
                        let WasmInstr::I32Const { value: ptr } = tmp_instrs.pop().unwrap() else {
                            return Err(Error {
                                message: format!("Unexpected array element value"),
                                loc: item.loc(),
                            });
                        };

                        bytes.extend_from_slice(&ptr.to_le_bytes());
                        bytes.extend_from_slice(&len.to_le_bytes());
                    }
                } else {
                    return Err(Error {
                        message: format!(
                            "Unsupported array literal element type: {}",
                            TypeFmt(&*self.registry, &item_type)
                        ),
                        loc: *loc,
                    });
                }

                let ptr = self.append_data(bytes);
                instrs.push(WasmInstr::I32Const { value: ptr as i32 });

                self.registry.const_slice_lens.be_mut().push(ConstSliceLen {
                    slice_ptr: ptr,
                    slice_len: items.len(),
                });

                return Ok(());
            }
            CodeExpr::ResultLiteral(ResultLiteralExpr {
                id: _,
                is_ok,
                result_type,
                value,
                loc,
            }) => {
                let result = self
                    .registry
                    .get_result_literal_type(ctx, result_type, loc)?;

                let mut value_type = Type::Void;
                if let Some(value) = value {
                    value_type = self.registry.get_expr_type(ctx, value)?;
                }

                if *is_ok {
                    if !is_type_compatible(&result.ok, &value_type) {
                        return Err(Error {
                            message: format!(
                                "Cannot create result, Ok type mismatch. Got {}, expected: {}",
                                TypeFmt(&*self.registry, &value_type),
                                TypeFmt(&*self.registry, &result.ok),
                            ),
                            loc: *loc,
                        });
                    }

                    if let Some(ok_value) = value {
                        self.codegen(ctx, instrs, ok_value)?;
                    }

                    // error value
                    instrs.push(WasmInstr::I32Const { value: 0 });

                    return Ok(());
                }

                if !is_type_compatible(&result.err, &value_type) {
                    return Err(Error {
                        message: format!(
                            "Cannot create result, Err type mismatch. Got {}, expected: {}",
                            TypeFmt(&*self.registry, &value_type),
                            TypeFmt(&*self.registry, &result.err),
                        ),
                        loc: *loc,
                    });
                }

                self.codegen_default_value(ctx, instrs, &result.ok);
                self.codegen(ctx, instrs, value.as_ref().unwrap())?;
            }

            CodeExpr::Ident(ident) => {
                let var = self.var_from_ident(ctx, ident)?;
                if let Some(inspect_info) = var.inspect_info() {
                    self.reporter.print_inspection(inspect_info);
                }
                self.codegen_var_get(ctx, instrs, &var)?;
            }
            CodeExpr::Let(LetExpr {
                id: _,
                is_inline,
                name,
                value,
                loc: _,
            }) => {
                if *is_inline {
                    let code_unit = self.build_code_unit(ctx, value)?;
                    self.registry.register_block_const(
                        ctx,
                        ConstDef {
                            const_name: name.repr,
                            code_unit,
                            loc: name.loc,
                        },
                    );
                    return Ok(());
                }

                let mut var_type = Type::Never;
                // any errors will be reported during value codegen
                if let Ok(t) = self.registry.get_expr_type(ctx, &value) {
                    var_type = t;
                }

                if name.repr == "_" {
                    self.codegen(ctx, instrs, value)?;

                    for _ in 0..self.registry.count_wasm_type_components(&var_type) {
                        instrs.push(WasmInstr::Drop);
                    }
                    return Ok(());
                }

                let local_index = self
                    .registry
                    .define_local(ctx, name.loc, name.repr, &var_type);
                let var = self.var_local(&name.repr, var_type, local_index, name.loc, None);
                if let Some(inspect_info) = var.inspect_info() {
                    self.reporter.print_inspection(inspect_info);
                }
                self.codegen_var_set_prepare(instrs, &var);
                self.codegen(ctx, instrs, value)?;
                self.codegen_var_set(ctx, instrs, &var)?;
            }
            CodeExpr::Cast(CastExpr {
                id: _,
                expr,
                casted_to,
                loc,
            }) => {
                self.codegen(ctx, instrs, expr)?;

                let castee_type = self.registry.get_expr_type(ctx, expr)?;
                let casted_to_type = self.registry.build_type(ctx, casted_to)?;

                if let Some(cast_op) = self.registry.get_cast_instr(&castee_type, &casted_to_type) {
                    instrs.push(cast_op);
                    return Ok(());
                }

                let mut castee_type_components = Vec::new();
                self.registry
                    .lower_type(&castee_type, &mut castee_type_components);

                let mut casted_to_type_components = Vec::new();
                self.registry
                    .lower_type(&casted_to_type, &mut casted_to_type_components);

                if castee_type_components != casted_to_type_components {
                    return Err(Error {
                        message: format!(
                            "Cannot cast from {} to {}",
                            TypeFmt(&*self.registry, &castee_type),
                            TypeFmt(&*self.registry, &casted_to_type)
                        ),
                        loc: *loc,
                    });
                }
            }
            CodeExpr::PrefixOp(PrefixOpExpr {
                id: _,
                op_tag,
                expr,
                op_loc,
                loc,
            }) => match op_tag {
                PrefixOpTag::Reference => {
                    let Some(VarInfo::Stored(VarInfoStored {
                        mut address,
                        offset,
                        var_type: _,
                        inspect_info,
                    })) = self.var_from_expr(ctx, expr)?
                    else {
                        return Err(Error {
                            message: format!(
                                "Invalid reference expression. Only struct reference fields allowed.",
                            ),
                            loc: *loc,
                        });
                    };
                    if let Some(inspect_info) = inspect_info {
                        self.reporter.print_inspection(&inspect_info);
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
                        self.reporter.print_inspection(inspect_info);
                    }
                    self.codegen_var_get(ctx, instrs, &var)?;
                }
                PrefixOpTag::Not => {
                    self.codegen(ctx, instrs, expr)?;

                    let operand_type = self.registry.get_expr_type(ctx, expr)?;
                    let mut wasm_components = Vec::new();
                    self.registry
                        .lower_type(&operand_type, &mut wasm_components);
                    if wasm_components.len() != 1 {
                        return Err(Error {
                            message: format!(
                                "Cannot apply not operation to expr of type {}",
                                TypeFmt(&*self.registry, &operand_type)
                            ),
                            loc: *loc,
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
                    }
                }
                PrefixOpTag::Positive => {
                    self.codegen(ctx, instrs, expr)?;
                }
                PrefixOpTag::Negative => {
                    if let CodeExpr::IntLiteral(int_literal) = expr.as_ref() {
                        self.codegen_int_literal(ctx, instrs, int_literal);
                        match instrs.last_mut().unwrap() {
                            WasmInstr::I32Const { value } => *value *= -1,
                            WasmInstr::I64Const { value } => *value *= -1,
                            _ => unreachable!(),
                        }
                        return Ok(());
                    };

                    let operand_type = self.registry.get_expr_type(ctx, expr)?;
                    let mut wasm_components = Vec::new();
                    self.registry
                        .lower_type(&operand_type, &mut wasm_components);
                    if wasm_components.len() != 1 {
                        return Err(Error {
                            message: format!(
                                "Cannot negate expr of type {}",
                                TypeFmt(&*self.registry, &operand_type)
                            ),
                            loc: *loc,
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
                let lhs_type = self.registry.get_expr_type(ctx, lhs)?;
                let rhs_type = self.registry.get_expr_type(ctx, rhs)?;

                if !is_type_compatible(&lhs_type, &rhs_type) {
                    return Err(Error {
                        message: format!(
                            "Operands are not of the same type: lhs = {}, rhs = {}",
                            TypeFmt(&*self.registry, &lhs_type),
                            TypeFmt(&*self.registry, &rhs_type),
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
                        self.reporter.print_inspection(inspect_info);
                    }

                    self.codegen_var_set_prepare(instrs, &var);
                    self.codegen_var_get(ctx, instrs, &var)?;
                    self.codegen(ctx, instrs, rhs)?;
                    self.codegen_binary_op(ctx, instrs, &base_op, &lhs_type, op_loc)?;

                    self.codegen_var_set(ctx, instrs, &var)?;
                    return Ok(());
                }

                self.codegen(ctx, instrs, lhs)?;
                self.codegen(ctx, instrs, rhs)?;
                self.codegen_binary_op(ctx, instrs, &op_tag, &lhs_type, op_loc)?;
            }

            CodeExpr::Assign(AssignExpr {
                id: _,
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
                    self.reporter.print_inspection(inspect_info);
                }

                let rhs_type = self.registry.get_expr_type(ctx, rhs)?;
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
                self.codegen(ctx, instrs, rhs)?;
                self.codegen_var_set(ctx, instrs, &var)?;
            }
            CodeExpr::FieldAccess(field_access) => {
                let var = self.var_from_field_access(ctx, field_access)?;
                if let Some(inspect_info) = var.inspect_info() {
                    self.reporter.print_inspection(inspect_info);
                }
                self.codegen_var_get(ctx, instrs, &var)?;
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

                        if self.reporter.in_inspection_mode {
                            self.reporter.print_inspection(&InspectInfo {
                                message: format!("{} // {}", fn_name.repr, ctor.variant_index),
                                loc: fn_name.loc,
                                linked_loc: Some(variant.loc),
                            });
                        }

                        instrs.push(WasmInstr::I32Const {
                            value: ctor.variant_index as i32,
                        });

                        if variant.variant_type == Type::Void && args.items.len() == 0 {
                            return Ok(());
                        }

                        if args.items.len() != 1 {
                            return Err(Error {
                                message: format!(
                                    "Non-void enum constructors require exactly one argument"
                                ),
                                loc: fn_name.loc,
                            });
                        }

                        let expr_type = self.registry.get_expr_type(ctx, &args.items[0])?;
                        if !is_type_compatible(&variant.variant_type, &expr_type) {
                            return Err(Error {
                                message: format!(
                                    "Invalid enum payload: {}, expected: {}",
                                    TypeFmt(&*self.registry, &expr_type),
                                    TypeFmt(&*self.registry, &variant.variant_type),
                                ),
                                loc: fn_name.loc,
                            });
                        }

                        self.codegen(ctx, instrs, &args.items[0])?;
                        return Ok(());
                    }
                }

                self.codegen_fn_call(ctx, instrs, &fn_name.repr, None, &args.items, &fn_name.loc)?;
            }
            CodeExpr::MethodCall(MethodCallExpr {
                id: _,
                lhs,
                field_name,
                args,
                loc: _,
            }) => {
                let lhs_type = self.registry.get_expr_type(ctx, lhs)?;
                let fn_name = self
                    .registry
                    .get_fn_name_from_method(&lhs_type, &field_name.repr);
                self.codegen_fn_call(
                    ctx,
                    instrs,
                    &fn_name,
                    Some(lhs),
                    &args.items,
                    &field_name.loc,
                )?;
            }
            CodeExpr::InlineFnCall(InlineFnCallExpr {
                id: _,
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
                    expr,
                    &fn_name.loc,
                )?;
            }
            CodeExpr::InlineMethodCall(InlineMethodCallExpr {
                id: _,
                lhs,
                field_name,
                type_args,
                args,
                loc: _,
            }) => {
                let lhs_type = self.registry.get_expr_type(ctx, lhs)?;
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
                    expr,
                    &field_name.loc,
                )?;
            }
            CodeExpr::IntrinsicCall(InlineFnCallExpr {
                id: _,
                fn_name,
                type_args,
                args,
                loc: _,
            }) => {
                if fn_name.repr == "unreachable" {
                    if args.items.len() != 0 || type_args.len() != 0 {
                        return Err(Error {
                            message: format!("@{}() accepts no arguments", fn_name.repr),
                            loc: fn_name.loc,
                        });
                    }

                    instrs.push(WasmInstr::Unreachable);
                    return Ok(());
                }

                if fn_name.repr == "memory_size" {
                    if args.items.len() != 0 || type_args.len() != 0 {
                        return Err(Error {
                            message: format!("@{}() accepts no arguments", fn_name.repr),
                            loc: fn_name.loc,
                        });
                    }

                    instrs.push(WasmInstr::MemorySize);
                    return Ok(());
                }

                if fn_name.repr == "memory_grow" {
                    if type_args.len() != 0 {
                        return Err(Error {
                            message: format!("@{}() accepts no type arguments", fn_name.repr),
                            loc: fn_name.loc,
                        });
                    }

                    let mut arg_types = Vec::new();
                    for arg in &args.items {
                        arg_types.push(self.registry.get_expr_type(ctx, arg)?);
                    }
                    let param_types = &[Type::U32];
                    if arg_types != param_types {
                        return Err(Error {
                            message: format!(
                                "Unexpected arguments [{}] for @{}(num_pages: u32): i32",
                                TypeListFmt(&*self.registry, &arg_types),
                                fn_name.repr,
                            ),
                            loc: fn_name.loc,
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
                            loc: fn_name.loc,
                        });
                    }

                    let mut arg_types = Vec::new();
                    for arg in &args.items {
                        arg_types.push(self.registry.get_expr_type(ctx, arg)?);
                    }
                    let param_types = &[Type::U32, Type::U32, Type::U32];
                    if arg_types != param_types {
                        return Err(Error {
                            message: format!(
                                "Unexpected arguments [{}] for @{}(dest: u32, source: u32: num_bytes: u32)",
                                TypeListFmt(&*self.registry, &arg_types),
                                fn_name.repr,
                            ),
                            loc: fn_name.loc,
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
                            loc: fn_name.loc,
                        });
                    }

                    if let Some(_) = ctx.fn_index {
                        return Err(Error {
                            message: format!("@{}() can only be used in globals", fn_name.repr),
                            loc: fn_name.loc,
                        });
                    }

                    // placeholder, filled in in `generate`
                    instrs.push(WasmInstr::I32Const { value: 0 });
                    return Ok(());
                }

                if fn_name.repr == "embed_file" {
                    if args.items.len() != 1 {
                        return bad_args_err(self, fn_name);
                    }

                    let CodeExpr::StringLiteral(str_expr) = &args.items[0] else {
                        return bad_args_err(self, fn_name);
                    };

                    let absolute_path =
                        self.reporter.fm.resolve_path(&str_expr.value, &fn_name.loc);
                    let bytes = fs::file_read(&absolute_path).map_err(|message| Error {
                        message,
                        loc: args.items[0].loc(),
                    })?;

                    let bytes_len = bytes.len();
                    let bytes_ptr = self.append_data(bytes);

                    instrs.push(WasmInstr::I32Const {
                        value: bytes_ptr as i32,
                    });

                    self.registry.const_slice_lens.be_mut().push(ConstSliceLen {
                        slice_ptr: bytes_ptr,
                        slice_len: bytes_len,
                    });

                    return Ok(());

                    fn bad_args_err(
                        self_: &CodeGenerator,
                        fn_name: &IdentExpr,
                    ) -> Result<(), Error> {
                        self_.reporter.error(&Error {
                            message: format!(
                                "Invalid arguments for @{}(relative_file_path: str)",
                                fn_name.repr,
                            ),
                            loc: fn_name.loc,
                        });

                        Ok(())
                    }
                }

                if fn_name.repr == "const_slice_len" {
                    if args.items.len() != 1 {
                        return bad_args_err(self, fn_name);
                    }

                    let mut slice_ptr_instrs = Vec::new();
                    if let Err(err) = self.codegen(ctx, &mut slice_ptr_instrs, &args.items[0]) {
                        self.report_error(&err);
                        return Ok(());
                    };

                    if slice_ptr_instrs.len() != 1 {
                        return bad_args_err(self, fn_name);
                    }

                    let WasmInstr::I32Const { value: slice_ptr } = &slice_ptr_instrs[0] else {
                        return bad_args_err(self, fn_name);
                    };

                    for const_slice_len in &self.registry.const_slice_lens {
                        if const_slice_len.slice_ptr == *slice_ptr as u32 {
                            instrs.push(WasmInstr::I32Const {
                                value: const_slice_len.slice_len as i32,
                            });
                            return Ok(());
                        }
                    }

                    return bad_args_err(self, fn_name);

                    fn bad_args_err(
                        self_: &CodeGenerator,
                        fn_name: &IdentExpr,
                    ) -> Result<(), Error> {
                        self_.reporter.error(&Error {
                            message: format!(
                                "Invalid arguments for @{}(items: const T[])",
                                fn_name.repr,
                            ),
                            loc: fn_name.loc,
                        });

                        Ok(())
                    }
                }

                if fn_name.repr == "inline_fn_call_loc" {
                    let mut inline_fn_call_loc = None;
                    // NOTE: not iterating in reverse to get the first inline scope
                    for scope in &self.registry.modules[ctx.module_id].scope_stack {
                        if scope.kind == ScopeKind::InlineFn {
                            inline_fn_call_loc = scope.inline_fn_call_loc.clone();
                        }
                    }

                    let Some(inline_fn_call_loc) = inline_fn_call_loc else {
                        self.report_error(&Error {
                            message: format!(
                                "Forbidden use of `@{}()` outside of inline fn",
                                fn_name.repr
                            ),
                            loc: fn_name.loc,
                        });
                        return Ok(());
                    };

                    let loc_str = self.process_const_string(
                        inline_fn_call_loc.to_string(&self.reporter.fm),
                        &inline_fn_call_loc,
                    );
                    // emit str struct values
                    instrs.push(WasmInstr::I32Const {
                        value: loc_str.ptr as i32,
                    });
                    instrs.push(WasmInstr::I32Const {
                        value: loc_str.len as i32,
                    });

                    return Ok(());
                }

                if fn_name.repr == "get_ok" {
                    if args.items.len() != 1 {
                        return bad_args_err(self, fn_name);
                    }

                    let arg_type = catch!(self.registry.get_expr_type(ctx, &args.items[0]), err, {
                        self.report_error(&err);
                        return Ok(());
                    });

                    let Type::Result(ResultType { ok: _, err }) = arg_type else {
                        return bad_args_err(self, fn_name);
                    };

                    self.codegen(ctx, instrs, &args.items[0])?;

                    // drop `err` leaving only `ok` on the stack
                    for _ in 0..self.registry.count_wasm_type_components(&err) {
                        instrs.push(WasmInstr::Drop);
                    }

                    return Ok(());

                    fn bad_args_err(
                        self_: &CodeGenerator,
                        fn_name: &IdentExpr,
                    ) -> Result<(), Error> {
                        self_.reporter.error(&Error {
                            message: format!(
                                "Invalid arguments for @{}(items: Result(T, E)): T",
                                fn_name.repr,
                            ),
                            loc: fn_name.loc,
                        });

                        Ok(())
                    }
                }

                if fn_name.repr == "get_err" {
                    if args.items.len() != 1 {
                        return bad_args_err(self, fn_name);
                    }

                    let arg_type = catch!(self.registry.get_expr_type(ctx, &args.items[0]), err, {
                        self.report_error(&err);
                        return Ok(());
                    });

                    let Type::Result(ResultType { ok, err }) = arg_type else {
                        return bad_args_err(self, fn_name);
                    };

                    self.codegen(ctx, instrs, &args.items[0])?;

                    // push `err` to temp local
                    let tmp_local_index =
                        self.registry
                            .define_unnamed_local(ctx, Loc::internal(), &err);
                    let tmp_local = VarInfo::Local(VarInfoLocal {
                        local_index: tmp_local_index,
                        var_type: *err.clone(),
                        inspect_info: None,
                    });
                    self.codegen_local_set(instrs, &err, tmp_local_index);

                    // drop `ok`
                    for _ in 0..self.registry.count_wasm_type_components(&ok) {
                        instrs.push(WasmInstr::Drop);
                    }

                    // pop `err` back
                    self.codegen_var_get(ctx, instrs, &tmp_local)?;

                    return Ok(());

                    fn bad_args_err(
                        self_: &CodeGenerator,
                        fn_name: &IdentExpr,
                    ) -> Result<(), Error> {
                        self_.reporter.error(&Error {
                            message: format!(
                                "Invalid arguments for @{}(items: Result(T, E)): E",
                                fn_name.repr,
                            ),
                            loc: fn_name.loc,
                        });

                        Ok(())
                    }
                }

                if fn_name.repr == "inspect_symbols" {
                    if !self.reporter.in_inspection_mode {
                        return Ok(());
                    }

                    let mut message = String::new();
                    for symbol in &self.registry.current_scope(ctx).symbols {
                        write!(message, "{} : {:?}\n", symbol.name, symbol.kind).unwrap();
                    }

                    self.reporter.print_inspection(&InspectInfo {
                        message,
                        loc: fn_name.loc,
                        linked_loc: None,
                    });
                    return Ok(());
                }

                self.report_error(&Error {
                    message: format!("Unknown intrinsic: {}", fn_name.repr),
                    loc: fn_name.loc,
                });
            }
            CodeExpr::Sizeof(SizeofExpr {
                id: _,
                type_expr,
                loc: _,
            }) => {
                let lo_type = self.registry.build_type(ctx, type_expr)?;
                let mut type_layout = TypeLayout::new();
                self.registry.get_type_layout(&lo_type, &mut type_layout);

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
                self.codegen(ctx, instrs, expr)?;
            }
            CodeExpr::Return(ReturnExpr { id: _, expr, loc }) => {
                let Some(fn_index) = ctx.fn_index else {
                    return Err(Error {
                        message: format!("Cannot use `return` in const context"),
                        loc: *loc,
                    });
                };

                let mut return_type = Type::Void;

                if let Some(return_expr) = expr {
                    self.codegen(ctx, instrs, return_expr)?;
                    return_type = self.registry.get_expr_type(ctx, &return_expr)?;
                };

                let fn_return_type = &self.registry.functions[fn_index].fn_type.output;
                if !is_type_compatible(fn_return_type, &return_type) {
                    return Err(Error {
                        message: format!(
                            "Invalid return type: {}, expected: {}",
                            TypeFmt(&*self.registry, &return_type),
                            TypeFmt(&*self.registry, &fn_return_type),
                        ),
                        loc: *loc,
                    });
                }

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
                        if let Ok(cond_type) = self.registry.get_expr_type(ctx, expr) {
                            if cond_type != Type::Bool {
                                self.report_error(&Error {
                                    message: format!(
                                        "Unexpected condition type: {}, expected: {}",
                                        TypeFmt(&*self.registry, &cond_type),
                                        TypeFmt(&*self.registry, &Type::Bool),
                                    ),
                                    loc: expr.loc(),
                                });
                            }
                        }

                        self.codegen(ctx, instrs, expr)?;

                        // `if` condition runs outside of then_branch's scope
                        self.registry.be_mut().enter_scope(ctx, ScopeKind::Block);
                    }
                    IfCond::Match(match_header) => {
                        // `if match` condition runs inside of then_branch's scope
                        self.registry.be_mut().enter_scope(ctx, ScopeKind::Block);

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
                        self.codegen(ctx, instrs, &code_expr)?;
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

                self.registry.be_mut().enter_scope(ctx, ScopeKind::Block);
                let terminates_early = self.codegen_code_block(ctx, instrs, &else_branch, true);
                if !terminates_early {
                    self.report_error(&Error {
                        message: format!(
                            "Match's else block must resolve to never, got other type"
                        ),
                        loc: else_branch.loc,
                    });
                }
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
                    if let Ok(cond_type) = self.registry.get_expr_type(ctx, cond) {
                        if cond_type != Type::Bool {
                            self.report_error(&Error {
                                message: format!(
                                    "Unexpected condition type: {}, expected: {}",
                                    TypeFmt(&*self.registry, &cond_type),
                                    TypeFmt(&*self.registry, &Type::Bool),
                                ),
                                loc: cond.loc(),
                            });
                        }
                    }

                    catch!(self.codegen(ctx, instrs, cond), err, {
                        self.report_error(&err);
                    });

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
                loc,
            }) => {
                let counter_type = self.registry.get_expr_type(ctx, start)?;
                if self.registry.get_expr_type(ctx, end)? != counter_type {
                    return Err(Error {
                        message: format!(
                            "Invalid range end type: {}, expected: {}",
                            TypeFmt(&*self.registry, &self.registry.get_expr_type(ctx, end)?),
                            TypeFmt(&*self.registry, &counter_type),
                        ),
                        loc: *loc,
                    });
                }

                let mut is_64_bit_counter = false;
                match self.registry.is_64_bit_int_tag(&counter_type, loc) {
                    Ok(is_64) => is_64_bit_counter = is_64,
                    Err(err) => self.report_error(&err),
                }

                self.registry.be_mut().enter_scope(ctx, ScopeKind::ForLoop);

                // define counter and set value to start
                let local_index =
                    self.registry
                        .define_local(ctx, counter.loc, counter.repr, &counter_type);
                let counter_var = self.var_local(
                    &counter.repr,
                    counter_type.clone(),
                    local_index,
                    counter.loc,
                    None,
                );
                if let Some(inspect_info) = counter_var.inspect_info() {
                    self.reporter.print_inspection(inspect_info);
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
                        self.codegen_binary_op(
                            ctx,
                            instrs,
                            &InfixOpTag::Equal,
                            &counter_type,
                            op_loc,
                        )?;
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
                        if is_64_bit_counter {
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
                        )?;
                        self.codegen_var_set(ctx, instrs, &counter_var)?;

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
                            return Err(Error {
                                message: format!("Cannot break outside of a loop"),
                                loc: *loc,
                            });
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
                            return Err(Error {
                                message: format!("Cannot continue outside of a loop"),
                                loc: *loc,
                            });
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
                let Some(first_arg) = args.items.first() else {
                    self.report_error(&Error {
                        message: format!("do-with expressions must have at least one argument"),
                        loc: with_loc.clone(),
                    });
                    return Ok(());
                };

                let arg_type = self.registry.get_expr_type(ctx, first_arg)?;

                for arg in &args.items {
                    let current_arg_type = self.registry.get_expr_type(ctx, arg)?;
                    if current_arg_type != arg_type {
                        self.report_error(&Error {
                            message: format!(
                                "do-with argument type mismatch. expected: {}, got: {}",
                                TypeFmt(&*self.registry, &arg_type),
                                TypeFmt(&*self.registry, &current_arg_type),
                            ),
                            loc: arg.loc(),
                        });
                        continue;
                    }

                    self.registry.be_mut().enter_scope(ctx, ScopeKind::InlineFn);

                    self.codegen(ctx, instrs, arg)?;

                    let arg_local_index =
                        self.registry
                            .define_local(ctx, with_loc.clone(), "it", &arg_type);

                    self.codegen_local_set(instrs, &arg_type, arg_local_index);
                    self.codegen(ctx, instrs, body)?;

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
                let lhs_type = self.registry.get_expr_type(ctx, lhs)?;
                catch!(self.codegen(ctx, instrs, lhs), err, {
                    self.report_error(&err);
                    return Ok(());
                });

                self.registry.be_mut().enter_scope(ctx, ScopeKind::Block);

                let lhs_local_index =
                    self.registry
                        .define_local(ctx, op_loc.clone(), "it", &lhs_type);

                self.codegen_local_set(instrs, &lhs_type, lhs_local_index);
                catch!(self.codegen(ctx, instrs, rhs), err, {
                    self.report_error(&err);
                    return Ok(());
                });
                self.registry.be_mut().exit_scope(ctx);
            }
            CodeExpr::Defer(DeferExpr {
                id: _,
                expr,
                loc: _,
            }) => {
                let code_unit = self.build_code_unit(ctx, expr)?;

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
                )?;
            }
            CodeExpr::PropagateError(PropagateErrorExpr { id: _, expr, loc }) => {
                self.codegen_catch(ctx, instrs, expr, None, None, loc)?;
            }
        };

        Ok(())
    }

    /// defines a local with match bind's name and pushes enum's variant to the stack
    fn codegen_match_header(
        &mut self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        header: &Box<MatchHeader>,
    ) -> Result<&EnumConstructor, Error> {
        let Some(symbol) = self
            .registry
            .current_scope(ctx)
            .get_symbol(&header.variant_name.repr)
        else {
            return Err(Error {
                message: format!("Unkown enum constructor: {}", header.variant_name.repr),
                loc: header.variant_name.loc,
            });
        };
        let SymbolKind::EnumConstructor = symbol.kind else {
            return Err(Error {
                message: format!("Not an enum constructor: {}", header.variant_name.repr),
                loc: header.variant_name.loc,
            });
        };

        let enum_ctor = &self.registry.enum_ctors[symbol.col_index].relax();
        let enum_index = enum_ctor.enum_index;
        let enum_def = &self.registry.enums[enum_ctor.enum_index].relax();
        let enum_variant = &enum_def.variants[enum_ctor.variant_index].relax();

        if self.reporter.in_inspection_mode {
            self.reporter.print_inspection(&InspectInfo {
                message: format!(
                    "{}\n{}({})",
                    TypeFmt(&*self.registry, &Type::EnumInstance { enum_index }),
                    header.variant_name.repr,
                    TypeFmt(&*self.registry, &enum_variant.variant_type)
                ),
                loc: header.variant_name.loc,
                linked_loc: Some(enum_variant.loc),
            });
        }

        let local_index = self.registry.define_local(
            ctx,
            header.variant_bind.loc,
            header.variant_bind.repr,
            &enum_variant.variant_type,
        );
        let local = self.var_local(
            &header.variant_bind.repr,
            enum_variant.variant_type.clone(),
            local_index,
            header.variant_bind.loc,
            None,
        );
        if let Some(inspect_info) = local.inspect_info() {
            self.reporter.print_inspection(inspect_info);
        }

        if let Ok(expr_to_match_type) = self.registry.get_expr_type(ctx, &header.expr_to_match) {
            let expected_expr_to_match_type = Type::EnumInstance {
                enum_index: enum_ctor.enum_index,
            };
            if !is_type_compatible(&expected_expr_to_match_type, &expr_to_match_type) {
                self.report_error(&Error {
                    message: format!(
                        "Unexpected type to match, expected: {}, got: {}",
                        TypeFmt(&*self.registry, &expected_expr_to_match_type),
                        TypeFmt(&*self.registry, &expr_to_match_type)
                    ),
                    loc: header.expr_to_match.loc(),
                });
            }
        };

        self.codegen_var_set_prepare(instrs, &local);
        self.codegen(ctx, instrs, &header.expr_to_match)?;
        self.codegen_var_set(ctx, instrs, &local)?;

        Ok(enum_ctor)
    }

    fn codegen_fn_call(
        &mut self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        fn_name: &str,
        receiver_arg: Option<&CodeExpr>,
        args: &Vec<CodeExpr>,
        loc: &Loc,
    ) -> Result<(), Error> {
        let fn_info = self
            .registry
            .get_fn_info_for_call(ctx, fn_name, loc)?
            .relax();

        let mut arg_types = Vec::new();
        if let Some(receiver_arg) = receiver_arg {
            arg_types.push(self.registry.get_expr_type(ctx, receiver_arg)?);
            self.codegen(ctx, instrs, receiver_arg)?;
        }
        for arg in args {
            arg_types.push(self.registry.get_expr_type(ctx, arg)?);
            self.codegen(ctx, instrs, arg)?;
        }

        if self.reporter.in_inspection_mode {
            let mut message = String::new();

            write!(&mut message, "fn {fn_name}(").unwrap();

            for (param, i) in fn_info.fn_params.iter().zip(0..) {
                if i != 0 {
                    message.push_str(", ");
                }

                message.push_str(&param.param_name);
                message.push_str(": ");
                write!(
                    &mut message,
                    "{}",
                    TypeFmt(&*self.registry, &param.param_type)
                )
                .unwrap();
            }

            let return_type = TypeFmt(&*self.registry, &fn_info.fn_type.output);
            write!(&mut message, "): {}", return_type).unwrap();

            self.reporter.print_inspection(&InspectInfo {
                message,
                loc: *loc,
                linked_loc: Some(fn_info.definition_loc.clone()),
            });
        }

        if !is_types_compatible(&fn_info.fn_type.inputs, &arg_types) {
            return Err(Error {
                message: format!(
                    "Invalid function arguments for function {}: [{}], expected [{}]",
                    fn_info.fn_name,
                    TypeListFmt(&*self.registry, &arg_types),
                    TypeListFmt(&*self.registry, &fn_info.fn_type.inputs),
                ),
                loc: *loc,
            });
        }

        instrs.push(WasmInstr::Call {
            fn_index: fn_info.wasm_fn_index,
        });

        Ok(())
    }

    fn populate_ctx_from_inline_fn_call(
        &mut self,
        ctx: &mut ExprContext,
        inline_fn_name: &str,
        type_args: &Vec<TypeExpr>,
        receiver_arg: Option<&CodeExpr>,
        args: &Vec<CodeExpr>,
        loc: &Loc,
    ) -> Result<&FnExpr, Error> {
        let Some(symbol) = self.registry.current_scope(ctx).get_symbol(inline_fn_name) else {
            return Err(Error {
                message: format!("Unknown inline fn: {}", inline_fn_name),
                loc: *loc,
            });
        };

        let inline_fn_def = self.registry.inline_fns[symbol.col_index];

        let mut all_args = Vec::new();
        if let Some(receiver_arg) = receiver_arg {
            all_args.push(self.build_code_unit(ctx, receiver_arg)?);
        }
        for arg in args {
            all_args.push(self.build_code_unit(ctx, arg)?);
        }

        let mut lo_type_args = Vec::new();
        for type_arg in type_args {
            lo_type_args.push(self.registry.build_type(ctx, &type_arg)?);
        }
        if lo_type_args.len() != inline_fn_def.decl.type_params.len() {
            return Err(Error {
                message: format!(
                    "Invalid number of type args, expected {}, got {}",
                    inline_fn_def.decl.type_params.len(),
                    type_args.len()
                ),
                loc: *loc,
            });
        }

        for (i, (type_param, type_arg)) in inline_fn_def
            .decl
            .type_params
            .iter()
            .zip(lo_type_args.iter())
            .enumerate()
        {
            self.registry.register_block_type(
                ctx,
                type_param,
                type_arg.clone(),
                *type_args[i].loc(),
            );
        }

        if all_args.len() != inline_fn_def.decl.params.len() {
            return Err(Error {
                message: format!(
                    "Invalid number of inline fn args, expected {}, got {}",
                    inline_fn_def.decl.params.len(),
                    all_args.len()
                ),
                loc: *loc,
            });
        }

        let mut arg_types = Vec::<Type>::new();
        for arg in &all_args {
            arg_types.push(arg.type_.clone());
        }

        for (inline_fn_param, inline_fn_arg) in
            inline_fn_def.decl.params.iter().zip(all_args.into_iter())
        {
            let const_def = ConstDef {
                const_name: inline_fn_param.param_name.repr,
                code_unit: inline_fn_arg,
                loc: inline_fn_param.loc,
            };

            if let Some(type_name) = get_infer_type_name(inline_fn_param)? {
                self.registry.register_block_type(
                    ctx,
                    type_name,
                    const_def.code_unit.type_.clone(),
                    inline_fn_param.loc,
                );
            }

            self.registry.register_block_const(ctx, const_def);
        }

        let self_type = self.registry.get_fn_self_type(
            ctx,
            &inline_fn_def.decl.fn_name,
            &inline_fn_def.decl.params,
        );

        let mut inline_fn_types = Vec::<Type>::new();
        for inline_fn_param in &inline_fn_def.decl.params {
            let inline_fn_type =
                self.registry
                    .get_fn_param_type(ctx, inline_fn_param, &self_type, true)?;
            inline_fn_types.push(inline_fn_type);
        }

        if !is_types_compatible(&inline_fn_types, &arg_types) {
            return Err(Error {
                message: format!(
                    "Invalid inline fn args, expected [{}], got [{}]",
                    TypeListFmt(&*self.registry, &inline_fn_types),
                    TypeListFmt(&*self.registry, &arg_types)
                ),
                loc: *loc,
            });
        }

        Ok(inline_fn_def)
    }

    fn codegen_inline_fn_call(
        &mut self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        inline_fn_name: &str,
        type_args: &Vec<TypeExpr>,
        receiver_arg: Option<&CodeExpr>,
        args: &Vec<CodeExpr>,
        call_expr: &CodeExpr,
        loc: &Loc,
    ) -> Result<(), Error> {
        self.registry.be_mut().enter_scope(ctx, ScopeKind::InlineFn);
        self.registry.current_scope(ctx).be_mut().inline_fn_call_loc = Some(*loc);

        let inline_fn_def = self.relax_mut().populate_ctx_from_inline_fn_call(
            ctx,
            inline_fn_name,
            type_args,
            receiver_arg,
            args,
            loc,
        );
        let inline_fn_def = catch!(inline_fn_def, err, {
            self.registry.be_mut().exit_scope(ctx);
            return Err(err);
        });

        let FnExprValue::Body(body) = &inline_fn_def.value else {
            unreachable!()
        };

        if let Some(expr_info_id) = self.registry.get_expr_info_id(ctx, call_expr) {
            let call_info = &self.registry.inline_fn_call_info[expr_info_id];
            // TODO: remove if no longer needed for debugging
            // stderr_writeln(format!("got_offset({})", call_info.inner_expr_offset));
            self.registry.current_scope(ctx).be_mut().expr_info_offset =
                call_info.inner_expr_offset;
        }

        self.codegen_code_block(ctx, instrs, body, false);

        self.registry.be_mut().exit_scope(ctx);

        Ok(())
    }

    fn codegen_catch(
        &mut self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        expr: &CodeExpr,
        error_bind: Option<&IdentExpr>,
        catch_body: Option<&CodeBlock>,
        loc: &Loc,
    ) -> Result<(), Error> {
        let expr_type = self.registry.get_expr_type(ctx, expr)?;
        let result = self.registry.assert_catchable_type(&expr_type, loc)?;

        self.registry.be_mut().enter_scope(ctx, ScopeKind::Block); // enter catch scope

        // put result on the stack
        self.codegen(ctx, instrs, expr)?;

        // pop error
        let (error_bind, error_bind_loc) = if let Some(error_bind) = error_bind {
            (error_bind.repr, error_bind.loc)
        } else {
            ("<err>", Loc::internal())
        };
        let err_local_index =
            self.registry
                .define_local(ctx, error_bind_loc.clone(), error_bind, &result.err);
        let err_var = self.var_local(
            &error_bind,
            result.err.as_ref().clone(),
            err_local_index,
            error_bind_loc.clone(),
            None,
        );
        if error_bind_loc.file_index != 0 {
            if let Some(inspect_info) = err_var.inspect_info() {
                self.reporter.print_inspection(inspect_info);
            }
        }
        self.codegen_var_set_prepare(instrs, &err_var);
        self.codegen_var_set(ctx, instrs, &err_var)?;

        // pop ok
        let ok_local_index = self.registry.define_local(ctx, *loc, "<ok>", &result.ok);
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
                    loc: *loc,
                });
            }
        } else {
            // return ok_type of function's result + caught error
            let fn_result = self.registry.get_result_literal_type(ctx, &None, loc)?;
            self.codegen_default_value(ctx, instrs, &fn_result.ok);
            self.codegen_var_get(ctx, instrs, &err_var)?;

            self.emit_deferred_for_return(ctx, instrs);
            instrs.push(WasmInstr::Return);
        }

        instrs.push(WasmInstr::Else);

        // no error, push ok value
        let ok_var = VarInfo::Local(VarInfoLocal {
            local_index: ok_local_index,
            var_type: result.ok.as_ref().clone(),
            inspect_info: None,
        });
        self.codegen_var_get(ctx, instrs, &ok_var)?;

        instrs.push(WasmInstr::BlockEnd);

        self.registry.be_mut().exit_scope(ctx); // exit catch scope

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
                self.registry.get_type_layout(&Type::U32, &mut tag_layout);

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
                self.registry.get_type_layout(&ok, &mut ok_layout);

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
            self.registry.lower_type(input, &mut inout_fn_type.inputs);
        }
        self.registry.lower_type(output, &mut inout_fn_type.outputs);

        for (fn_type, type_index) in self.wasm_fn_types.iter().zip(0..) {
            if *fn_type == inout_fn_type {
                return type_index;
            }
        }

        self.wasm_fn_types.be_mut().push(inout_fn_type);
        self.wasm_fn_types.len() as u32 - 1
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
            | CodeExpr::InlineFnCall(_)
            | CodeExpr::If(_)
            | CodeExpr::Sizeof(_)
            | CodeExpr::While(_)
            | CodeExpr::For(_)
            | CodeExpr::Defer(_)
            | CodeExpr::InlineMethodCall(_) => false,

            CodeExpr::Break(_) | CodeExpr::Continue(_) | CodeExpr::Return(_) => true,

            CodeExpr::Paren(paren_expr) => self.is_naturally_divergent(&paren_expr.expr),
            CodeExpr::IntrinsicCall(intrinsic) => intrinsic.fn_name.repr == "unreachable",

            CodeExpr::Catch(catch_) => self.is_naturally_divergent(&catch_.lhs),
            CodeExpr::InfixOp(infix) => {
                self.is_naturally_divergent(&infix.lhs) || self.is_naturally_divergent(&infix.rhs)
            }
            CodeExpr::PrefixOp(prefix) => self.is_naturally_divergent(&prefix.expr),
            CodeExpr::Match(match_) => self.is_naturally_divergent(&match_.header.expr_to_match),
            CodeExpr::Pipe(pipe) => {
                self.is_naturally_divergent(&pipe.lhs) || self.is_naturally_divergent(&pipe.rhs)
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

    fn codegen_var_get(
        &self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        var: &VarInfo,
    ) -> Result<(), Error> {
        match var {
            VarInfo::Local(VarInfoLocal {
                local_index,
                var_type,
                inspect_info: _,
            }) => {
                for i in 0..self.registry.count_wasm_type_components(var_type) {
                    instrs.push(WasmInstr::LocalGet {
                        local_index: local_index + i,
                    });
                }
            }
            VarInfo::Global(VarInfoGlobal {
                global_index,
                var_type,
                inspect_info: _,
            }) => {
                for i in 0..self.registry.count_wasm_type_components(var_type) {
                    instrs.push(WasmInstr::GlobalGet {
                        global_index: global_index + i,
                    });
                }
            }
            VarInfo::Const(VarInfoConst {
                code_unit,
                loc: _,
                inspect_info: _,
            }) => {
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
                inspect_info: _,
            }) => {
                let mut loads = Vec::new();
                self.codegen_load_or_store(&mut loads, &var_type, *offset, false);

                if loads.len() == 0 {
                    return Ok(());
                }

                for instr in &address.instrs {
                    instrs.push(instr.clone());
                }

                if loads.len() > 1 {
                    let addr_local_index = self.registry.create_or_get_addr_local(ctx);
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
                inspect_info: _,
                loc,
            }) => {
                for instr in &struct_value.instrs {
                    instrs.push(instr.clone());
                }
                for _ in 0..*drops_before {
                    instrs.push(WasmInstr::Drop);
                }

                if *drops_after > 0 {
                    let local_index = self.registry.define_unnamed_local(ctx, *loc, var_type);

                    let var = VarInfo::Local(VarInfoLocal {
                        local_index,
                        var_type: var_type.clone(),
                        inspect_info: None,
                    });
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
    fn codegen_var_set_prepare(&self, instrs: &mut Vec<WasmInstr>, var: &VarInfo) {
        match var {
            VarInfo::Stored(VarInfoStored {
                address,
                offset: _,
                var_type,
                inspect_info: _,
            }) => {
                if self.registry.count_wasm_type_components(var_type) == 0 {
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
        var: &VarInfo,
    ) -> Result<(), Error> {
        match var {
            VarInfo::Local(VarInfoLocal {
                local_index,
                var_type,
                inspect_info: _,
            }) => {
                self.codegen_local_set(instrs, var_type, *local_index);
            }
            VarInfo::Global(VarInfoGlobal {
                global_index,
                var_type,
                inspect_info: _,
            }) => {
                for i in (0..self.registry.count_wasm_type_components(var_type)).rev() {
                    instrs.push(WasmInstr::GlobalSet {
                        global_index: global_index + i,
                    });
                }
            }
            VarInfo::Stored(VarInfoStored {
                address: _,
                offset,
                var_type,
                inspect_info: _,
            }) => {
                let mut stores = Vec::new();
                self.codegen_load_or_store(&mut stores, &var_type, *offset, true);

                if stores.len() > 1 {
                    let tmp_value_local_index =
                        self.registry
                            .define_unnamed_local(ctx, Loc::internal(), var_type);
                    self.codegen_local_set(instrs, var_type, tmp_value_local_index);

                    let addr_local_index = self.registry.create_or_get_addr_local(ctx);
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
                return Err(Error {
                    message: format!("Cannot mutate a constant"),
                    loc: *loc,
                });
            }
        };

        Ok(())
    }

    fn codegen_local_set(&self, instrs: &mut Vec<WasmInstr>, local_type: &Type, local_index: u32) {
        for i in (0..self.registry.count_wasm_type_components(local_type)).rev() {
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

    fn var_from_expr(
        &mut self,
        ctx: &mut ExprContext,
        expr: &CodeExpr,
    ) -> Result<Option<VarInfo>, Error> {
        Ok(match expr {
            CodeExpr::Ident(ident) => Some(self.var_from_ident(ctx, ident)?),
            CodeExpr::FieldAccess(field_access) => {
                Some(self.var_from_field_access(ctx, field_access)?)
            }
            CodeExpr::Paren(ParenExpr {
                id: _,
                expr,
                has_trailing_comma: _,
                loc: _,
            }) => self.var_from_expr(ctx, expr)?,
            CodeExpr::PrefixOp(PrefixOpExpr {
                id: _,
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
    ) -> VarInfo {
        let mut inspect_info = None;
        if self.reporter.in_inspection_mode {
            inspect_info = Some(InspectInfo {
                message: format!(
                    "let {}: {}",
                    local_name,
                    TypeFmt(&*self.registry, &local_type)
                ),
                loc: loc,
                linked_loc,
            })
        };

        VarInfo::Local(VarInfoLocal {
            local_index,
            var_type: local_type,
            inspect_info,
        })
    }

    fn var_from_ident(&self, ctx: &ExprContext, ident: &IdentExpr) -> Result<VarInfo, Error> {
        let Some(symbol) = self.registry.current_scope(ctx).get_symbol(ident.repr) else {
            return Err(Error {
                message: format!("Unknown variable: {}", ident.repr),
                loc: ident.loc,
            });
        };

        match symbol.kind {
            SymbolKind::Local => {
                let local = &ctx.locals[symbol.col_index];

                Ok(self.var_local(
                    &ident.repr,
                    local.local_type.clone(),
                    local.local_index,
                    ident.loc,
                    Some(local.definition_loc.clone()),
                ))
            }
            SymbolKind::Global => {
                let global = &self.registry.globals[symbol.col_index];

                let mut inspect_info = None;
                if self.reporter.in_inspection_mode {
                    inspect_info = Some(InspectInfo {
                        message: format!(
                            "global {}: {}",
                            ident.repr,
                            TypeFmt(&*self.registry, &global.global_type)
                        ),
                        loc: ident.loc,
                        linked_loc: Some(symbol.loc),
                    });
                }

                Ok(VarInfo::Global(VarInfoGlobal {
                    global_index: global.global_index,
                    var_type: global.global_type.clone(),
                    inspect_info,
                }))
            }
            SymbolKind::Const => {
                let const_def = &self.registry.constants[symbol.col_index];

                let mut inspect_info = None;
                if self.reporter.in_inspection_mode {
                    inspect_info = Some(InspectInfo {
                        message: format!(
                            "inline let {}: {}",
                            ident.repr,
                            TypeFmt(&*self.registry, &const_def.code_unit.type_)
                        ),
                        loc: ident.loc,
                        linked_loc: Some(const_def.loc),
                    })
                }

                Ok(VarInfo::Const(VarInfoConst {
                    code_unit: const_def.code_unit.relax(),
                    inspect_info,
                    loc: ident.loc,
                }))
            }
            SymbolKind::EnumConstructor => {
                let enum_ctor = &self.registry.enum_ctors[symbol.col_index];

                let var_type = Type::EnumInstance {
                    enum_index: enum_ctor.enum_index,
                };

                let mut inspect_info = None;
                if self.reporter.in_inspection_mode {
                    inspect_info = Some(InspectInfo {
                        message: format!(
                            "inline let {}: {}",
                            ident.repr,
                            TypeFmt(&*self.registry, &var_type)
                        ),
                        loc: ident.loc,
                        linked_loc: Some(enum_ctor.loc),
                    })
                }

                Ok(VarInfo::VoidEnumValue(VarInfoVoidEnumValue {
                    variant_index: enum_ctor.variant_index,
                    inspect_info,
                    var_type,
                    loc: ident.loc,
                }))
            }
            SymbolKind::TypeAlias
            | SymbolKind::Struct
            | SymbolKind::Enum
            | SymbolKind::InlineFn
            | SymbolKind::Function => Err(Error {
                message: format!(
                    "Expected variable, found {:?} '{}'",
                    symbol.kind, ident.repr
                ),
                loc: ident.loc,
            }),
        }
    }

    fn var_from_field_access(
        &mut self,
        ctx: &mut ExprContext,
        field_access: &FieldAccessExpr,
    ) -> Result<VarInfo, Error> {
        let lhs_type = self
            .registry
            .get_expr_type(ctx, field_access.lhs.as_ref())?;

        let field = self
            .registry
            .get_struct_or_struct_ref_field(&lhs_type, field_access)?
            .relax();

        let mut inspect_info = None;
        if self.reporter.in_inspection_mode {
            inspect_info = Some(InspectInfo {
                message: format!(
                    "{}.{}: {}",
                    TypeFmt(&*self.registry, &lhs_type),
                    field.field_name,
                    TypeFmt(&*self.registry, &field.field_type),
                ),
                loc: field_access.field_name.loc,
                linked_loc: Some(field.loc),
            })
        };

        if let Type::Pointer { pointee: _ } = lhs_type {
            return Ok(VarInfo::Stored(VarInfoStored {
                address: self.build_code_unit(ctx, &field_access.lhs)?,
                offset: field.byte_offset,
                var_type: field.field_type.clone(),
                inspect_info,
            }));
        }

        if let Some(var) = self.var_from_expr(ctx, &field_access.lhs.as_ref())? {
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
                    inspect_info: parent_inspect_info,
                }) => {
                    if let Some(inspect_info) = parent_inspect_info {
                        self.reporter.print_inspection(&inspect_info);
                    }

                    return Ok(VarInfo::Local(VarInfoLocal {
                        local_index: local_index + field.field_index,
                        var_type: field.field_type.clone(),
                        inspect_info,
                    }));
                }
                VarInfo::Stored(VarInfoStored {
                    address,
                    offset,
                    var_type: _,
                    inspect_info: parent_inspect_info,
                }) => {
                    if let Some(inspect_info) = parent_inspect_info {
                        self.reporter.print_inspection(&inspect_info);
                    }

                    return Ok(VarInfo::Stored(VarInfoStored {
                        address,
                        offset: offset + field.byte_offset,
                        var_type: field.field_type.clone(),
                        inspect_info,
                    }));
                }
                VarInfo::StructValueField(VarInfoStructValueField {
                    struct_value,
                    drops_before,
                    drops_after,
                    var_type: _,
                    inspect_info: parent_inspect_info,
                    loc: _,
                }) => {
                    if let Some(inspect_info) = parent_inspect_info {
                        self.reporter.print_inspection(&inspect_info);
                    }

                    let struct_components_count =
                        self.registry.count_wasm_type_components(&lhs_type);
                    let field_components_count =
                        self.registry.count_wasm_type_components(&field.field_type);

                    return Ok(VarInfo::StructValueField(VarInfoStructValueField {
                        struct_value,
                        drops_before: drops_before + struct_components_count
                            - field.field_index
                            - field_components_count,
                        drops_after: drops_after + field.field_index,
                        var_type: field.field_type.clone(),
                        inspect_info,
                        loc: field_access.field_name.loc,
                    }));
                }
            };
        };

        let struct_components_count = self.registry.count_wasm_type_components(&lhs_type);
        let field_components_count = self.registry.count_wasm_type_components(&field.field_type);

        return Ok(VarInfo::StructValueField(VarInfoStructValueField {
            struct_value: self.build_code_unit(ctx, &field_access.lhs)?,
            drops_before: struct_components_count - field.field_index - field_components_count,
            drops_after: field.field_index,
            var_type: field.field_type.clone(),
            inspect_info,
            loc: field_access.field_name.loc,
        }));
    }

    fn var_from_deref(
        &mut self,
        ctx: &mut ExprContext,
        addr_expr: &CodeExpr,
        op_loc: &Loc,
    ) -> Result<VarInfo, Error> {
        let addr_type = self.registry.get_expr_type(ctx, addr_expr)?;

        if let Type::Pointer { pointee } = &addr_type {
            let mut inspect_info = None;
            if self.reporter.in_inspection_mode {
                inspect_info = Some(InspectInfo {
                    message: format!("<deref>: {}", TypeFmt(&*self.registry, &pointee)),
                    loc: op_loc.clone(),
                    linked_loc: None,
                })
            };

            return Ok(VarInfo::Stored(VarInfoStored {
                address: self.build_code_unit(ctx, addr_expr)?,
                offset: 0,
                var_type: pointee.as_ref().clone(),
                inspect_info,
            }));
        };

        return Err(Error {
            message: format!(
                "Cannot dereference expression of type '{}'",
                TypeFmt(&*self.registry, &addr_type)
            ),
            loc: addr_expr.loc(),
        });
    }

    fn build_code_unit(
        &mut self,
        ctx: &mut ExprContext,
        expr: &CodeExpr,
    ) -> Result<CodeUnit, Error> {
        let mut code_unit = CodeUnit {
            type_: self.registry.get_expr_type(ctx, expr)?,
            instrs: Vec::new(),
        };
        self.codegen(ctx, &mut code_unit.instrs, expr)?;

        Ok(code_unit)
    }

    fn process_const_string(&self, value: String, loc: &Loc) -> Str {
        if let None = *self.registry.first_string_usage {
            *self.registry.first_string_usage.be_mut() = Some(*loc);
        }

        let string_len = value.as_bytes().len() as u32;

        for pooled_str in self.registry.string_pool.iter() {
            if *pooled_str.value == value {
                return Str {
                    ptr: pooled_str.ptr,
                    len: string_len,
                };
            }
        }

        let ptr = self.append_data(value.clone().into_bytes());

        self.registry
            .string_pool
            .be_mut()
            .push(PooledString { value, ptr });

        return Str {
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
        self.registry.datas.be_mut().push(WasmData::Active {
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

    fn codegen_int_literal(
        &self,
        ctx: &mut ExprContext,
        instrs: &mut Vec<WasmInstr>,
        IntLiteralExpr {
            id: _,
            tag,
            repr: _,
            value,
            loc,
        }: &IntLiteralExpr,
    ) {
        let mut is_64_bit = false;
        if let Some(tag) = tag {
            match self
                .registry
                .get_type_or_err(ctx, tag, loc)
                .and_then(|tag_type| self.registry.is_64_bit_int_tag(&tag_type, loc))
            {
                Ok(is_64) => is_64_bit = is_64,
                Err(err) => self.report_error(&err),
            }
        }

        if is_64_bit {
            instrs.push(WasmInstr::I64Const {
                value: *value as i64,
            });
        } else {
            instrs.push(WasmInstr::I32Const {
                value: *value as i32,
            });
        }
    }

    fn codegen_binary_op(
        &self,
        ctx: &ExprContext,
        instrs: &mut Vec<WasmInstr>,
        op_tag: &InfixOpTag,
        operand_type: &Type,
        op_loc: &Loc,
    ) -> Result<(), Error> {
        let kind = self.get_binary_op_kind(ctx, op_tag, operand_type, op_loc)?;
        instrs.push(WasmInstr::BinaryOp { kind });
        Ok(())
    }

    fn get_binary_op_kind(
        &self,
        ctx: &ExprContext,
        op_tag: &InfixOpTag,
        operand_type: &Type,
        op_loc: &Loc,
    ) -> Result<WasmBinaryOpKind, Error> {
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
                return Err(Error {
                    message: format!(
                        "Operator `{}` is incompatible with operands of type {}",
                        op_loc.read_span(&self.registry.modules[ctx.module_id].source),
                        TypeFmt(&*self.registry, operand_type)
                    ),
                    loc: op_loc.clone(),
                });
            }
        }

        use InfixOpTag::*;
        use WasmBinaryOpKind::*;

        return Ok(match wasm_op_type {
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
                    return Err(incompatible_op_err(self, ctx, operand_type, op_loc));
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
                    return Err(incompatible_op_err(self, ctx, operand_type, op_loc));
                }
                _ => unreachable!(),
            },
        });

        fn incompatible_op_err(
            self_: &CodeGenerator,
            ctx: &ExprContext,
            op_type: &Type,
            op_loc: &Loc,
        ) -> Error {
            Error {
                message: format!(
                    "Operator `{}` is incompatible with operands of type {}",
                    op_loc.read_span(&self_.registry.modules[ctx.module_id].source),
                    TypeFmt(&*self_.registry, op_type)
                ),
                loc: op_loc.clone(),
            }
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

    // TODO: remove tag after migration
    fn report_error(&self, err: &Error) {
        let marked_error = Error {
            message: format!("(codegen) {}", err.message),
            loc: err.loc.clone(),
        };
        self.reporter.error(&marked_error);
    }
}
