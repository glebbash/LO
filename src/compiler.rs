use alloc::{string::String, vec, vec::Vec};

use crate::{
    parser::SExpr,
    wasm_module::{Export, ExportType, Expr, FnCode, FnType, Instr, ValueType, WasmModule},
};

// TODO: use exprs
pub fn compile_module(_exprs: Vec<SExpr>) -> WasmModule {
    let mut module = WasmModule::default();

    module.fn_types.push(FnType {
        inputs: vec![],
        outputs: vec![ValueType::I32],
    });

    module.fn_codes.push(FnCode {
        locals: vec![],
        expr: Expr {
            instrs: vec![Instr::I32Const(42), Instr::Return],
        },
    });

    module.exports.push(Export {
        export_type: ExportType::Func,
        export_name: String::from("main"),
        exported_item_index: 0,
    });

    // let mut fn_names = BTreeMap::<u32, &str>::new();

    module
}
