use crate::{
    parser::SExpr,
    wasm_module::{Export, ExportType, Expr, FnCode, FnType, Instr, Memory, ValueType, WasmModule},
};
use alloc::{boxed::Box, collections::BTreeMap, format, string::String, vec, vec::Vec};

struct FnDef {
    fn_type: FnType,
    locals: Vec<SExpr>,
    args: BTreeMap<String, u32>,
    body: Vec<SExpr>,
}

pub fn compile_module(exprs: &Vec<SExpr>) -> Result<WasmModule, String> {
    let mut module = WasmModule::default();

    let mut fn_defs = BTreeMap::<String, FnDef>::new();
    let mut fn_exports = BTreeMap::<String, String>::new();
    let mut memory_names = Vec::<String>::new();

    for expr in exprs {
        let SExpr::List(items) = expr else {
            return Err(String::from("Unexpected atom"));
        };

        let [SExpr::Atom(op), other @ ..] = &items[..] else {
            return Err(String::from("Expected operation, got a simple list"));
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

                let mut fn_inputs = vec![];
                let mut fn_args = BTreeMap::new();

                for (idx, input) in inputs.iter().enumerate() {
                    let SExpr::List(name_and_type) = input else {
                        return Err(String::from("Unexpected atom in function params list"));
                    };

                    let [SExpr::Atom(p_name), SExpr::Atom(p_type)] = &name_and_type[..] else {
                        return Err(String::from("Expected name and parameter pairs in function params list"));
                    };

                    fn_args.insert(p_name.clone(), idx as u32);
                    fn_inputs.push(parse_value_type(p_type)?);
                }

                let mut fn_outputs = vec![];
                for input in outputs {
                    fn_outputs.push(parse_value_type(get_atom_text(input)?)?);
                }

                fn_defs.insert(
                    name.clone(),
                    FnDef {
                        fn_type: FnType {
                            inputs: fn_inputs,
                            outputs: fn_outputs,
                        },
                        args: fn_args,
                        locals: locals.clone(),
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
        let _ = fn_def.locals;

        module.fn_codes.push(FnCode {
            locals: vec![], // TODO: implement
            expr: Expr {
                instrs: parse_instrs(&fn_def.body, &fn_def.args, &fn_defs)?,
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
    fn_args: &BTreeMap<String, u32>,
    fn_defs: &BTreeMap<String, FnDef>,
) -> Result<Instr, String> {
    let items = match expr {
        SExpr::List(items) => items,
        SExpr::Atom(local_name) => {
            let Some(&idx) = fn_args.get(local_name.as_str()) else {
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
            lhs: Box::new(parse_instr(lhs, fn_args, fn_defs)?),
            rhs: Box::new(parse_instr(rhs, fn_args, fn_defs)?),
        },
        ("i32.ge_s", [lhs, rhs]) => Instr::I32GreaterEqualSigned {
            lhs: Box::new(parse_instr(lhs, fn_args, fn_defs)?),
            rhs: Box::new(parse_instr(rhs, fn_args, fn_defs)?),
        },
        ("i32.add", [lhs, rhs]) => Instr::I32Add {
            lhs: Box::new(parse_instr(lhs, fn_args, fn_defs)?),
            rhs: Box::new(parse_instr(rhs, fn_args, fn_defs)?),
        },
        ("i32.sub", [lhs, rhs]) => Instr::I32Sub {
            lhs: Box::new(parse_instr(lhs, fn_args, fn_defs)?),
            rhs: Box::new(parse_instr(rhs, fn_args, fn_defs)?),
        },
        ("i32.mul", [lhs, rhs]) => Instr::I32Mul {
            lhs: Box::new(parse_instr(lhs, fn_args, fn_defs)?),
            rhs: Box::new(parse_instr(rhs, fn_args, fn_defs)?),
        },
        ("i32.load", [address]) => Instr::I32Load {
            align: 1,
            offset: 0,
            address_instr: Box::new(parse_instr(address, fn_args, fn_defs)?),
        },
        ("i32.load", [SExpr::Atom(align), SExpr::Atom(offset), address]) => Instr::I32Load {
            align: align
                .parse()
                .map_err(|_| format!("Parsing i32.load align failed"))?,
            offset: offset
                .parse()
                .map_err(|_| format!("Parsing i32.load offset failed"))?,
            address_instr: Box::new(parse_instr(address, fn_args, fn_defs)?),
        },
        ("i32.load8_u", [address]) => Instr::I32Load8Unsigned {
            align: 0,
            offset: 0,
            address_instr: Box::new(parse_instr(address, fn_args, fn_defs)?),
        },
        ("if", [SExpr::Atom(block_type), cond, then_branch, else_branch]) => Instr::If {
            block_type: parse_value_type(block_type)?,
            cond: Box::new(parse_instr(cond, fn_args, fn_defs)?),
            then_branch: Box::new(parse_instr(then_branch, fn_args, fn_defs)?),
            else_branch: Box::new(parse_instr(else_branch, fn_args, fn_defs)?),
        },
        ("if", [cond, then_branch]) => Instr::IfSingleBranch {
            cond: Box::new(parse_instr(cond, fn_args, fn_defs)?),
            then_branch: Box::new(parse_instr(then_branch, fn_args, fn_defs)?),
        },
        ("return", values) => Instr::Return {
            values: parse_instrs(values, fn_args, fn_defs)?,
        },
        (fn_name, args) => {
            #[cfg(test)]
            dbg!(fn_defs.keys().collect::<Vec<_>>());

            let Some(fn_idx) = fn_defs.keys().position(|k| k == fn_name) else {
                return Err(format!("Unknown instuction or function: {fn_name}"));
            };

            Instr::Call {
                fn_idx: fn_idx as u32,
                args: parse_instrs(args, fn_args, fn_defs)?,
            }
        }
    };

    Ok(instr)
}

fn parse_instrs(
    exprs: &[SExpr],
    fn_args: &BTreeMap<String, u32>,
    fn_defs: &BTreeMap<String, FnDef>,
) -> Result<Vec<Instr>, String> {
    exprs
        .iter()
        .map(|expr| parse_instr(expr, fn_args, fn_defs))
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
