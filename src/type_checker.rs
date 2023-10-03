use crate::ir::*;
use alloc::vec;

pub fn get_lole_type(ctx: &BlockContext, instr: &LoleInstr) -> LoleType {
    match instr {
        LoleInstr::Unreachable => LoleType::Void,
        LoleInstr::U32ConstLazy { value: _ } => LoleType::Primitive(LolePrimitiveType::U32),
        LoleInstr::U32Const { value: _ } => LoleType::Primitive(LolePrimitiveType::U32),
        LoleInstr::I64Const { value: _ } => LoleType::Primitive(LolePrimitiveType::I64),

        LoleInstr::MultiValueEmit { values } => {
            let mut types = vec![];
            for value in values {
                types.push(get_lole_type(ctx, value));
            }
            LoleType::Tuple(types)
        }
        LoleInstr::NoEmit { expr } => get_lole_type(ctx, expr),
        LoleInstr::StructLoad {
            struct_name,
            address_instr: _,
            address_local_index: _,
            base_byte_offset: _,
            primitive_loads: _,
        }
        | LoleInstr::StructGet {
            struct_name,
            base_index: _,
            primitive_gets: _,
        } => LoleType::StructInstance {
            name: struct_name.clone(),
        },

        // type-checked in the complier:
        LoleInstr::Casted {
            value_type,
            expr: _,
        } => value_type.clone(),
        LoleInstr::Set { bind: _ } => LoleType::Void,
        LoleInstr::Drop {
            value: _,
            drop_count: _,
        } => LoleType::Void,
        LoleInstr::Return { value: _ } => LoleType::Void,
        LoleInstr::MemorySize => LoleType::Primitive(LolePrimitiveType::I32),
        LoleInstr::MemoryGrow { size: _ } => LoleType::Primitive(LolePrimitiveType::I32),

        LoleInstr::BinaryOp {
            kind: _,
            lhs,
            rhs: _,
        } => get_lole_type(ctx, lhs),
        LoleInstr::Load {
            kind,
            align: _,
            offset: _,
            address_instr: _,
        } => kind.clone(),
        LoleInstr::GlobalGet { global_index } => {
            let global_def = ctx
                .module
                .globals
                .values()
                .find(|global| global.index == *global_index)
                .unwrap();

            global_def.value_type.clone()
        }
        LoleInstr::TypedLocalGet {
            local_index: _,
            value_type,
        } => value_type.clone(),
        LoleInstr::LocalGet { local_index } => {
            let local_def = ctx
                .fn_ctx
                .locals
                .values()
                .find(|local| local.index == *local_index)
                .unwrap();

            local_def.value_type.clone()
        }
        LoleInstr::Call {
            return_type,
            fn_index: _,
            args: _,
        } => return_type.clone(),
        LoleInstr::If {
            block_type,
            cond: _,
            then_branch: _,
            else_branch: _,
        }
        | LoleInstr::Block {
            block_type,
            body: _,
        }
        | LoleInstr::Loop {
            block_type,
            body: _,
        } => block_type.clone(),
        LoleInstr::Branch { label_index: _ } => LoleType::Void,
    }
}
