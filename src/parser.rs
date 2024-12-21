use crate::{core::*, ir::*, lexer::*, wasm::*};
use alloc::{boxed::Box, collections::BTreeMap, format, str, string::String, vec, vec::Vec};
use LoTokenType::*;

const RECEIVER_PARAM_NAME: &str = "self";

pub fn init<'a>(mode: CompilerMode) -> ModuleContext<'a> {
    let mut ctx = ModuleContext::default();
    ctx.mode = mode;

    if ctx.mode == CompilerMode::Inspect {
        stdout_writeln("[");
    }

    return ctx;
}

pub fn parse_file(
    ctx: &mut ModuleContext,
    file_path: &str,
    loc: &LoLocation,
) -> Result<u32, LoError> {
    let file_path = resolve_path(file_path, &loc.file_name);

    if let Some(file_index) = ctx.included_modules.get(&file_path) {
        return Ok(*file_index);
    }

    let chars = file_read_utf8(&file_path).map_err(|message| LoError {
        message,
        loc: loc.clone(),
    })?;

    let file_index = parse_file_contents(ctx, file_path, &chars)?;

    return Ok(file_index);
}

pub fn parse_file_contents(
    ctx: &mut ModuleContext,
    file_path: String,
    chars: &str,
) -> Result<u32, LoError> {
    let tokens = Lexer::lex(&file_path, &chars)?;
    let mut tokens = LoTokenStream::new(tokens.tokens, tokens.end_loc);

    let file_index = ctx.included_modules.len() as u32;
    if ctx.mode == CompilerMode::Inspect {
        stdout_writeln(format!(
            "{{ \"type\": \"file\", \
                \"index\": {file_index}, \
                \"path\": \"{file_path}\" }}, "
        ));
    }
    ctx.included_modules.insert(file_path, file_index);

    parse_file_tokens(ctx, &mut tokens)?;

    return Ok(file_index);
}

fn parse_file_tokens(ctx: &mut ModuleContext, tokens: &mut LoTokenStream) -> Result<(), LoError> {
    while tokens.peek().is_some() {
        parse_top_level_expr(ctx, tokens)?;
        tokens.expect(LoTokenType::Delim, ";")?;
    }

    if let Some(unexpected) = tokens.peek() {
        return Err(LoError {
            message: format!("Unexpected token on top level: {}", unexpected.value),
            loc: unexpected.loc.clone(),
        });
    }

    Ok(())
}

pub fn finalize(ctx: &mut ModuleContext) -> Result<(), LoError> {
    // push function exports
    for fn_export in &ctx.fn_exports {
        let fn_def = ctx.fn_defs.get(&fn_export.in_name).unwrap(); // safe

        ctx.wasm_module.borrow_mut().exports.push(WasmExport {
            export_type: WasmExportType::Func,
            export_name: fn_export.out_name.clone(),
            exported_item_index: fn_def.get_absolute_index(ctx),
        });
    }

    // push function codes
    for mut fn_body in ctx.fn_bodies.take() {
        let fn_def = ctx
            .fn_defs
            .values()
            .find(|fd| fd.local && fd.fn_index == fn_body.fn_index)
            .unwrap();

        let mut fn_ctx = FnContext {
            lo_fn_type: &fn_def.type_,
            locals_last_index: fn_body.locals_last_index,
            non_arg_wasm_locals: vec![],
            defers: vec![],
        };

        let locals_block = Block {
            locals: fn_body.locals,
            ..Default::default()
        };

        let mut block_ctx = BlockContext {
            module: &ctx,
            fn_ctx: &mut fn_ctx,
            block: Block::child_of(ctx, &locals_block).of_kind(LoBlockKind::Function),
        };

        let mut contents = parse_block_contents(&mut block_ctx, &mut fn_body.body, LoType::Void)?;

        if !contents.has_return && !contents.has_never {
            if let Some(mut values) = get_deferred(&mut block_ctx) {
                contents.exprs.append(&mut values);
            };

            let return_type = &fn_def.type_.output;

            match return_type {
                LoType::Void => {}
                LoType::Never => {
                    return Err(LoError {
                        message: format!("This function terminates but is marked as `never`"),
                        loc: fn_def.loc.clone(),
                    });
                }
                _ => {
                    return Err(LoError {
                        message: format!("Missing return expression"),
                        loc: fn_def.loc.clone(),
                    });
                }
            }
        }

        let mut locals = Vec::<WasmLocals>::new();
        for local_type in &block_ctx.fn_ctx.non_arg_wasm_locals {
            if let Some(wasm_locals) = locals.last_mut() {
                if wasm_locals.value_type == *local_type {
                    wasm_locals.count += 1;
                    continue;
                }
            }
            locals.push(WasmLocals {
                count: 1,
                value_type: local_type.clone(),
            });
        }

        let mut instrs = vec![];
        lower_exprs(&mut instrs, &contents.exprs);

        ctx.wasm_module.borrow_mut().codes.push(WasmFn {
            locals,
            expr: WasmExpr { instrs },
        });
    }

    if ctx.mode != CompilerMode::Inspect {
        // put __DATA_SIZE__ value into all globals that contain it
        for global_index in &ctx.indicies_of_data_size_globals {
            let instrs = &mut ctx.wasm_module.borrow_mut().globals[*global_index]
                .initial_value
                .instrs;

            // drop stub value
            instrs.clear();

            lower_expr(
                instrs,
                &LoInstr::U32Const {
                    value: *ctx.data_size.borrow(),
                },
            );
        }
    }

    if ctx.mode == CompilerMode::Compile || ctx.mode == CompilerMode::Eval {
        write_debug_info(ctx)?;
    }

    if ctx.mode == CompilerMode::Inspect {
        stdout_writeln("{ \"type\": \"end\" }");

        stdout_writeln("]");
    }

    Ok(())
}

// TODO: add local names (requires sizable refactoring to achieve)
fn write_debug_info(ctx: &mut ModuleContext) -> Result<(), LoError> {
    use crate::wasm::*;

    let mut wasm_module = ctx.wasm_module.borrow_mut();

    let first_own_fn_index = ctx.imported_fns_count;
    let own_fns_count = wasm_module.functions.len() as u32;

    /* function names */
    {
        for fn_index in first_own_fn_index..first_own_fn_index + own_fns_count {
            let (fn_name, _) = ctx
                .fn_defs
                .iter()
                .find(|(_, v)| v.get_absolute_index(ctx) == fn_index)
                .unwrap();

            wasm_module.debug_fn_info.push(WasmDebugFnInfo {
                fn_index,
                fn_name: fn_name.clone(),
            })
        }
    }

    Ok(())
}

fn parse_top_level_expr(
    ctx: &mut ModuleContext,
    tokens: &mut LoTokenStream,
) -> Result<(), LoError> {
    if tokens.peek().is_none() {
        return Ok(());
    }

    if let Some(_) = tokens.eat(Symbol, "fn")? {
        return parse_fn_def(ctx, tokens, false);
    }

    if let Some(_) = tokens.eat(Symbol, "macro")? {
        return parse_macro_def(ctx, tokens);
    }

    if let Some(_) = tokens.eat(Operator, "*")? {
        let offset = parse_const_expr(ctx, tokens, 2)?;
        let Some(WasmType::I32) = offset.get_type(ctx).to_wasm_type() else {
            return Err(LoError {
                message: format!("Invalid memory offset"),
                loc: tokens.loc().clone(),
            });
        };

        tokens.expect(Operator, "=")?;

        let bytes = if let Some(data) = tokens.eat_any(StringLiteral)? {
            let value = Lexer::unescape_string(&data.value);
            value.as_bytes().iter().map(|b| *b).collect()
        } else {
            parse_const_sequence(ctx, tokens)?.1
        };

        let mut instrs = vec![];
        lower_expr(&mut instrs, &offset);

        ctx.wasm_module.borrow_mut().datas.push(WasmData::Active {
            offset: WasmExpr { instrs },
            bytes,
        });

        return Ok(());
    }

    if let Some(_) = tokens.eat(Symbol, "memory")? {
        parse_memory(ctx, tokens, true)?;
        return Ok(());
    }

    if let Some(_) = tokens.eat(Symbol, "export")? {
        if let Some(_) = tokens.eat(Symbol, "fn")? {
            return parse_fn_def(ctx, tokens, true);
        }

        if let Some(_) = tokens.eat(Symbol, "memory")? {
            let (memory_index, _) = parse_memory(ctx, tokens, true)?;

            ctx.wasm_module.borrow_mut().exports.push(WasmExport {
                export_type: WasmExportType::Mem,
                export_name: "memory".into(),
                exported_item_index: memory_index,
            });

            return Ok(());
        }

        if let Some(_) = tokens.eat(Symbol, "existing")? {
            tokens.expect(Symbol, "fn")?;
            let in_name = parse_nested_symbol(tokens)?;
            if let None = ctx.fn_defs.get(&in_name.value) {
                return Err(LoError {
                    message: format!("Cannot export unknown function {}", in_name.value),
                    loc: in_name.loc,
                });
            }

            tokens.expect(Symbol, "as")?;
            let out_name = tokens.expect_any(StringLiteral)?;
            let out_name = Lexer::unescape_string(&out_name.value);

            ctx.fn_exports.push(FnExport {
                in_name: in_name.value,
                out_name,
            });

            return Ok(());
        }
    }

    if let Some(_) = tokens.eat(Symbol, "import")? {
        tokens.expect(Symbol, "from")?;
        let module_name = tokens.expect_any(StringLiteral)?;
        let module_name = Lexer::unescape_string(&module_name.value);

        tokens.expect(Delim, "{")?;
        while let None = tokens.eat(Delim, "}")? {
            if let Some(_) = tokens.eat(Symbol, "memory")? {
                let (_, limits) = parse_memory(ctx, tokens, false)?;
                tokens.expect(LoTokenType::Delim, ";")?;

                ctx.memory_defined = true;
                ctx.wasm_module.borrow_mut().imports.push(WasmImport {
                    module_name: module_name.clone(),
                    item_name: "memory".into(),
                    item_desc: WasmImportDesc::Memory(limits),
                });

                continue;
            }

            tokens.expect(Symbol, "fn")?;
            let fn_decl = parse_fn_decl(ctx, tokens)?;
            tokens.expect(LoTokenType::Delim, ";")?;

            if ctx.fn_defs.contains_key(&fn_decl.fn_name) {
                return Err(LoError {
                    message: format!("Cannot redefine function: {}", fn_decl.fn_name),
                    loc: fn_decl.loc,
                });
            }

            let type_index = ctx.insert_fn_type(fn_decl.wasm_type);

            let fn_index = ctx.imported_fns_count;
            ctx.imported_fns_count += 1;

            let fn_def = FnDef {
                local: false,
                fn_index,
                fn_params: fn_decl.fn_params,
                type_: fn_decl.lo_type,
                loc: fn_decl.loc,
            };
            ctx.fn_defs.insert(fn_decl.fn_name.clone(), fn_def);
            ctx.wasm_module.borrow_mut().imports.push(WasmImport {
                module_name: module_name.clone(),
                item_name: fn_decl.method_name,
                item_desc: WasmImportDesc::Func { type_index },
            });
        }

        return Ok(());
    }

    if let Some(_) = tokens.eat(Symbol, "global")?.cloned() {
        let mutable = true;
        let global_name = parse_nested_symbol(tokens)?;
        tokens.expect(Operator, "=")?;

        let global_index = ctx.globals.len();

        let global_value: LoInstr;
        if let Some(_) = tokens.eat(LoTokenType::Operator, "@")? {
            tokens.expect(LoTokenType::Symbol, "data_size")?;

            ctx.indicies_of_data_size_globals.push(global_index);
            global_value = LoInstr::U32Const { value: 0 }; // stub, will changed in `finalize`
        } else {
            global_value = parse_const_expr(ctx, tokens, 0)?;
        }

        let lo_type = global_value.get_type(ctx);
        let Some(wasm_type) = lo_type.to_wasm_type() else {
            return Err(LoError {
                message: format!(
                    "Unsupported top level type: {lo_type}, only primitives are supported"
                ),
                loc: global_name.loc.clone(),
            });
        };

        if ctx.globals.contains_key(&global_name.value) {
            return Err(LoError {
                message: format!("Cannot redefine global: {}", global_name.value),
                loc: global_name.loc,
            });
        }

        if ctx.mode == CompilerMode::Inspect {
            let source_index = ctx.get_loc_module_index(&global_name.loc);
            let source_range = RangeDisplay(&global_name.loc);

            let global_name = &global_name.value;

            stdout_writeln(format!(
                "{{ \"type\": \"info\", \
                    \"hover\": \"let {global_name}: {lo_type}\", \
                    \"loc\": \"{source_index}/{source_range}\" }}, ",
            ));
        }

        ctx.globals.insert(
            global_name.value.clone(),
            GlobalDef {
                index: global_index as u32,
                value_type: lo_type,
                loc: global_name.loc,
            },
        );

        let mut instrs = vec![];
        lower_expr(&mut instrs, &global_value);

        ctx.wasm_module.borrow_mut().globals.push(WasmGlobal {
            mutable,
            value_type: wasm_type,
            initial_value: WasmExpr { instrs },
        });

        return Ok(());
    }

    if let Some(_) = tokens.eat(Symbol, "struct")? {
        let struct_name = parse_nested_symbol(tokens)?;

        if let Some(_) = ctx.type_scope.get(&struct_name.value) {
            return Err(LoError {
                message: format!("Cannot redefine type {}", struct_name.value),
                loc: struct_name.loc,
            });
        }

        // declare not fully defined struct to use in self-references
        ctx.struct_defs.push(StructDef {
            name: struct_name.value.clone(),
            fields: vec![],
            fully_defined: false,
            loc: struct_name.loc,
        });

        ctx.type_scope.insert(
            struct_name.value.clone(),
            LoType::StructInstance {
                name: struct_name.value.clone(),
            },
        );

        let mut field_index = 0;
        let mut byte_offset = 0;
        let mut struct_fields = Vec::<StructField>::new();

        tokens.expect(Delim, "{")?;
        while let None = tokens.eat(Delim, "}")? {
            let field_name = tokens.expect_any(Symbol)?.clone();
            tokens.expect(Operator, ":")?;
            let field_type_loc = tokens.loc().clone();
            let field_type = parse_const_lo_type(ctx, tokens)?;
            if !tokens.next_is(Delim, "}")? {
                tokens.expect(Delim, ",")?;
            }

            if struct_fields
                .iter()
                .find(|f| f.name == field_name.value)
                .is_some()
            {
                return Err(LoError {
                    message: format!(
                        "Found duplicate struct field name: '{}' of struct {}",
                        field_name.value, struct_name.value,
                    ),
                    loc: field_name.loc,
                });
            }

            let mut stats = EmitComponentStats::default();
            field_type
                .emit_sized_component_stats(ctx, &mut stats, &mut vec![])
                .map_err(|err| LoError {
                    message: err,
                    loc: field_type_loc,
                })?;

            struct_fields.push(StructField {
                name: field_name.value,
                value_type: field_type,
                field_index,
                byte_offset,
                loc: field_name.loc,
            });

            field_index += stats.count;
            byte_offset += stats.byte_length;
        }

        let struct_def = ctx.get_struct_def_mut(&struct_name.value).unwrap();
        struct_def.fields.append(&mut struct_fields);
        struct_def.fully_defined = true;

        return Ok(());
    }

    if let Some(_) = tokens.eat(Symbol, "type")?.cloned() {
        let type_alias = parse_nested_symbol(tokens)?;
        tokens.expect(Operator, "=")?;
        let actual_type = parse_const_lo_type(ctx, tokens)?;

        if let Some(_) = ctx.type_scope.get(&type_alias.value) {
            return Err(LoError {
                message: format!("Cannot redefine type: {}", type_alias.value),
                loc: type_alias.loc.clone(),
            });
        }

        ctx.type_scope.insert(type_alias.value, actual_type);

        return Ok(());
    }

    if let Some(_) = tokens.eat(Symbol, "const")?.cloned() {
        let const_name = parse_nested_symbol(tokens)?;
        tokens.expect(Operator, "=")?;
        let const_value = parse_const_expr(ctx, tokens, 0)?;

        if ctx.constants.borrow().contains_key(&const_name.value) {
            return Err(LoError {
                message: format!("Duplicate constant: {}", const_name.value),
                loc: const_name.loc.clone(),
            });
        }

        if ctx.mode == CompilerMode::Inspect {
            let source_index = ctx.get_loc_module_index(&const_name.loc);
            let source_range = RangeDisplay(&const_name.loc);

            let const_name = &const_name.value;
            let const_type = const_value.get_type(ctx);

            stdout_writeln(format!(
                "{{ \"type\": \"info\", \
                    \"hover\": \"const {const_name}: {const_type}\", \
                    \"loc\": \"{source_index}/{source_range}\" }}, ",
            ));
        }

        ctx.constants.borrow_mut().insert(
            const_name.value,
            ConstDef {
                value: const_value,
                loc: const_name.loc,
            },
        );

        return Ok(());
    }

    if let Some(_) = tokens.eat(Symbol, "include")?.cloned() {
        let file_path = tokens.expect_any(StringLiteral)?;
        let loc = &file_path.loc;
        let file_path = Lexer::unescape_string(&file_path.value);

        let target_index = parse_file(ctx, &file_path, loc)?;

        if ctx.mode == CompilerMode::Inspect {
            let source_index = ctx.get_loc_module_index(loc);
            let source_range = RangeDisplay(loc);
            let target_range = "1:1-1:1";

            stdout_writeln(format!(
                "{{ \"type\": \"info\", \
                    \"link\": \"{target_index}/{target_range}\", \
                    \"loc\": \"{source_index}/{source_range}\" }}, ",
            ));
        }

        return Ok(());
    }

    let unexpected = tokens.peek().unwrap();
    return Err(LoError {
        message: format!("Unexpected top level token: {}", unexpected.value),
        loc: unexpected.loc.clone(),
    });
}

