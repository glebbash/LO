use crate::{
    parser::SExpr,
    wasm_module::{
        WasmBinaryOpKind, WasmExport, WasmExportType, WasmExpr, WasmFn, WasmFnType, WasmGlobal,
        WasmGlobalKind, WasmImport, WasmImportDesc, WasmInstr, WasmLimits, WasmLoadKind,
        WasmLocals, WasmModule, WasmStoreKind, WasmValueType,
    },
};
use alloc::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    format,
    rc::Rc,
    string::String,
    vec,
    vec::Vec,
};
use core::cell::RefCell;

struct FnBody {
    fn_index: u32,
    locals: RefCell<BTreeMap<String, LocalDef>>,
    locals_last_index: u32,
    body: Vec<SExpr>,
}

struct StructDef {
    fields: Vec<StructField>,
}

struct StructField {
    name: String,
    value_type: WasmValueType,
}

enum LoleValueType {
    Primitive(WasmValueType),
    StructInstance { name: String },
}

#[derive(Default)]
struct ModuleContext {
    fn_defs: BTreeMap<String, FnDef>,
    fn_bodies: BTreeMap<String, FnBody>,
    fn_exports: BTreeMap<String, String>,
    memory_names: Vec<String>,
    struct_defs: BTreeMap<String, StructDef>,
    enum_kinds: BTreeMap<String, u32>,
    globals: BTreeMap<String, GlobalDef>,
    imported_fns_count: u32,
}

struct FnDef {
    local: bool,
    fn_index: u32,
}

struct FnContext<'a> {
    module: &'a ModuleContext,
    locals: &'a mut BTreeMap<String, LocalDef>,
    locals_last_index: u32,
    non_arg_locals: Vec<WasmValueType>,
}

struct LocalDef {
    index: u32,
    value_type: LoleValueType,
}

struct GlobalDef {
    index: u32,
    mutable: bool,
}

pub fn compile_module(exprs: &Vec<SExpr>) -> Result<WasmModule, String> {
    let mut module = WasmModule::default();

    let mut ctx = ModuleContext::default();

    for expr in exprs {
        compile_top_level_expr(expr, &mut ctx, &mut module)?;
    }

    // push function exports
    for (in_name, out_name) in ctx.fn_exports.iter() {
        module.exports.push(WasmExport {
            export_type: WasmExportType::Func,
            export_name: out_name.clone(),
            exported_item_index: ctx
                .fn_defs
                .get(in_name)
                .map(|fd| ctx.imported_fns_count + fd.fn_index)
                .ok_or_else(|| format!("Cannot export unknown function {in_name}"))?,
        });
    }

    let mut fn_defs_sorted = ctx.fn_bodies.values().collect::<Vec<_>>();
    fn_defs_sorted.sort_by_key(|fd| fd.fn_index);

    // push function codes
    for fn_def in fn_defs_sorted {
        let mut fn_ctx = FnContext {
            module: &ctx,
            locals: &mut fn_def.locals.borrow_mut(),
            locals_last_index: fn_def.locals_last_index,
            non_arg_locals: vec![],
        };

        let instrs = parse_instrs(&fn_def.body, &mut fn_ctx)?;

        let mut locals = vec![];
        for local_type in fn_ctx.non_arg_locals {
            locals.push(WasmLocals {
                count: 1,
                value_type: local_type.clone(),
            });
        }

        module.codes.push(WasmFn {
            locals,
            expr: WasmExpr { instrs },
        });
    }

    Ok(module)
}

