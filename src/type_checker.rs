use crate::wasm_module::{WasmInstr, WasmValueType};
use alloc::{vec, vec::Vec};

pub fn get_type(instr: &WasmInstr) -> Vec<WasmValueType> {
    match instr {
        WasmInstr::Store { .. } => vec![],
        WasmInstr::NoInstr { .. } => vec![],
        WasmInstr::Return { .. } => vec![],
        WasmInstr::LocalSet { .. } => vec![],
        WasmInstr::GlobalSet { .. } => vec![],
        WasmInstr::MultiValueLocalSet { .. } => vec![],
        WasmInstr::LoopBreak { .. } => vec![],
        WasmInstr::LoopContinue { .. } => vec![],
        WasmInstr::IfSingleBranch { .. } => vec![],

        WasmInstr::I32Const { .. } => vec![WasmValueType::I32],
        WasmInstr::BinaryOp { lhs, .. } => get_type(lhs),
        WasmInstr::If { block_type, .. } => vec![block_type.clone()],

        WasmInstr::MultiValueEmit { values, .. } => values.iter().map(get_type).flatten().collect(),

        // TODO: implement
        // WasmInstr::Load {
        //     kind,
        //     align,
        //     offset,
        //     address_instr,
        //     loc,
        // } => todo!(),
        // WasmInstr::LocalGet { local_index, loc } => todo!(),
        // WasmInstr::GlobalGet { local_index, loc } => todo!(),
        // WasmInstr::Loop { instrs, loc } => todo!(),
        // WasmInstr::Call {
        //     fn_index,
        //     args,
        //     loc,
        // } => todo!(),
        _ => vec![],
    }
}