fn parse_memory(
    ctx: &mut ModuleContext,
    tokens: &mut LoTokenStream,
    add_memory: bool,
) -> Result<(u32, WasmLimits), LoError> {
    let memory_name = String::from("memory");
    if ctx.memories.contains_key(&memory_name) {
        return Err(LoError {
            message: format!("Duplicate memory definition: {memory_name}"),
            loc: tokens.loc().clone(),
        });
    }

    let mut memory_limits = WasmLimits { min: 0, max: None };

    tokens.expect(Delim, "{")?;
    while let None = tokens.eat(Delim, "}")? {
        let prop = tokens.expect_any(Symbol)?.clone();
        tokens.expect(Operator, ":")?;

        match prop.value.as_str() {
            "min_pages" => {
                let value = parse_u32_literal(tokens.expect_any(IntLiteral)?)?;
                memory_limits.min = value;
            }
            "max_pages" => {
                let value = parse_u32_literal(tokens.expect_any(IntLiteral)?)?;
                memory_limits.max = Some(value);
            }
            "data_start" => {
                let value = parse_u32_literal(tokens.expect_any(IntLiteral)?)?;
                *ctx.data_size.borrow_mut() = value;
            }
            _ => {
                return Err(LoError {
                    message: format!("Invalid memory property"),
                    loc: prop.loc,
                });
            }
        }

        if !tokens.next_is(Delim, "}")? {
            tokens.expect(Delim, ",")?;
        }
    }

    let memory_index = ctx.wasm_module.borrow().memories.len() as u32;

    if add_memory {
        ctx.wasm_module
            .borrow_mut()
            .memories
            .push(memory_limits.clone());
        ctx.memories.insert(memory_name.clone(), memory_index);
        ctx.memory_defined = true;
    }

    Ok((memory_index, memory_limits))
}

fn parse_fn_def(
    ctx: &mut ModuleContext,
    tokens: &mut LoTokenStream,
    exported: bool,
) -> Result<(), LoError> {
    let fn_decl = parse_fn_decl(ctx, tokens)?;
    let body = collect_block_tokens(tokens)?;

    if ctx.fn_defs.contains_key(&fn_decl.fn_name) {
        return Err(LoError {
            message: format!("Cannot redefine function: {}", fn_decl.fn_name),
            loc: fn_decl.loc,
        });
    }

    if exported {
        ctx.fn_exports.push(FnExport {
            in_name: fn_decl.fn_name.clone(),
            out_name: fn_decl.fn_name.clone(),
        });
    }

    let locals_last_index = fn_decl.wasm_type.inputs.len() as u32;
    let type_index = ctx.insert_fn_type(fn_decl.wasm_type);
    ctx.wasm_module.borrow_mut().functions.push(type_index);

    let fn_index = ctx.wasm_module.borrow_mut().functions.len() as u32 - 1;

    ctx.fn_defs.insert(
        fn_decl.fn_name,
        FnDef {
            local: true,
            fn_index,
            fn_params: fn_decl.fn_params,
            type_: fn_decl.lo_type,
            loc: fn_decl.loc,
        },
    );

    ctx.fn_bodies.borrow_mut().push(FnBody {
        fn_index,
        locals: fn_decl.locals,
        locals_last_index,
        body,
    });

    return Ok(());
}

fn parse_macro_def(ctx: &mut ModuleContext, tokens: &mut LoTokenStream) -> Result<(), LoError> {
    let macro_name = parse_nested_symbol(tokens)?;
    tokens.expect(Operator, "!")?;

    if ctx.macros.contains_key(&macro_name.value) {
        return Err(LoError {
            message: format!("Cannot redefine macro: {}", macro_name.value),
            loc: macro_name.loc,
        });
    }

    let (receiver_type, _) = extract_method_receiver_and_name(ctx, &macro_name)?;
    let mut type_params = Vec::<String>::new();

    if let Some(_) = tokens.eat(Operator, "<")? {
        while let None = tokens.eat(Operator, ">")? {
            let p_name = tokens.expect_any(Symbol)?.clone();
            if !tokens.next_is(Operator, ">")? {
                tokens.expect(Delim, ",")?;
            }

            if get_type_by_name(ctx, &ctx.type_scope, &p_name, false).is_ok() {
                return Err(LoError {
                    message: format!("Type parameter shadows existing type: {}", p_name.value),
                    loc: p_name.loc.clone(),
                });
            }

            for param in &type_params {
                if *param == p_name.value {
                    return Err(LoError {
                        message: format!("Found duplicate type parameter: {}", p_name.value),
                        loc: p_name.loc.clone(),
                    });
                }
            }

            type_params.push(p_name.value);
        }
    }

    let mut new_type_scope = LoTypeScope {
        parent: Some(&ctx.type_scope),
        ..Default::default()
    };
    for type_param in &type_params {
        new_type_scope.insert(
            type_param.clone(),
            LoType::MacroTypeArg {
                name: type_param.clone(),
            },
        )
    }

    let params = parse_fn_params(ctx, &new_type_scope, tokens, &receiver_type)?;
    let return_type = if let Some(_) = tokens.eat(Operator, ":")? {
        parse_lo_type_(ctx, &new_type_scope, tokens, false)?
    } else {
        LoType::Void
    };
    let body = collect_block_tokens(tokens)?;

    ctx.macros.insert(
        macro_name.value.clone(),
        MacroDef {
            type_params,
            params,
            return_type,
            body,
            loc: macro_name.loc,
        },
    );

    return Ok(());
}

struct FnDecl {
    fn_name: String,
    method_name: String,
    loc: LoLocation,
    fn_params: Vec<FnParam>,
    lo_type: LoFnType,
    wasm_type: WasmFnType,
    locals: BTreeMap<String, LocalDef>,
}

fn parse_fn_decl(ctx: &mut ModuleContext, tokens: &mut LoTokenStream) -> Result<FnDecl, LoError> {
    let fn_name = parse_nested_symbol(tokens)?;
    let (receiver_type, method_name) = extract_method_receiver_and_name(ctx, &fn_name)?;

    let params = parse_fn_params(ctx, &ctx.type_scope, tokens, &receiver_type)?;

    let mut fn_decl = FnDecl {
        fn_name: fn_name.value.clone(),
        fn_params: params.clone(),
        method_name,
        loc: fn_name.loc.clone(),
        lo_type: LoFnType {
            inputs: vec![],
            output: LoType::Void,
        },
        wasm_type: WasmFnType {
            inputs: vec![],
            outputs: vec![],
        },
        locals: BTreeMap::new(),
    };

    for param in params {
        let local_def = LocalDef {
            index: fn_decl.wasm_type.inputs.len() as u32,
            value_type: param.type_.clone(),
            loc: param.loc,
        };
        fn_decl.locals.insert(param.name, local_def);

        param
            .type_
            .emit_components(ctx, &mut fn_decl.wasm_type.inputs);

        fn_decl.lo_type.inputs.push(param.type_);
    }

    let lo_output = if let Some(_) = tokens.eat(Operator, ":")? {
        parse_const_lo_type(ctx, tokens)?
    } else {
        LoType::Void
    };

    lo_output.emit_components(&ctx, &mut fn_decl.wasm_type.outputs);
    fn_decl.lo_type.output = lo_output;

    Ok(fn_decl)
}

fn parse_fn_params(
    ctx: &ModuleContext,
    type_scope: &LoTypeScope,
    tokens: &mut LoTokenStream,
    receiver_type: &Option<LoType>,
) -> Result<Vec<FnParam>, LoError> {
    let mut params = Vec::new();

    tokens.expect(Delim, "(")?;

    if let Some(receiver_type) = &receiver_type {
        if let Some(self_token) = tokens.eat(Symbol, RECEIVER_PARAM_NAME)?.cloned() {
            if !tokens.next_is(Delim, ")")? {
                tokens.expect(Delim, ",")?;
            }

            params.push(FnParam {
                name: String::from(RECEIVER_PARAM_NAME),
                type_: receiver_type.clone(),
                loc: self_token.loc,
            });
        } else if let Some(_) = tokens.eat(Operator, "&")? {
            let self_token = tokens.expect(Symbol, RECEIVER_PARAM_NAME)?.clone();
            if !tokens.next_is(Delim, ")")? {
                tokens.expect(Delim, ",")?;
            }

            params.push(FnParam {
                name: String::from(RECEIVER_PARAM_NAME),
                type_: LoType::Pointer(Box::new(receiver_type.clone())),
                loc: self_token.loc,
            });
        };
    }

    while let None = tokens.eat(Delim, ")")? {
        let p_name = tokens.expect_any(Symbol)?.clone();
        tokens.expect(Operator, ":")?;
        let p_type = parse_lo_type_(ctx, type_scope, tokens, false)?;
        if !tokens.next_is(Delim, ")")? {
            tokens.expect(Delim, ",")?;
        }

        for param in &params {
            if param.name == p_name.value {
                return Err(LoError {
                    message: format!(
                        "Found function param with conflicting name: {}",
                        p_name.value
                    ),
                    loc: p_name.loc.clone(),
                });
            }
        }

        params.push(FnParam {
            name: p_name.value,
            type_: p_type,
            loc: p_name.loc,
        });
    }

    Ok(params)
}

fn parse_block(
    ctx: &mut BlockContext,
    tokens: &mut LoTokenStream,
) -> Result<Vec<LoInstr>, LoError> {
    let mut block_tokens = collect_block_tokens(tokens)?;
    let contents = parse_block_contents(ctx, &mut block_tokens, LoType::Void)?;
    Ok(contents.exprs)
}

fn collect_block_tokens(tokens: &mut LoTokenStream) -> Result<LoTokenStream, LoError> {
    let mut output = LoTokenStream::new(vec![], LoLocation::internal());

    let mut depth = 0;
    tokens.expect(Delim, "{")?;
    loop {
        if let Some(t) = tokens.eat(Delim, "{")? {
            output.tokens.push(t.clone());
            depth += 1;
            continue;
        }
        if let Some(t) = tokens.eat(Delim, "}")? {
            if depth == 0 {
                output.terminal_token = t.clone();
                break;
            }
            output.tokens.push(t.clone());
            depth -= 1;
            continue;
        }
        output.tokens.push(tokens.next().unwrap().clone());
    }

    Ok(output)
}

