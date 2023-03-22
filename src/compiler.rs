use crate::{
    parser::SExpr,
    wasm_module::{
        Expr, Instr, WasmExport, WasmExportType, WasmFnCode, WasmFnType, WasmLocals, WasmMemory,
        WasmModule, WasmValueType,
    },
};
use alloc::{boxed::Box, collections::BTreeMap, format, string::String, vec, vec::Vec};
use core::cell::RefCell;

struct FnDef {
    fn_type: WasmFnType,
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

enum ValueType {
    Primitive(WasmValueType),
    StructInstance { name: String },
}

#[derive(Default)]
struct ModuleContext {
    fn_defs: BTreeMap<String, FnDef>,
    fn_exports: BTreeMap<String, String>,
    memory_names: Vec<String>,
    struct_defs: BTreeMap<String, StructDef>,
}

struct FnContext<'a> {
    module: &'a ModuleContext,
    locals: &'a mut BTreeMap<String, LocalDef>,
    locals_last_index: u32,
    non_arg_locals: Vec<WasmValueType>,
}

struct LocalDef {
    index: u32,
    value_type: ValueType,
}

pub fn compile_module(exprs: &Vec<SExpr>) -> Result<WasmModule, String> {
    let mut module = WasmModule::default();

    let mut ctx = ModuleContext::default();

    for expr in exprs {
        let SExpr::List(items) = expr else {
            return Err(format!("Unexpected atom"));
        };

        let [SExpr::Atom(op), other @ ..] = &items[..] else {
            return Err(format!("Expected operation, got a simple list"));
        };

        // TODO: cleanup
        match (op.as_str(), other) {
            ("mem", [SExpr::Atom(mem_name), SExpr::Atom(min_literal), SExpr::Atom(min_memory)])
                if min_literal == ":min" =>
            {
                if ctx.memory_names.contains(mem_name) {
                    return Err(format!("Duplicate memory definition: {mem_name}"));
                }
                ctx.memory_names.push(mem_name.clone());
                module.memories.push(WasmMemory {
                    min: min_memory
                        .parse()
                        .map_err(|_| format!("Parsing mem :min (u32) failed"))?,
                    max: None,
                });
            }
            (
                "mem",
                [SExpr::Atom(mem_name), SExpr::Atom(min_literal), SExpr::Atom(min_memory), SExpr::Atom(max_literal), SExpr::Atom(max_memory)],
            ) if min_literal == ":min" && max_literal == ":max" => {
                if ctx.memory_names.contains(mem_name) {
                    return Err(format!("Duplicate memory definition: {mem_name}"));
                }

                ctx.memory_names.push(mem_name.clone());
                module.memories.push(WasmMemory {
                    min: min_memory
                        .parse()
                        .map_err(|_| format!("Parsing mem :min (u32) failed"))?,
                    max: Some(
                        max_memory
                            .parse()
                            .map_err(|_| format!("Parsing mem :max (u32) failed"))?,
                    ),
                });
            }
            (
                "fn",
                [SExpr::Atom(name), SExpr::List(input_exprs), SExpr::List(output_exprs), SExpr::List(body)],
            ) => {
                if ctx.fn_defs.contains_key(name) {
                    return Err(format!("Cannot redefine function: {name}"));
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

                    let value_type = build_value_type(p_type, &ctx)?;
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

                    let value_type = build_value_type(l_type, &ctx)?;
                    emit_value_components(&value_type, &ctx, &mut outputs);
                }

                ctx.fn_defs.insert(
                    name.clone(),
                    FnDef {
                        fn_type: WasmFnType { inputs, outputs },
                        locals: RefCell::new(locals),
                        locals_last_index,
                        body: body.clone(),
                    },
                );
            }
            (
                "export",
                [SExpr::Atom(mem_literal), SExpr::Atom(in_name), SExpr::Atom(as_literal), SExpr::Atom(out_name)],
            ) if mem_literal == "mem" && as_literal == ":as" => {
                if !ctx.memory_names.contains(in_name) {
                    return Err(format!("Cannot export mem {in_name}, not found"));
                }

                module.exports.push(WasmExport {
                    export_type: WasmExportType::Mem,
                    export_name: out_name.clone(),
                    exported_item_index: ctx
                        .memory_names
                        .iter()
                        .position(|n| n == in_name)
                        .unwrap(),
                });
            }
            ("export", [SExpr::Atom(in_name), SExpr::Atom(as_literal), SExpr::Atom(out_name)])
                if as_literal == ":as" =>
            {
                ctx.fn_exports.insert(in_name.clone(), out_name.clone());
            }
            ("struct", [SExpr::Atom(s_name), field_defs @ ..]) => {
                if ctx.struct_defs.contains_key(s_name) {
                    return Err(format!("Cannot redefine struct {s_name}"));
                }

                let mut fields = Vec::<StructField>::new();
                for field_def in field_defs {
                    let SExpr::List(name_and_type) = field_def else {
                        return Err(format!("Unexpected atom in stuct fields list"));
                    };

                    let [SExpr::Atom(f_name), SExpr::Atom(f_type)] = &name_and_type[..] else {
                        return Err(format!("Expected name and parameter pairs in stuct fields list"));
                    };

                    if fields.iter().find(|f| &f.name == f_name).is_some() {
                        return Err(format!(
                            "Found duplicate struct field name: '{f_name}' of struct {s_name}"
                        ));
                    }

                    fields.push(StructField {
                        name: f_name.clone(),
                        value_type: parse_value_type(f_type)?,
                    });
                }

                ctx.struct_defs.insert(s_name.clone(), StructDef { fields });
            }
            (op, _) => return Err(format!("Unknown operation: {op}")),
        };
    }

    // push function exports
    for (in_name, out_name) in ctx.fn_exports.iter() {
        module.exports.push(WasmExport {
            export_type: WasmExportType::Func,
            export_name: out_name.clone(),
            exported_item_index: ctx
                .fn_defs
                .keys()
                .position(|fn_name| fn_name == in_name)
                .ok_or_else(|| format!("Cannot export unknown function {in_name}"))?,
        });
    }

    // push function codes
    for fn_meta in ctx.fn_defs.values() {
        let mut fn_ctx = FnContext {
            module: &ctx,
            locals: &mut fn_meta.locals.borrow_mut(),
            locals_last_index: fn_meta.locals_last_index,
            non_arg_locals: vec![],
        };

        let instrs = parse_instrs(&fn_meta.body, &mut fn_ctx)?;

        let mut locals = vec![];
        for local_type in fn_ctx.non_arg_locals {
            locals.push(WasmLocals {
                count: 1,
                value_type: local_type.clone(),
            });
        }

        module.fn_codes.push(WasmFnCode {
            locals,
            expr: Expr { instrs },
        });
    }

    // push function types
    for fn_def in ctx.fn_defs.into_values() {
        module.fn_types.push(fn_def.fn_type);
    }

    Ok(module)
}

