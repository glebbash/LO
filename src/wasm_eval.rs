use crate::wasm::*;
use alloc::{format, string::String, vec::Vec};
use core::cell::RefCell;

#[derive(Debug)]
pub struct EvalError {
    pub message: String,
}

#[derive(Default)]
pub struct WasmEval {
    wasm_module: WasmModule,
    state: RefCell<EvalState>,
}

impl WasmEval {
    // TODO: add module verify step
    pub fn eval(wasm_module: WasmModule) -> Result<Vec<WasmValue>, EvalError> {
        let self_ = WasmEval {
            wasm_module,
            ..Default::default()
        };

        self_.eval_main()
    }

    fn eval_main(&self) -> Result<Vec<WasmValue>, EvalError> {
        let Some(fn_index) = self.get_exported_fn_index("main") else {
            return Err(EvalError {
                message: format!("`main` function is not exported"),
            });
        };

        self.call_fn(fn_index)?;

        let (fn_type, _) = self.get_fn_info(fn_index)?;
        let mut values = Vec::new();
        for _ in 0..fn_type.outputs.len() {
            values.push(self.state.borrow_mut().stack.pop().unwrap());
        }
        values.reverse();

        Ok(values)
    }

    fn call_fn(&self, fn_index: u32) -> Result<(), EvalError> {
        let (fn_type, code) = self.get_fn_info(fn_index)?;

        let mut call_frame = CallFrame::default();

        for _ in (0..fn_type.inputs.len()).rev() {
            let value = self.state.borrow_mut().stack.pop().unwrap();
            call_frame.locals.push(value);
        }
        for local in &code.locals {
            for _ in 0..local.count {
                let value = WasmValue::default_for_type(&local.value_type);
                call_frame.locals.push(value);
            }
        }
        self.state.borrow_mut().call_stack.push(call_frame);

        self.eval_expr(&code.expr)?;

        self.state.borrow_mut().call_stack.pop();

        Ok(())
    }

    fn eval_expr(&self, expr: &WasmExpr) -> Result<(), EvalError> {
        let mut blocks = Vec::<BlockState>::new();

        for instr in &expr.instrs {
            match instr {
                WasmInstr::BlockStart {
                    block_kind,
                    block_type,
                } => {
                    let WasmBlockKind::If = block_kind else {
                        todo!()
                    };
                    let WasmBlockType::NoOut = block_type else {
                        todo!()
                    };

                    let WasmValue::I32 { value: cond } =
                        self.state.borrow_mut().stack.pop().unwrap()
                    else {
                        unreachable!()
                    };

                    blocks.push(BlockState {
                        cond: cond == 1,
                        in_then: true,
                    });
                }
                WasmInstr::Else => {
                    blocks.last_mut().unwrap().in_then = false;
                }
                WasmInstr::BlockEnd => {
                    blocks.pop().unwrap();
                }
                _ if blocks.last().is_some() && blocks.last().unwrap().should_skip() => {
                    continue;
                }
                WasmInstr::Return => {
                    break;
                }
                WasmInstr::Call { fn_index } => {
                    self.call_fn(*fn_index)?;
                }
                WasmInstr::Branch { .. } => todo!(),

                WasmInstr::I32Const { value } => {
                    let value = WasmValue::I32 { value: *value };
                    self.state.borrow_mut().stack.push(value);
                }
                WasmInstr::I64Const { value } => {
                    let value = WasmValue::I64 { value: *value };
                    self.state.borrow_mut().stack.push(value);
                }
                WasmInstr::F32Const { value } => {
                    let value = WasmValue::F32 { value: *value };
                    self.state.borrow_mut().stack.push(value);
                }
                WasmInstr::F64Const { value } => {
                    let value = WasmValue::F64 { value: *value };
                    self.state.borrow_mut().stack.push(value);
                }

                WasmInstr::LocalGet { local_index } => {
                    let mut state = self.state.borrow_mut();
                    let frame = state.call_stack.last_mut().unwrap();
                    let value = frame.locals[*local_index as usize].clone();
                    state.stack.push(value);
                }
                WasmInstr::LocalSet { local_index } => {
                    let mut state = self.state.borrow_mut();
                    let value = state.stack.pop().unwrap();
                    let frame = state.call_stack.last_mut().unwrap();
                    frame.locals[*local_index as usize] = value;
                }
                WasmInstr::GlobalGet { .. } => todo!(),
                WasmInstr::GlobalSet { .. } => todo!(),
                WasmInstr::Load { .. } => todo!(),
                WasmInstr::Store { .. } => todo!(),

                WasmInstr::Unreachable => todo!(),
                WasmInstr::Drop => todo!(),
                WasmInstr::MemorySize => todo!(),
                WasmInstr::MemoryGrow => todo!(),
                WasmInstr::MemoryCopy => todo!(),

                WasmInstr::I64ExtendI32u => todo!(),
                WasmInstr::I64ExtendI32s => todo!(),
                WasmInstr::I32WrapI64 => todo!(),
                WasmInstr::BinaryOp { kind } => {
                    let stack = &mut self.state.borrow_mut().stack;
                    let WasmValue::I32 { value: rhs } = stack.pop().unwrap() else {
                        unreachable!();
                    };
                    let WasmValue::I32 { value: lhs } = stack.pop().unwrap() else {
                        unreachable!();
                    };

                    match kind {
                        WasmBinaryOpKind::I32_LT_U => {
                            let value = if lhs < rhs { 1 } else { 0 };
                            stack.push(WasmValue::I32 { value })
                        }
                        WasmBinaryOpKind::I32_ADD => {
                            let value = lhs + rhs;
                            stack.push(WasmValue::I32 { value });
                        }
                        WasmBinaryOpKind::I32_SUB => {
                            let value = lhs - rhs;
                            stack.push(WasmValue::I32 { value });
                        }
                        WasmBinaryOpKind::I32_MUL => {
                            let value = lhs * rhs;
                            stack.push(WasmValue::I32 { value });
                        }
                        WasmBinaryOpKind::I32_EQ
                        | WasmBinaryOpKind::I32_NE
                        | WasmBinaryOpKind::I32_LT_S
                        | WasmBinaryOpKind::I32_GT_S
                        | WasmBinaryOpKind::I32_GT_U
                        | WasmBinaryOpKind::I32_LE_S
                        | WasmBinaryOpKind::I32_LE_U
                        | WasmBinaryOpKind::I32_GE_S
                        | WasmBinaryOpKind::I32_GE_U
                        | WasmBinaryOpKind::I32_DIV_S
                        | WasmBinaryOpKind::I32_DIV_U
                        | WasmBinaryOpKind::I32_REM_S
                        | WasmBinaryOpKind::I32_REM_U
                        | WasmBinaryOpKind::I32_AND
                        | WasmBinaryOpKind::I32_OR
                        | WasmBinaryOpKind::I32_SHL
                        | WasmBinaryOpKind::I32_SHR_S
                        | WasmBinaryOpKind::I32_SHR_U => todo!(),

                        WasmBinaryOpKind::I64_EQ
                        | WasmBinaryOpKind::I64_NE
                        | WasmBinaryOpKind::I64_LT_S
                        | WasmBinaryOpKind::I64_LT_U
                        | WasmBinaryOpKind::I64_GT_S
                        | WasmBinaryOpKind::I64_GT_U
                        | WasmBinaryOpKind::I64_LE_S
                        | WasmBinaryOpKind::I64_LE_U
                        | WasmBinaryOpKind::I64_GE_S
                        | WasmBinaryOpKind::I64_GE_U
                        | WasmBinaryOpKind::I64_ADD
                        | WasmBinaryOpKind::I64_SUB
                        | WasmBinaryOpKind::I64_MUL
                        | WasmBinaryOpKind::I64_DIV_S
                        | WasmBinaryOpKind::I64_DIV_U
                        | WasmBinaryOpKind::I64_REM_S
                        | WasmBinaryOpKind::I64_REM_U
                        | WasmBinaryOpKind::I64_AND
                        | WasmBinaryOpKind::I64_OR
                        | WasmBinaryOpKind::I64_SHL
                        | WasmBinaryOpKind::I64_SHR_S
                        | WasmBinaryOpKind::I64_SHR_U => todo!(),

                        WasmBinaryOpKind::F32_EQ
                        | WasmBinaryOpKind::F32_NE
                        | WasmBinaryOpKind::F32_LT
                        | WasmBinaryOpKind::F32_GT
                        | WasmBinaryOpKind::F32_LE
                        | WasmBinaryOpKind::F32_GE
                        | WasmBinaryOpKind::F32_ADD
                        | WasmBinaryOpKind::F32_SUB
                        | WasmBinaryOpKind::F32_MUL
                        | WasmBinaryOpKind::F32_DIV => todo!(),

                        WasmBinaryOpKind::F64_EQ
                        | WasmBinaryOpKind::F64_NE
                        | WasmBinaryOpKind::F64_LT
                        | WasmBinaryOpKind::F64_GT
                        | WasmBinaryOpKind::F64_LE
                        | WasmBinaryOpKind::F64_GE
                        | WasmBinaryOpKind::F64_ADD
                        | WasmBinaryOpKind::F64_SUB
                        | WasmBinaryOpKind::F64_MUL
                        | WasmBinaryOpKind::F64_DIV => todo!(),
                    }
                }
            }
        }

        Ok(())
    }

