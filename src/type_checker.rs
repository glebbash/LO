use crate::{ast::*, ir::*, wasm::*};
use alloc::{format, string::String, vec, vec::Vec};

pub fn get_types(
    ctx: &BlockContext,
    instrs: &Vec<LoleExpr>,
) -> Result<Vec<WasmType>, CompileError> {
    let mut types = vec![];
    for instr in instrs {
        types.append(&mut get_type(ctx, instr)?);
    }
    Ok(types)
}

pub fn get_lole_type(ctx: &BlockContext, instr: &LoleExpr) -> Result<LoleType, String> {
    match instr {
        LoleExpr::Unreachable => Ok(LoleType::Void),
        LoleExpr::I32ConstLazy { value: _ } => Ok(LoleType::Primitive(LolePrimitiveType::I32)),
        LoleExpr::I32Const { value: _ } => Ok(LoleType::Primitive(LolePrimitiveType::I32)),
        LoleExpr::I64Const { value: _ } => Ok(LoleType::Primitive(LolePrimitiveType::I64)),

        LoleExpr::MultiValueEmit { values } => {
            let mut types = vec![];
            for value in values {
                types.push(get_lole_type(ctx, value)?);
            }
            Ok(LoleType::Tuple(types))
        }
        LoleExpr::NoEmit { expr } => get_lole_type(ctx, expr),
        LoleExpr::StructLoad {
            struct_name,
            address_instr: _,
            address_local_index: _,
            base_byte_offset: _,
            primitive_loads: _,
        }
        | LoleExpr::StructGet {
            struct_name,
            base_index: _,
            primitive_gets: _,
        } => Ok(LoleType::StructInstance {
            name: struct_name.clone(),
        }),

        // type-checked in the complier:
        LoleExpr::Casted {
            value_type,
            expr: _,
        } => Ok(value_type.clone()),
        LoleExpr::Set { bind: _ } => Ok(LoleType::Void),
        LoleExpr::Drop {
            value: _,
            drop_count: _,
        } => Ok(LoleType::Void),
        LoleExpr::Return { value: _ } => Ok(LoleType::Void),
        LoleExpr::MemorySize => Ok(LoleType::Primitive(LolePrimitiveType::I32)),
        LoleExpr::MemoryGrow { size: _ } => Ok(LoleType::Primitive(LolePrimitiveType::I32)),

        LoleExpr::BinaryOp {
            kind: _,
            lhs,
            rhs: _,
        } => get_lole_type(ctx, lhs),
        LoleExpr::Load {
            kind,
            align: _,
            offset: _,
            address_instr: _,
        } => Ok(kind.clone()),
        LoleExpr::GlobalGet { global_index } => {
            let global_def = ctx
                .module
                .globals
                .values()
                .find(|global| global.index == *global_index);

            // TODO: handle struct fields: {s . x} should be supported
            let Some(global_def) = global_def else {
                return Err(format!("shouldn't happen"));
            };

            Ok(global_def.value_type.clone())
        }
        LoleExpr::LocalGet { local_index } => {
            let local_def = ctx
                .fn_ctx
                .locals
                .values()
                .find(|local| local.index == *local_index);

            // TODO: handle struct fields: {s . x} should be supported
            let Some(local_def) = local_def else {
                return Err(format!("shouldn't happen"));
            };

            Ok(local_def.value_type.clone())
        }
        LoleExpr::Call {
            fn_type_index,
            fn_index: _,
            args: _,
        } => {
            let lole_fn_type = &ctx.module.lole_fn_types.get(fn_type_index).unwrap();
            Ok(lole_fn_type.output.clone())
        }
        LoleExpr::If {
            block_type,
            cond: _,
            then_branch: _,
            else_branch: _,
        }
        | LoleExpr::Block {
            block_type,
            body: _,
        }
        | LoleExpr::Loop {
            block_type,
            body: _,
        } => Ok(block_type.clone()),
        LoleExpr::Branch { label_index: _ } => Ok(LoleType::Void),
    }
}

pub fn get_type(ctx: &BlockContext, instr: &LoleExpr) -> Result<Vec<WasmType>, CompileError> {
    // TODO: use this for testing get_lole_type
    // let lole_type = get_lole_type(ctx, instr).map_err(|message| CompileError {
    //     message,
    //     loc: Location::internal(),
    // })?;

    // let mut wasm_types = vec![];
    // lole_type.emit_components(ctx.module, &mut wasm_types);
    // Ok(wasm_types)

    Ok(match instr {
        LoleExpr::Unreachable => vec![],
        LoleExpr::I32ConstLazy { value: _ } => vec![WasmType::I32],
        LoleExpr::I32Const { value: _ } => vec![WasmType::I32],
        LoleExpr::I64Const { value: _ } => vec![WasmType::I64],
        LoleExpr::MultiValueEmit { values } => get_types(ctx, values)?,
        LoleExpr::StructLoad {
            primitive_loads,
            struct_name: _,
            address_instr: _,
            address_local_index: _,
            base_byte_offset: _,
        } => get_types(ctx, primitive_loads)?,
        LoleExpr::StructGet {
            primitive_gets,
            struct_name: _,
            base_index: _,
        } => get_types(ctx, primitive_gets)?,
        LoleExpr::NoEmit { expr: instr } => get_type(ctx, instr)?,

        LoleExpr::Casted {
            value_type,
            expr: _,
        } => {
            let mut wasm_types = vec![];
            value_type.emit_components(ctx.module, &mut wasm_types);
            wasm_types
        }
        LoleExpr::Set { bind: _ } => vec![],
        LoleExpr::Drop {
            value: _,
            drop_count: _,
        } => vec![],
        LoleExpr::Return { value: _ } => vec![],
        LoleExpr::MemorySize => vec![WasmType::I32],
        LoleExpr::MemoryGrow { size: _ } => vec![WasmType::I32],

        LoleExpr::BinaryOp { lhs, rhs, kind: _ } => {
            get_type(ctx, rhs)?;
            return get_type(ctx, lhs);
        }
        LoleExpr::Load {
            kind,
            align: _,
            offset: _,
            address_instr: _,
        } => vec![kind.to_wasm_type().unwrap()],
        LoleExpr::GlobalGet { global_index } => {
            let wasm_global = ctx
                .module
                .wasm_module
                .globals
                .get(*global_index as usize)
                .ok_or_else(|| CompileError::unreachable(file!(), line!()))?;

            vec![wasm_global.kind.value_type]
        }
        LoleExpr::LocalGet { local_index } => {
            let local_index = *local_index as usize;
            let locals_len = ctx.fn_ctx.fn_type.inputs.len();
            if local_index < locals_len {
                vec![ctx.fn_ctx.fn_type.inputs[local_index]]
            } else {
                vec![ctx.fn_ctx.non_arg_locals[local_index - locals_len]]
            }
        }
        LoleExpr::Call {
            fn_type_index,
            fn_index: _,
            args: _,
        } => {
            let fn_type = &ctx.module.wasm_module.types[*fn_type_index as usize];
            fn_type.outputs.clone()
        }
        LoleExpr::If {
            block_type,
            cond: _,
            then_branch: _,
            else_branch: _,
        }
        | LoleExpr::Block {
            block_type,
            body: _,
        }
        | LoleExpr::Loop {
            block_type,
            body: _,
        } => {
            if let Some(wasm_type) = block_type.to_wasm_type() {
                vec![wasm_type]
            } else {
                vec![]
            }
        }
        LoleExpr::Branch { label_index: _ } => vec![],
    })
}