fn parse_expr(
    ctx: &mut BlockContext,
    tokens: &mut LoTokenStream,
    min_bp: u32,
) -> Result<LoInstr, LoError> {
    let mut primary = parse_primary(ctx, tokens)?;

    while tokens.peek().is_some() {
        let op_symbol = tokens.peek().unwrap().clone();
        let Some(op) = InfixOp::parse(op_symbol) else {
            break;
        };

        if op.info.bp < min_bp {
            break;
        }

        tokens.next(); // skip operator
        primary = parse_postfix(ctx, tokens, primary, op)?;
    }

    Ok(primary)
}

fn parse_primary(ctx: &mut BlockContext, tokens: &mut LoTokenStream) -> Result<LoInstr, LoError> {
    if tokens.next_is_any(IntLiteral)? {
        return parse_const_int(tokens);
    }

    if let Some(value) = tokens.eat_any(CharLiteral)? {
        return Ok(LoInstr::U32Const {
            value: Lexer::parse_char_literal_value(&value.value),
        }
        .casted(LoType::U8));
    }

    if let Some(value) = tokens.eat_any(StringLiteral)? {
        let value = Lexer::unescape_string(&value.value);
        return parse_const_str(ctx.module, tokens, value);
    }

    if let Some(_) = tokens.eat(Delim, "[")? {
        let (item_type, bytes) = parse_const_sequence(ctx.module, tokens)?;
        let bytes_ptr = ctx.module.append_data(bytes);

        return Ok(
            LoInstr::U32Const { value: bytes_ptr }.casted(LoType::Pointer(Box::new(item_type)))
        );
    }

    if let Some(_) = tokens.eat(Symbol, "true")?.cloned() {
        return Ok(LoInstr::U32Const { value: 1 }.casted(LoType::Bool));
    }

    if let Some(_) = tokens.eat(Symbol, "false")?.cloned() {
        return Ok(LoInstr::U32Const { value: 0 }.casted(LoType::Bool));
    }

    if let Some(_) = tokens.eat(Symbol, "unreachable")? {
        return Ok(LoInstr::Unreachable);
    }

    if let Some(_) = tokens.eat(Delim, "(")? {
        let expr = parse_expr(ctx, tokens, 0)?;
        tokens.expect(Delim, ")")?;
        return Ok(expr);
    }

    if let Some(return_token) = tokens.eat(Symbol, "return")?.cloned() {
        let mut value = if tokens.next_is(Delim, ";")? {
            LoInstr::NoInstr
        } else {
            parse_expr(ctx, tokens, 0)?
        };

        let return_type = value.get_type(ctx.module);
        let expected_return_type = &ctx.fn_ctx.lo_fn_type.output;

        if return_type != *expected_return_type {
            return Err(LoError {
                message: format!(
                    "Invalid return type, \
                        expected {expected_return_type}, got {return_type}",
                ),
                loc: return_token.loc,
            });
        }

        if let Some(mut values) = get_deferred(ctx) {
            values.insert(0, value);
            value = LoInstr::MultiValueEmit { values }.casted(LoType::Void);
        }

        return Ok(LoInstr::Return {
            value: Box::new(value),
        });
    }

    if let Some(t) = tokens.eat(Symbol, "sizeof")?.cloned() {
        let value_type = parse_lo_type(ctx, tokens)?;

        return Ok(LoInstr::U32Const {
            value: value_type
                .sized_comp_stats(&ctx.module)
                .map_err(|err| LoError {
                    message: err,
                    loc: t.loc.clone(),
                })?
                .byte_length as u32,
        });
    }

    if let Some(_) = tokens.eat(Symbol, "defer")? {
        let deffered_expr = parse_expr(ctx, tokens, 0)?;

        ctx.fn_ctx.defers.push(deffered_expr);

        return Ok(LoInstr::NoInstr);
    }

    if let Some(_) = tokens.eat(Symbol, "__memory_size")? {
        tokens.expect(Delim, "(")?;
        tokens.expect(Delim, ")")?;
        return Ok(LoInstr::MemorySize {});
    }

    if let Some(t) = tokens.eat(Symbol, "__memory_grow")?.cloned() {
        tokens.expect(Delim, "(")?;
        let num_bytes = parse_expr(ctx, tokens, 0)?;
        tokens.eat(Delim, ",")?; // optional
        tokens.expect(Delim, ")")?;

        let num_bytes_type = num_bytes.get_type(ctx.module);
        if num_bytes_type != LoType::U32 {
            return Err(LoError {
                message: format!(
                    "Invalid arguments for {}, got [{}], expected [{}]",
                    t.value,
                    num_bytes_type,
                    LoType::U32
                ),
                loc: t.loc,
            });
        };

        return Ok(LoInstr::MemoryGrow {
            num_bytes: Box::new(num_bytes),
        });
    }

    if let Some(t) = tokens.eat(Symbol, "__memory_copy")?.cloned() {
        tokens.expect(Delim, "(")?;

        let destination = parse_expr(ctx, tokens, 0)?;
        let destination_type = destination.get_type(ctx.module);
        tokens.expect(Delim, ",")?;

        let source = parse_expr(ctx, tokens, 0)?;
        let source_type = source.get_type(ctx.module);
        tokens.expect(Delim, ",")?;

        let num_bytes = parse_expr(ctx, tokens, 0)?;
        let num_bytes_type = num_bytes.get_type(ctx.module);
        tokens.eat(Delim, ",")?; // optional

        tokens.expect(Delim, ")")?;

        if destination_type != LoType::U32
            || source_type != LoType::U32
            || num_bytes_type != LoType::U32
        {
            return Err(LoError {
                message: format!(
                    "Invalid arguments for {}, got [{}], expected [{}]",
                    t.value,
                    ListDisplay(&vec![destination_type, source_type, num_bytes_type]),
                    ListDisplay(&vec![LoType::U32, LoType::U32, LoType::U32]),
                ),
                loc: t.loc,
            });
        };

        return Ok(LoInstr::MemoryCopy {
            destination: Box::new(destination),
            source: Box::new(source),
            num_bytes: Box::new(num_bytes),
        });
    }

    if let Some(t) = tokens.eat(Symbol, "__debug_typeof")?.cloned() {
        let loc = tokens.peek().unwrap_or(&t).loc.clone();

        let expr = parse_expr(ctx, tokens, 0)?;
        let expr_type = expr.get_type(ctx.module);
        debug(format!(
            "{}",
            LoError {
                message: format!("{expr_type:?}"),
                loc,
            }
        ));
        return Ok(LoInstr::NoInstr);
    }

    if let Some(dbg_token) = tokens.eat(Symbol, "dbg")?.cloned() {
        let message = tokens.expect_any(StringLiteral)?;
        let message = Lexer::unescape_string(&message.value);
        let debug_message = format!("{} - {}", dbg_token.loc, message);
        return parse_const_str(ctx.module, tokens, debug_message);
    }

    if let Some(_) = tokens.eat(Symbol, "if")? {
        let cond = parse_expr(ctx, tokens, 0)?;

        let then_branch = parse_block(
            &mut BlockContext {
                module: ctx.module,
                fn_ctx: ctx.fn_ctx,
                block: Block::child_of(ctx.module, &ctx.block),
            },
            tokens,
        )?;

        let mut else_branch = None;
        if let Some(_) = tokens.eat(Symbol, "else")? {
            let else_ctx = &mut BlockContext {
                module: ctx.module,
                fn_ctx: ctx.fn_ctx,
                block: Block::child_of(ctx.module, &ctx.block),
            };
            if tokens.next_is(Symbol, "if")? {
                else_branch = Some(vec![parse_expr(else_ctx, tokens, 0)?]);
            } else {
                else_branch = Some(parse_block(else_ctx, tokens)?)
            }
        }

        return Ok(LoInstr::If {
            block_type: LoBlockType::void(),
            cond: Box::new(cond),
            then_branch,
            else_branch,
        });
    }

    if let Some(_) = tokens.eat(Symbol, "loop")? {
        let mut ctx = BlockContext {
            module: ctx.module,
            fn_ctx: ctx.fn_ctx,
            block: Block::child_of(ctx.module, &ctx.block).of_kind(LoBlockKind::Loop),
        };

        let mut body = parse_block(&mut ctx, tokens)?;

        let implicit_continue = LoInstr::Branch { label_index: 0 };
        body.push(implicit_continue);

        return Ok(LoInstr::Block {
            block_type: LoBlockType::void(),
            body: vec![LoInstr::Loop {
                block_type: LoBlockType::void(),
                body,
            }],
        });
    }

    if let Some(for_loop) = tokens.eat(Symbol, "for")?.cloned() {
        let counter = tokens.expect_any(Symbol).cloned()?;
        tokens.expect(Symbol, "in")?;
        let counter_ctx = &mut BlockContext {
            module: ctx.module,
            fn_ctx: ctx.fn_ctx,
            block: Block::child_of(ctx.module, &ctx.block),
        };

        let start_count = parse_expr(counter_ctx, tokens, 0)?;
        tokens.expect(Operator, "..")?;
        let end_count = parse_expr(counter_ctx, tokens, 0)?;

        let counter_type = start_count.get_type(counter_ctx.module);
        if end_count.get_type(counter_ctx.module) != counter_type {
            return Err(LoError {
                message: format!(
                    "Invalid end count type: {}, expected: {counter_type}",
                    end_count.get_type(counter_ctx.module)
                ),
                loc: for_loop.loc,
            });
        }

        let check_op_kind;
        let add_op_kind;
        let step_instr;
        match counter_type {
            LoType::Bool | LoType::I8 | LoType::U8 | LoType::I32 | LoType::U32 => {
                check_op_kind = WasmBinaryOpKind::I32_EQ;
                add_op_kind = WasmBinaryOpKind::I32_ADD;
                step_instr = LoInstr::U32Const { value: 1 };
            }
            LoType::I64 | LoType::U64 => {
                check_op_kind = WasmBinaryOpKind::I64_EQ;
                add_op_kind = WasmBinaryOpKind::I64_ADD;
                step_instr = LoInstr::U64Const { value: 1 };
            }
            _ => {
                return Err(LoError {
                    message: format!("Invalid counter type: {counter_type}",),
                    loc: for_loop.loc,
                })
            }
        };

        let init_instr = define_local(counter_ctx, &counter, start_count, counter_type.clone())?;
        let get_counter_instr = LoInstr::LocalGet {
            local_index: counter_ctx
                .block
                .get_own_local(&counter.value)
                .unwrap()
                .index,
            value_type: counter_type.clone(),
        };

        let break_instr = LoInstr::Branch { label_index: 2 };
        let implicit_continue = LoInstr::Branch { label_index: 0 };

        let end_check_instr = LoInstr::If {
            block_type: LoBlockType::void(),
            cond: Box::new(LoInstr::BinaryOp {
                kind: check_op_kind,
                lhs: Box::new(get_counter_instr.clone()),
                rhs: Box::new(end_count),
            }),
            then_branch: vec![break_instr],
            else_branch: None,
        };
        let update_instr = compile_set(
            counter_ctx,
            LoInstr::BinaryOp {
                kind: add_op_kind,
                lhs: Box::new(get_counter_instr.clone()),
                rhs: Box::new(step_instr),
            },
            get_counter_instr,
            &for_loop.loc,
        )?;

        let loop_body_ctx = &mut BlockContext {
            module: counter_ctx.module,
            fn_ctx: counter_ctx.fn_ctx,
            block: Block::child_of(ctx.module, &counter_ctx.block).of_kind(LoBlockKind::ForLoop),
        };
        let loop_body = parse_block(loop_body_ctx, tokens)?;

        let instrs = vec![
            init_instr,
            LoInstr::Block {
                block_type: LoBlockType::void(),
                body: vec![LoInstr::Loop {
                    body: vec![
                        end_check_instr,
                        LoInstr::Block {
                            block_type: LoBlockType::void(),
                            body: loop_body,
                        },
                        update_instr,
                        implicit_continue,
                    ],
                    block_type: LoBlockType::void(),
                }],
            },
        ];

        return Ok(LoInstr::MultiValueEmit { values: instrs }.casted(LoType::Void));
    }

    if let Some(break_token) = tokens.eat(Symbol, "break")? {
        let mut label_index = 1; // 0 = loop, 1 = loop wrapper block

        let mut current_block = &ctx.block;
        loop {
            if current_block.block_kind == LoBlockKind::Function {
                return Err(LoError {
                    message: format!("Cannot break outside of a loop"),
                    loc: break_token.loc.clone(),
                });
            }

            if current_block.block_kind == LoBlockKind::Loop {
                break;
            }

            if current_block.block_kind == LoBlockKind::ForLoop {
                label_index += 1;
                break;
            }

            current_block = current_block.parent.unwrap();
            label_index += 1;
        }

        return Ok(LoInstr::Branch { label_index });
    }

    if let Some(continue_token) = tokens.eat(Symbol, "continue")? {
        let mut label_index = 0; // 0 = loop, 1 = loop wrapper block

        let mut current_block = &ctx.block;
        loop {
            if current_block.block_kind == LoBlockKind::Function {
                return Err(LoError {
                    message: format!("Cannot continue outside of a loop"),
                    loc: continue_token.loc.clone(),
                });
            }

            if current_block.block_kind == LoBlockKind::Loop {
                break;
            }

            if current_block.block_kind == LoBlockKind::ForLoop {
                break;
            }

            current_block = current_block.parent.unwrap();
            label_index += 1;
        }

        return Ok(LoInstr::Branch { label_index });
    }

    if let Some(_) = tokens.eat(Symbol, "let")?.cloned() {
        let local_name = tokens.expect_any(Symbol)?.clone();
        tokens.expect(Operator, "=")?;
        let value = parse_expr(ctx, tokens, 0)?;
        let value_type = value.get_type(ctx.module);

        if local_name.value == "_" {
            let drop_count = value_type.emit_components(&ctx.module, &mut vec![]);

            return Ok(LoInstr::Drop {
                value: Box::new(value),
                drop_count,
            });
        }

        if let Some(_) = ctx.module.globals.get(&local_name.value) {
            return Err(LoError {
                message: format!("Local name collides with global: {}", local_name.value),
                loc: local_name.loc.clone(),
            });
        };

        return define_local(ctx, &local_name, value, value_type);
    }

    if let Some(ok_token) = tokens.eat(Symbol, "Ok")?.cloned() {
        let expected_ok_type: LoType;
        let expected_err_type: LoType;
        if let Some(_) = tokens.eat(Operator, "::")? {
            tokens.expect(Delim, "<")?;
            expected_ok_type = parse_lo_type(ctx, tokens)?;
            tokens.expect(Delim, ",")?;
            expected_err_type = parse_lo_type(ctx, tokens)?;
            tokens.expect(Delim, ">")?;
        } else {
            let LoType::Result { ok_type, err_type } = &ctx.fn_ctx.lo_fn_type.output else {
                return Err(LoError {
                    message: format!("Cannot infer Result type from function's return type",),
                    loc: ok_token.loc.clone(),
                });
            };

            expected_ok_type = *ok_type.clone();
            expected_err_type = *err_type.clone();
        }

        tokens.expect(Delim, "(")?;
        let mut ok_value = LoInstr::NoInstr;
        if expected_ok_type != LoType::Void {
            ok_value = parse_expr(ctx, tokens, 0)?;
        };
        tokens.expect(Delim, ")")?;

        let ok_value_type = ok_value.get_type(ctx.module);
        if ok_value_type != expected_ok_type {
            return Err(LoError {
                message: format!(
                    "Invalid Ok type: {}, expected: {}",
                    ok_value_type, expected_ok_type
                ),
                loc: ok_token.loc.clone(),
            });
        };

        return Ok(LoInstr::MultiValueEmit {
            values: vec![ok_value, LoInstr::I32Const { value: 0 }],
        }
        .casted(LoType::Result {
            ok_type: Box::new(expected_ok_type),
            err_type: Box::new(expected_err_type),
        }));
    }

    if let Some(err_token) = tokens.eat(Symbol, "Err")?.cloned() {
        let expected_ok_type: LoType;
        let expected_err_type: LoType;
        if let Some(_) = tokens.eat(Operator, "::")? {
            tokens.expect(Delim, "<")?;
            expected_ok_type = parse_lo_type(ctx, tokens)?;
            tokens.expect(Delim, ",")?;
            expected_err_type = parse_lo_type(ctx, tokens)?;
            tokens.expect(Delim, ">")?;
        } else {
            let LoType::Result { ok_type, err_type } = &ctx.fn_ctx.lo_fn_type.output else {
                return Err(LoError {
                    message: format!("Cannot infer Result type from function's return type",),
                    loc: err_token.loc.clone(),
                });
            };

            expected_ok_type = *ok_type.clone();
            expected_err_type = *err_type.clone();
        }

        tokens.expect(Delim, "(")?;
        let err_value = parse_expr(ctx, tokens, 0)?;
        tokens.expect(Delim, ")")?;

        let err_value_type = err_value.get_type(ctx.module);
        if err_value_type != expected_err_type {
            return Err(LoError {
                message: format!(
                    "Invalid Err type: {}, expected: {}",
                    err_value_type, expected_err_type
                ),
                loc: err_token.loc.clone(),
            });
        };

        return Ok(LoInstr::MultiValueEmit {
            values: vec![expected_ok_type.get_default_value(ctx.module), err_value],
        }
        .casted(LoType::Result {
            ok_type: Box::new(expected_ok_type),
            err_type: Box::new(expected_err_type),
        }));
    }

    if let Some(token) = tokens.peek().cloned() {
        if let Some(op) = PrefixOp::parse(token) {
            let min_bp = op.info.get_min_bp_for_next();
            tokens.next(); // skip operator

            match op.tag {
                PrefixOpTag::Not => {
                    return Ok(LoInstr::BinaryOp {
                        kind: WasmBinaryOpKind::I32_EQ,
                        lhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
                        rhs: Box::new(LoInstr::U32Const { value: 0 }),
                    });
                }
                PrefixOpTag::Positive => {
                    let value = parse_expr(ctx, tokens, min_bp)?;
                    return cast_to_signed(value, &op.token.loc);
                }
                PrefixOpTag::Negative => {
                    let value = parse_expr(ctx, tokens, min_bp)?;
                    return negate(value, &op.token.loc);
                }
                PrefixOpTag::Dereference => {
                    let pointer = Box::new(parse_expr(ctx, tokens, min_bp)?);
                    let pointer_type = pointer.get_type(ctx.module);

                    let LoType::Pointer(pointee_type) = pointer_type else {
                        return Err(LoError {
                            message: format!("Cannot dereference {pointer_type:?}"),
                            loc: op.token.loc,
                        });
                    };

                    return compile_load(ctx, &pointee_type, &pointer, 0).map_err(|err| LoError {
                        message: err,
                        loc: op.token.loc,
                    });
                }
            }
        }
    }

    if let Some(_) = tokens.eat(Operator, ".")? {
        let struct_name = parse_nested_symbol(tokens)?;
        let Some(struct_def) = ctx.module.get_struct_def(&struct_name.value) else {
            return Err(LoError {
                message: format!("Can not create unknown struct: {}", struct_name.value),
                loc: struct_name.loc,
            });
        };

        return parse_struct_literal(ctx, tokens, struct_name, struct_def);
    }

    let value = parse_nested_symbol(tokens)?;

    // must go first, macro values shadow locals
    if let Some(macro_value) = ctx.block.get_macro_arg(&value.value) {
        return Ok(macro_value.clone());
    }

    if let Some(_) = tokens.eat(Operator, "!")? {
        return parse_macro_call(ctx, tokens, &value, None);
    }

    if let Some(local) = ctx.block.get_local(&value.value) {
        if ctx.module.mode == CompilerMode::Inspect {
            let source_index = ctx.module.get_loc_module_index(&value.loc);
            let source_range = RangeDisplay(&value.loc);
            let target_index = ctx.module.get_loc_module_index(&local.loc);
            let target_range = RangeDisplay(&local.loc);

            let local_name = &value.value;
            let value_type = &local.value_type;

            stdout_writeln(format!(
                "{{ \"type\": \"info\", \
                    \"link\": \"{target_index}/{target_range}\", \
                    \"hover\": \"let {local_name}: {value_type}\", \
                    \"loc\": \"{source_index}/{source_range}\" }}, ",
            ));
        }

        return compile_local_get(&ctx.module, local.index, &local.value_type).map_err(|message| {
            LoError {
                message,
                loc: value.loc,
            }
        });
    };

    if let Some(const_def) = ctx.module.constants.borrow().get(&value.value) {
        if ctx.module.mode == CompilerMode::Inspect {
            let source_index = ctx.module.get_loc_module_index(&value.loc);
            let source_range = RangeDisplay(&value.loc);
            let target_index = ctx.module.get_loc_module_index(&const_def.loc);
            let target_range = RangeDisplay(&const_def.loc);

            let const_name = &value.value;
            let const_type = const_def.value.get_type(ctx.module);

            stdout_writeln(format!(
                "{{ \"type\": \"info\", \
                    \"link\": \"{target_index}/{target_range}\", \
                    \"hover\": \"const {const_name}: {const_type}\", \
                    \"loc\": \"{source_index}/{source_range}\" }}, ",
            ));
        }

        return Ok(const_def.value.clone());
    }

    if let Some(global) = ctx.module.globals.get(&value.value) {
        if ctx.module.mode == CompilerMode::Inspect {
            let source_index = ctx.module.get_loc_module_index(&value.loc);
            let source_range = RangeDisplay(&value.loc);
            let target_index = ctx.module.get_loc_module_index(&global.loc);
            let target_range = RangeDisplay(&global.loc);

            let global_name = &value.value;
            let global_type = &global.value_type;

            stdout_writeln(format!(
                "{{ \"type\": \"info\", \
                    \"link\": \"{target_index}/{target_range}\", \
                    \"hover\": \"let {global_name}: {global_type}\", \
                    \"loc\": \"{source_index}/{source_range}\" }}, ",
            ));
        }

        return Ok(LoInstr::GlobalGet {
            global_index: global.index,
        });
    };

    if let Some(fn_def) = ctx.module.fn_defs.get(&value.value) {
        let mut args = vec![];
        parse_fn_call_args(ctx, tokens, &mut args)?;
        typecheck_fn_call_args(
            ctx.module,
            &fn_def.type_.inputs,
            &args,
            &value.value,
            &value.loc,
        )?;

        if ctx.module.mode == CompilerMode::Inspect {
            let source_index = ctx.module.get_loc_module_index(&value.loc);
            let source_range = RangeDisplay(&value.loc);
            let target_index = ctx.module.get_loc_module_index(&fn_def.loc);
            let target_range = RangeDisplay(&fn_def.loc);

            let fn_name = &value.value;
            let params = ListDisplay(&fn_def.fn_params);
            let return_type = &fn_def.type_.output;

            stdout_writeln(format!(
                "{{ \"type\": \"info\", \
                    \"link\": \"{target_index}/{target_range}\", \
                    \"hover\": \"fn {fn_name}({params}): {return_type}\", \
                    \"loc\": \"{source_index}/{source_range}\" }}, ",
            ));
        }

        return Ok(LoInstr::Call {
            fn_index: fn_def.get_absolute_index(ctx.module),
            return_type: fn_def.type_.output.clone(),
            args,
        });
    }

    return Err(LoError {
        message: format!("Reading unknown variable: {}", value.value),
        loc: value.loc,
    });
}