fn compile_top_level_expr(
    expr: &SExpr,
    ctx: &mut ModuleContext,
    module: &mut WasmModule,
) -> Result<(), String> {
    let SExpr::List(items) = expr else {
        return Err(format!("Unexpected atom"));
    };

    let [SExpr::Atom(op), other @ ..] = &items[..] else {
        return Err(format!("Expected operation, got a simple list"));
    };

    match op.as_str() {
        "mem" => match other {
            [SExpr::Atom(mem_name), SExpr::Atom(min_literal), SExpr::Atom(min_memory), optional @ ..]
                if min_literal == ":min" =>
            {
                if ctx.memory_names.contains(mem_name) {
                    return Err(format!("Duplicate memory definition: {mem_name}"));
                }

                let min_memory = min_memory
                    .parse()
                    .map_err(|_| format!("Parsing {op} :min (u32) failed"))?;

                let max_memory = match optional {
                    [SExpr::Atom(max_literal), SExpr::Atom(max_memory)]
                        if max_literal == ":max" =>
                    {
                        Some(
                            max_memory
                                .parse::<u32>()
                                .map_err(|_| format!("Parsing {op} :max (u32) failed"))?,
                        )
                    }
                    [] => None,
                    _ => return Err(format!("Invalid arguments for {op}")),
                };

                ctx.memory_names.push(mem_name.clone());
                module.memories.push(WasmLimits {
                    min: min_memory,
                    max: max_memory,
                });
            }
            _ => return Err(format!("Invalid arguments for {op}")),
        },
        "fn" => match other {
            [SExpr::Atom(fn_name), SExpr::List(input_exprs), SExpr::List(output_exprs), SExpr::List(body)] =>
            {
                if ctx.fn_defs.contains_key(fn_name) {
                    return Err(format!("Cannot redefine function: {fn_name}"));
                }

                let mut locals = BTreeMap::new();
                let mut locals_last_index = 0;

                let mut inputs = vec![];
                for input_expr in input_exprs.iter() {
                    let SExpr::List(name_and_type) = input_expr else {
                        return Err(format!("Unexpected atom in function params list"));
                    };

                    let [SExpr::Atom(p_name), SExpr::Atom(p_type)] = &name_and_type[..] else {
                        return Err(format!("Expected name and parameter pairs in function params list"));
                    };

                    if locals.contains_key(p_name) {
                        return Err(format!(
                            "Found function param with conflicting name: {p_name}"
                        ));
                    }

                    let value_type = LoleValueType::parse(p_type, &ctx)?;
                    let comp_count = emit_value_components(&value_type, &ctx, &mut inputs);

                    locals.insert(
                        p_name.clone(),
                        LocalDef {
                            index: locals_last_index,
                            value_type,
                        },
                    );

                    locals_last_index += comp_count;
                }

                let mut outputs = vec![];
                for output_expr in output_exprs {
                    let l_type = match output_expr {
                        SExpr::Atom(atom_text) => atom_text,
                        _ => return Err(format!("Atom expected, list found")),
                    };

                    let value_type = LoleValueType::parse(l_type, &ctx)?;
                    emit_value_components(&value_type, &ctx, &mut outputs);
                }

                module.types.push(WasmFnType { inputs, outputs });
                let type_index = module.types.len() as u32 - 1;
                let fn_index = module.functions.len() as u32;

                module.functions.push(type_index);
                ctx.fn_defs.insert(
                    fn_name.clone(),
                    FnDef {
                        local: true,
                        fn_index,
                    },
                );
                ctx.fn_bodies.insert(
                    fn_name.clone(),
                    FnBody {
                        fn_index,
                        locals: RefCell::new(locals),
                        locals_last_index,
                        body: body.clone(),
                    },
                );
            }
            _ => return Err(format!("Invalid arguments for {op}")),
        },
        "import" => match other {
            [SExpr::Atom(fn_literal), SExpr::Atom(fn_name), /* * */
             SExpr::List(input_exprs), SExpr::List(output_exprs), /* * */
             SExpr::Atom(from_literal), SExpr::Atom(module_name), SExpr::Atom(extern_fn_name)]
                if fn_literal == "fn" && from_literal == ":from" =>
            {
                if ctx.fn_defs.contains_key(fn_name) {
                    return Err(format!("Cannot redefine function: {fn_name}"));
                }

                let mut param_names = BTreeSet::new();

                let mut inputs = vec![];
                for input_expr in input_exprs.iter() {
                    let SExpr::List(name_and_type) = input_expr else {
                        return Err(format!("Unexpected atom in function params list"));
                    };

                    let [SExpr::Atom(p_name), SExpr::Atom(p_type)] = &name_and_type[..] else {
                        return Err(format!("Expected name and parameter pairs in function params list"));
                    };

                    if param_names.contains(p_name) {
                        return Err(format!(
                            "Found function param with conflicting name: {p_name}"
                        ));
                    }

                    let value_type = LoleValueType::parse(p_type, &ctx)?;
                    emit_value_components(&value_type, &ctx, &mut inputs);

                    param_names.insert(p_name.clone());
                }

                let mut outputs = vec![];
                for output_expr in output_exprs {
                    let l_type = match output_expr {
                        SExpr::Atom(atom_text) => atom_text,
                        _ => return Err(format!("Atom expected, list found")),
                    };

                    let value_type = LoleValueType::parse(l_type, &ctx)?;
                    emit_value_components(&value_type, &ctx, &mut outputs);
                }

                let type_index = module.types.len() as u32;
                let fn_index =  ctx.imported_fns_count;

                ctx.imported_fns_count += 1;
                ctx.fn_defs.insert(fn_name.clone(), FnDef {
                    local: false,
                    fn_index,
                });
                module.types.push(WasmFnType { inputs, outputs });
                module.imports.push(WasmImport {
                    module_name: module_name.clone(),
                    item_name: extern_fn_name.clone(),
                    item_desc: WasmImportDesc::Func { type_index },
                });
            }
            _ => return Err(format!("Invalid arguments for {op}")),
        },
        "export" => match other {
            [SExpr::Atom(mem_literal), SExpr::Atom(in_name), SExpr::Atom(as_literal), SExpr::Atom(out_name)]
                if mem_literal == "mem" && as_literal == ":as" =>
            {
                if !ctx.memory_names.contains(in_name) {
                    return Err(format!("Cannot export mem {in_name}, not found"));
                }

                module.exports.push(WasmExport {
                    export_type: WasmExportType::Mem,
                    export_name: out_name.clone(),
                    exported_item_index: ctx.memory_names.iter().position(|n| n == in_name).unwrap()
                        as u32,
                });
            }
            [SExpr::Atom(in_name), SExpr::Atom(as_literal), SExpr::Atom(out_name)]
                if as_literal == ":as" =>
            {
                ctx.fn_exports.insert(in_name.clone(), out_name.clone());
            }
            _ => return Err(format!("Invalid arguments for {op}")),
        },
        "struct" => match other {
            [SExpr::Atom(struct_name), field_defs @ ..] => {
                if ctx.struct_defs.contains_key(struct_name) {
                    return Err(format!("Cannot redefine struct {struct_name}"));
                }

                let fields = parse_struct_field_defs(field_defs, struct_name)?;

                ctx.struct_defs
                    .insert(struct_name.clone(), StructDef { fields });
            }
            _ => return Err(format!("Invalid arguments for {op}")),
        },
        "enum" => match other {
            [SExpr::Atom(enum_name), variants @ ..] => {
                for (kind, variant) in variants.iter().enumerate() {
                    let SExpr::List(variant_body) = variant else {
                        return Err(format!("Invalid arguments for {op}"));
                    };

                    let [SExpr::Atom(struct_name), field_defs @ ..] = &variant_body[..] else {
                        return Err(format!("Invalid arguments for {op}"));
                    };

                    let full_name = format!("{enum_name}/{struct_name}");
                    if ctx.struct_defs.contains_key(struct_name) {
                        return Err(format!("Cannot redefine struct {full_name}"));
                    }

                    let fields = parse_struct_field_defs(field_defs, &full_name)?;

                    ctx.struct_defs
                        .insert(full_name.clone(), StructDef { fields });

                    ctx.enum_kinds.insert(full_name, kind as u32);
                }

                ctx.struct_defs.insert(
                    enum_name.clone(),
                    StructDef {
                        fields: vec![
                            StructField {
                                name: String::from("kind"),
                                value_type: WasmValueType::I32,
                            },
                            StructField {
                                name: String::from("ref"),
                                value_type: WasmValueType::I32,
                            },
                        ],
                    },
                );
            }
            _ => return Err(format!("Invalid arguments for {op}")),
        },
        "global" => {
            let (mutable, global_name, global_type, global_value) = match other {
                [SExpr::Atom(mutable_literal), SExpr::Atom(global_name), SExpr::Atom(global_type), global_value]
                    if mutable_literal == "mut" =>
                {
                    (true, global_name, global_type, global_value)
                }
                [SExpr::Atom(global_name), SExpr::Atom(global_type), global_value] => {
                    (false, global_name, global_type, global_value)
                }
                _ => return Err(format!("Invalid arguments for {op}")),
            };

            if ctx.globals.contains_key(global_name) {
                return Err(format!("Cannot redefine global: {global_name}"));
            }

            let value_type = WasmValueType::parse(global_type)?;
            let initial_value = WasmExpr {
                instrs: vec![parse_const_instr(global_value, &ctx)?],
            };

            ctx.globals.insert(
                global_name.clone(),
                GlobalDef {
                    index: ctx.globals.len() as u32,
                    mutable,
                },
            );

            module.globals.push(WasmGlobal {
                kind: WasmGlobalKind {
                    value_type,
                    mutable,
                },
                initial_value,
            });
        }
        _ => return Err(format!("Unknown operation: {op}")),
    }

    Ok(())
}

