use crate::{
    parser::SExpr,
    wasm_module::{
        Export, ExportType, Expr, FnCode, FnType, Instr, Locals, Memory, ValueType, WasmModule,
    },
};
use alloc::{boxed::Box, collections::BTreeMap, format, string::String, vec, vec::Vec};

struct FnDef {
    fn_type: FnType,
    locals_and_args: BTreeMap<String, u32>,
    local_types: Vec<ValueType>,
    body: Vec<SExpr>,
}

pub fn compile_module(exprs: &Vec<SExpr>) -> Result<WasmModule, String> {
    let mut module = WasmModule::default();

    let mut fn_defs = BTreeMap::<String, FnDef>::new();
    let mut fn_exports = BTreeMap::<String, String>::new();
    let mut memory_names = Vec::<String>::new();

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
                if memory_names.contains(mem_name) {
                    return Err(format!("Duplicate memory definition: {mem_name}"));
                }
                memory_names.push(mem_name.clone());
                module.memories.push(Memory {
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
                if memory_names.contains(mem_name) {
                    return Err(format!("Duplicate memory definition: {mem_name}"));
                }
                memory_names.push(mem_name.clone());
                module.memories.push(Memory {
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
                [SExpr::Atom(name), SExpr::List(inputs), SExpr::List(outputs), SExpr::List(locals), SExpr::List(body)],
            ) => {
                if fn_defs.contains_key(name) {
                    return Err(format!("Cannot redefine function: {name}"));
                }

                let mut locals_and_args = BTreeMap::new();

                let mut input_types = vec![];
                for input in inputs.iter() {
                    let SExpr::List(name_and_type) = input else {
                        return Err(format!("Unexpected atom in function params list"));
                    };

                    let [SExpr::Atom(p_name), SExpr::Atom(p_type)] = &name_and_type[..] else {
                        return Err(format!("Expected name and parameter pairs in function params list"));
                    };

                    if locals_and_args.contains_key(p_name) {
                        return Err(format!(
                            "Found function param with conflicting name: {p_name}"
                        ));
                    }

                    locals_and_args.insert(p_name.clone(), locals_and_args.len() as u32);
                    input_types.push(parse_value_type(p_type)?);
                }

                let mut output_types = vec![];
                for input in outputs {
                    output_types.push(parse_value_type(get_atom_text(input)?)?);
                }

                let mut local_types = vec![];
                for local in locals.iter() {
                    let SExpr::List(name_and_type) = local else {
                        return Err(format!("Unexpected atom in function locals list"));
                    };

                    let [SExpr::Atom(l_name), SExpr::Atom(l_type)] = &name_and_type[..] else {
                        return Err(format!("Expected name and parameter pairs in function locals list"));
                    };

                    if locals_and_args.contains_key(l_name) {
                        return Err(format!(
                            "Found function local with conflicting name: {l_name}"
                        ));
                    }

                    locals_and_args.insert(l_name.clone(), locals_and_args.len() as u32);
                    local_types.push(parse_value_type(l_type)?);
                }

                fn_defs.insert(
                    name.clone(),
                    FnDef {
                        fn_type: FnType {
                            inputs: input_types,
                            outputs: output_types,
                        },
                        local_types,
                        locals_and_args,
                        body: body.clone(),
                    },
                );
            }
            (
                "export",
                [SExpr::Atom(mem_literal), SExpr::Atom(in_name), SExpr::Atom(as_literal), SExpr::Atom(out_name)],
            ) if mem_literal == "mem" && as_literal == ":as" => {
                if !memory_names.contains(in_name) {
                    return Err(format!("Cannot export mem {in_name}, not found"));
                }

                module.exports.push(Export {
                    export_type: ExportType::Mem,
                    export_name: out_name.clone(),
                    exported_item_index: memory_names.iter().position(|n| n == in_name).unwrap(),
                });
            }
            ("export", [SExpr::Atom(in_name), SExpr::Atom(as_literal), SExpr::Atom(out_name)])
                if as_literal == ":as" =>
            {
                fn_exports.insert(in_name.clone(), out_name.clone());
            }
            (op, _) => return Err(format!("Unknown operation: {op}")),
        };
    }

    // push function exports
    for (in_name, out_name) in fn_exports.into_iter() {
        module.exports.push(Export {
            export_type: ExportType::Func,
            export_name: out_name,
            exported_item_index: fn_defs
                .keys()
                .position(|fn_name| *fn_name == in_name)
                .ok_or_else(|| format!("Cannot export unknown function {in_name}"))?,
        });
    }

    // push function codes
    for fn_def in fn_defs.values() {
        let locals = fn_def
            .local_types
            .iter()
            .map(|l| Locals {
                count: 1,
                value_type: l.clone(),
            })
            .collect();

        module.fn_codes.push(FnCode {
            locals,
            expr: Expr {
                instrs: parse_instrs(&fn_def.body, &fn_def.locals_and_args, &fn_defs)?,
            },
        });
    }

    // push function types
    for fn_def in fn_defs.into_values() {
        module.fn_types.push(fn_def.fn_type);
    }

    Ok(module)
}

fn parse_instr(
    expr: &SExpr,
    locals: &BTreeMap<String, u32>,
    fn_defs: &BTreeMap<String, FnDef>,
) -> Result<Instr, String> {
    let items = match expr {
        SExpr::List(items) => items,
        SExpr::Atom(local_name) => {
            let Some(&idx) = locals.get(local_name.as_str()) else {
                return Err(format!("Unknown location for local.get: {local_name}"));
            };

            return Ok(Instr::LocalGet(idx));
        }
    };

    let [SExpr::Atom(op), args @ ..] = &items[..] else {
        return Err(format!("Expected operation, got a simple list"));
    };

    let instr = match (op.as_str(), &args[..]) {
        ("i32", [SExpr::Atom(value)]) => {
            Instr::I32Const(value.parse().map_err(|_| format!("Parsing i32 failed"))?)
        }
        ("i32.lt_s", [lhs, rhs]) => Instr::I32LessThenSigned {
            lhs: Box::new(parse_instr(lhs, locals, fn_defs)?),
            rhs: Box::new(parse_instr(rhs, locals, fn_defs)?),
        },
        ("i32.ge_s", [lhs, rhs]) => Instr::I32GreaterEqualSigned {
            lhs: Box::new(parse_instr(lhs, locals, fn_defs)?),
            rhs: Box::new(parse_instr(rhs, locals, fn_defs)?),
        },
        ("i32.ne", [lhs, rhs]) => Instr::I32NotEqual {
            lhs: Box::new(parse_instr(lhs, locals, fn_defs)?),
            rhs: Box::new(parse_instr(rhs, locals, fn_defs)?),
        },
        ("i32.add", [lhs, rhs]) => Instr::I32Add {
            lhs: Box::new(parse_instr(lhs, locals, fn_defs)?),
            rhs: Box::new(parse_instr(rhs, locals, fn_defs)?),
        },
        ("i32.sub", [lhs, rhs]) => Instr::I32Sub {
            lhs: Box::new(parse_instr(lhs, locals, fn_defs)?),
            rhs: Box::new(parse_instr(rhs, locals, fn_defs)?),
        },
        ("i32.mul", [lhs, rhs]) => Instr::I32Mul {
            lhs: Box::new(parse_instr(lhs, locals, fn_defs)?),
            rhs: Box::new(parse_instr(rhs, locals, fn_defs)?),
        },
        ("set", [SExpr::Atom(local_name), value]) => {
            let Some(&local_idx) = locals.get(local_name.as_str()) else {
                return Err(format!("Unknown location for set: {local_name}"));
            };

            Instr::LocalSet {
                local_idx,
                value: Box::new(parse_instr(value, locals, fn_defs)?),
            }
        }
        ("set", [SExpr::List(local_names), value]) => {
            let mut local_idxs = vec![];

            for local_name_expr in local_names {
                let SExpr::Atom(local_name) = local_name_expr else {
                    return Err(format!("Unexpected list in lhs of set"));
                };

                let Some(&local_idx) = locals.get(local_name.as_str()) else {
                    return Err(format!("Unknown location for set: {local_name}"));
                };

                local_idxs.push(local_idx);
            }

            Instr::MultiValueLocalSet {
                local_idxs,
                value: Box::new(parse_instr(value, locals, fn_defs)?),
            }
        }
        ("pack", exprs) => Instr::MultiValueEmit {
            values: parse_instrs(exprs, locals, fn_defs)?,
        },
        ("i32.load", [address]) => Instr::I32Load {
            align: 1,
            offset: 0,
            address_instr: Box::new(parse_instr(address, locals, fn_defs)?),
        },
        ("i32.load", [SExpr::Atom(align), SExpr::Atom(offset), address]) => Instr::I32Load {
            align: align
                .parse()
                .map_err(|_| format!("Parsing i32.load align failed"))?,
            offset: offset
                .parse()
                .map_err(|_| format!("Parsing i32.load offset failed"))?,
            address_instr: Box::new(parse_instr(address, locals, fn_defs)?),
        },
        ("i32.load8_u", [address]) => Instr::I32Load8Unsigned {
            align: 0,
            offset: 0,
            address_instr: Box::new(parse_instr(address, locals, fn_defs)?),
        },
        ("if", [SExpr::Atom(block_type), cond, then_branch, else_branch]) => Instr::If {
            block_type: parse_value_type(block_type)?,
            cond: Box::new(parse_instr(cond, locals, fn_defs)?),
            then_branch: Box::new(parse_instr(then_branch, locals, fn_defs)?),
            else_branch: Box::new(parse_instr(else_branch, locals, fn_defs)?),
        },
        ("if", [cond, then_branch]) => Instr::IfSingleBranch {
            cond: Box::new(parse_instr(cond, locals, fn_defs)?),
            then_branch: Box::new(parse_instr(then_branch, locals, fn_defs)?),
        },
        ("loop", [SExpr::List(exprs)]) => Instr::Loop {
            instrs: parse_instrs(exprs, locals, fn_defs)?,
        },
        ("break", []) => Instr::LoopBreak,
        ("continue", []) => Instr::LoopContinue,
        ("return", values) => Instr::Return {
            values: parse_instrs(values, locals, fn_defs)?,
        },
        (fn_name, args) => {
            let Some(fn_idx) = fn_defs.keys().position(|k| k == fn_name) else {
                return Err(format!("Unknown instuction or function: {fn_name}"));
            };

            Instr::Call {
                fn_idx: fn_idx as u32,
                args: parse_instrs(args, locals, fn_defs)?,
            }
        }
    };

    Ok(instr)
}

fn parse_instrs(
    exprs: &[SExpr],
    locals: &BTreeMap<String, u32>,
    fn_defs: &BTreeMap<String, FnDef>,
) -> Result<Vec<Instr>, String> {
    exprs
        .iter()
        .map(|expr| parse_instr(expr, locals, fn_defs))
        .collect()
}

fn parse_value_type(name: &str) -> Result<ValueType, String> {
    match name {
        "i32" => Ok(ValueType::I32),
        "i64" => Ok(ValueType::I64),
        "f32" => Ok(ValueType::F32),
        "f64" => Ok(ValueType::F64),
        "v128" => Ok(ValueType::V128),
        "funcref" => Ok(ValueType::FuncRef),
        "externref" => Ok(ValueType::ExternRef),
        _ => return Err(format!("Unknown value type: {name}")),
    }
}

fn get_atom_text(expr: &SExpr) -> Result<&str, String> {
    match expr {
        SExpr::Atom(atom_text) => Ok(&atom_text),
        _ => return Err(format!("Atom expected, list found")),
    }
}
