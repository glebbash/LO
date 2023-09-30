use crate::{ir::*, wasm::*};
use alloc::vec::Vec;

pub fn lower_exprs(out: &mut Vec<WasmInstr>, exprs: Vec<LoleExpr>) {
    for expr in exprs.into_iter() {
        lower_expr(out, expr);
    }
}

pub fn lower_expr(out: &mut Vec<WasmInstr>, expr: LoleExpr) {
    match expr {
        LoleExpr::Unreachable => out.push(WasmInstr::Unreachable),
        LoleExpr::Drop { value, drop_count } => {
            lower_expr(out, *value);
            for _ in 0..drop_count {
                out.push(WasmInstr::Drop);
            }
        }
        LoleExpr::BinaryOp { kind, lhs, rhs } => {
            lower_expr(out, *lhs);
            lower_expr(out, *rhs);
            out.push(WasmInstr::BinaryOp { kind })
        }
        LoleExpr::MemorySize => out.push(WasmInstr::MemorySize),
        LoleExpr::MemoryGrow { size } => {
            lower_expr(out, *size);
            out.push(WasmInstr::MemoryGrow);
        }
        LoleExpr::Load {
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
        LoleExpr::StructLoad {
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
        LoleExpr::LocalGet { local_index } => out.push(WasmInstr::LocalGet { local_index }),
        LoleExpr::GlobalGet { global_index } => out.push(WasmInstr::GlobalGet { global_index }),
        LoleExpr::StructGet {
            struct_name: _,
            base_index: _,
            primitive_gets,
        } => {
            lower_exprs(out, primitive_gets);
        }
        LoleExpr::I32ConstLazy { value } => out.push(WasmInstr::I32Const {
            value: *value.borrow(),
        }),
        LoleExpr::I32Const { value } => out.push(WasmInstr::I32Const { value }),
        LoleExpr::I64Const { value } => out.push(WasmInstr::I64Const { value }),
        LoleExpr::Set { bind } => match bind {
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
        LoleExpr::Return { value } => {
            lower_expr(out, *value);
            out.push(WasmInstr::Return);
        }
        LoleExpr::Block { block_type, body } => {
            out.push(WasmInstr::BlockStart {
                block_type: WasmBlockType::Block,
                return_type: block_type.to_wasm_type(),
            });
            lower_exprs(out, body);
            out.push(WasmInstr::BlockEnd);
        }
        LoleExpr::Loop { block_type, body } => {
            out.push(WasmInstr::BlockStart {
                block_type: WasmBlockType::Loop,
                return_type: block_type.to_wasm_type(),
            });
            lower_exprs(out, body);
            out.push(WasmInstr::BlockEnd);
        }
        LoleExpr::If {
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
        LoleExpr::Branch { label_index } => out.push(WasmInstr::Branch { label_index }),
        LoleExpr::Call {
            fn_type_index: _,
            fn_index,
            args,
        } => {
            for arg in args {
                lower_expr(out, arg);
            }
            out.push(WasmInstr::Call { fn_index });
        }
        LoleExpr::MultiValueEmit { values } => {
            lower_exprs(out, values);
        }
        LoleExpr::NoEmit { expr: _ } => {}
        LoleExpr::Casted {
            expr,
            value_type: _,
        } => {
            lower_expr(out, *expr);
        }
    }
}
