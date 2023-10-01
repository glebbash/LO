use crate::{ir::*, wasm::*};
use alloc::{vec, vec::Vec};

pub fn get_lole_type(ctx: &BlockContext, instr: &LoleExpr) -> LoleType {
    match instr {
        LoleExpr::Unreachable => LoleType::Void,
        LoleExpr::U32ConstLazy { value: _ } => LoleType::Primitive(LolePrimitiveType::U32),
        LoleExpr::U32Const { value: _ } => LoleType::Primitive(LolePrimitiveType::U32),
        LoleExpr::I64Const { value: _ } => LoleType::Primitive(LolePrimitiveType::I64),

        LoleExpr::MultiValueEmit { values } => {
            let mut types = vec![];
            for value in values {
                types.push(get_lole_type(ctx, value));
            }
            LoleType::Tuple(types)
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
        } => LoleType::StructInstance {
            name: struct_name.clone(),
        },

        // type-checked in the complier:
        LoleExpr::Casted {
            value_type,
            expr: _,
        } => value_type.clone(),
        LoleExpr::Set { bind: _ } => LoleType::Void,
        LoleExpr::Drop {
            value: _,
            drop_count: _,
        } => LoleType::Void,
        LoleExpr::Return { value: _ } => LoleType::Void,
        LoleExpr::MemorySize => LoleType::Primitive(LolePrimitiveType::I32),
        LoleExpr::MemoryGrow { size: _ } => LoleType::Primitive(LolePrimitiveType::I32),

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
        } => kind.clone(),
        LoleExpr::GlobalGet { global_index } => {
            let global_def = ctx
                .module
                .globals
                .values()
                .find(|global| global.index == *global_index)
                .unwrap();

            global_def.value_type.clone()
        }
        LoleExpr::TypedLocalGet {
            local_index: _,
            value_type,
        } => value_type.clone(),
        LoleExpr::LocalGet { local_index } => {
            let local_def = ctx
                .fn_ctx
                .locals
                .values()
                .find(|local| local.index == *local_index)
                .unwrap();

            local_def.value_type.clone()
        }
        LoleExpr::Call {
            return_type,
            fn_index: _,
            args: _,
        } => return_type.clone(),
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
        } => block_type.clone(),
        LoleExpr::Branch { label_index: _ } => LoleType::Void,
    }
}

pub fn get_type(ctx: &BlockContext, instr: &LoleExpr) -> Vec<WasmType> {
    let lole_type = get_lole_type(ctx, instr);

    let mut wasm_types = vec![];
    lole_type.emit_components(ctx.module, &mut wasm_types);
    wasm_types
}
