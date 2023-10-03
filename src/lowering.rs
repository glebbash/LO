use crate::{ir::*, wasm::*};
use alloc::vec::Vec;

pub fn lower_exprs(out: &mut Vec<WasmInstr>, exprs: Vec<LoleInstr>) {
    for expr in exprs.into_iter() {
        lower_expr(out, expr);
    }
}

pub fn lower_expr(out: &mut Vec<WasmInstr>, expr: LoleInstr) {
    match expr {
        LoleInstr::Unreachable => out.push(WasmInstr::Unreachable),
        LoleInstr::Drop { value, drop_count } => {
            lower_expr(out, *value);
            for _ in 0..drop_count {
                out.push(WasmInstr::Drop);
            }
        }
        LoleInstr::BinaryOp { kind, lhs, rhs } => {
            lower_expr(out, *lhs);
            lower_expr(out, *rhs);
            out.push(WasmInstr::BinaryOp { kind })
        }
        LoleInstr::MemorySize => out.push(WasmInstr::MemorySize),
        LoleInstr::MemoryGrow { size } => {
            lower_expr(out, *size);
            out.push(WasmInstr::MemoryGrow);
        }
        LoleInstr::Load {
            kind,
            align,
            offset,
            address_instr,
        } => {
            lower_expr(out, *address_instr);
            out.push(WasmInstr::Load {
                kind: kind.to_load_kind().unwrap(),
                align,
                offset,
            });
        }
        LoleInstr::StructLoad {
            struct_name: _,
            address_instr,
            address_local_index,
            base_byte_offset: _,
            primitive_loads,
        } => {
            lower_expr(out, *address_instr);
            out.push(WasmInstr::LocalSet {
                local_index: address_local_index,
            });
            lower_exprs(out, primitive_loads);
        }
        LoleInstr::LocalGet { local_index }
        | LoleInstr::TypedLocalGet {
            local_index,
            value_type: _,
        } => out.push(WasmInstr::LocalGet { local_index }),
        LoleInstr::GlobalGet { global_index } => out.push(WasmInstr::GlobalGet { global_index }),
        LoleInstr::StructGet {
            struct_name: _,
            base_index: _,
            primitive_gets,
        } => {
            lower_exprs(out, primitive_gets);
        }
        LoleInstr::U32ConstLazy { value } => out.push(WasmInstr::I32Const {
            value: *value.borrow() as i32,
        }),
        LoleInstr::U32Const { value } => out.push(WasmInstr::I32Const {
            value: value as i32,
        }),
        LoleInstr::I64Const { value } => out.push(WasmInstr::I64Const { value }),
        LoleInstr::Set { bind } => match bind {
            LoleSetBind::Local { index } => out.push(WasmInstr::LocalSet { local_index: index }),
            LoleSetBind::Global { index } => out.push(WasmInstr::GlobalSet {
                global_index: index,
            }),
            LoleSetBind::Memory {
                align,
                offset,
                kind,
                address_instr,
                value_local_index,
            } => {
                out.push(WasmInstr::LocalSet {
                    local_index: value_local_index,
                });
                lower_expr(out, *address_instr);
                out.push(WasmInstr::LocalGet {
                    local_index: value_local_index,
                });
                out.push(WasmInstr::Store {
                    align,
                    offset,
                    kind,
                });
            }
        },
        LoleInstr::Return { value } => {
            lower_expr(out, *value);
            out.push(WasmInstr::Return);
        }
        LoleInstr::Block { block_type, body } => {
            out.push(WasmInstr::BlockStart {
                block_type: WasmBlockType::Block,
                return_type: block_type.to_wasm_type(),
            });
            lower_exprs(out, body);
            out.push(WasmInstr::BlockEnd);
        }
        LoleInstr::Loop { block_type, body } => {
            out.push(WasmInstr::BlockStart {
                block_type: WasmBlockType::Loop,
                return_type: block_type.to_wasm_type(),
            });
            lower_exprs(out, body);
            out.push(WasmInstr::BlockEnd);
        }
        LoleInstr::If {
            block_type,
            cond,
            then_branch,
            else_branch,
        } => {
            lower_expr(out, *cond);
            out.push(WasmInstr::BlockStart {
                block_type: WasmBlockType::If,
                return_type: block_type.to_wasm_type(),
            });
            lower_exprs(out, then_branch);
            if let Some(else_branch) = else_branch {
                out.push(WasmInstr::Else);
                lower_exprs(out, else_branch);
            }
            out.push(WasmInstr::BlockEnd);
        }
        LoleInstr::Branch { label_index } => out.push(WasmInstr::Branch { label_index }),
        LoleInstr::Call {
            fn_index,
            args,
            return_type: _,
        } => {
            for arg in args {
                lower_expr(out, arg);
            }
            out.push(WasmInstr::Call { fn_index });
        }
        LoleInstr::MultiValueEmit { values } => {
            lower_exprs(out, values);
        }
        LoleInstr::NoEmit { expr: _ } => {}
        LoleInstr::Casted {
            expr,
            value_type: _,
        } => {
            lower_expr(out, *expr);
        }
    }
}