    fn get_fn_info(&self, fn_index: u32) -> Result<(&WasmFnType, &WasmFn), EvalError> {
        let type_index = self.wasm_module.functions.get(fn_index as usize).unwrap();
        let fn_type = self.wasm_module.types.get(*type_index as usize).unwrap();
        let code = self.wasm_module.codes.get(fn_index as usize).unwrap();
        Ok((fn_type, code))
    }

    fn get_exported_fn_index(&self, fn_name: &str) -> Option<u32> {
        for export in &self.wasm_module.exports {
            if export.export_type == WasmExportType::Func && export.export_name == fn_name {
                return Some(export.exported_item_index);
            }
        }

        None
    }
}

// state

#[derive(Default, Debug)]
struct EvalState {
    stack: Vec<WasmValue>,
    call_stack: Vec<CallFrame>,
}

#[derive(Default, Debug)]
struct CallFrame {
    locals: Vec<WasmValue>,
}

struct BlockState {
    cond: bool,
    in_then: bool,
}

impl BlockState {
    fn should_skip(&self) -> bool {
        self.cond != self.in_then
    }
}

// values

#[derive(Debug, Clone)]
pub enum WasmValue {
    I32 { value: i32 },
    I64 { value: i64 },
    F32 { value: f32 },
    F64 { value: f64 },
}

impl WasmValue {
    fn default_for_type(wasm_type: &WasmType) -> Self {
        match wasm_type {
            WasmType::I32 => WasmValue::I32 { value: 0 },
            WasmType::I64 => WasmValue::I64 { value: 0 },
            WasmType::F32 => WasmValue::F32 { value: 0.0 },
            WasmType::F64 => WasmValue::F64 { value: 0.0 },
        }
    }
}

impl core::fmt::Display for WasmValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            WasmValue::I32 { value } => write!(f, "{value}"),
            WasmValue::I64 { value } => write!(f, "{value}"),
            WasmValue::F32 { value } => write!(f, "{value}"),
            WasmValue::F64 { value } => write!(f, "{value}"),
        }
    }
}
