use crate::{ast::*, ir::*, lexer::*, lowering::*, operators::*, tokens::*, wasi_io::*, wasm::*};
use alloc::{boxed::Box, collections::BTreeMap, format, str, string::String, vec, vec::Vec};
use LoTokenType::*;

const DEFER_UNTIL_RETURN_LABEL: &str = "return";
const RECEIVER_PARAM_NAME: &str = "self";

pub fn parse(mut tokens: LoTokenStream) -> Result<WasmModule, LoError> {
    let mut ctx = ModuleContext::default();
    parse_file(&mut ctx, &mut tokens)?;
    process_delayed_actions(&mut ctx)?;
    Ok(ctx.wasm_module.take())
}

fn parse_file(ctx: &mut ModuleContext, tokens: &mut LoTokenStream) -> Result<(), LoError> {
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

// TODO: copied from v1, review
fn process_delayed_actions(ctx: &mut ModuleContext) -> Result<(), LoError> {
    // push function exports
    for fn_export in &ctx.fn_exports {
        let Some(fn_def) = ctx.fn_defs.get(&fn_export.in_name) else {
            return Err(LoError {
                message: format!("Cannot export unknown function {}", fn_export.in_name),
                loc: fn_export.loc.clone(),
            });
        };

        ctx.wasm_module.borrow_mut().exports.push(WasmExport {
            export_type: WasmExportType::Func,
            export_name: fn_export.out_name.clone(),
            exported_item_index: ctx.imported_fns_count + fn_def.fn_index,
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
            module: &ctx,
            lo_fn_type: &fn_def.kind,
            locals_last_index: fn_body.locals_last_index,
            non_arg_wasm_locals: vec![],
            defers: BTreeMap::default(),
        };

        let locals_block = Block {
            block_type: BlockType::Block,
            parent: None,
            locals: fn_body.locals,
        };

        let mut block_ctx = BlockContext {
            module: &ctx,
            fn_ctx: &mut fn_ctx,
            block: Block {
                block_type: BlockType::Function,
                parent: Some(&locals_block),
                locals: BTreeMap::new(),
            },
        };

        let mut lo_exprs = parse_block_contents(&mut block_ctx, &mut fn_body.body)?;
        if let Some(values) = get_deferred(&mut block_ctx, DEFER_UNTIL_RETURN_LABEL) {
            lo_exprs.append(&mut values?);
        };

        let mut locals = Vec::<WasmLocals>::new();
        for local_type in &block_ctx.fn_ctx.non_arg_wasm_locals {
            if let Some(wasm_locals) = locals.last_mut() {
                if (*wasm_locals).value_type == *local_type {
                    wasm_locals.count += 1;
                    continue;
                }
            }
            locals.push(WasmLocals {
                count: 1,
                value_type: local_type.clone(),
            });
        }

        // TODO: move to better place
        let mut instrs = vec![];
        lower_exprs(&mut instrs, lo_exprs);

        ctx.wasm_module.borrow_mut().codes.push(WasmFn {
            locals,
            expr: WasmExpr { instrs },
        });
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

    if let Some(_) = tokens.eat(Symbol, "memory")? {
        return parse_memory(ctx, tokens, false);
    }

    if let Some(_) = tokens.eat(Symbol, "export")? {
        if let Some(_) = tokens.eat(Symbol, "fn")? {
            return parse_fn_def(ctx, tokens, true);
        }

        if let Some(_) = tokens.eat(Symbol, "memory")? {
            return parse_memory(ctx, tokens, true);
        }

        if let Some(_) = tokens.eat(Symbol, "existing")? {
            tokens.expect(Symbol, "fn")?;
            let in_name = tokens.expect_any(Symbol)?.clone();
            tokens.expect(Symbol, "as")?;
            let out_name = tokens.expect_any(StringLiteral)?.clone();

            ctx.fn_exports.push(FnExport {
                in_name: in_name.value,
                out_name: out_name.value,
                loc: in_name.loc,
            });

            return Ok(());
        }
    }

    if let Some(_) = tokens.eat(Symbol, "import")? {
        tokens.expect(Symbol, "from")?;
        let module_name = tokens.expect_any(StringLiteral)?.clone();

        tokens.expect(Delim, "{")?;
        while let None = tokens.eat(Delim, "}")? {
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
                type_index,
                kind: fn_decl.lo_type,
            };
            ctx.fn_defs.insert(fn_decl.fn_name.clone(), fn_def);
            ctx.wasm_module.borrow_mut().imports.push(WasmImport {
                module_name: module_name.value.clone(),
                item_name: fn_decl.method_name,
                item_desc: WasmImportDesc::Func { type_index },
            });
        }

        return Ok(());
    }

    if let Some(let_token) = tokens.eat(Symbol, "let")?.cloned() {
        let mutable = true;
        let global_name = tokens.expect_any(Symbol)?.clone();
        tokens.expect(Operator, "=")?;

        let global_value = parse_const_expr(ctx, tokens, 0)?;

        let lo_type = global_value.get_type(ctx);
        let Some(wasm_type) = lo_type.to_wasm_type() else {
            return Err(LoError {
                message: format!("Unsupported type: {lo_type}"),
                // TODO: value.loc() is not available
                loc: let_token.loc,
            });
        };

        if ctx.globals.contains_key(&global_name.value) {
            return Err(LoError {
                message: format!("Cannot redefine global: {}", global_name.value),
                loc: global_name.loc,
            });
        }

        ctx.globals.insert(
            global_name.value.clone(),
            GlobalDef {
                index: ctx.globals.len() as u32,
                mutable,
                value_type: lo_type,
            },
        );

        let mut initial_value = WasmExpr { instrs: vec![] };
        lower_expr(&mut initial_value.instrs, global_value);

        ctx.wasm_module.borrow_mut().globals.push(WasmGlobal {
            kind: WasmGlobalKind {
                value_type: wasm_type,
                mutable,
            },
            initial_value,
        });

        return Ok(());
    }

    if let Some(_) = tokens.eat(Symbol, "struct")? {
        let struct_name = tokens.expect_any(Symbol)?.clone();

        if ctx.struct_defs.contains_key(&struct_name.value) {
            return Err(LoError {
                message: format!("Cannot redefine struct {}", struct_name.value),
                loc: struct_name.loc,
            });
        }

        // declare not fully defined struct to use in self-references
        ctx.struct_defs.insert(
            struct_name.value.clone(),
            StructDef {
                fields: vec![],
                fully_defined: false,
            },
        );

        let mut field_index = 0;
        let mut byte_offset = 0;
        let mut struct_fields = Vec::<StructField>::new();

        tokens.expect(Delim, "{")?;
        while let None = tokens.eat(Delim, "}")? {
            let field_name = tokens.expect_any(Symbol)?.clone();
            tokens.expect(Operator, ":")?;
            let field_type = parse_lo_type(ctx, tokens)?;
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
                    // TODO: field_type.loc() is not available
                    loc: field_name.loc,
                })?;

            struct_fields.push(StructField {
                name: field_name.value,
                value_type: field_type,
                field_index,
                byte_offset,
            });

            field_index += stats.count;
            byte_offset += stats.byte_length;
        }

        let struct_def = ctx.struct_defs.get_mut(&struct_name.value).unwrap();
        struct_def.fields.append(&mut struct_fields);
        struct_def.fully_defined = true;

        return Ok(());
    }

    if let Some(_) = tokens.eat(Symbol, "type")?.cloned() {
        let type_alias = tokens.expect_any(Symbol)?.clone();
        tokens.expect(Operator, "=")?;
        let actual_type = parse_lo_type(ctx, tokens)?;

        if ctx.type_aliases.borrow().contains_key(&type_alias.value) {
            return Err(LoError {
                message: format!("Duplicate type alias: {}", type_alias.value),
                loc: type_alias.loc.clone(),
            });
        }

        ctx.type_aliases
            .borrow_mut()
            .insert(type_alias.value, actual_type);

        return Ok(());
    }

    if let Some(_) = tokens.eat(Symbol, "const")?.cloned() {
        let const_name = tokens.expect_any(Symbol)?.clone();
        tokens.expect(Operator, "=")?;
        let const_value = parse_const_expr(ctx, tokens, 0)?;

        if ctx.constants.borrow().contains_key(&const_name.value) {
            return Err(LoError {
                message: format!("Duplicate constant: {}", const_name.value),
                loc: const_name.loc.clone(),
            });
        }

        ctx.constants
            .borrow_mut()
            .insert(const_name.value, const_value);

        return Ok(());
    }

    if let Some(_) = tokens.eat(Symbol, "include")? {
        let module_path = tokens.expect_any(StringLiteral)?;

        if !ctx.included_modules.insert(module_path.value.clone()) {
            // do not include module twice
            return Ok(());
        };

        let file_name = format!("{}.lo", module_path.value);
        let mod_fd = fd_open(&file_name).map_err(|err| LoError {
            message: format!("Cannot load file {file_name}: {err}"),
            loc: module_path.loc.clone(),
        })?;

        let source_buf = fd_read_all_and_close(mod_fd);
        let source = str::from_utf8(source_buf.as_slice()).unwrap();

        let mut tokens = lex_all(&file_name, source)?;
        return parse_file(ctx, &mut tokens);
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
    exported: bool,
) -> Result<(), LoError> {
    if let Some(_) = tokens.eat(Operator, "@")? {
        let offset = parse_u32_literal(tokens.expect_any(IntLiteral)?)?;
        tokens.expect(Operator, "=")?;
        let data = tokens.expect_any(StringLiteral)?;

        let bytes = data.value.as_bytes().iter().map(|b| *b).collect();

        ctx.wasm_module.borrow_mut().datas.push(WasmData::Active {
            offset: WasmExpr {
                instrs: vec![WasmInstr::I32Const {
                    value: offset as i32,
                }],
            },
            bytes,
        });

        return Ok(());
    }

    let memory_name = String::from("memory");
    if ctx.memories.contains_key(&memory_name) {
        return Err(LoError {
            message: format!("Duplicate memory definition: {memory_name}"),
            loc: tokens.peek().unwrap().loc.clone(),
        });
    }

    let mut memory_limits = WasmLimits { min: 0, max: None };

    tokens.expect(Delim, "{")?;
    while let None = tokens.eat(Delim, "}")? {
        let prop = tokens.expect_any(Symbol)?.clone();
        match prop.value.as_str() {
            "min_pages" => {
                tokens.expect(Operator, ":")?;
                let value = parse_u32_literal(tokens.expect_any(IntLiteral)?)?;
                memory_limits.min = value;
            }
            "max_pages" => {
                tokens.expect(Operator, ":")?;
                let value = parse_u32_literal(tokens.expect_any(IntLiteral)?)?;
                memory_limits.max = Some(value);
            }
            _ => {
                return Err(LoError {
                    message: format!("ayo"),
                    loc: prop.loc,
                })
            }
        }
    }

    let memory_index = ctx.wasm_module.borrow().memories.len() as u32;
    ctx.wasm_module.borrow_mut().memories.push(memory_limits);
    ctx.memories.insert(memory_name.clone(), memory_index);

    if exported {
        ctx.wasm_module.borrow_mut().exports.push(WasmExport {
            export_type: WasmExportType::Mem,
            export_name: "memory".into(),
            exported_item_index: memory_index,
        });
    }

    Ok(())
}

fn parse_fn_def(
    ctx: &mut ModuleContext,
    tokens: &mut LoTokenStream,
    exported: bool,
) -> Result<(), LoError> {
    let fn_decl = parse_fn_decl(ctx, tokens)?;
    let contents = parse_block_extract_contents(ctx, tokens)?;

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
            loc: fn_decl.loc.clone(),
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
            type_index,
            kind: fn_decl.lo_type,
        },
    );

    ctx.fn_bodies.borrow_mut().push(FnBody {
        fn_index,
        type_index,
        locals: fn_decl.locals,
        locals_last_index,
        body: contents,
    });

    return Ok(());
}

