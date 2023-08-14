use crate::{
    common::{CompileError, Location},
    compiler::FnContext,
    wasm_module::{WasmImportDesc, WasmInstr, WasmValueType},
};
use alloc::{format, vec, vec::Vec};

pub fn get_types(
    ctx: &FnContext,
    instrs: &Vec<WasmInstr>,
) -> Result<Vec<WasmValueType>, CompileError> {
    let mut types = vec![];
    for instr in instrs {
        types.append(&mut get_type(ctx, instr)?);
    }
    Ok(types)
}

pub fn get_type(ctx: &FnContext, instr: &WasmInstr) -> Result<Vec<WasmValueType>, CompileError> {
    Ok(match instr {
        WasmInstr::Unreachable { .. } => vec![],
        WasmInstr::LoopBreak { .. } => vec![],
        WasmInstr::LoopContinue { .. } => vec![],
        WasmInstr::I32ConstLazy { .. } => vec![WasmValueType::I32],
        WasmInstr::I32Const { .. } => vec![WasmValueType::I32],
        WasmInstr::I64Const { .. } => vec![WasmValueType::I64],
        WasmInstr::MultiValueEmit { values, .. } => get_types(ctx, values)?,
        WasmInstr::StructLoad {
            primitive_loads, ..
        } => get_types(ctx, primitive_loads)?,
        WasmInstr::StructGet { primitive_gets, .. } => get_types(ctx, primitive_gets)?,
        WasmInstr::NoEmit { instr } => get_type(ctx, instr)?,

        // type checked in the complier:
        WasmInstr::NoTypeCheck { .. } => vec![],
        WasmInstr::Set { .. } => vec![],
        WasmInstr::Drop { .. } => vec![],
        WasmInstr::Loop { .. } => vec![],

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
                .ok_or_else(|| unreachable_err(line!()))?;

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
        WasmInstr::Call {
            fn_index,
            args,
            loc,
        } => {
            let arg_types = get_types(ctx, args)?;

            let (fn_name, fn_def) = ctx
                .module
                .fn_defs
                .iter()
                .find(|(_, fd)| fd.get_absolute_index(ctx.module) == *fn_index)
                .ok_or_else(|| unreachable_err(line!()))?;

            let type_index = if fn_def.local {
                ctx.module
                    .wasm_module
                    .functions
                    .get(fn_def.fn_index as usize)
                    .ok_or_else(|| unreachable_err(line!()))?
            } else {
                let WasmImportDesc::Func { type_index } = ctx
                    .module
                    .wasm_module
                    .imports
                    .get(fn_def.fn_index as usize)
                    .map(|i| &i.item_desc)
                    .ok_or_else(|| unreachable_err(line!()))?;

                type_index
            };

            let fn_type = ctx
                .module
                .wasm_module
                .types
                .get(*type_index as usize)
                .ok_or_else(|| unreachable_err(line!()))?;

            if fn_type.inputs != arg_types {
                return Err(CompileError {
                    message: format!(
                        "TypeError: Mismatched arguments for function \
                            '{fn_name}', expected {inputs:?}, got {args:?}",
                        inputs = fn_type.inputs,
                        args = arg_types,
                    ),
                    loc: loc.clone(),
                });
            }

            fn_type.outputs.clone()
        }
        WasmInstr::Return { value, loc } => {
            let return_type = get_type(ctx, value)?;
            if return_type != ctx.fn_type.outputs {
                return Err(CompileError {
                    message: format!(
                        "TypeError: Invalid return type, \
                            expected {outputs:?}, got {return_type:?}",
                        outputs = ctx.fn_type.outputs,
                    ),
                    loc: loc.clone(),
                });
            }
            vec![]
        }
    })
}

fn unreachable_err(line: u32) -> CompileError {
    CompileError {
        message: format!("Unreachable in {}, {}", file!(), line),
        loc: Location::internal(),
    }
}