fn build_value_type(l_type: &String, ctx: &ModuleContext) -> Result<ValueType, String> {
    if let Ok(value_type) = parse_value_type(l_type) {
        return Ok(ValueType::Primitive(value_type));
    }

    if ctx.struct_defs.contains_key(l_type) {
        return Ok(ValueType::StructInstance {
            name: l_type.clone(),
        });
    }

    Err(format!("Unknown value type: {l_type}"))
}

fn emit_value_components(
    value_type: &ValueType,
    ctx: &ModuleContext,
    components: &mut Vec<WasmValueType>,
) -> u32 {
    match value_type {
        ValueType::Primitive(value_type) => {
            components.push(*value_type);
            1
        }
        ValueType::StructInstance { name } => {
            let fields = &ctx.struct_defs.get(name).unwrap().fields;

            for field in fields {
                components.push(field.value_type);
            }

            fields.len() as u32
        }
    }
}

fn parse_instr(expr: &SExpr, ctx: &mut FnContext) -> Result<Instr, String> {
    let items = match expr {
        SExpr::List(items) => items,
        SExpr::Atom(local_name) => {
            if local_name.chars().all(|c| c.is_ascii_digit()) {
                return Ok(Instr::I32Const(
                    local_name
                        .parse()
                        .map_err(|_| format!("Parsing i32 (implicit) failed"))?,
                ));
            }

            let Some(local) = ctx.locals.get(local_name.as_str()) else {
                return Err(format!("Unknown location for local.get: {local_name}"));
            };

            if let ValueType::StructInstance { name } = &local.value_type {
                let struct_def = ctx.module.struct_defs.get(name).unwrap();

                let mut values = vec![];

                for field_offset in 0..struct_def.fields.len() {
                    values.push(Instr::LocalGet(local.index + field_offset as u32));
                }

                return Ok(Instr::MultiValueEmit { values });
            }

            return Ok(Instr::LocalGet(local.index));
        }
    };

    let [SExpr::Atom(op), args @ ..] = &items[..] else {
        return Err(format!("Expected operation, got a simple list"));
    };

    let instr = match (op.as_str(), &args[..]) {
        ("i32", [SExpr::Atom(value)]) => {
            Instr::I32Const(value.parse().map_err(|_| format!("Parsing i32 failed"))?)
        }
        ("i32.lt_s" | "<", [lhs, rhs]) => Instr::I32LessThenSigned {
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.ge_s" | ">=", [lhs, rhs]) => Instr::I32GreaterEqualSigned {
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.ne" | "!=", [lhs, rhs]) => Instr::I32NotEqual {
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.add" | "+", [lhs, rhs]) => Instr::I32Add {
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.sub" | "-", [lhs, rhs]) => Instr::I32Sub {
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.mul" | "*", [lhs, rhs]) => Instr::I32Mul {
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("set" | "=", [SExpr::Atom(local_name), value]) => {
            let Some(local) = ctx.locals.get(local_name.as_str()) else {
                return Err(format!("Unknown location for set: {local_name}"));
            };

            match &local.value_type {
                ValueType::StructInstance { name } => {
                    let struct_def = ctx.module.struct_defs.get(name).unwrap();

                    let mut local_idxs = vec![];
                    for field_offset in 0..struct_def.fields.len() {
                        local_idxs.push(local.index + field_offset as u32);
                    }

                    return Ok(Instr::MultiValueLocalSet {
                        local_idxs,
                        value: Box::new(parse_instr(value, ctx)?),
                    });
                }
                ValueType::Primitive(_) => Instr::LocalSet {
                    local_idx: local.index,
                    value: Box::new(parse_instr(value, ctx)?),
                },
            }
        }
        ("set" | "=", [SExpr::List(local_names), value]) => {
            let mut local_idxs = vec![];

            for local_name_expr in local_names {
                let SExpr::Atom(local_name) = local_name_expr else {
                    return Err(format!("Unexpected list in lhs of set"));
                };

                let Some(local) = ctx.locals.get(local_name.as_str()) else {
                    return Err(format!("Unknown location for set: {local_name}"));
                };

                match &local.value_type {
                    ValueType::StructInstance { name } => {
                        let struct_def = ctx.module.struct_defs.get(name).unwrap();

                        for field_offset in 0..struct_def.fields.len() {
                            local_idxs.push(local.index + field_offset as u32);
                        }
                    }
                    ValueType::Primitive(_) => local_idxs.push(local.index),
                }
            }

            Instr::MultiValueLocalSet {
                local_idxs,
                value: Box::new(parse_instr(value, ctx)?),
            }
        }
        ("let" | ":", [SExpr::Atom(local_name), SExpr::Atom(local_type)]) => {
            if ctx.locals.contains_key(local_name) {
                return Err(format!("Duplicate local definition: {local_name}"));
            }

            let value_type = build_value_type(local_type, ctx.module)?;
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

            // TODO: find a better way
            Instr::Nop
        }
        // TODO: validate that number of fields matches
        ("new", [SExpr::Atom(s_name), values @ ..]) => {
            if !ctx.module.struct_defs.contains_key(s_name) {
                return Err(format!("Unknown struct encountered in set: {s_name}"));
            }

            Instr::MultiValueEmit {
                values: parse_instrs(values, ctx)?,
            }
        }
        ("get" | ".", [SExpr::Atom(local_name), SExpr::Atom(f_name)]) => {
            let Some(local) = ctx.locals.get(local_name.as_str()) else {
                return Err(format!("Unknown location for get: {local_name}"));
            };

            let ValueType::StructInstance { name: s_name } = &local.value_type else {
                return Err(format!("Trying to get field '{f_name}' on non struct: {local_name}"));
            };

            let struct_def = match ctx.module.struct_defs.get(s_name) {
                Some(struct_def) => struct_def,
                None => return Err(format!("Unknown struct in get: {s_name}")),
            };

            let Some(field_offset) = struct_def.fields.iter().position(|f| f.name == *f_name) else {
                return Err(format!("Unknown field {f_name} in struct {s_name}"));
            };

            Instr::LocalGet(local.index + field_offset as u32)
        }
        ("set" | "=", [SExpr::Atom(local_name), SExpr::Atom(f_name), value]) => {
            let Some(local) = ctx.locals.get(local_name.as_str()) else {
                return Err(format!("Unknown location for set: {local_name}"));
            };

            let ValueType::StructInstance { name: s_name } = &local.value_type else {
                return Err(format!("Trying to set field '{f_name}' on non struct: {local_name}"));
            };

            let struct_def = match ctx.module.struct_defs.get(s_name) {
                Some(struct_def) => struct_def,
                None => return Err(format!("Unknown struct in set: {s_name}")),
            };

            let Some(field_offset) = struct_def.fields.iter().position(|f| f.name == *f_name) else {
                return Err(format!("Unknown field {f_name} in struct {s_name}"));
            };

            Instr::LocalSet {
                local_idx: local.index + field_offset as u32,
                value: Box::new(parse_instr(value, ctx)?),
            }
        }
        ("pack", exprs) => Instr::MultiValueEmit {
            values: parse_instrs(exprs, ctx)?,
        },
        ("i32.load", [address]) => Instr::I32Load {
            align: 1,
            offset: 0,
            address_instr: Box::new(parse_instr(address, ctx)?),
        },
        ("i32.load", [SExpr::Atom(align), SExpr::Atom(offset), address]) => Instr::I32Load {
            align: align
                .parse()
                .map_err(|_| format!("Parsing i32.load align failed"))?,
            offset: offset
                .parse()
                .map_err(|_| format!("Parsing i32.load offset failed"))?,
            address_instr: Box::new(parse_instr(address, ctx)?),
        },
        ("i32.load8_u", [address]) => Instr::I32Load8Unsigned {
            align: 0,
            offset: 0,
            address_instr: Box::new(parse_instr(address, ctx)?),
        },
        ("if", [SExpr::Atom(block_type), cond, then_branch, else_branch]) => Instr::If {
            block_type: parse_value_type(block_type)?,
            cond: Box::new(parse_instr(cond, ctx)?),
            then_branch: Box::new(parse_instr(then_branch, ctx)?),
            else_branch: Box::new(parse_instr(else_branch, ctx)?),
        },
        ("if", [cond, then_branch]) => Instr::IfSingleBranch {
            cond: Box::new(parse_instr(cond, ctx)?),
            then_branch: Box::new(parse_instr(then_branch, ctx)?),
        },
        ("loop", [SExpr::List(exprs)]) => Instr::Loop {
            instrs: parse_instrs(exprs, ctx)?,
        },
        ("break", []) => Instr::LoopBreak,
        ("continue", []) => Instr::LoopContinue,
        ("return", values) => Instr::Return {
            values: parse_instrs(values, ctx)?,
        },
        (fn_name, args) => {
            let Some(fn_idx) = ctx.module.fn_defs.keys().position(|k| k == fn_name) else {
                return Err(format!("Unknown instruction or function: {fn_name}"));
            };

            Instr::Call {
                fn_idx: fn_idx as u32,
                args: parse_instrs(args, ctx)?,
            }
        }
    };

    Ok(instr)
}

fn parse_instrs(exprs: &[SExpr], ctx: &mut FnContext) -> Result<Vec<Instr>, String> {
    exprs.iter().map(|expr| parse_instr(expr, ctx)).collect()
}

fn parse_value_type(name: &str) -> Result<WasmValueType, String> {
    match name {
        "i32" => Ok(WasmValueType::I32),
        "i64" => Ok(WasmValueType::I64),
        "f32" => Ok(WasmValueType::F32),
        "f64" => Ok(WasmValueType::F64),
        "v128" => Ok(WasmValueType::V128),
        "funcref" => Ok(WasmValueType::FuncRef),
        "externref" => Ok(WasmValueType::ExternRef),
        _ => return Err(format!("Unknown value type: {name}")),
    }
}
