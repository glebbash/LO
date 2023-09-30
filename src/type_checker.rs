use crate::{ast::*, ir::*, wasm::*};
use alloc::format;
use alloc::{vec, vec::Vec};

pub fn get_types(
    ctx: &BlockContext,
    instrs: &Vec<WasmInstr>,
) -> Result<Vec<WasmType>, CompileError> {
    let mut types = vec![];
    for instr in instrs {
        types.append(&mut get_type(ctx, instr)?);
    }
    Ok(types)
}

pub fn get_lole_type(ctx: &BlockContext, instr: &WasmInstr) -> Result<LoleType, CompileError> {
    match instr {
        WasmInstr::Unreachable { .. } => Ok(LoleType::Void),
        WasmInstr::I32ConstLazy { .. } => Ok(LoleType::Primitive(LolePrimitiveType::I32)),
        WasmInstr::I32Const { .. } => Ok(LoleType::Primitive(LolePrimitiveType::I32)),
        WasmInstr::I64Const { .. } => Ok(LoleType::Primitive(LolePrimitiveType::I64)),

        // TODO: implement
        // WasmInstr::MultiValueEmit { values, .. } => get_types(ctx, values)?,
        WasmInstr::NoEmit { .. } => Ok(LoleType::Void),
        WasmInstr::StructLoad { struct_name, .. } | WasmInstr::StructGet { struct_name, .. } => {
            Ok(LoleType::StructInstance {
                name: struct_name.clone(),
            })
        }

        // type-checked in the complier:
        WasmInstr::NoTypeCheck { .. } => Ok(LoleType::Void),
        WasmInstr::Set { .. } => Ok(LoleType::Void),
        WasmInstr::Drop { .. } => Ok(LoleType::Void),
        WasmInstr::Return { .. } => Ok(LoleType::Void),
        WasmInstr::MemorySize { .. } => Ok(LoleType::Primitive(LolePrimitiveType::I32)),
        WasmInstr::MemoryGrow { .. } => Ok(LoleType::Primitive(LolePrimitiveType::I32)),

        WasmInstr::BinaryOp { lhs, rhs, .. } => {
            get_lole_type(ctx, rhs)?;
            return get_lole_type(ctx, lhs);
        }

        // TODO: implement
        // WasmInstr::Load {
        //     kind,
        //     address_instr,
        //     ..
        // } => {
        //     get_type(ctx, &address_instr)?;
        //     // TODO: use primitive type
        //     vec![kind.get_primitive_type().to_wasm_type()]
        // }
        WasmInstr::GlobalGet { global_index, .. } => {
            let global_def = ctx
                .module
                .globals
                .values()
                .find(|global| global.index == *global_index);

            // TODO: handle struct fields: {s . x} should be supported
            let Some(global_def) = global_def else {
                return Err(CompileError {
                    message: format!("shouldn't happen"),
                    loc: Location::internal(),
                });
            };

            Ok(global_def.value_type.clone())
        }
        WasmInstr::LocalGet { local_index, .. } => {
            let local_def = ctx
                .fn_ctx
                .locals
                .values()
                .find(|local| local.index == *local_index);

            // TODO: handle struct fields: {s . x} should be supported
            let Some(local_def) = local_def else {
                return Err(CompileError {
                    message: format!("shouldn't happen"),
                    loc: Location::internal(),
                });
            };

            Ok(local_def.value_type.clone())
        }
        // TODO: implement
        // WasmInstr::Call { fn_type_index, .. } => {
        //     let fn_type = &ctx.module.wasm_module.types[*fn_type_index as usize];
        //     fn_type.outputs.clone()
        // }
        // WasmInstr::If { block_type, .. }
        // | WasmInstr::Block { block_type, .. }
        // | WasmInstr::Loop { block_type, .. } => {
        //     if let Some(block_type) = block_type {
        //         vec![*block_type]
        //     } else {
        //         vec![]
        //     }
        // }
        WasmInstr::Branch { .. } => Ok(LoleType::Void),

        _ => Err(CompileError {
            message: format!("not implemented yet"),
            loc: Location::internal(),
        }),
    }
}

pub fn get_type(ctx: &BlockContext, instr: &WasmInstr) -> Result<Vec<WasmType>, CompileError> {
    Ok(match instr {
        WasmInstr::Unreachable { .. } => vec![],
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
        WasmInstr::Return { .. } => vec![],
        WasmInstr::MemorySize { .. } => vec![WasmType::I32],
        WasmInstr::MemoryGrow { .. } => vec![WasmType::I32],

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
            // TODO: use primitive type
            vec![kind.get_primitive_type().to_wasm_type()]
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
            let locals_len = ctx.fn_ctx.fn_type.inputs.len();
            if local_index < locals_len {
                vec![ctx.fn_ctx.fn_type.inputs[local_index]]
            } else {
                vec![ctx.fn_ctx.non_arg_locals[local_index - locals_len]]
            }
        }
        WasmInstr::Call { fn_type_index, .. } => {
            let fn_type = &ctx.module.wasm_module.types[*fn_type_index as usize];
            fn_type.outputs.clone()
        }
        WasmInstr::If { block_type, .. }
        | WasmInstr::Block { block_type, .. }
        | WasmInstr::Loop { block_type, .. } => {
            if let Some(block_type) = block_type {
                vec![*block_type]
            } else {
                vec![]
            }
        }
        WasmInstr::Branch { .. } => vec![],
    })
}
