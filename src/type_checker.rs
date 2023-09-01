use crate::{ast::*, ir::*, wasm::*};
use alloc::{vec, vec::Vec};

pub fn get_types(ctx: &FnContext, instrs: &Vec<WasmInstr>) -> Result<Vec<WasmType>, CompileError> {
    let mut types = vec![];
    for instr in instrs {
        types.append(&mut get_type(ctx, instr)?);
    }
    Ok(types)
}

pub fn get_type(ctx: &FnContext, instr: &WasmInstr) -> Result<Vec<WasmType>, CompileError> {
    Ok(match instr {
        WasmInstr::Unreachable { .. } => vec![],
        WasmInstr::LoopBreak { .. } => vec![],
        WasmInstr::LoopContinue { .. } => vec![],
        WasmInstr::I32ConstLazy { .. } => vec![WasmType::I32],
        WasmInstr::I32Const { .. } => vec![WasmType::I32],
        WasmInstr::I64Const { .. } => vec![WasmType::I64],
        WasmInstr::MultiValueEmit { values, .. } => get_types(ctx, values)?,
        WasmInstr::StructLoad {
            primitive_loads, ..
        } => get_types(ctx, primitive_loads)?,
        WasmInstr::StructGet { primitive_gets, .. } => get_types(ctx, primitive_gets)?,
        WasmInstr::NoEmit { instr } => get_type(ctx, instr)?,

        // type-checked in the complier:
        WasmInstr::NoTypeCheck { .. } => vec![],
        WasmInstr::Set { .. } => vec![],
        WasmInstr::Drop { .. } => vec![],
        WasmInstr::Loop { .. } => vec![],
        WasmInstr::Return { .. } => vec![],
        WasmInstr::MemorySize { .. } => vec![WasmType::I32],
        WasmInstr::MemoryGrow { .. } => vec![WasmType::I32],

        WasmInstr::IfSingleBranch {
            cond, then_branch, ..
        } => {
            get_type(ctx, cond)?;
            get_type(ctx, &then_branch)?
        }
        WasmInstr::BinaryOp { lhs, rhs, .. } => {
            get_type(ctx, rhs)?;
            return get_type(ctx, lhs);
        }
        WasmInstr::Load {
            kind,
            address_instr,
            ..
        } => {
            get_type(ctx, &address_instr)?;
            vec![kind.get_value_type()]
        }
        WasmInstr::If {
            block_type,
            cond,
            then_branch,
            else_branch,
            ..
        } => {
            get_type(ctx, &cond)?;
            get_type(ctx, &then_branch)?;
            get_type(ctx, &else_branch)?;
            vec![block_type.clone()]
        }
        WasmInstr::GlobalGet { global_index, .. } => {
            let wasm_global = ctx
                .module
                .wasm_module
                .globals
                .get(*global_index as usize)
                .ok_or_else(|| CompileError::unreachable(file!(), line!()))?;

            vec![wasm_global.kind.value_type]
        }
        WasmInstr::LocalGet { local_index, .. } => {
            let local_index = *local_index as usize;
            if local_index < ctx.fn_type.inputs.len() {
                vec![ctx.fn_type.inputs[local_index]]
            } else {
                vec![ctx.non_arg_locals[local_index - ctx.fn_type.inputs.len()]]
            }
        }
        // TODO: clean up, logic with functions and imported functions is confusing
        WasmInstr::Call { fn_type_index, .. } => {
            let fn_type = ctx
                .module
                .wasm_module
                .types
                .get(*fn_type_index as usize)
                .ok_or_else(|| CompileError::unreachable(file!(), line!()))?;

            fn_type.outputs.clone()
        }
    })
}