fn parse_struct_literal(
    ctx: &mut BlockContext,
    tokens: &mut LoTokenStream,
    struct_name: LoToken,
    struct_def: &StructDef,
) -> Result<LoInstr, LoError> {
    let mut values = vec![];
    tokens.expect(Delim, "{")?;
    while let None = tokens.eat(Delim, "}")? {
        let field_name = tokens.expect_any(Symbol)?.clone();
        tokens.expect(Operator, ":")?;
        let field_value_loc = tokens.loc().clone();
        let field_value = parse_expr(ctx, tokens, 0)?;

        if !tokens.next_is(Delim, "}")? {
            tokens.expect(Delim, ",")?;
        }

        let field_index = values.len();
        let Some(struct_field) = struct_def.fields.get(field_index) else {
            return Err(LoError {
                message: format!("Excess field values"),
                loc: field_name.loc,
            });
        };

        if &field_name.value != &struct_field.name {
            return Err(LoError {
                message: format!("Unexpected field name, expecting: `{}`", struct_field.name),
                loc: field_name.loc,
            });
        }

        let field_value_type = field_value.get_type(ctx.module);
        if field_value_type != struct_field.value_type {
            return Err(LoError {
                message: format!(
                    "Invalid type for field {}.{}, expected: {}, got: {}",
                    struct_name.value, field_name.value, struct_field.value_type, field_value_type
                ),
                loc: field_value_loc,
            });
        }
        values.push(field_value);
    }

    if values.len() < struct_def.fields.len() {
        let missing_fields = struct_def
            .fields
            .iter()
            .skip(values.len())
            .map(|f| &f.name)
            .collect::<Vec<_>>();
        let missing_fields = ListDisplay(&missing_fields);

        return Err(LoError {
            message: format!("Missing struct fields: {missing_fields}"),
            loc: struct_name.loc,
        });
    }

    return Ok(
        LoInstr::MultiValueEmit { values }.casted(LoType::StructInstance {
            name: struct_name.value,
        }),
    );
}

fn cast_to_signed(value: LoInstr, loc: &LoLocation) -> Result<LoInstr, LoError> {
    match value {
        LoInstr::U32Const { value } => {
            return Ok(LoInstr::I32Const {
                value: value as i32,
            })
        }
        LoInstr::U64Const { value } => {
            return Ok(LoInstr::I64Const {
                value: value as i64,
            })
        }
        _ => {
            return Err(LoError {
                message: format!("Cannot cast this expression to signed integer"),
                loc: loc.clone(),
            });
        }
    }
}

fn negate(value: LoInstr, loc: &LoLocation) -> Result<LoInstr, LoError> {
    match value {
        LoInstr::U32Const { value } => {
            return Ok(LoInstr::I32Const {
                value: -(value as i32),
            })
        }
        LoInstr::U64Const { value } => {
            return Ok(LoInstr::I64Const {
                value: -(value as i64),
            })
        }
        _ => {
            return Err(LoError {
                message: format!("Cannot negate this expression"),
                loc: loc.clone(),
            });
        }
    }
}

fn define_local(
    ctx: &mut BlockContext,
    local_name: &LoToken,
    value: LoInstr,
    value_type: LoType,
) -> Result<LoInstr, LoError> {
    if ctx.block.get_own_local(&local_name.value).is_some() {
        return Err(LoError {
            message: format!("Duplicate local definition: {}", local_name.value),
            loc: local_name.loc.clone(),
        });
    }

    if ctx.module.mode == CompilerMode::Inspect {
        let source_index = ctx.module.get_loc_module_index(&local_name.loc);
        let source_range = RangeDisplay(&local_name.loc);

        let local_name = &local_name.value;

        stdout_writeln(format!(
            "{{ \"type\": \"info\", \
                \"hover\": \"let {local_name}: {value_type}\", \
                \"loc\": \"{source_index}/{source_range}\" }}, ",
        ));
    }

    let local_index = ctx.fn_ctx.locals_last_index;
    let comp_count = value_type.emit_components(&ctx.module, &mut ctx.fn_ctx.non_arg_wasm_locals);
    ctx.fn_ctx.locals_last_index += comp_count;

    ctx.block.locals.insert(
        local_name.value.clone(),
        LocalDef {
            index: local_index,
            value_type,
            loc: local_name.loc.clone(),
        },
    );

    let local_indicies = local_index..local_index + comp_count;
    let values = local_indicies
        .map(|i| LoInstr::UntypedLocalGet { local_index: i })
        .collect();
    let bind_instr = LoInstr::MultiValueEmit { values };
    return compile_set(ctx, value, bind_instr, &local_name.loc);
}

