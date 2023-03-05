use crate::{
    parser::SExpr,
    wasm_module::{Export, ExportType, Expr, FnCode, FnType, Instr, ValueType, WasmModule},
};
use alloc::{collections::BTreeMap, string::String, vec, vec::Vec};

pub fn compile_module(exprs: &Vec<SExpr>) -> WasmModule {
    let mut module = WasmModule::default();

    let mut fn_types = BTreeMap::<String, FnType>::new();
    let mut fn_codes = BTreeMap::<String, FnCode>::new();
    let mut fn_exports = BTreeMap::<String, String>::new();

    for expr in exprs {
        let SExpr::List(items) = expr else {
            panic!("Unexpected atom");
        };

        let [SExpr::Atom(op), other @ ..] = &items[..] else {
            panic!("Expected operation, got a simple list");
        };

        // TODO: cleanup
        match (op.as_str(), &other[..]) {
            ("::", [SExpr::Atom(name), SExpr::List(inputs), SExpr::List(outputs)]) => {
                if fn_types.contains_key(name) {
                    panic!("Cannot redefine function type: {name}");
                }

                fn_types.insert(
                    name.clone(),
                    FnType {
                        inputs: inputs
                            .iter()
                            .map(get_atom_text)
                            .map(parse_value_type)
                            .collect(),
                        outputs: outputs
                            .iter()
                            .map(get_atom_text)
                            .map(parse_value_type)
                            .collect(),
                    },
                );
            }
            ("fn", [SExpr::Atom(name), SExpr::List(params), SExpr::List(instrs)]) => {
                if fn_codes.contains_key(name) {
                    panic!("Cannot redefine function body: {name}");
                }

                let fn_args = params
                    .iter()
                    .enumerate()
                    .map(|(idx, input)| {
                        let SExpr::Atom(param_name) = input else {
                            panic!("Unexpected list is parameters");
                        };

                        (param_name.as_str(), idx as u32)
                    })
                    .collect::<BTreeMap<_, _>>();

                fn_codes.insert(
                    name.clone(),
                    FnCode {
                        locals: vec![], // TODO: implement
                        expr: Expr {
                            instrs: parse_instrs(instrs, &fn_args),
                        },
                    },
                );
            }
            ("export", [SExpr::Atom(in_name), SExpr::Atom(as_literal), SExpr::Atom(out_name)])
                if as_literal == ":as" =>
            {
                if !fn_types.contains_key(in_name) {
                    panic!("Cannot export {in_name}, export type is unknown");
                }

                fn_exports.insert(in_name.clone(), out_name.clone());
            }
            (op, _) => panic!("Unknown operation: {op}"),
        };
    }

    let fn_names = fn_types.keys().cloned().collect::<Vec<_>>();

    if !fn_codes.keys().eq(fn_names.iter()) {
        // TODO: better error message?
        panic!("Function types and codes do not match");
    }

    for (fn_index, fn_name) in fn_names.iter().enumerate() {
        module.fn_types.push(fn_types.remove(fn_name).unwrap());
        module.fn_codes.push(fn_codes.remove(fn_name).unwrap());

        if let Some(export_name) = fn_exports.remove(fn_name) {
            module.exports.push(Export {
                export_type: ExportType::Func,
                export_name,
                exported_item_index: fn_index,
            });
        }
    }

    module
}

fn parse_instrs(exprs: &Vec<SExpr>, fn_args: &BTreeMap<&str, u32>) -> Vec<Instr> {
    let mut instrs = vec![];

    for expr in exprs {
        let SExpr::List(items) = expr else {
            panic!("Unexpected atom");
        };

        let [SExpr::Atom(op), args @ ..] = &items[..] else {
            panic!("Expected operation, got a simple list");
        };

        let instr = match (op.as_str(), &args[..]) {
            ("i32.const", [SExpr::Atom(value)]) => Instr::I32Const(value.parse().unwrap()),
            ("local.get", [SExpr::Atom(local_name)]) => {
                let Some(&idx) = fn_args.get(local_name.as_str()) else {
                    panic!("Unknown location for local.get: {local_name}");
                };

                Instr::LocalGet(idx)
            }
            ("return", []) => Instr::Return,
            (instr, _) => panic!("Unkown instuction: {instr}"),
        };

        instrs.push(instr);
    }

    instrs
}

fn parse_value_type(name: &str) -> ValueType {
    match name {
        "i32" => ValueType::I32,
        "i64" => ValueType::I64,
        "f32" => ValueType::F32,
        "f64" => ValueType::F64,
        "v128" => ValueType::V128,
        "funcref" => ValueType::FuncRef,
        "externref" => ValueType::ExternRef,
        _ => panic!("Unknown value type: {name}"),
    }
}

fn get_atom_text(expr: &SExpr) -> &str {
    match expr {
        SExpr::Atom(atom_text) => &atom_text,
        _ => panic!("Atom expected, list found"),
    }
}