struct FnDecl {
    fn_name: String,
    method_name: String,
    loc: LoLocation,
    lo_type: LoFnType,
    wasm_type: WasmFnType,
    locals: BTreeMap<String, LocalDef>,
}

fn parse_fn_decl(ctx: &mut ModuleContext, tokens: &mut LoTokenStream) -> Result<FnDecl, LoError> {
    let is_plain_fn = if let Some((t1, t2)) = tokens.peek2() {
        t1.is_any(Symbol) && t2.is(Delim, "(")
    } else {
        false
    };

    let receiver_type = if !is_plain_fn {
        let receiver_type = parse_lo_type(ctx, tokens)?;
        tokens.expect(Operator, ".")?;
        Some(receiver_type)
    } else {
        None
    };
    let method_name = tokens.expect_any(Symbol)?.clone();

    let mut fn_decl = FnDecl {
        fn_name: method_name.value.clone(),
        method_name: method_name.value,
        loc: method_name.loc,
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

    if let Some(receiver_type) = receiver_type {
        fn_decl.fn_name = fn_name_for_method(&receiver_type, &fn_decl.fn_name);
        add_fn_param(ctx, &mut fn_decl, RECEIVER_PARAM_NAME.into(), receiver_type);
    }

    tokens.expect(Delim, "(")?;
    while let None = tokens.eat(Delim, ")")? {
        let p_name = tokens.expect_any(Symbol)?.clone();
        tokens.expect(Operator, ":")?;
        let p_type = parse_lo_type(ctx, tokens)?;
        if !tokens.next_is(Delim, ")")? {
            tokens.expect(Delim, ",")?;
        }

        if fn_decl.locals.contains_key(&p_name.value) {
            return Err(LoError {
                message: format!(
                    "Found function param with conflicting name: {}",
                    p_name.value
                ),
                loc: p_name.loc.clone(),
            });
        }

        add_fn_param(ctx, &mut fn_decl, p_name.value, p_type);
    }

    let lo_output = if let Some(_) = tokens.eat(Operator, "->")? {
        parse_lo_type(ctx, tokens)?
    } else {
        LoType::Void
    };

    lo_output.emit_components(&ctx, &mut fn_decl.wasm_type.outputs);
    fn_decl.lo_type.output = lo_output;

    Ok(fn_decl)
}

fn add_fn_param(ctx: &ModuleContext, fn_decl: &mut FnDecl, p_name: String, p_type: LoType) {
    let local_def = LocalDef {
        index: fn_decl.wasm_type.inputs.len() as u32,
        value_type: p_type.clone(),
    };
    fn_decl.locals.insert(p_name.clone(), local_def);

    p_type.emit_components(ctx, &mut fn_decl.wasm_type.inputs);
    fn_decl.lo_type.inputs.push(p_type);
}

fn parse_block(
    ctx: &mut BlockContext,
    tokens: &mut LoTokenStream,
) -> Result<Vec<LoInstr>, LoError> {
    let mut contents = parse_block_extract_contents(ctx.module, tokens)?;
    parse_block_contents(ctx, &mut contents)
}

fn parse_block_extract_contents(
    _ctx: &ModuleContext,
    tokens: &mut LoTokenStream,
) -> Result<LoTokenStream, LoError> {
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

fn parse_block_contents(
    ctx: &mut BlockContext,
    tokens: &mut LoTokenStream,
) -> Result<Vec<LoInstr>, LoError> {
    let mut exprs = vec![];
    while tokens.peek().is_some() {
        let expr_start = tokens.peek().unwrap().loc.clone();
        let expr = parse_expr(ctx, tokens, 0)?;
        tokens.expect(Delim, ";")?;

        let expr_type = expr.get_type(ctx.module);
        if expr_type != LoType::Void {
            return Err(LoError {
                message: format!("TypeError: Excess values"),
                loc: expr_start,
            });
        }

        exprs.push(expr);
    }
    Ok(exprs)
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
    if let Some(int) = tokens.eat_any(IntLiteral)? {
        return Ok(LoInstr::U32Const {
            value: parse_u32_literal(&int)?,
        });
    }

    if let Some(_) = tokens.eat(Symbol, "i64")? {
        let int = tokens.expect_any(IntLiteral)?;
        return Ok(LoInstr::I64Const {
            value: parse_i64_literal(&int)?,
        });
    }

    if let Some(_) = tokens.eat(Symbol, "true")?.cloned() {
        return Ok(LoInstr::Casted {
            value_type: LoType::Bool,
            expr: Box::new(LoInstr::U32Const { value: 1 }),
        });
    }

    if let Some(_) = tokens.eat(Symbol, "false")?.cloned() {
        return Ok(LoInstr::Casted {
            value_type: LoType::Bool,
            expr: Box::new(LoInstr::U32Const { value: 0 }),
        });
    }

    if let Some(_) = tokens.eat(Symbol, "__DATA_SIZE__")? {
        return Ok(LoInstr::U32ConstLazy {
            value: ctx.module.data_size.clone(),
        });
    }

    if let Some(_) = tokens.eat(Symbol, "unreachable")? {
        return Ok(LoInstr::Unreachable);
    }

    if let Some(value) = tokens.eat_any(StringLiteral)? {
        let string_len = value.value.as_bytes().len() as u32;

        let string_ptr_ = ctx.module.string_pool.borrow().get(&value.value).cloned();
        let string_ptr = match string_ptr_ {
            Some(string_ptr) => string_ptr,
            None => {
                let new_string_ptr = *ctx.module.data_size.borrow();
                ctx.module
                    .string_pool
                    .borrow_mut()
                    .insert(value.value.clone(), new_string_ptr);

                *ctx.module.data_size.borrow_mut() += string_len;
                ctx.module
                    .wasm_module
                    .borrow_mut()
                    .datas
                    .push(WasmData::Active {
                        offset: WasmExpr {
                            instrs: vec![WasmInstr::I32Const {
                                value: new_string_ptr as i32,
                            }],
                        },
                        bytes: value.value.as_bytes().to_vec(),
                    });
                new_string_ptr
            }
        };

        return Ok(LoInstr::MultiValueEmit {
            values: vec![
                LoInstr::U32Const { value: string_ptr },
                LoInstr::U32Const { value: string_len },
            ],
        });
    }

    if let Some(_) = tokens.eat(Delim, "(")? {
        let expr = parse_expr(ctx, tokens, 0)?;
        tokens.expect(Delim, ")")?;
        return Ok(expr);
    }

    if let Some(return_token) = tokens.eat(Symbol, "return")?.cloned() {
        let value = if tokens.peek().is_none() || tokens.next_is(Delim, ";")? {
            LoInstr::Casted {
                value_type: LoType::Void,
                expr: Box::new(LoInstr::MultiValueEmit { values: vec![] }),
            }
        } else {
            parse_expr(ctx, tokens, 0)?
        };

        let return_type = value.get_type(ctx.module);
        if return_type != ctx.fn_ctx.lo_fn_type.output {
            return Err(LoError {
                message: format!(
                    "TypeError: Invalid return type, \
                        expected {output:?}, got {return_type:?}",
                    output = ctx.fn_ctx.lo_fn_type.output,
                ),
                loc: return_token.loc,
            });
        }

        let return_expr = LoInstr::Return {
            value: Box::new(value),
        };
        if let Some(values) = get_deferred(ctx, DEFER_UNTIL_RETURN_LABEL) {
            let mut values = values?;
            values.push(return_expr);
            return Ok(LoInstr::Casted {
                value_type: LoType::Void,
                expr: Box::new(LoInstr::MultiValueEmit { values }),
            });
        } else {
            return Ok(return_expr);
        }
    }

    if let Some(t) = tokens.eat(Symbol, "sizeof")?.cloned() {
        let value_type = parse_lo_type(ctx.module, tokens)?;

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

    if let Some(_) = tokens.eat(Symbol, "char_code")?.cloned() {
        let value = tokens.expect_any(StringLiteral)?;

        let Some(char) = value.value.chars().next() else {
            return Err(LoError {
                message: format!("String must not be empty"),
                loc: value.loc.clone(),
            });
        };

        return Ok(LoInstr::U32Const { value: char as u32 });
    }

    if let Some(_) = tokens.eat(Symbol, "defer")? {
        let defer_label = if let Some(_) = tokens.eat(Operator, "@")? {
            tokens.expect_any(Symbol)?.value.clone()
        } else {
            String::from(DEFER_UNTIL_RETURN_LABEL)
        };
        let deffered_expr = parse_expr(ctx, tokens, 0)?;

        let deferred = ctx
            .fn_ctx
            .defers
            .entry(defer_label)
            .or_insert_with(|| vec![]);

        deferred.push(deffered_expr);

        return Ok(LoInstr::Casted {
            value_type: LoType::Void,
            expr: Box::new(LoInstr::MultiValueEmit { values: vec![] }),
        });
    }

    if let Some(_) = tokens.eat(Symbol, "defer_eval")? {
        tokens.expect(Operator, "@")?;
        let defer_label = tokens.expect_any(Symbol)?.clone();

        let Some(values) = get_deferred(ctx, &defer_label.value) else {
            return Err(LoError {
                message: format!("Unknown defer scope: {}", defer_label.value),
                loc: defer_label.loc.clone(),
            });
        };

        return Ok(LoInstr::Casted {
            value_type: LoType::Void,
            expr: Box::new(LoInstr::MultiValueEmit { values: values? }),
        });
    }

    if let Some(_) = tokens.eat(Symbol, "__memory_size")? {
        tokens.expect(Delim, "(")?;
        tokens.expect(Delim, ")")?;
        return Ok(LoInstr::MemorySize {});
    }

    if let Some(t) = tokens.eat(Symbol, "__memory_grow")?.cloned() {
        tokens.expect(Delim, "(")?;
        let size = parse_expr(ctx, tokens, 0)?;
        tokens.expect(Delim, ")")?;

        let size_type = size.get_type(ctx.module);
        if size_type != LoType::U32 {
            return Err(LoError {
                message: format!("Invalid arguments for {}", t.value),
                loc: t.loc,
            });
        };

        return Ok(LoInstr::MemoryGrow {
            size: Box::new(size),
        });
    }

    if let Some(_) = tokens.eat(Symbol, "__debug_here")?.cloned() {
        debug(format!(
            "imported fn count: {}",
            ctx.module.imported_fns_count
        ));
        for fn_name in ctx.module.fn_defs.keys() {
            debug(format!("{fn_name}"));
        }
        return Err(LoError::unreachable(file!(), line!()));
    }

    if let Some(t) = tokens.eat(Symbol, "__debug_typeof")?.cloned() {
        let loc = tokens.peek().unwrap_or(&t).loc.clone();

        let expr = parse_expr(ctx, tokens, 0)?;
        let expr_type = expr.get_type(ctx.module);
        crate::wasi_io::debug(format!(
            "{}",
            String::from(LoError {
                message: format!("{expr_type:?}"),
                loc,
            })
        ));
        return Ok(LoInstr::Casted {
            value_type: LoType::Void,
            expr: Box::new(LoInstr::MultiValueEmit { values: vec![] }),
        });
    }

    // TODO: support `else`
    if let Some(_) = tokens.eat(Symbol, "if")? {
        let cond = parse_expr(ctx, tokens, 0)?;

        let then_branch = parse_block(
            &mut BlockContext {
                module: ctx.module,
                fn_ctx: ctx.fn_ctx,
                block: Block {
                    block_type: BlockType::Block,
                    parent: Some(&ctx.block),
                    locals: BTreeMap::new(),
                },
            },
            tokens,
        )?;

        return Ok(LoInstr::If {
            block_type: LoType::Void,
            cond: Box::new(cond),
            then_branch,
            else_branch: None,
        });
    }

    if let Some(_) = tokens.eat(Symbol, "loop")? {
        let mut ctx = BlockContext {
            module: ctx.module,
            fn_ctx: ctx.fn_ctx,
            block: Block {
                block_type: BlockType::Loop,
                parent: Some(&ctx.block),
                locals: BTreeMap::new(),
            },
        };

        let mut body = parse_block(&mut ctx, tokens)?;

        // add implicit continue
        body.push(LoInstr::Branch { label_index: 0 });

        return Ok(LoInstr::Block {
            block_type: LoType::Void,
            body: vec![LoInstr::Loop {
                block_type: LoType::Void,
                body,
            }],
        });
    }

    if let Some(_) = tokens.eat(Symbol, "break")? {
        let mut label_index = 1; // 0 = loop, 1 = loop wrapper block

        let mut current_block = &ctx.block;
        loop {
            if current_block.block_type == BlockType::Loop {
                break;
            }

            current_block = current_block.parent.unwrap();
            label_index += 1;
        }

        return Ok(LoInstr::Branch { label_index });
    }

    if let Some(let_token) = tokens.eat(Symbol, "let")?.cloned() {
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

        if ctx.block.get_own_local(&local_name.value).is_some() {
            return Err(LoError {
                message: format!("Duplicate local definition: {}", local_name.value),
                loc: local_name.loc.clone(),
            });
        }

        let local_indicies = ctx.push_local(local_name.value.clone(), value_type.clone());
        let values = local_indicies
            .map(|i| LoInstr::UntypedLocalGet { local_index: i })
            .collect();
        let bind_instr = LoInstr::MultiValueEmit { values };
        return compile_set(ctx, value, bind_instr, &let_token.loc);
    }

    if let Some(token) = tokens.peek().cloned() {
        if let Some(op) = PrefixOp::parse(token) {
            let min_bp = op.info.get_min_bp_for_next();
            tokens.next(); // skip operator

            match op.tag {
                PrefixOpTag::Not => {
                    return Ok(LoInstr::BinaryOp {
                        kind: WasmBinaryOpKind::I32Equal,
                        lhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
                        rhs: Box::new(LoInstr::U32Const { value: 0 }),
                    });
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

                    return compile_load(ctx, &pointee_type, pointer, 0).map_err(|err| LoError {
                        message: err,
                        loc: op.token.loc,
                    });
                }
            }
        }
    }

    if let Some(value) = tokens.eat_any(Symbol)?.cloned() {
        if let Some(const_value) = ctx.module.constants.borrow().get(&value.value) {
            return Ok(const_value.clone());
        }

        if let Some(fn_def) = ctx.module.fn_defs.get(&value.value) {
            let mut args = vec![];
            parse_fn_call_args(ctx, tokens, &mut args)?;

            return Ok(LoInstr::Call {
                fn_index: fn_def.get_absolute_index(ctx.module),
                return_type: fn_def.kind.output.clone(),
                args,
            });
        }

        if let Some(struct_def) = ctx.module.struct_defs.get(&value.value) {
            let struct_name = value;

            let mut values = vec![];
            tokens.expect(Delim, "{")?;
            while let None = tokens.eat(Delim, "}")? {
                let field_name = tokens.expect_any(Symbol)?.clone();
                tokens.expect(Operator, ":")?;
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
                        message: format!(
                            "Unexpected field name, expecting: `{}`",
                            struct_field.name
                        ),
                        loc: field_name.loc,
                    });
                }

                let field_value_type = field_value.get_type(ctx.module);
                if field_value_type != struct_field.value_type {
                    return Err(LoError {
                        message: format!(
                            "Invalid type for field {}.{}, expected: {}, got: {}",
                            struct_name.value,
                            field_name.value,
                            struct_field.value_type,
                            field_value_type
                        ),
                        // TODO: field_value.loc() is not available
                        loc: field_name.loc.clone(),
                    });
                }
                values.push(field_value);
            }

            return Ok(LoInstr::Casted {
                value_type: LoType::StructInstance {
                    name: struct_name.value,
                },
                expr: Box::new(LoInstr::MultiValueEmit { values }),
            });
        };

        if let Some(global) = ctx.module.globals.get(&value.value) {
            return Ok(LoInstr::GlobalGet {
                global_index: global.index,
            });
        };

        let Some(local) = ctx.block.get_local(&value.value) else {
            return Err(LoError {
                message: format!("Reading unknown variable: {}", value.value),
                loc: value.loc,
            });
        };

        return compile_local_get(&ctx.module, local.index, &local.value_type).map_err(|message| {
            LoError {
                message,
                loc: value.loc,
            }
        });
    }

    let unexpected = tokens.peek().unwrap();
    return Err(LoError {
        message: format!("Unexpected token: {}", unexpected.value),
        loc: unexpected.loc.clone(),
    });
}