fn parse_macro_call(
    ctx: &mut BlockContext,
    tokens: &mut LoTokenStream,
    macro_token: &LoToken,
    receiver: Option<LoInstr>,
) -> Result<LoInstr, LoError> {
    let macro_name = if let Some(receiver) = &receiver {
        let receiver_type = receiver.get_type(ctx.module);
        get_fn_name_from_method(&receiver_type, &macro_token.value)
    } else {
        macro_token.value.clone()
    };

    let Some(macro_def) = ctx.module.macros.get(&macro_name) else {
        return Err(LoError {
            message: format!("Unknown macro: {}", macro_name),
            loc: macro_token.loc.clone(),
        });
    };

    let type_scope = {
        let mut type_args = Vec::new();

        if let Some(_) = tokens.eat(Operator, "<")? {
            while let None = tokens.eat(Operator, ">")? {
                let macro_arg = parse_lo_type(ctx, tokens)?;
                type_args.push(macro_arg);
                if !tokens.next_is(Operator, ">")? {
                    tokens.expect(Delim, ",")?;
                }
            }
        }

        if type_args.len() != macro_def.type_params.len() {
            return Err(LoError {
                message: format!(
                    "Invalid number of type params, expected {}, got {}",
                    macro_def.type_params.len(),
                    type_args.len()
                ),
                loc: macro_token.loc.clone(),
            });
        }

        let mut type_scope = LoTypeScope::default();
        for (name, value) in macro_def.type_params.iter().zip(type_args) {
            type_scope.insert(name.clone(), value.clone());
        }

        type_scope
    };
    let return_type = macro_def.return_type.resolve_macro_type_args(&type_scope)?;

    let macro_args = {
        let mut args = vec![];
        if let Some(receiver) = receiver {
            args.push(receiver);
        }
        parse_fn_call_args(ctx, tokens, &mut args)?;

        let mut params = Vec::new();
        for param in &macro_def.params {
            params.push(param.type_.resolve_macro_type_args(&type_scope)?);
        }
        typecheck_fn_call_args(ctx.module, &params, &args, &macro_name, &macro_token.loc)?;

        let mut macro_args = BTreeMap::new();
        for (param, value) in macro_def.params.iter().zip(args) {
            macro_args.insert(param.name.clone(), value.clone());
        }

        macro_args
    };

    let macro_ctx = &mut BlockContext {
        module: ctx.module,
        fn_ctx: ctx.fn_ctx,
        block: Block {
            parent: Some(&ctx.block),
            type_scope: Some(type_scope.with_parent(ctx.module, &ctx.block)),
            macro_args: Some(macro_args),
            ..Default::default()
        },
    };

    let exprs =
        parse_block_contents(macro_ctx, &mut macro_def.body.clone(), return_type.clone())?.exprs;

    if ctx.module.mode == CompilerMode::Inspect {
        let source_index = ctx.module.get_loc_module_index(&macro_token.loc);
        let source_range = RangeDisplay(&macro_token.loc);
        let target_index = ctx.module.get_loc_module_index(&macro_def.loc);
        let target_range = RangeDisplay(&macro_def.loc);

        let params = ListDisplay(&macro_def.params);
        let type_params = ListDisplay(&macro_def.type_params);
        let return_type = &macro_def.return_type;

        stdout_writeln(format!(
            "{{ \"type\": \"info\", \
                \"link\": \"{target_index}/{target_range}\", \
                \"hover\": \"fn {macro_name}!<{type_params}>({params}): {return_type}\", \
                \"loc\": \"{source_index}/{source_range}\" }}, ",
        ));
    }

    return Ok(LoInstr::MultiValueEmit { values: exprs }.casted(return_type));
}

struct BlockContents {
    exprs: Vec<LoInstr>,
    has_never: bool,
    has_return: bool,
}

fn parse_block_contents(
    ctx: &mut BlockContext,
    tokens: &mut LoTokenStream,
    expected_type: LoType,
) -> Result<BlockContents, LoError> {
    let mut resolved_type = LoType::Void;
    let mut contents = BlockContents {
        exprs: vec![],
        has_never: false,
        has_return: false,
    };

    while tokens.peek().is_some() {
        let expr_loc = tokens.peek().unwrap().loc.clone();
        let expr = parse_expr(ctx, tokens, 0)?;
        tokens.expect(Delim, ";")?;

        let expr_type = expr.get_type(ctx.module);
        if expr_type == LoType::Never {
            contents.has_never = true;
            if let LoInstr::Return { .. } = &expr {
                contents.has_return = true;
            }
        } else if expr_type != LoType::Void {
            if expr_type != expected_type {
                return Err(LoError {
                    message: format!("Expression resolved to `{expr_type}`, but block expected `{expected_type}`"),
                    loc: expr_loc,
                });
            }

            if resolved_type != LoType::Void {
                return Err(LoError {
                    message: format!(
                        "Multiple non-void expressions in the block are not supported"
                    ),
                    loc: expr_loc,
                });
            }

            resolved_type = expr_type;
        }

        contents.exprs.push(expr);
    }

    if let Some(t) = tokens.peek() {
        return Err(LoError {
            message: format!("Unexpected token at the end of block: {t:?}"),
            loc: t.loc.clone(),
        });
    }

    if !contents.has_never && resolved_type != expected_type {
        return Err(LoError {
            message: format!("Block resolved to {resolved_type} but {expected_type} was expected"),
            loc: tokens.terminal_token.loc.clone(),
        });
    }

    // This hints the wasm compilers that the block won't terminate
    if !contents.has_return && contents.has_never {
        contents.exprs.push(LoInstr::Unreachable);
    }

    Ok(contents)
}

fn parse_postfix(
    ctx: &mut BlockContext,
    tokens: &mut LoTokenStream,
    primary: LoInstr,
    mut op: InfixOp,
) -> Result<LoInstr, LoError> {
    let min_bp = op.info.get_min_bp_for_next();

    Ok(match op.tag {
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
        | InfixOpTag::BitAnd
        | InfixOpTag::BitOr
        | InfixOpTag::ShiftLeft
        | InfixOpTag::ShiftRight => {
            let lhs = primary;
            let rhs = parse_expr(ctx, tokens, min_bp)?;
            LoInstr::BinaryOp {
                kind: get_binary_op(ctx.module, &op, &lhs, &rhs)?,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }
        }
        InfixOpTag::And | InfixOpTag::Or => {
            let lhs = primary;
            let rhs = parse_expr(ctx, tokens, min_bp)?;
            LoInstr::BinaryOp {
                kind: get_binary_op(ctx.module, &op, &lhs, &rhs)?,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }
            .casted(LoType::Bool)
        }
        InfixOpTag::AddAssign
        | InfixOpTag::SubAssign
        | InfixOpTag::MulAssign
        | InfixOpTag::DivAssign
        | InfixOpTag::ModAssign
        | InfixOpTag::BitAndAssign
        | InfixOpTag::BitOrAssign
        | InfixOpTag::ShiftLeftAssign
        | InfixOpTag::ShiftRightAssign => {
            op.tag = get_op_additional_to_assign(&op.tag)?;

            let lhs = primary;
            let rhs = parse_expr(ctx, tokens, min_bp)?;

            let value = LoInstr::BinaryOp {
                kind: get_binary_op(ctx.module, &op, &lhs, &rhs)?,
                lhs: Box::new(lhs.clone()),
                rhs: Box::new(rhs),
            };

            compile_set(ctx, value, lhs, &op.token.loc)?
        }
        InfixOpTag::Assign => {
            let value = parse_expr(ctx, tokens, min_bp)?;
            let value_type = value.get_type(ctx.module);
            let bind_type = primary.get_type(ctx.module);

            if value_type != bind_type {
                return Err(LoError {
                    message: format!(
                        "Invalid types for '{}', needed {bind_type}, got {value_type}",
                        op.token.value
                    ),
                    loc: op.token.loc.clone(),
                });
            }
            compile_set(ctx, value, primary, &op.token.loc)?
        }
        InfixOpTag::Cast => {
            let cast_type = parse_lo_type(ctx, tokens)?;

            build_cast(ctx.module, primary, cast_type, &op.token.loc)?
        }
        InfixOpTag::FieldAccess => {
            let field_or_method_name = tokens.expect_any(Symbol)?.clone();
            if let Some(_) = tokens.eat(Operator, "!")? {
                return parse_macro_call(ctx, tokens, &field_or_method_name, Some(primary));
            }

            if tokens.next_is(Delim, "(").unwrap_or(false) {
                let method_name = field_or_method_name;
                let receiver_type = primary.get_type(ctx.module);

                let fn_name = get_fn_name_from_method(&receiver_type, &method_name.value);
                let Some(fn_def) = ctx.module.fn_defs.get(&fn_name) else {
                    return Err(LoError {
                        message: format!("Unknown function: {fn_name}"),
                        loc: method_name.loc,
                    });
                };

                let mut args = vec![primary];
                parse_fn_call_args(ctx, tokens, &mut args)?;
                typecheck_fn_call_args(
                    ctx.module,
                    &fn_def.type_.inputs,
                    &args,
                    &fn_name,
                    &method_name.loc,
                )?;

                if ctx.module.mode == CompilerMode::Inspect {
                    let source_index = ctx.module.get_loc_module_index(&method_name.loc);
                    let source_range = RangeDisplay(&method_name.loc);
                    let target_index = ctx.module.get_loc_module_index(&fn_def.loc);
                    let target_range = RangeDisplay(&fn_def.loc);

                    let params = ListDisplay(&fn_def.fn_params);
                    let return_type = &fn_def.type_.output;

                    stdout_writeln(format!(
                        "{{ \"type\": \"info\", \
                            \"link\": \"{target_index}/{target_range}\", \
                            \"hover\": \"fn {fn_name}({params}): {return_type}\", \
                            \"loc\": \"{source_index}/{source_range}\" }}, ",
                    ));
                }

                return Ok(LoInstr::Call {
                    fn_index: fn_def.get_absolute_index(ctx.module),
                    return_type: fn_def.type_.output.clone(),
                    args,
                });
            }

            let field_name = field_or_method_name;

            if let LoInstr::StructGet {
                struct_name,
                base_index,
                ..
            } = &primary
            {
                let struct_def = ctx.module.get_struct_def(struct_name).unwrap(); // safe
                let Some(field) = struct_def
                    .fields
                    .iter()
                    .find(|f| &f.name == &field_name.value)
                else {
                    return Err(LoError {
                        message: format!(
                            "Unknown field {} in struct {struct_name}",
                            field_name.value
                        ),
                        loc: field_name.loc,
                    });
                };

                if ctx.module.mode == CompilerMode::Inspect {
                    let source_index = ctx.module.get_loc_module_index(&field_name.loc);
                    let source_range = RangeDisplay(&field_name.loc);
                    let target_index = ctx.module.get_loc_module_index(&field.loc);
                    let target_range = RangeDisplay(&field.loc);

                    let field_name = &field_name.value;
                    let field_type = &field.value_type;

                    stdout_writeln(format!(
                        "{{ \"type\": \"info\", \
                            \"link\": \"{target_index}/{target_range}\", \
                            \"hover\": \"{struct_name}\\n{field_name}: {field_type}\", \
                            \"loc\": \"{source_index}/{source_range}\" }}, ",
                    ));
                }

                return compile_local_get(
                    &ctx.module,
                    base_index + field.field_index,
                    &field.value_type,
                )
                .map_err(|message| LoError {
                    message,
                    loc: op.token.loc,
                });
            };

            if let LoInstr::StructLoad {
                struct_name,
                address_instr,
                base_byte_offset,
                ..
            } = &primary
            {
                // safe to unwrap as it was already checked in `StructLoad`
                let struct_def = ctx.module.get_struct_def(struct_name).unwrap();

                let Some(field) = struct_def
                    .fields
                    .iter()
                    .find(|f| f.name == *field_name.value)
                else {
                    return Err(LoError {
                        message: format!(
                            "Unknown field {} in struct {struct_name}",
                            field_name.value
                        ),
                        loc: field_name.loc,
                    });
                };

                if ctx.module.mode == CompilerMode::Inspect {
                    let source_index = ctx.module.get_loc_module_index(&field_name.loc);
                    let source_range = RangeDisplay(&field_name.loc);
                    let target_index = ctx.module.get_loc_module_index(&field.loc);
                    let target_range = RangeDisplay(&field.loc);

                    let field_name = &field_name.value;
                    let field_type = &field.value_type;

                    stdout_writeln(format!(
                        "{{ \"type\": \"info\", \
                            \"link\": \"{target_index}/{target_range}\", \
                            \"hover\": \"{struct_name}\\n{field_name}: {field_type}\", \
                            \"loc\": \"{source_index}/{source_range}\" }}, ",
                    ));
                }

                return compile_load(
                    ctx,
                    &field.value_type,
                    address_instr,
                    base_byte_offset + field.byte_offset,
                )
                .map_err(|e| LoError {
                    message: e,
                    loc: op.token.loc,
                });
            }

            let primary_type = primary.get_type(ctx.module);
            if let LoType::Pointer(pointee_type) = &primary_type {
                if let LoType::StructInstance { name: struct_name } = pointee_type.as_ref() {
                    let struct_def = ctx.module.get_struct_def(struct_name).unwrap();
                    let Some(field) = struct_def
                        .fields
                        .iter()
                        .find(|f| f.name == *field_name.value)
                    else {
                        return Err(LoError {
                            message: format!(
                                "Unknown field {} in struct {struct_name}",
                                field_name.value
                            ),
                            loc: field_name.loc.clone(),
                        });
                    };

                    if ctx.module.mode == CompilerMode::Inspect {
                        let source_index = ctx.module.get_loc_module_index(&field_name.loc);
                        let source_range = RangeDisplay(&field_name.loc);
                        let target_index = ctx.module.get_loc_module_index(&field.loc);
                        let target_range = RangeDisplay(&field.loc);

                        let field_name = &field_name.value;
                        let field_type = &field.value_type;

                        stdout_writeln(format!(
                            "{{ \"type\": \"info\", \
                                \"link\": \"{target_index}/{target_range}\", \
                                \"hover\": \"{struct_name}\\n{field_name}: {field_type}\", \
                                \"loc\": \"{source_index}/{source_range}\" }}, ",
                        ));
                    }

                    return compile_load(ctx, &field.value_type, &primary, field.byte_offset)
                        .map_err(|e| LoError {
                            message: e,
                            loc: op.token.loc.clone(),
                        });
                };
            };

            return Err(LoError {
                message: format!(
                    "Trying to get field '{}' on non struct: {primary_type}",
                    field_name.value
                ),
                loc: field_name.loc,
            });
        }
        InfixOpTag::Catch => parse_catch(ctx, tokens, primary, op, false)?,
        InfixOpTag::ErrorPropagation => parse_catch(ctx, tokens, primary, op, true)?,
    })
}