fn parse_struct_field_defs(exprs: &[SExpr], struct_name: &str) -> Result<Vec<StructField>, String> {
    let mut fields = Vec::<StructField>::new();
    for field_def in exprs {
        let SExpr::List(name_and_type) = field_def else {
            return Err(format!("Unexpected atom in fields list of struct {struct_name}"));
        };

        let [SExpr::Atom(f_name), SExpr::Atom(f_type)] = &name_and_type[..] else {
            return Err(format!("Expected name and parameter pairs in fields list of struct {struct_name}"));
        };

        if fields.iter().find(|f| &f.name == f_name).is_some() {
            return Err(format!(
                "Found duplicate struct field name: '{f_name}' of struct {struct_name}"
            ));
        }

        fields.push(StructField {
            name: f_name.clone(),
            value_type: WasmValueType::parse(f_type)?,
        });
    }
    Ok(fields)
}

fn emit_value_components(
    value_type: &LoleValueType,
    ctx: &ModuleContext,
    components: &mut Vec<WasmValueType>,
) -> u32 {
    match value_type {
        LoleValueType::Primitive(value_type) => {
            components.push(*value_type);
            1
        }
        LoleValueType::StructInstance { name } => {
            let fields = &ctx.struct_defs.get(name).unwrap().fields;

            for field in fields {
                components.push(field.value_type);
            }

            fields.len() as u32
        }
    }
}