fn parse_postfix(
    ctx: &mut BlockContext,
    tokens: &mut LoTokenStream,
    primary: LoInstr,
    op: InfixOp,
) -> Result<LoInstr, LoError> {
    let min_bp = op.info.get_min_bp_for_next();

    // TODO: typecheck that operands are actually numbers
    Ok(match op.tag {
        InfixOpTag::Equal => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Equal,
            lhs: Box::new(primary),
            rhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::NotEqual => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32NotEqual,
            lhs: Box::new(primary),
            rhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::And => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32And,
            lhs: Box::new(primary),
            rhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::Or => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Or,
            lhs: Box::new(primary),
            rhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::Less => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32LessThenUnsigned,
            lhs: Box::new(primary),
            rhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::Greater => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32GreaterThanUnsigned,
            lhs: Box::new(primary),
            rhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::GreaterEqual => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32GreaterEqualUnsigned,
            lhs: Box::new(primary),
            rhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::Add => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Add,
            lhs: Box::new(primary),
            rhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::Sub => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Sub,
            lhs: Box::new(primary),
            rhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::Mul => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Mul,
            lhs: Box::new(primary),
            rhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::Div => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32DivUnsigned,
            lhs: Box::new(primary),
            rhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::Mod => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32RemUnsigned,
            lhs: Box::new(primary),
            rhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::Assign => {
            let value = parse_expr(ctx, tokens, min_bp)?;
            let value_type = value.get_type(ctx.module);
            let bind_type = primary.get_type(ctx.module);

            if value_type != bind_type {
                return Err(LoError {
                    message: format!(
                        "TypeError: Invalid types for '{}', \
                        needed {bind_type}, got {value_type}",
                        op.token.value
                    ),
                    loc: op.token.loc.clone(),
                });
            }

            compile_set(ctx, value, primary, &op.token.loc)?
        }
        InfixOpTag::AddAssign => {
            let value = LoInstr::BinaryOp {
                kind: WasmBinaryOpKind::I32Add,
                lhs: Box::new(primary.clone()),
                rhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
            };
            // TODO: lhs.loc() is not available
            compile_set(ctx, value, primary, &op.token.loc)?
        }
        InfixOpTag::SubAssign => {
            let value = LoInstr::BinaryOp {
                kind: WasmBinaryOpKind::I32Sub,
                lhs: Box::new(primary.clone()),
                rhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
            };
            // TODO: lhs.loc() is not available
            compile_set(ctx, value, primary, &op.token.loc)?
        }
        InfixOpTag::Cast => LoInstr::Casted {
            value_type: parse_lo_type(ctx.module, tokens)?,
            expr: Box::new(primary),
        },
        InfixOpTag::FieldAccess => {
            let field_or_method_name = tokens.expect_any(Symbol)?.clone();
            if !tokens.next_is(Delim, "(").unwrap_or(false) {
                let field_name = field_or_method_name;

                if let LoInstr::StructGet {
                    struct_name,
                    base_index,
                    ..
                } = &primary
                {
                    let struct_def = ctx.module.struct_defs.get(struct_name).unwrap(); // safe
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

                    return compile_local_get(
                        &ctx.module,
                        base_index + field.field_index,
                        &field.value_type,
                    )
                    .map_err(|message| LoError {
                        message,
                        // TODO: lhs.loc() is not available
                        loc: op.token.loc,
                    });
                };

                if let LoInstr::StructLoad {
                    struct_name,
                    address_instr,
                    base_byte_offset,
                    ..
                } = primary
                {
                    // safe to unwrap as it was already checked in `StructLoad`
                    let struct_def = ctx.module.struct_defs.get(&struct_name).unwrap();

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

                let lhs_type = primary.get_type(ctx.module);
                return Err(LoError {
                    message: format!(
                        "Trying to get field '{}' on non struct: {lhs_type}",
                        field_name.value
                    ),
                    loc: field_name.loc,
                });
            }

            let method_name = field_or_method_name;
            let recevier_type = primary.get_type(ctx.module);

            let mut args = vec![primary];
            parse_fn_call_args(ctx, tokens, &mut args)?;

            let fn_name = fn_name_for_method(&recevier_type, &method_name.value);
            let Some(fn_def) = ctx.module.fn_defs.get(&fn_name) else {
                return Err(LoError {
                    message: format!("Unknown function: {fn_name}"),
                    loc: method_name.loc,
                });
            };

            LoInstr::Call {
                fn_index: fn_def.fn_index,
                return_type: fn_def.kind.output.clone(),
                args,
            }
        }
        InfixOpTag::RefFieldAccess => {
            let field_name = tokens.expect_any(Symbol)?;

            let struct_ref = Box::new(primary);
            let struct_ref_type = struct_ref.get_type(ctx.module);

            let LoType::Pointer(pointee_type) = &struct_ref_type else {
                return Err(LoError {
                    message: format!("Cannot dereference {struct_ref_type:?}"),
                    loc: op.token.loc.clone(),
                });
            };
            let LoType::StructInstance { name: struct_name } = pointee_type.as_ref() else {
                return Err(LoError {
                    message: format!("Cannot dereference {struct_ref_type:?}"),
                    loc: op.token.loc.clone(),
                });
            };

            let struct_def = ctx.module.struct_defs.get(struct_name).unwrap();
            let Some(field) = struct_def
                .fields
                .iter()
                .find(|f| f.name == *field_name.value)
            else {
                return Err(LoError {
                    message: format!("Unknown field {} in struct {struct_name}", field_name.value),
                    loc: field_name.loc.clone(),
                });
            };

            compile_load(ctx, &field.value_type, struct_ref, field.byte_offset).map_err(|e| {
                LoError {
                    message: e,
                    loc: op.token.loc.clone(),
                }
            })?
        }
    })
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

fn parse_const_expr(
    ctx: &ModuleContext,
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
    ctx: &ModuleContext,
    tokens: &mut LoTokenStream,
) -> Result<LoInstr, LoError> {
    if let Some(int) = tokens.eat_any(IntLiteral)? {
        return Ok(LoInstr::U32Const {
            value: parse_u32_literal(&int)?,
        });
    }

    if let Some(_) = tokens.eat(Symbol, "i64")? {
        let int = tokens.expect_any(IntLiteral)?;
        return Ok(LoInstr::I64Const {
            value: parse_i64_literal(&int)?,
        });
    }

    if let Some(_) = tokens.eat(Symbol, "true")? {
        return Ok(LoInstr::Casted {
            value_type: LoType::Bool,
            expr: Box::new(LoInstr::U32Const { value: 1 }),
        });
    }

    if let Some(_) = tokens.eat(Symbol, "false")? {
        return Ok(LoInstr::Casted {
            value_type: LoType::Bool,
            expr: Box::new(LoInstr::U32Const { value: 0 }),
        });
    }

    if let Some(_) = tokens.eat(Symbol, "__DATA_SIZE__")? {
        return Ok(LoInstr::U32ConstLazy {
            value: ctx.data_size.clone(),
        });
    }

    if let Some(value) = tokens.eat_any(Symbol)?.cloned() {
        if let Some(const_value) = ctx.constants.borrow().get(&value.value) {
            return Ok(const_value.clone());
        }

        let Some(global) = ctx.globals.get(&value.value) else {
            return Err(LoError {
                message: format!("Reading unknown variable: {}", value.value),
                loc: value.loc,
            });
        };

        return Ok(LoInstr::GlobalGet {
            global_index: global.index,
        });
    }

    let unexpected = tokens.peek().unwrap();
    return Err(LoError {
        message: format!("Unexpected token in const context: {}", unexpected.value),
        loc: unexpected.loc.clone(),
    });
}

fn parse_const_postfix(
    ctx: &ModuleContext,
    tokens: &mut LoTokenStream,
    primary: LoInstr,
    op: InfixOp,
) -> Result<LoInstr, LoError> {
    let min_bp = op.info.get_min_bp_for_next();

    Ok(match op.tag {
        InfixOpTag::Equal => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Equal,
            lhs: Box::new(primary),
            rhs: Box::new(parse_const_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::NotEqual => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32NotEqual,
            lhs: Box::new(primary),
            rhs: Box::new(parse_const_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::And => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32And,
            lhs: Box::new(primary),
            rhs: Box::new(parse_const_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::Or => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Or,
            lhs: Box::new(primary),
            rhs: Box::new(parse_const_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::Less => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32LessThenUnsigned,
            lhs: Box::new(primary),
            rhs: Box::new(parse_const_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::Greater => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32GreaterThanUnsigned,
            lhs: Box::new(primary),
            rhs: Box::new(parse_const_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::GreaterEqual => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32GreaterEqualUnsigned,
            lhs: Box::new(primary),
            rhs: Box::new(parse_const_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::Add => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Add,
            lhs: Box::new(primary),
            rhs: Box::new(parse_const_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::Sub => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Sub,
            lhs: Box::new(primary),
            rhs: Box::new(parse_const_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::Mul => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Mul,
            lhs: Box::new(primary),
            rhs: Box::new(parse_const_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::Div => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32DivUnsigned,
            lhs: Box::new(primary),
            rhs: Box::new(parse_const_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::Mod => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32RemUnsigned,
            lhs: Box::new(primary),
            rhs: Box::new(parse_const_expr(ctx, tokens, min_bp)?),
        },
        InfixOpTag::Cast => LoInstr::Casted {
            value_type: parse_lo_type(ctx, tokens)?,
            expr: Box::new(primary),
        },
        _ => {
            return Err(LoError {
                message: format!("Unsupported operator in const context: {}", op.token.value),
                loc: op.token.loc,
            });
        }
    })
}

fn parse_lo_type(ctx: &ModuleContext, tokens: &mut LoTokenStream) -> Result<LoType, LoError> {
    parse_lo_type_checking_ref(ctx, tokens, false)
}

fn parse_lo_type_checking_ref(
    ctx: &ModuleContext,
    tokens: &mut LoTokenStream,
    is_referenced: bool,
) -> Result<LoType, LoError> {
    if let Some(_) = tokens.eat(Operator, "&")? {
        let pointee = parse_lo_type_checking_ref(ctx, tokens, true)?;
        return Ok(LoType::Pointer(Box::new(pointee)));
    }

    if let Some(_) = tokens.eat(Operator, "&*")? {
        let pointee = parse_lo_type_checking_ref(ctx, tokens, true)?;
        return Ok(LoType::Pointer(Box::new(pointee)));
    }

    if let Some(_) = tokens.eat(Delim, "(")? {
        tokens.expect(Delim, ")")?;
        return Ok(LoType::Void);
    }

    let token = tokens.expect_any(Symbol)?;
    match token.value.as_str() {
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
            if let Some(actual_type) = ctx.type_aliases.borrow().get(&token.value) {
                return Ok(actual_type.clone());
            }

            if let Some(struct_def) = ctx.struct_defs.get(&token.value) {
                if !struct_def.fully_defined && !is_referenced {
                    return Err(LoError {
                        message: format!("Cannot use partially defined struct"),
                        loc: token.loc.clone(),
                    });
                }

                return Ok(LoType::StructInstance {
                    name: token.value.clone(),
                });
            }

            return Err(LoError {
                message: format!("Unexpected token in type: {}", token.value),
                loc: token.loc.clone(),
            });
        }
    }
}

fn parse_u32_literal(int: &LoToken) -> Result<u32, LoError> {
    int.value.parse().map_err(|_| LoError {
        message: format!("Parsing u32 (implicit) failed"),
        loc: int.loc.clone(),
    })
}

fn parse_i64_literal(int: &LoToken) -> Result<i64, LoError> {
    int.value.parse().map_err(|_| LoError {
        message: format!("Parsing i64 failed"),
        loc: int.loc.clone(),
    })
}

fn fn_name_for_method(receiver_type: &LoType, method_name: &str) -> String {
    format!("{receiver_type}_{method_name}")
}

// TODO: copied from v1, review
fn get_deferred(
    ctx: &mut BlockContext,
    defer_label: &str,
) -> Option<Result<Vec<LoInstr>, LoError>> {
    let Some(deferred) = ctx.fn_ctx.defers.get(defer_label) else {
        return None;
    };

    let mut deferred = deferred.clone();
    deferred.reverse();

    Some(Ok(deferred))
}

// TODO: copied from v1, review
fn compile_load(
    ctx: &mut BlockContext,
    value_type: &LoType,
    address_instr: Box<LoInstr>,
    base_byte_offset: u32,
) -> Result<LoInstr, String> {
    if let Ok(_) = value_type.to_load_kind() {
        return Ok(LoInstr::Load {
            kind: value_type.clone(),
            align: 0,
            offset: base_byte_offset,
            address_instr: address_instr.clone(),
        });
    }

    if let LoType::Tuple(item_types) = value_type {
        let mut item_gets = vec![];
        let mut item_byte_offset = 0;
        for item_type in item_types {
            item_gets.push(compile_load(
                ctx,
                item_type,
                address_instr.clone(),
                base_byte_offset + item_byte_offset,
            )?);
            item_byte_offset += item_type.sized_comp_stats(&ctx.module)?.byte_length;
        }

        return Ok(LoInstr::Casted {
            value_type: value_type.clone(),
            expr: Box::new(LoInstr::MultiValueEmit { values: item_gets }),
        });
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
            align: 1,
            offset: comp.byte_offset,
            address_instr: Box::new(LoInstr::UntypedLocalGet {
                local_index: address_local_index,
            }),
        });
    }

    Ok(LoInstr::StructLoad {
        struct_name: name.clone(),
        address_instr,
        address_local_index,
        base_byte_offset,
        primitive_loads,
    })
}

// TODO: copied from v1, review
fn compile_local_get(
    ctx: &ModuleContext,
    base_index: u32,
    value_type: &LoType,
) -> Result<LoInstr, String> {
    let comp_count = value_type.emit_components(ctx, &mut vec![]);
    if comp_count == 1 {
        return Ok(LoInstr::LocalGet {
            local_index: base_index,
            value_type: value_type.clone(),
        });
    }

    if let LoType::Tuple(item_types) = value_type {
        let mut item_gets = vec![];
        for (item_index, item_type) in (0..).zip(item_types) {
            item_gets.push(compile_local_get(ctx, base_index + item_index, item_type)?);
        }

        return Ok(LoInstr::Casted {
            value_type: value_type.clone(),
            expr: Box::new(LoInstr::MultiValueEmit { values: item_gets }),
        });
    }

    let LoType::StructInstance { name } = value_type else {
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

// TODO: copied from v1, review
fn compile_set(
    ctx: &mut BlockContext,
    value_instr: LoInstr,
    bind_instr: LoInstr,
    bind_loc: &LoLocation,
) -> Result<LoInstr, LoError> {
    let mut values = vec![];
    compile_set_binds(&mut values, ctx, bind_instr, bind_loc, None)?;
    values.push(value_instr);
    values.reverse();

    Ok(LoInstr::Casted {
        value_type: LoType::Void,
        expr: Box::new(LoInstr::MultiValueEmit { values }),
    })
}

// TODO: copied from v1, review
fn compile_set_binds(
    output: &mut Vec<LoInstr>,
    ctx: &mut BlockContext,
    bind_instr: LoInstr,
    bind_loc: &LoLocation,
    address_index: Option<u32>,
) -> Result<(), LoError> {
    Ok(match bind_instr {
        LoInstr::LocalGet {
            local_index,
            value_type: _,
        }
        | LoInstr::UntypedLocalGet { local_index } => {
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
                compile_set_binds(&mut values, ctx, value, bind_loc, Some(address_local_index))?;
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
        // TODO: improve this? (StructGet/MultiValueEmit/NoEmit)
        LoInstr::StructGet { primitive_gets, .. } => {
            for value in primitive_gets {
                compile_set_binds(output, ctx, value, bind_loc, address_index)?;
            }
        }
        LoInstr::MultiValueEmit { values } => {
            for value in values {
                compile_set_binds(output, ctx, value, bind_loc, address_index)?;
            }
        }
        LoInstr::Casted {
            expr: instr,
            value_type: _,
        } => {
            compile_set_binds(output, ctx, *instr, bind_loc, address_index)?;
        }
        _ => {
            return Err(LoError {
                message: format!("Invalid bind"),
                loc: bind_loc.clone(),
            });
        }
    })
}
