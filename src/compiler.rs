use alloc::{collections::BTreeMap, string::String, vec, vec::Vec};

use crate::{
    parser::SExpr,
    wasm_module::{Export, ExportType, Expr, FnCode, FnType, Instr, ValueType, WasmModule},
};

pub fn compile_module(exprs: Vec<SExpr>) -> WasmModule {
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
            ("::", [SExpr::Atom(name), SExpr::List(_inputs), SExpr::Atom(_output)]) => {
                if fn_types.contains_key(name) {
                    panic!("Cannot redefine function type: {name}");
                }

                fn_types.insert(
                    name.clone(),
                    FnType {
                        inputs: vec![],                // TODO: implement
                        outputs: vec![ValueType::I32], // TODO: implement
                    },
                );
            }
            ("fn", [SExpr::Atom(name), SExpr::List(_inputs), SExpr::List(_instrs)]) => {
                if fn_codes.contains_key(name) {
                    panic!("Cannot redefine function body: {name}");
                }

                fn_codes.insert(
                    name.clone(),
                    FnCode {
                        locals: vec![], // TODO: implement
                        expr: Expr {
                            instrs: vec![Instr::I32Const(42), Instr::Return],
                        }, // TODO: implement
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

    // TODO: validate key matching in types and codes
    let fn_names = fn_types.keys().cloned().collect::<Vec<_>>();

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
