use crate::{
    compiler::{emit_value_components, CompileError, FnContext},
    wasm_module::{WasmInstr, WasmValueType},
};
use alloc::{format, vec, vec::Vec};

pub fn get_type(ctx: &FnContext, instr: &WasmInstr) -> Result<Vec<WasmValueType>, CompileError> {
    Ok(match instr {
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
        WasmInstr::BinaryOp { lhs, .. } => return get_type(ctx, lhs),
        WasmInstr::If { block_type, .. } => vec![block_type.clone()],

        WasmInstr::MultiValueEmit { values, .. } => get_types(ctx, values)?,

        // TODO: implement
        // WasmInstr::Load {
        //     kind,
        //     align,
        //     offset,
        //     address_instr,
        //     loc,
        // } => todo!(),
        // WasmInstr::Loop { instrs, loc } => todo!(),
        // WasmInstr::GlobalGet { local_index, loc } => todo!(),
        // WasmInstr::LocalGet { local_index, .. } => {
        //     let (_, local_def) = ctx
        //         .locals
        //         .iter()
        //         .find(|(_, v)| v.index == *local_index)
        //         .expect(&format!("Unreachable in {}, {}", file!(), line!()));

        //     let mut local_types = vec![];
        //     let _ = emit_value_components(&local_def.value_type, ctx.module, &mut local_types);

        //     local_types
        // }
        // TODO: uncomment once tests are passing
        // WasmInstr::Call {
        //     fn_index,
        //     args,
        //     loc,
        // } => {
        //     let arg_types = get_types(ctx, args)?;

        //     let fn_type = ctx
        //         .module
        //         .wasm_module
        //         .types
        //         .get(*fn_index as usize)
        //         .expect(&format!("Unreachable in {}, {}", file!(), line!()));

        //     if fn_type.inputs.len() != arg_types.len() {
        //         let (fn_name, _) = ctx
        //             .module
        //             .fn_defs
        //             .iter()
        //             .find(|(_, fd)| fd.get_absolute_index(ctx.module) == *fn_index)
        //             .expect(&format!("Unreachable in {}, {}", file!(), line!()));

        //         return Err(CompileError {
        //             message: format!(
        //                 "TypeError: Mismatched arguments for function \
        //                     '{fn_name}', expected {inputs:?}, got {args:?}",
        //                 inputs = fn_type.inputs,
        //                 args = arg_types
        //             ),
        //             loc: loc.clone(),
        //         });
        //     }

        //     fn_type.outputs.clone()
        // }
        _ => vec![],
    })
}

fn get_types(ctx: &FnContext, instrs: &Vec<WasmInstr>) -> Result<Vec<WasmValueType>, CompileError> {
    instrs
        .iter()
        .map(|v| get_type(ctx, v))
        .collect::<Result<Vec<_>, _>>()
        .map(|ts| ts.into_iter().flatten().collect())
}