fn parse_instr(expr: &SExpr, ctx: &mut FnContext) -> Result<WasmInstr, String> {
    let items = match expr {
        SExpr::List(items) => items,
        SExpr::Atom(var_name) => {
            if var_name.chars().all(|c| c.is_ascii_digit()) {
                return Ok(WasmInstr::I32Const(
                    var_name
                        .parse()
                        .map_err(|_| format!("Parsing i32 (implicit) failed"))?,
                ));
            }

            if let Some(global) = ctx.module.globals.get(var_name.as_str()) {
                return Ok(WasmInstr::GlobalGet(global.index));
            };

            let Some(local) = ctx.locals.get(var_name.as_str()) else {
                return Err(format!("Reading unknown variable: {var_name}"));
            };

            if let LoleValueType::StructInstance { name } = &local.value_type {
                let struct_def = ctx.module.struct_defs.get(name).unwrap();

                let mut values = vec![];

                for field_offset in 0..struct_def.fields.len() {
                    values.push(WasmInstr::LocalGet(local.index + field_offset as u32));
                }

                return Ok(WasmInstr::MultiValueEmit { values });
            }

            return Ok(WasmInstr::LocalGet(local.index));
        }
    };

    let [SExpr::Atom(op), args @ ..] = &items[..] else {
        return Err(format!("Expected operation, got a simple list"));
    };

    let instr = match (op.as_str(), &args[..]) {
        ("i32", [SExpr::Atom(value)]) => {
            WasmInstr::I32Const(value.parse().map_err(|_| format!("Parsing i32 failed"))?)
        }
        ("i32.eq" | "==", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Equals,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.ne" | "!=", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32NotEqual,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.not" | "not" | "!", [lhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Equals,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(WasmInstr::I32Const(0)),
        },
        ("i32.lt_s" | "<", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32LessThenSigned,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.ge_s" | ">=", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32GreaterEqualSigned,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.and" | "&&", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32And,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.or" | "||", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Or,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.add" | "+", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Add,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.sub" | "-", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Sub,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.mul" | "*", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Mul,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("if", [SExpr::Atom(block_type), cond, then_branch, else_branch]) => WasmInstr::If {
            block_type: WasmValueType::parse(block_type)?,
            cond: Box::new(parse_instr(cond, ctx)?),
            then_branch: Box::new(parse_instr(then_branch, ctx)?),
            else_branch: Box::new(parse_instr(else_branch, ctx)?),
        },
        ("if", [cond, then_branch]) => WasmInstr::IfSingleBranch {
            cond: Box::new(parse_instr(cond, ctx)?),
            then_branch: Box::new(parse_instr(then_branch, ctx)?),
        },
        ("loop", [SExpr::List(exprs)]) => WasmInstr::Loop {
            instrs: parse_instrs(exprs, ctx)?,
        },
        ("break", []) => WasmInstr::LoopBreak,
        ("continue", []) => WasmInstr::LoopContinue,
        ("return", values) => WasmInstr::Return {
            value: Box::new(WasmInstr::MultiValueEmit {
                values: parse_instrs(values, ctx)?,
            }),
        },
        // TODO: support custom aligns and offsets
        ("store", [SExpr::Atom(store_kind), address_expr, value_expr]) => {
            let Some(struct_def) = ctx.module.struct_defs.get(store_kind) else {
                return Ok(WasmInstr::Store {
                    kind: WasmStoreKind::parse(store_kind)?,
                    align: 0,
                    offset: 0,
                    value_instr: Box::new(parse_instr(value_expr, ctx)?),
                    address_instr: Rc::new(parse_instr(address_expr, ctx)?),
                })
            };

            let mut value_instrs = match parse_instr(value_expr, ctx)? {
                WasmInstr::MultiValueEmit { values } => values,
                instr => vec![instr],
            };

            // TODO: add better check if type inference is implemented
            if value_instrs.len() != struct_def.fields.len() {
                return Err(format!(
                    "Invalid number of receiving variables for {op}, needed {}, got {}",
                    struct_def.fields.len(),
                    value_instrs.len()
                ));
            }

            let address_instr = Rc::new(parse_instr(address_expr, ctx)?);

            let mut offset = 0;
            let mut instrs = Vec::<WasmInstr>::with_capacity(struct_def.fields.len());

            for field in struct_def.fields.iter() {
                instrs.push(WasmInstr::Store {
                    kind: WasmStoreKind::from_value_type(&field.value_type)?,
                    align: 1,
                    offset,
                    value_instr: Box::new(value_instrs.remove(0)),
                    address_instr: address_instr.clone(),
                });

                offset += field.value_type.byte_size();
            }

            WasmInstr::MultiValueEmit { values: instrs }
        }
        // TODO: support custom aligns and offsets
        ("load", [SExpr::Atom(load_kind), address_expr]) => {
            let Some(struct_def) = ctx.module.struct_defs.get(load_kind) else {
                return Ok(WasmInstr::Load {
                    kind: WasmLoadKind::parse(load_kind)?,
                    align: 0,
                    offset: 0,
                    address_instr: Rc::new(parse_instr(address_expr, ctx)?),
                })
            };

            let address_instr = Rc::new(parse_instr(address_expr, ctx)?);

            let mut offset = 0;
            let mut primitive_loads = Vec::<WasmInstr>::with_capacity(struct_def.fields.len());

            for field in &struct_def.fields {
                primitive_loads.push(WasmInstr::Load {
                    kind: WasmLoadKind::from_value_type(&field.value_type)?,
                    align: 1,
                    offset,
                    address_instr: address_instr.clone(),
                });

                offset += field.value_type.byte_size();
            }

            WasmInstr::MultiValueEmit {
                values: primitive_loads,
            }
        }
        /*
            TODO: validate that number values matches the one of fields
            WARNING: it's not that simple because it involves nested `MultiValueEmit`s
        */
        ("struct.new", [SExpr::Atom(s_name), values @ ..]) => {
            if !ctx.module.struct_defs.contains_key(s_name) {
                return Err(format!("Unknown struct encountered in {op}: {s_name}"));
            }

            WasmInstr::MultiValueEmit {
                values: parse_instrs(values, ctx)?,
            }
        }
        ("enum.kind", [SExpr::Atom(enum_variant)]) => {
            let Some(kind) = ctx.module.enum_kinds.get(enum_variant) else {
                return Err(format!("Unknown enum variant in {op}: {enum_variant}"));
            };

            WasmInstr::I32Const(*kind as i32)
        }
        ("sizeof", [SExpr::Atom(type_name)]) => {
            let value_type = LoleValueType::parse(type_name, ctx.module)?;

            WasmInstr::I32Const(value_type.byte_size(ctx.module) as i32)
        }
        ("let" | ":", [SExpr::Atom(local_name), SExpr::Atom(local_type)]) => {
            if let Some(_) = ctx.module.globals.get(local_name.as_str()) {
                return Err(format!("Local name collides with global: {local_name}"));
            };

            if ctx.locals.contains_key(local_name) {
                return Err(format!("Duplicate local definition: {local_name}"));
            }

            let value_type = LoleValueType::parse(local_type, ctx.module)?;
            let comp_count =
                emit_value_components(&value_type, &ctx.module, &mut ctx.non_arg_locals);

            ctx.locals.insert(
                local_name.clone(),
                LocalDef {
                    index: ctx.locals_last_index,
                    value_type,
                },
            );

            ctx.locals_last_index += comp_count;

            WasmInstr::NoInstr
        }
        ("set" | "=", [SExpr::Atom(var_name), value]) => {
            if let Some(global) = ctx.module.globals.get(var_name.as_str()) {
                if !global.mutable {
                    return Err(format!("Setting immutable global: {var_name}"));
                }

                return Ok(WasmInstr::GlobalSet {
                    global_index: global.index,
                    value: Box::new(parse_instr(value, ctx)?),
                });
            };

            let Some(local) = ctx.locals.get(var_name.as_str()) else {
                return Err(format!("Unknown variable: {var_name}"));
            };

            match &local.value_type {
                LoleValueType::StructInstance { name } => {
                    let struct_def = ctx.module.struct_defs.get(name).unwrap();

                    let mut local_indices = vec![];
                    for field_offset in 0..struct_def.fields.len() {
                        local_indices.push(local.index + field_offset as u32);
                    }

                    return Ok(WasmInstr::MultiValueLocalSet {
                        local_indices,
                        value: Box::new(parse_instr(value, ctx)?),
                    });
                }
                LoleValueType::Primitive(_) => WasmInstr::LocalSet {
                    local_index: local.index,
                    value: Box::new(parse_instr(value, ctx)?),
                },
            }
        }
        ("set" | "=", [SExpr::List(local_names), value]) => {
            let mut local_indices = vec![];

            for local_name_expr in local_names {
                let SExpr::Atom(local_name) = local_name_expr else {
                    return Err(format!("Unexpected list in lhs of set"));
                };

                if let Some(_) = ctx.module.globals.get(local_name.as_str()) {
                    return Err(format!(
                        "Cannot set globals in multivalue set: {local_name}"
                    ));
                };

                let Some(local) = ctx.locals.get(local_name.as_str()) else {
                    return Err(format!("Unknown location for set: {local_name}"));
                };

                match &local.value_type {
                    LoleValueType::StructInstance { name } => {
                        let struct_def = ctx.module.struct_defs.get(name).unwrap();

                        for field_offset in 0..struct_def.fields.len() {
                            local_indices.push(local.index + field_offset as u32);
                        }
                    }
                    LoleValueType::Primitive(_) => local_indices.push(local.index),
                }
            }

            WasmInstr::MultiValueLocalSet {
                local_indices,
                value: Box::new(parse_instr(value, ctx)?),
            }
        }
        ("set" | "=", [SExpr::Atom(local_name), SExpr::Atom(f_name), value]) => {
            if let Some(_) = ctx.module.globals.get(local_name.as_str()) {
                return Err(format!(
                    "Setting struct field from global variable: {local_name}"
                ));
            };

            let Some(local) = ctx.locals.get(local_name.as_str()) else {
                return Err(format!("Unknown location for {op}: {local_name}"));
            };

            let LoleValueType::StructInstance { name: s_name } = &local.value_type else {
                return Err(format!("Trying to set field '{f_name}' on non struct: {local_name}"));
            };

            let struct_def = match ctx.module.struct_defs.get(s_name) {
                Some(struct_def) => struct_def,
                None => return Err(format!("Unknown struct in {op}: {s_name}")),
            };

            let Some(field_offset) = struct_def.fields.iter().position(|f| f.name == *f_name) else {
                return Err(format!("Unknown field {f_name} in struct {s_name}"));
            };

            WasmInstr::LocalSet {
                local_index: local.index + field_offset as u32,
                value: Box::new(parse_instr(value, ctx)?),
            }
        }
        ("get" | ".", [SExpr::Atom(local_name), SExpr::Atom(f_name)]) => {
            if let Some(_) = ctx.module.globals.get(local_name.as_str()) {
                return Err(format!(
                    "Getting struct field from global variable: {local_name}"
                ));
            };

            let Some(local) = ctx.locals.get(local_name.as_str()) else {
                return Err(format!("Reading unknown variable: {local_name}"));
            };

            let LoleValueType::StructInstance { name: s_name } = &local.value_type else {
                return Err(format!("Trying to get field '{f_name}' on non struct: {local_name}"));
            };

            let struct_def = match ctx.module.struct_defs.get(s_name) {
                Some(struct_def) => struct_def,
                None => return Err(format!("Unknown struct in get: {s_name}")),
            };

            let Some(field_offset) = struct_def.fields.iter().position(|f| f.name == *f_name) else {
                return Err(format!("Unknown field {f_name} in struct {s_name}"));
            };

            WasmInstr::LocalGet(local.index + field_offset as u32)
        }
        ("pack" | "do", exprs) => WasmInstr::MultiValueEmit {
            values: parse_instrs(exprs, ctx)?,
        },
        (fn_name, args) => {
            let Some(fn_def) = ctx.module.fn_defs.get(fn_name) else {
                return Err(format!("Unknown instruction or function: {fn_name}"));
            };
            let fn_index = if fn_def.local {
                fn_def.fn_index + ctx.module.imported_fns_count
            } else {
                fn_def.fn_index
            };

            WasmInstr::Call {
                fn_index,
                args: parse_instrs(args, ctx)?,
            }
        }
    };

    Ok(instr)
}

fn parse_const_instr(expr: &SExpr, ctx: &ModuleContext) -> Result<WasmInstr, String> {
    let items = match expr {
        SExpr::List(items) => items,
        SExpr::Atom(global_name) => {
            if global_name.chars().all(|c| c.is_ascii_digit()) {
                return Ok(WasmInstr::I32Const(
                    global_name
                        .parse()
                        .map_err(|_| format!("Parsing i32 (implicit) failed"))?,
                ));
            }

            let Some(global) = ctx.globals.get(global_name.as_str()) else {
                return Err(format!("Unknown location for global.get: {global_name}"));
            };

            return Ok(WasmInstr::GlobalGet(global.index));
        }
    };

    let [SExpr::Atom(op), args @ ..] = &items[..] else {
        return Err(format!("Expected operation, got a simple list"));
    };

    let instr = match (op.as_str(), &args[..]) {
        ("i32", [SExpr::Atom(value)]) => {
            WasmInstr::I32Const(value.parse().map_err(|_| format!("Parsing i32 failed"))?)
        }
        (instr_name, _args) => {
            return Err(format!("Unknown instruction: {instr_name}"));
        }
    };

    Ok(instr)
}

fn parse_instrs(exprs: &[SExpr], ctx: &mut FnContext) -> Result<Vec<WasmInstr>, String> {
    exprs.iter().map(|expr| parse_instr(expr, ctx)).collect()
}

// types

impl LoleValueType {
    fn parse(l_type: &String, ctx: &ModuleContext) -> Result<Self, String> {
        if let Ok(value_type) = WasmValueType::parse(l_type) {
            return Ok(Self::Primitive(value_type));
        }

        if ctx.struct_defs.contains_key(l_type) {
            return Ok(Self::StructInstance {
                name: l_type.clone(),
            });
        }

        Err(format!("Unknown value type: {l_type}"))
    }

    fn byte_size(&self, ctx: &ModuleContext) -> u32 {
        match self {
            Self::Primitive(primitive) => primitive.byte_size(),
            Self::StructInstance { name } => {
                let struct_def = ctx.struct_defs.get(name).unwrap();

                let mut size = 0;
                for field in &struct_def.fields {
                    size += Self::Primitive(field.value_type).byte_size(ctx);
                }
                size
            }
        }
    }
}

impl WasmValueType {
    fn parse(name: &str) -> Result<Self, String> {
        match name {
            "bool" | "u8" | "u32" | "i32" | "ptr" => Ok(Self::I32),
            "i64" => Ok(Self::I64),
            "f32" => Ok(Self::F32),
            "f64" => Ok(Self::F64),
            "v128" => Ok(Self::V128),
            "funcref" => Ok(Self::FuncRef),
            "externref" => Ok(Self::ExternRef),
            _ => return Err(format!("Unknown value type: {name}")),
        }
    }

    fn byte_size(&self) -> u32 {
        match self {
            Self::FuncRef | Self::ExternRef => 4, // TODO: check this
            Self::I32 | Self::F32 => 4,
            Self::I64 | Self::F64 => 8,
            Self::V128 => 16,
        }
    }
}

impl WasmLoadKind {
    fn parse(kind: &str) -> Result<Self, String> {
        match kind {
            "i32" => Ok(Self::I32),
            "i32/u8" => Ok(Self::I32U8),
            _ => Err(format!("Unknown load kind: {kind}")),
        }
    }

    fn from_value_type(value_type: &WasmValueType) -> Result<Self, String> {
        match value_type {
            WasmValueType::I32 => Ok(Self::I32),
            _ => return Err(format!("Unsupported type for load: {value_type:?}")),
        }
    }
}

impl WasmStoreKind {
    fn parse(kind: &str) -> Result<Self, String> {
        match kind {
            "i32" => Ok(Self::I32),
            "i32/u8" => Ok(Self::I32U8),
            _ => return Err(format!("Unknown store kind: {kind}")),
        }
    }

    fn from_value_type(value_type: &WasmValueType) -> Result<Self, String> {
        match value_type {
            WasmValueType::I32 => Ok(Self::I32),
            _ => Err(format!("Unsupported type for store: {value_type:?}")),
        }
    }
}