fn parse_catch(
    ctx: &mut BlockContext,
    tokens: &mut LoTokenStream,
    primary: LoInstr,
    op: InfixOp,
    rethrow: bool,
) -> Result<LoInstr, LoError> {
    let primary_type = primary.get_type(ctx.module);
    let LoType::Result {
        ok_type: caught_ok_type,
        err_type,
        ..
    } = primary_type
    else {
        return Err(LoError {
            message: format!(
                "Trying to catch an error from the expression of type: {primary_type}",
            ),
            loc: op.token.loc,
        });
    };

    let catch_ctx = &mut BlockContext {
        module: ctx.module,
        fn_ctx: ctx.fn_ctx,
        block: Block::child_of(ctx.module, &ctx.block),
    };

    let error_bind = if !rethrow {
        let mut error_bind = tokens.expect_any(Symbol)?.clone();
        if error_bind.value == "_" {
            error_bind.value = "<ignored error>".into(); // make sure it's not accesible
        }
        error_bind
    } else {
        op.token // `?` becomes the error bind, will also be hoverable
    };

    let bind_err_instr = define_local(
        catch_ctx,
        &error_bind,
        LoInstr::NoInstr, // pop error value from the stack
        *err_type.clone(),
    )?;

    let catch_body = if !rethrow {
        let mut catch_block = collect_block_tokens(tokens)?;
        parse_block_contents(catch_ctx, &mut catch_block, *caught_ok_type.clone())?.exprs
    } else {
        assert_fn_can_throw(catch_ctx.fn_ctx, &err_type, &error_bind.loc)?;

        let LoType::Result {
            ok_type: fn_ok_type,
            ..
        } = &catch_ctx.fn_ctx.lo_fn_type.output
        else {
            unreachable!()
        };

        let mut return_value = LoInstr::MultiValueEmit {
            values: vec![
                fn_ok_type.get_default_value(ctx.module),
                LoInstr::LocalGet {
                    local_index: catch_ctx
                        .block
                        .get_own_local(&error_bind.value)
                        .unwrap()
                        .index,
                    value_type: *err_type.clone(),
                },
            ],
        };

        // TODO: this is duplicated like 3 times, should wrap in a function
        if let Some(mut values) = get_deferred(catch_ctx) {
            values.insert(0, return_value);
            return_value = LoInstr::MultiValueEmit { values }.casted(LoType::Void);
        }

        vec![LoInstr::Return {
            value: Box::new(return_value),
        }]
    };

    let error_value = compile_local_get(
        ctx.module,
        catch_ctx
            .block
            .get_own_local(&error_bind.value)
            .unwrap() // safe
            .index,
        &err_type,
    )
    .unwrap(); // safe

    let mut bind_ok_instr = LoInstr::NoInstr;
    let mut ok_value = LoInstr::NoInstr;

    if *caught_ok_type != LoType::Void {
        let tmp_ok_local_name = "<ok>";
        bind_ok_instr = define_local(
            catch_ctx,
            &LoToken {
                value: tmp_ok_local_name.into(),
                ..error_bind
            },
            LoInstr::NoInstr, // pop ok value from the stack
            *caught_ok_type.clone(),
        )?;
        ok_value = compile_local_get(
            ctx.module,
            catch_ctx
                .block
                .get_own_local(tmp_ok_local_name)
                .unwrap() // safe
                .index,
            &caught_ok_type,
        )
        .unwrap(); // safe
    };

    Ok(LoInstr::MultiValueEmit {
        values: vec![
            primary,
            bind_err_instr,
            bind_ok_instr,
            LoInstr::If {
                block_type: LoBlockType::in_out(ctx.module, &[], &caught_ok_type),
                cond: Box::new(error_value), // error_value == 0 means no error
                then_branch: catch_body,
                else_branch: Some(vec![ok_value]),
            },
        ],
    }
    .casted(*caught_ok_type.clone()))
}

fn assert_fn_can_throw(
    ctx: &FnContext,
    error_type: &LoType,
    throw_loc: &LoLocation,
) -> Result<(), LoError> {
    let LoType::Result { err_type, .. } = &ctx.lo_fn_type.output else {
        return Err(LoError {
            message: format!(
                "Cannot throw {error_type}, function can only return {output}",
                output = ctx.lo_fn_type.output,
            ),
            loc: throw_loc.clone(),
        });
    };
    if *error_type != **err_type {
        return Err(LoError {
            message: format!("Invalid throw type, expected {err_type}, got {error_type}",),
            loc: throw_loc.clone(),
        });
    }

    Ok(())
}

fn get_op_additional_to_assign(op: &InfixOpTag) -> Result<InfixOpTag, LoError> {
    match op {
        InfixOpTag::AddAssign => Ok(InfixOpTag::Add),
        InfixOpTag::SubAssign => Ok(InfixOpTag::Sub),
        InfixOpTag::MulAssign => Ok(InfixOpTag::Mul),
        InfixOpTag::DivAssign => Ok(InfixOpTag::Div),
        InfixOpTag::ModAssign => Ok(InfixOpTag::Mod),
        InfixOpTag::BitAndAssign => Ok(InfixOpTag::BitAnd),
        InfixOpTag::BitOrAssign => Ok(InfixOpTag::BitOr),
        InfixOpTag::ShiftLeftAssign => Ok(InfixOpTag::ShiftLeft),
        InfixOpTag::ShiftRightAssign => Ok(InfixOpTag::ShiftRight),
        _ => unreachable!(),
    }
}

fn get_binary_op(
    ctx: &ModuleContext,
    op: &InfixOp,
    lhs: &LoInstr,
    rhs: &LoInstr,
) -> Result<WasmBinaryOpKind, LoError> {
    let lhs_type = lhs.get_type(ctx);
    let rhs_type = rhs.get_type(ctx);

    if lhs_type != rhs_type {
        return Err(LoError {
            message: format!(
                "Operands of `{}` have incompatible types: {} and {}",
                op.token.value, lhs_type, rhs_type
            ),
            loc: op.token.loc.clone(),
        });
    }

    Ok(match op.tag {
        InfixOpTag::Equal => match lhs_type {
            LoType::Bool
            | LoType::I8
            | LoType::U8
            | LoType::I16
            | LoType::U16
            | LoType::I32
            | LoType::U32 => WasmBinaryOpKind::I32_EQ,
            LoType::I64 | LoType::U64 => WasmBinaryOpKind::I64_EQ,
            LoType::F32 => WasmBinaryOpKind::F32_EQ,
            LoType::F64 => WasmBinaryOpKind::F64_EQ,
            operand_type => return err_incompatible_op(op, operand_type),
        },
        InfixOpTag::NotEqual => match lhs_type {
            LoType::Bool
            | LoType::I8
            | LoType::U8
            | LoType::I16
            | LoType::U16
            | LoType::I32
            | LoType::U32 => WasmBinaryOpKind::I32_NE,
            LoType::I64 | LoType::U64 => WasmBinaryOpKind::I64_NE,
            LoType::F32 => WasmBinaryOpKind::F32_NE,
            LoType::F64 => WasmBinaryOpKind::F64_NE,
            operand_type => return err_incompatible_op(op, operand_type),
        },
        InfixOpTag::Less => match lhs_type {
            LoType::I8 | LoType::I16 | LoType::I32 => WasmBinaryOpKind::I32_LT_S,
            LoType::Bool | LoType::U8 | LoType::U16 | LoType::U32 => WasmBinaryOpKind::I32_LT_U,
            LoType::I64 => WasmBinaryOpKind::I64_LT_S,
            LoType::U64 => WasmBinaryOpKind::I64_LT_U,
            LoType::F32 => WasmBinaryOpKind::F32_LT,
            LoType::F64 => WasmBinaryOpKind::F64_LT,
            operand_type => return err_incompatible_op(op, operand_type),
        },
        InfixOpTag::Greater => match lhs_type {
            LoType::I8 | LoType::I16 | LoType::I32 => WasmBinaryOpKind::I32_GT_S,
            LoType::Bool | LoType::U8 | LoType::U16 | LoType::U32 => WasmBinaryOpKind::I32_GT_U,
            LoType::I64 => WasmBinaryOpKind::I64_GT_S,
            LoType::U64 => WasmBinaryOpKind::I64_GT_U,
            LoType::F32 => WasmBinaryOpKind::F32_GT,
            LoType::F64 => WasmBinaryOpKind::F64_GT,
            operand_type => return err_incompatible_op(op, operand_type),
        },
        InfixOpTag::LessEqual => match lhs_type {
            LoType::I8 | LoType::I16 | LoType::I32 => WasmBinaryOpKind::I32_LE_S,
            LoType::Bool | LoType::U8 | LoType::U16 | LoType::U32 => WasmBinaryOpKind::I32_LE_U,
            LoType::I64 => WasmBinaryOpKind::I64_LE_S,
            LoType::U64 => WasmBinaryOpKind::I64_LE_U,
            LoType::F32 => WasmBinaryOpKind::F32_LE,
            LoType::F64 => WasmBinaryOpKind::F64_LE,
            operand_type => return err_incompatible_op(op, operand_type),
        },
        InfixOpTag::GreaterEqual => match lhs_type {
            LoType::I8 | LoType::I16 | LoType::I32 => WasmBinaryOpKind::I32_GE_S,
            LoType::Bool | LoType::U8 | LoType::U16 | LoType::U32 => WasmBinaryOpKind::I32_GE_U,
            LoType::I64 => WasmBinaryOpKind::I64_GE_S,
            LoType::U64 => WasmBinaryOpKind::I64_GE_U,
            LoType::F32 => WasmBinaryOpKind::F32_GE,
            LoType::F64 => WasmBinaryOpKind::F64_GE,
            operand_type => return err_incompatible_op(op, operand_type),
        },
        InfixOpTag::Add => match lhs_type {
            LoType::Bool
            | LoType::I8
            | LoType::U8
            | LoType::I16
            | LoType::U16
            | LoType::I32
            | LoType::U32 => WasmBinaryOpKind::I32_ADD,
            LoType::I64 | LoType::U64 => WasmBinaryOpKind::I64_ADD,
            LoType::F32 => WasmBinaryOpKind::F32_ADD,
            LoType::F64 => WasmBinaryOpKind::F64_ADD,
            operand_type => return err_incompatible_op(op, operand_type),
        },
        InfixOpTag::Sub => match lhs_type {
            LoType::Bool
            | LoType::I8
            | LoType::U8
            | LoType::I16
            | LoType::U16
            | LoType::I32
            | LoType::U32 => WasmBinaryOpKind::I32_SUB,
            LoType::I64 | LoType::U64 => WasmBinaryOpKind::I64_SUB,
            LoType::F32 => WasmBinaryOpKind::F32_SUB,
            LoType::F64 => WasmBinaryOpKind::F64_SUB,
            operand_type => return err_incompatible_op(op, operand_type),
        },
        InfixOpTag::Mul => match lhs_type {
            LoType::Bool
            | LoType::I8
            | LoType::U8
            | LoType::I16
            | LoType::U16
            | LoType::I32
            | LoType::U32 => WasmBinaryOpKind::I32_MUL,
            LoType::I64 | LoType::U64 => WasmBinaryOpKind::I64_MUL,
            LoType::F32 => WasmBinaryOpKind::F32_MUL,
            LoType::F64 => WasmBinaryOpKind::F64_MUL,
            operand_type => return err_incompatible_op(op, operand_type),
        },
        InfixOpTag::Div => match lhs_type {
            LoType::I8 | LoType::I16 | LoType::I32 => WasmBinaryOpKind::I32_DIV_S,
            LoType::Bool | LoType::U8 | LoType::U16 | LoType::U32 => WasmBinaryOpKind::I32_DIV_U,
            LoType::I64 => WasmBinaryOpKind::I64_DIV_S,
            LoType::U64 => WasmBinaryOpKind::I64_DIV_U,
            LoType::F32 => WasmBinaryOpKind::F32_DIV,
            LoType::F64 => WasmBinaryOpKind::F64_DIV,
            operand_type => return err_incompatible_op(op, operand_type),
        },
        InfixOpTag::Mod => match lhs_type {
            LoType::I8 | LoType::I16 | LoType::I32 => WasmBinaryOpKind::I32_REM_S,
            LoType::Bool | LoType::U8 | LoType::U16 | LoType::U32 => WasmBinaryOpKind::I32_REM_U,
            LoType::I64 => WasmBinaryOpKind::I64_REM_S,
            LoType::U64 => WasmBinaryOpKind::I64_REM_U,
            operand_type => return err_incompatible_op(op, operand_type),
        },
        InfixOpTag::ShiftLeft => match lhs_type {
            LoType::I8 | LoType::I16 | LoType::I32 => WasmBinaryOpKind::I32_SHL,
            LoType::Bool | LoType::U8 | LoType::U16 | LoType::U32 => WasmBinaryOpKind::I32_SHL,
            LoType::I64 => WasmBinaryOpKind::I64_SHL,
            LoType::U64 => WasmBinaryOpKind::I64_SHL,
            operand_type => return err_incompatible_op(op, operand_type),
        },
        InfixOpTag::ShiftRight => match lhs_type {
            LoType::I8 | LoType::I16 | LoType::I32 => WasmBinaryOpKind::I32_SHR_S,
            LoType::Bool | LoType::U8 | LoType::U16 | LoType::U32 => WasmBinaryOpKind::I32_SHR_U,
            LoType::I64 => WasmBinaryOpKind::I64_SHR_S,
            LoType::U64 => WasmBinaryOpKind::I64_SHR_U,
            operand_type => return err_incompatible_op(op, operand_type),
        },
        InfixOpTag::And => match lhs_type {
            LoType::Bool
            | LoType::I8
            | LoType::U8
            | LoType::I16
            | LoType::U16
            | LoType::I32
            | LoType::U32 => WasmBinaryOpKind::I32_AND,
            LoType::I64 | LoType::U64 => WasmBinaryOpKind::I64_AND,
            operand_type => return err_incompatible_op(op, operand_type),
        },
        InfixOpTag::Or => match lhs_type {
            LoType::Bool
            | LoType::I8
            | LoType::U8
            | LoType::I16
            | LoType::U16
            | LoType::I32
            | LoType::U32 => WasmBinaryOpKind::I32_OR,
            LoType::I64 | LoType::U64 => WasmBinaryOpKind::I64_OR,
            operand_type => return err_incompatible_op(op, operand_type),
        },
        InfixOpTag::BitAnd => match lhs_type {
            LoType::Bool
            | LoType::I8
            | LoType::U8
            | LoType::I16
            | LoType::U16
            | LoType::I32
            | LoType::U32 => WasmBinaryOpKind::I32_AND,
            LoType::I64 | LoType::U64 => WasmBinaryOpKind::I64_AND,
            operand_type => return err_incompatible_op(op, operand_type),
        },
        InfixOpTag::BitOr => match lhs_type {
            LoType::Bool
            | LoType::I8
            | LoType::U8
            | LoType::I16
            | LoType::U16
            | LoType::I32
            | LoType::U32 => WasmBinaryOpKind::I32_OR,
            LoType::I64 | LoType::U64 => WasmBinaryOpKind::I64_OR,
            operand_type => return err_incompatible_op(op, operand_type),
        },
        _ => unreachable!(),
    })
}

fn err_incompatible_op<T>(op: &InfixOp, operand_type: LoType) -> Result<T, LoError> {
    Err(LoError {
        message: format!(
            "Operator `{}` is incompatible with operands of type {}",
            op.token.value, operand_type
        ),
        loc: op.token.loc.clone(),
    })
}

// TODO: support all numeric types
fn build_cast(
    ctx: &ModuleContext,
    value: LoInstr,
    wanted_type: LoType,
    loc: &LoLocation,
) -> Result<LoInstr, LoError> {
    let actual_type = value.get_type(ctx);

    if wanted_type == LoType::I64 {
        if actual_type == LoType::I32 {
            return Ok(LoInstr::I64FromI32Signed {
                expr: Box::new(value),
            });
        }

        if actual_type == LoType::U32 {
            return Ok(LoInstr::I64FromI32Unsigned {
                expr: Box::new(value),
            });
        }
    }

    if wanted_type == LoType::U64 {
        if actual_type == LoType::I32 {
            return Ok(LoInstr::I64FromI32Signed {
                expr: Box::new(value),
            }
            .casted(wanted_type));
        }

        if actual_type == LoType::U8 || actual_type == LoType::U32 {
            return Ok(LoInstr::I64FromI32Unsigned {
                expr: Box::new(value),
            }
            .casted(wanted_type));
        }
    }

    if wanted_type == LoType::I32 {
        if actual_type == LoType::I64 || actual_type == LoType::U64 {
            return Ok(LoInstr::I32FromI64 {
                expr: Box::new(value),
            });
        }
    }

    if wanted_type == LoType::U32 {
        if actual_type == LoType::I64 || actual_type == LoType::U64 {
            return Ok(LoInstr::I32FromI64 {
                expr: Box::new(value),
            }
            .casted(wanted_type));
        }
    }

    let mut actual_wasm_types = vec![];
    actual_type.emit_components(ctx, &mut actual_wasm_types);

    let mut wanted_wasm_types = vec![];
    wanted_type.emit_components(ctx, &mut wanted_wasm_types);

    if actual_wasm_types != wanted_wasm_types {
        return Err(LoError {
            message: format!("`{}` cannot be casted to `{}`", actual_type, wanted_type),
            loc: loc.clone(),
        });
    }

    Ok(value.casted(wanted_type))
}

fn parse_fn_call_args(
    ctx: &mut BlockContext,
    tokens: &mut LoTokenStream,
    args: &mut Vec<LoInstr>,
) -> Result<(), LoError> {
    tokens.expect(Delim, "(")?;
    while let None = tokens.eat(Delim, ")")? {
        args.push(parse_expr(ctx, tokens, 0)?);

        if !tokens.next_is(Delim, ")")? {
            tokens.expect(Delim, ",")?;
        }
    }

    Ok(())
}

fn typecheck_fn_call_args(
    ctx: &ModuleContext,
    params: &Vec<LoType>,
    args: &Vec<LoInstr>,
    fn_name: &str,
    fn_call_loc: &LoLocation,
) -> Result<(), LoError> {
    let mut arg_types = vec![];
    for arg in args {
        arg_types.push(arg.get_type(ctx));
    }

    if arg_types != *params {
        return Err(LoError {
            message: format!(
                "Invalid arguments for `{}` call: [{}], expected: [{}]",
                fn_name,
                ListDisplay(&arg_types),
                ListDisplay(params)
            ),
            loc: fn_call_loc.clone(),
        });
    }

    Ok(())
}

fn parse_const_expr(
    ctx: &mut ModuleContext,
    tokens: &mut LoTokenStream,
    min_bp: u32,
) -> Result<LoInstr, LoError> {
    let mut primary = parse_const_primary(ctx, tokens)?;

    while tokens.peek().is_some() {
        let op_symbol = tokens.peek().unwrap().clone();
        let Some(op) = InfixOp::parse(op_symbol) else {
            break;
        };

        if op.info.bp < min_bp {
            break;
        }

        tokens.next(); // skip operator
        primary = parse_const_postfix(ctx, tokens, primary, op)?;
    }

    Ok(primary)
}

fn parse_const_primary(
    ctx: &mut ModuleContext,
    tokens: &mut LoTokenStream,
) -> Result<LoInstr, LoError> {
    if tokens.next_is_any(IntLiteral)? {
        return parse_const_int(tokens);
    }

    if let Some(value) = tokens.eat_any(CharLiteral)? {
        return Ok(LoInstr::U32Const {
            value: Lexer::parse_char_literal_value(&value.value),
        }
        .casted(LoType::U8));
    }

    if let Some(value) = tokens.eat_any(StringLiteral)? {
        let value = Lexer::unescape_string(&value.value);
        return parse_const_str(ctx, tokens, value);
    }

    if let Some(_) = tokens.eat(Delim, "[")? {
        let (item_type, bytes) = parse_const_sequence(ctx, tokens)?;
        let bytes_ptr = ctx.append_data(bytes);

        return Ok(
            LoInstr::U32Const { value: bytes_ptr }.casted(LoType::Pointer(Box::new(item_type)))
        );
    }

    if let Some(_) = tokens.eat(Symbol, "true")? {
        return Ok(LoInstr::U32Const { value: 1 }.casted(LoType::Bool));
    }

    if let Some(_) = tokens.eat(Symbol, "false")? {
        return Ok(LoInstr::U32Const { value: 0 }.casted(LoType::Bool));
    }

    if let Some(token) = tokens.peek().cloned() {
        if let Some(op) = PrefixOp::parse(token) {
            let min_bp = op.info.get_min_bp_for_next();
            tokens.next(); // skip operator

            match op.tag {
                PrefixOpTag::Positive => {
                    let value = parse_const_expr(ctx, tokens, min_bp + 1)?;
                    return cast_to_signed(value, &op.token.loc);
                }
                PrefixOpTag::Negative => {
                    let value = parse_const_expr(ctx, tokens, min_bp + 1)?;
                    return negate(value, &op.token.loc);
                }
                _ => {}
            }
        }
    }

    let value = parse_nested_symbol(tokens)?;

    if let Some(const_def) = ctx.constants.borrow().get(&value.value) {
        return Ok(const_def.value.clone());
    }

    let Some(global) = ctx.globals.get(&value.value) else {
        return Err(LoError {
            message: format!("Reading unknown variable in const context: {}", value.value),
            loc: value.loc,
        });
    };

    return Ok(LoInstr::GlobalGet {
        global_index: global.index,
    });
}

fn parse_const_postfix(
    ctx: &ModuleContext,
    tokens: &mut LoTokenStream,
    primary: LoInstr,
    op: InfixOp,
) -> Result<LoInstr, LoError> {
    let _min_bp = op.info.get_min_bp_for_next();

    Ok(match op.tag {
        InfixOpTag::Cast => {
            let cast_type = parse_const_lo_type(ctx, tokens)?;

            build_cast(ctx, primary, cast_type, &op.token.loc)?
        }
        _ => {
            return Err(LoError {
                message: format!("Unsupported operator in const context: {}", op.token.value),
                loc: op.token.loc,
            });
        }
    })
}

fn parse_const_lo_type(ctx: &ModuleContext, tokens: &mut LoTokenStream) -> Result<LoType, LoError> {
    parse_lo_type_(ctx, &ctx.type_scope, tokens, false)
}

fn parse_lo_type(ctx: &BlockContext, tokens: &mut LoTokenStream) -> Result<LoType, LoError> {
    if let Some(type_scope) = &ctx.block.type_scope {
        parse_lo_type_(ctx.module, &type_scope, tokens, false)
    } else {
        parse_const_lo_type(ctx.module, tokens)
    }
}

fn parse_lo_type_(
    ctx: &ModuleContext,
    type_scope: &LoTypeScope,
    tokens: &mut LoTokenStream,
    is_referenced: bool,
) -> Result<LoType, LoError> {
    let primary = parse_lo_type_primary(ctx, type_scope, tokens, is_referenced)?;

    if let Some(_) = tokens.eat(Symbol, "of")? {
        // TODO: attach as metadata and use in type equality check
        parse_lo_type_(ctx, type_scope, tokens, is_referenced)?;

        return Ok(primary);
    }

    return Ok(primary);
}

fn parse_lo_type_primary(
    ctx: &ModuleContext,
    type_scope: &LoTypeScope,
    tokens: &mut LoTokenStream,
    is_referenced: bool,
) -> Result<LoType, LoError> {
    if let Some(_) = tokens.eat(Delim, "(")? {
        let type_ = parse_lo_type_(ctx, type_scope, tokens, false)?;
        tokens.expect(Delim, ")")?;
        return Ok(type_);
    }

    if let Some(_) = tokens.eat(Operator, "&")? {
        let pointee = parse_lo_type_primary(ctx, type_scope, tokens, true)?;
        return Ok(LoType::Pointer(Box::new(pointee)));
    }

    if let Some(_) = tokens.eat(Operator, "*&")? {
        let pointee = parse_lo_type_primary(ctx, type_scope, tokens, true)?;
        return Ok(LoType::Pointer(Box::new(pointee)));
    }

    if let Some(_) = tokens.eat(Symbol, "Result")? {
        tokens.expect(Operator, "<")?;
        let ok_type = parse_lo_type_(ctx, type_scope, tokens, false)?;
        tokens.expect(Delim, ",")?;
        let err_type = parse_lo_type_(ctx, type_scope, tokens, false)?;
        tokens.expect(Operator, ">")?;

        return Ok(LoType::Result {
            ok_type: Box::new(ok_type),
            err_type: Box::new(err_type),
        });
    }

    let token = parse_nested_symbol(tokens)?;
    get_type_by_name(ctx, type_scope, &token, is_referenced)
}

fn get_type_by_name(
    ctx: &ModuleContext,
    type_scope: &LoTypeScope,
    token: &LoToken,
    is_referenced: bool,
) -> Result<LoType, LoError> {
    match token.value.as_str() {
        "never" => Ok(LoType::Never),
        "void" => Ok(LoType::Void),
        "bool" => Ok(LoType::Bool),
        "u8" => Ok(LoType::U8),
        "i8" => Ok(LoType::I8),
        "u16" => Ok(LoType::U16),
        "i16" => Ok(LoType::I16),
        "u32" => Ok(LoType::U32),
        "i32" => Ok(LoType::I32),
        "f32" => Ok(LoType::F32),
        "u64" => Ok(LoType::U64),
        "i64" => Ok(LoType::I64),
        "f64" => Ok(LoType::F64),
        _ => {
            let Some(type_) = type_scope.get(&token.value) else {
                return Err(LoError {
                    message: format!("Unknown type: {}", token.value),
                    loc: token.loc.clone(),
                });
            };

            let mut is_type_alias = true;

            if let LoType::StructInstance { name } = type_ {
                let struct_def = ctx.get_struct_def(name).unwrap(); // safe because of if let
                if !struct_def.fully_defined && !is_referenced {
                    return Err(LoError {
                        message: format!("Cannot use partially defined struct: {name}"),
                        loc: token.loc.clone(),
                    });
                }

                if *name == token.value {
                    is_type_alias = false;

                    if ctx.mode == CompilerMode::Inspect {
                        let source_index = ctx.get_loc_module_index(&token.loc);
                        let source_range = RangeDisplay(&token.loc);
                        let target_index = ctx.get_loc_module_index(&struct_def.loc);
                        let target_range = RangeDisplay(&struct_def.loc);

                        let fields = ListDisplay(&struct_def.fields);

                        stdout_writeln(format!(
                            "{{ \"type\": \"info\", \
                                \"link\": \"{target_index}/{target_range}\", \
                                \"hover\": \"struct {name} {{ {fields} }}\", \
                                \"loc\": \"{source_index}/{source_range}\" }}, ",
                        ));
                    }
                }
            }

            if ctx.mode == CompilerMode::Inspect && is_type_alias {
                let source_index = ctx.get_loc_module_index(&token.loc);
                let source_range = RangeDisplay(&token.loc);

                let type_name = &token.value;

                // TODO: add links
                stdout_writeln(format!(
                    "{{ \"type\": \"info\", \
                        \"hover\": \"type {type_name} = {type_}\", \
                        \"loc\": \"{source_index}/{source_range}\" }}, ",
                ));
            }

            return Ok(type_.clone());
        }
    }
}

fn parse_const_str(
    ctx: &ModuleContext,
    tokens: &mut LoTokenStream,
    mut value: String,
) -> Result<LoInstr, LoError> {
    if !ctx.memory_defined && ctx.mode != CompilerMode::Inspect {
        return Err(LoError {
            message: format!("Cannot use strings with no memories defined"),
            loc: tokens.loc().clone(),
        });
    }

    let is_null_terminated = tokens.eat(IntLiteral, "0")?.is_some();
    if is_null_terminated {
        value.push('\0');
    }

    let string_len = value.as_bytes().len() as u32;

    let string_ptr = ctx.string_pool.borrow().get(&value).cloned();
    let string_ptr = match string_ptr {
        Some(string_ptr) => string_ptr,
        None => {
            let new_string_ptr = ctx.append_data(value.clone().into_bytes());
            ctx.string_pool.borrow_mut().insert(value, new_string_ptr);
            new_string_ptr
        }
    };

    if is_null_terminated {
        return Ok(
            LoInstr::U32Const { value: string_ptr }.casted(LoType::Pointer(Box::new(LoType::U8)))
        );
    }

    Ok(LoInstr::MultiValueEmit {
        values: vec![
            LoInstr::U32Const { value: string_ptr },
            LoInstr::U32Const { value: string_len },
        ],
    }
    .casted(LoType::StructInstance {
        name: format!("str"),
    }))
}

// TODO: support sequences of any type
fn parse_const_sequence(
    ctx: &ModuleContext,
    tokens: &mut LoTokenStream,
) -> Result<(LoType, Vec<u8>), LoError> {
    let item_type = parse_const_lo_type(ctx, tokens)?;
    if item_type != LoType::U8
        && item_type
            != (LoType::StructInstance {
                name: format!("str"),
            })
    {
        return Err(LoError {
            message: format!("Unsupported sequence element type: {}", item_type),
            loc: tokens.loc().clone(),
        });
    }

    tokens.expect(Delim, "]")?;

    let mut bytes = vec![];

    tokens.expect(Delim, "[")?;
    while let None = tokens.eat(Delim, "]")? {
        if item_type == LoType::U8 {
            let byte = tokens.expect_any(IntLiteral)?;
            bytes.push(parse_u8_literal(byte)?);
        } else if item_type
            == (LoType::StructInstance {
                name: format!("str"),
            })
        {
            let value = tokens.expect_any(StringLiteral)?;
            let value = Lexer::unescape_string(&value.value);
            let len = value.len();
            let ptr = ctx.append_data(value.into_bytes());

            bytes.extend_from_slice(&ptr.to_le_bytes());
            bytes.extend_from_slice(&len.to_le_bytes());
        } else {
            unreachable!()
        }

        if !tokens.next_is(Delim, "]")? {
            tokens.expect(Delim, ",")?;
        }
    }

    return Ok((item_type, bytes));
}

fn parse_nested_symbol(tokens: &mut LoTokenStream) -> Result<LoToken, LoError> {
    let mut nested_symbol = tokens.expect_any(Symbol)?.clone();
    while let Some(_) = tokens.eat(Operator, "::")? {
        let path_part = tokens.expect_any(Symbol)?;
        nested_symbol.value += "::";
        nested_symbol.value += path_part.value.as_str();
        nested_symbol.loc.end_pos = path_part.loc.end_pos.clone();
    }
    Ok(nested_symbol)
}

fn extract_method_receiver_and_name(
    ctx: &ModuleContext,
    token: &LoToken,
) -> Result<(Option<LoType>, String), LoError> {
    Ok(match token.value.rsplitn(2, "::").collect::<Vec<_>>()[..] {
        [method_name, receiver_name] => {
            let mut token = LoToken {
                type_: LoTokenType::Symbol,
                value: String::from(receiver_name),
                loc: token.loc.clone(),
            };

            // TODO: correct `end_pos` info is lost during creation of nested_symbol
            token.loc.end_pos = token.loc.pos.clone();

            (
                Some(get_type_by_name(ctx, &ctx.type_scope, &token, false)?),
                String::from(method_name),
            )
        }
        [fn_name] => (None, String::from(fn_name)),
        _ => unreachable!(),
    })
}

fn parse_const_int(tokens: &mut LoTokenStream) -> Result<LoInstr, LoError> {
    let int_literal = tokens.expect_any(IntLiteral)?.clone();

    if let Some(_) = tokens.eat(Symbol, "i64")? {
        return Ok(LoInstr::I64Const {
            value: parse_i64_literal(&int_literal)?,
        });
    }

    if let Some(_) = tokens.eat(Symbol, "u64")? {
        return Ok(LoInstr::U64Const {
            value: parse_u64_literal(&int_literal)?,
        });
    }

    return Ok(LoInstr::U32Const {
        value: parse_u32_literal(&int_literal)?,
    });
}

fn parse_u8_literal(int: &LoToken) -> Result<u8, LoError> {
    Ok(Lexer::parse_int_literal_value(&int.value) as u8)
}

fn parse_u32_literal(int: &LoToken) -> Result<u32, LoError> {
    Ok(Lexer::parse_int_literal_value(&int.value) as u32)
}

fn parse_i64_literal(int: &LoToken) -> Result<i64, LoError> {
    Ok(Lexer::parse_int_literal_value(&int.value) as i64)
}

fn parse_u64_literal(int: &LoToken) -> Result<u64, LoError> {
    Ok(Lexer::parse_int_literal_value(&int.value))
}

fn get_fn_name_from_method(receiver_type: &LoType, method_name: &str) -> String {
    let resolved_receiver_type = receiver_type.deref_rec();
    format!("{resolved_receiver_type}::{method_name}")
}

fn get_deferred(ctx: &mut BlockContext) -> Option<Vec<LoInstr>> {
    if ctx.fn_ctx.defers.len() == 0 {
        return None;
    };

    let mut deferred = ctx.fn_ctx.defers.clone();
    deferred.reverse();

    Some(deferred)
}

fn compile_load(
    ctx: &mut BlockContext,
    value_type: &LoType,
    address_instr: &LoInstr,
    base_byte_offset: u32,
) -> Result<LoInstr, String> {
    if let Ok(_) = value_type.to_load_kind() {
        return Ok(LoInstr::Load {
            kind: value_type.clone(),
            align: 0,
            offset: base_byte_offset,
            address_instr: Box::new(address_instr.clone()),
        });
    }

    if let LoType::Tuple(item_types) = value_type {
        let mut item_gets = vec![];
        let mut item_byte_offset = 0;
        for item_type in item_types {
            item_gets.push(compile_load(
                ctx,
                item_type,
                address_instr,
                base_byte_offset + item_byte_offset,
            )?);
            item_byte_offset += item_type.sized_comp_stats(&ctx.module)?.byte_length;
        }

        return Ok(LoInstr::MultiValueEmit { values: item_gets }.casted(value_type.clone()));
    }

    let LoType::StructInstance { name } = value_type else {
        return Err(format!("Unsupported type for compile_load: {value_type:?}"));
    };

    let mut components = vec![];
    let mut stats = EmitComponentStats {
        count: 0,
        byte_length: base_byte_offset,
    };

    value_type.emit_sized_component_stats(&ctx.module, &mut stats, &mut components)?;

    let address_local_index = ctx.fn_ctx.locals_last_index;
    ctx.fn_ctx.non_arg_wasm_locals.push(WasmType::I32);
    ctx.fn_ctx.locals_last_index += 1;

    let mut primitive_loads = vec![];
    for comp in components.into_iter() {
        primitive_loads.push(LoInstr::Load {
            kind: comp.value_type,
            align: 0,
            offset: comp.byte_offset,
            address_instr: Box::new(LoInstr::UntypedLocalGet {
                local_index: address_local_index,
            }),
        });
    }

    Ok(LoInstr::StructLoad {
        struct_name: name.clone(),
        address_instr: Box::new(address_instr.clone()),
        address_local_index,
        base_byte_offset,
        primitive_loads,
    })
}

fn compile_local_get(
    ctx: &ModuleContext,
    base_index: u32,
    value_type: &LoType,
) -> Result<LoInstr, String> {
    if let LoType::Tuple(item_types) = value_type {
        let mut item_gets = vec![];
        for (item_index, item_type) in (0..).zip(item_types) {
            item_gets.push(compile_local_get(ctx, base_index + item_index, item_type)?);
        }

        return Ok(LoInstr::MultiValueEmit { values: item_gets }.casted(value_type.clone()));
    }

    let comp_count = value_type.emit_components(ctx, &mut vec![]);

    let LoType::StructInstance { name } = value_type else {
        if comp_count == 1 {
            return Ok(LoInstr::LocalGet {
                local_index: base_index,
                value_type: value_type.clone(),
            });
        }

        return Err(format!("Unsupported type for compile_load: {value_type:?}"));
    };

    let mut primitive_gets = vec![];
    for field_index in 0..comp_count {
        primitive_gets.push(LoInstr::UntypedLocalGet {
            local_index: base_index + field_index as u32,
        });
    }

    Ok(LoInstr::StructGet {
        struct_name: name.clone(),
        base_index,
        primitive_gets,
    })
}

fn compile_set(
    ctx: &mut BlockContext,
    value_instr: LoInstr,
    bind_instr: LoInstr,
    loc: &LoLocation,
) -> Result<LoInstr, LoError> {
    let mut values = vec![];
    compile_set_binds(&mut values, ctx, bind_instr, None).map_err(|message| LoError {
        message,
        loc: loc.clone(),
    })?;
    values.push(value_instr);
    values.reverse();

    Ok(LoInstr::MultiValueEmit { values }.casted(LoType::Void))
}

fn compile_set_binds(
    output: &mut Vec<LoInstr>,
    ctx: &mut BlockContext,
    bind_instr: LoInstr,
    address_index: Option<u32>,
) -> Result<(), String> {
    Ok(match bind_instr {
        LoInstr::LocalGet { local_index, .. } | LoInstr::UntypedLocalGet { local_index } => {
            output.push(LoInstr::Set {
                bind: LoSetBind::Local { index: local_index },
            });
        }
        LoInstr::GlobalGet { global_index } => {
            output.push(LoInstr::Set {
                bind: LoSetBind::Global {
                    index: global_index,
                },
            });
        }
        LoInstr::Load {
            kind,
            align,
            offset,
            address_instr,
        } => {
            let value_local_index = ctx.fn_ctx.locals_last_index;
            ctx.fn_ctx
                .non_arg_wasm_locals
                .push(kind.to_wasm_type().unwrap());
            ctx.fn_ctx.locals_last_index += 1;

            let address_instr = match address_index {
                Some(local_index) => Box::new(LoInstr::UntypedLocalGet { local_index }),
                None => address_instr,
            };

            output.push(LoInstr::Set {
                bind: LoSetBind::Memory {
                    align,
                    offset,
                    kind: WasmStoreKind::from_load_kind(&kind.to_load_kind().unwrap()),
                    address_instr,
                    value_local_index,
                },
            });
        }
        LoInstr::StructLoad {
            primitive_loads,
            address_instr,
            address_local_index,
            ..
        } => {
            let mut values = vec![];

            for value in primitive_loads {
                compile_set_binds(&mut values, ctx, value, Some(address_local_index))?;
            }

            values.push(LoInstr::Set {
                bind: LoSetBind::Local {
                    index: address_local_index,
                },
            });
            values.push(*address_instr);

            values.reverse();

            output.push(LoInstr::MultiValueEmit { values });
        }
        LoInstr::StructGet { primitive_gets, .. } => {
            for value in primitive_gets {
                compile_set_binds(output, ctx, value, address_index)?;
            }
        }
        LoInstr::MultiValueEmit { values } => {
            for value in values {
                compile_set_binds(output, ctx, value, address_index)?;
            }
        }
        LoInstr::Casted { expr, .. } => {
            compile_set_binds(output, ctx, *expr, address_index)?;
        }
        _ => {
            return Err(format!("Invalid left-hand side in assignment"));
        }
    })
}

// LoTokenStream

#[derive(Clone)]
pub struct LoTokenStream {
    pub tokens: Vec<LoToken>,
    pub index: usize,
    pub terminal_token: LoToken,
}

impl LoTokenStream {
    pub fn new(tokens: Vec<LoToken>, end_location: LoLocation) -> Self {
        Self {
            tokens,
            index: 0,
            terminal_token: LoToken {
                type_: LoTokenType::Symbol,
                value: "<EOF>".into(),
                loc: end_location,
            },
        }
    }

    pub fn expect_any(&mut self, type_: LoTokenType) -> Result<&LoToken, LoError> {
        match self.peek() {
            Some(token) if token.is_any(type_) => Ok(self.next().unwrap()),
            other => {
                let unexpected = other.unwrap_or(&self.terminal_token);
                Err(LoError {
                    message: format!("Unexpected token '{}', wanted {type_:?}", unexpected.value),
                    loc: unexpected.loc.clone(),
                })
            }
        }
    }

    pub fn expect(&mut self, type_: LoTokenType, value: &str) -> Result<&LoToken, LoError> {
        match self.peek() {
            Some(token) if token.is(type_, value) => Ok(self.next().unwrap()),
            other => {
                let unexpected = other.unwrap_or(&self.terminal_token);
                Err(LoError {
                    message: format!("Unexpected token '{}', wanted '{value}'", unexpected.value),
                    loc: unexpected.loc.clone(),
                })
            }
        }
    }

    pub fn eat_any(&mut self, type_: LoTokenType) -> Result<Option<&LoToken>, LoError> {
        let was_some = self.peek().is_some();
        match self.expect_any(type_) {
            Ok(t) => Ok(Some(t)),
            Err(_) if was_some => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub fn eat(&mut self, type_: LoTokenType, value: &str) -> Result<Option<&LoToken>, LoError> {
        let was_some = self.peek().is_some();
        match self.expect(type_, value) {
            Ok(t) => Ok(Some(t)),
            Err(_) if was_some => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub fn next_is(&mut self, type_: LoTokenType, value: &str) -> Result<bool, LoError> {
        match self.peek() {
            Some(token) if token.is(type_, value) => Ok(true),
            Some(_) => Ok(false),
            _ => self.err_eof(format!("Unexpected EOF")),
        }
    }

    pub fn next_is_any(&mut self, type_: LoTokenType) -> Result<bool, LoError> {
        match self.peek() {
            Some(token) if token.is_any(type_) => Ok(true),
            Some(_) => Ok(false),
            _ => self.err_eof(format!("Unexpected EOF")),
        }
    }

    pub fn peek(&self) -> Option<&LoToken> {
        self.tokens.get(self.index)
    }

    pub fn next(&mut self) -> Option<&LoToken> {
        let token = self.tokens.get(self.index);
        self.index += 1;
        token
    }

    pub fn current(&self) -> &LoToken {
        if let Some(token) = self.tokens.get(self.index) {
            &token
        } else {
            &self.terminal_token
        }
    }

    pub fn loc(&self) -> &LoLocation {
        &self.current().loc
    }

    fn err_eof<T>(&self, message: String) -> Result<T, LoError> {
        Err(LoError {
            message,
            loc: self.terminal_token.loc.clone(),
        })
    }
}

impl LoToken {
    pub fn is_any(&self, type_: LoTokenType) -> bool {
        self.type_ == type_
    }

    pub fn is(&self, type_: LoTokenType, value: &str) -> bool {
        self.is_any(type_) && self.value == value
    }
}
