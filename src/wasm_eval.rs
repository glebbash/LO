use crate::{core::*, wasm::*};
use alloc::{format, string::String, vec, vec::Vec};
use core::cell::RefCell;

const PAGE_SIZE: usize = 65_536;

#[derive(Debug)]
pub struct EvalError {
    pub message: String,
}

#[derive(Default)]
pub struct WasmEval {
    wasm_module: WasmModule,
    fn_imports_len: usize,
    state: RefCell<EvalState>,
}

impl WasmEval {
    pub fn eval(wasm_module: WasmModule) -> Result<(), EvalError> {
        let mut eval = WasmEval {
            wasm_module,
            ..Default::default()
        };

        eval.init_module()?;
        eval.eval_main()?;

        Ok(())
    }

    // TODO: add module verify step
    fn init_module(&mut self) -> Result<(), EvalError> {
        for global in &self.wasm_module.globals {
            let value = WasmValue::default_for_type(&global.kind.value_type);
            self.state.borrow_mut().globals.push(value);
        }

        if let Some(memory) = self.wasm_module.memories.first() {
            self.state.borrow_mut().memory = LinearMemory {
                bytes: vec![0; memory.min as usize * PAGE_SIZE],
            };

            for data in &self.wasm_module.datas {
                match data {
                    WasmData::Active { offset, bytes } => {
                        self.eval_expr(offset)?;
                        let offset = self.state.borrow_mut().pop_i32() as usize;

                        self.state.borrow_mut().memory.bytes[offset..offset + bytes.len()]
                            .copy_from_slice(&bytes);
                    }
                }
            }
        }

        for (import, i) in self.wasm_module.imports.iter().zip(0..) {
            if let WasmImportDesc::Func { type_index } = import.item_desc {
                let fn_type = &self.wasm_module.types[type_index as usize];

                // fn fd_write(fd: u32, iovs: u32, iovs_len: u32, nwritten: u32): u32;
                if import.module_name == "wasi_snapshot_preview1"
                    && import.item_name == "fd_write"
                    && &fn_type.inputs[..]
                        == &[WasmType::I32, WasmType::I32, WasmType::I32, WasmType::I32]
                    && &fn_type.outputs[..] == &[WasmType::I32]
                {
                    self.state
                        .borrow_mut()
                        .host_fns
                        .push(String::from("fd_write"));
                    self.fn_imports_len += 1;
                    continue;
                }

                return Err(EvalError {
                    message: format!(
                        "Cannot satisfy fn import {}, type: {:?}",
                        import.item_name, fn_type
                    ),
                });
            }

            return Err(EvalError {
                message: format!("Cannot satisfy import {} (#{i})", import.item_name),
            });
        }

        Ok(())
    }

    fn eval_main(&self) -> Result<(), EvalError> {
        if let Some(fn_index) = self.get_exported_fn_index("_start") {
            self.call_fn(fn_index)?;
            return Ok(());
        };

        let Some(fn_index) = self.get_exported_fn_index("main") else {
            return Err(EvalError {
                message: format!("Neither `_start` nor `main` function is exported"),
            });
        };

        self.call_fn(fn_index)?;

        let (fn_type, _) = self.get_fn_info(fn_index)?;
        let mut values = Vec::new();
        for _ in 0..fn_type.outputs.len() {
            values.push(self.state.borrow_mut().stack.pop().unwrap());
        }
        values.reverse();

        stdout_write(format!("result of `main` is: {}\n", ListDisplay(&values)));
        Ok(())
    }

    fn call_fn(&self, fn_index: u32) -> Result<(), EvalError> {
        if fn_index < self.fn_imports_len as u32 {
            return self.call_host_fn(fn_index);
        }

        let (fn_type, code) = self.get_fn_info(fn_index)?;

        let mut call_frame = CallFrame::default();
        for _ in 0..fn_type.inputs.len() {
            let value = self.state.borrow_mut().stack.pop().unwrap();
            call_frame.locals.push(value);
        }
        call_frame.locals.reverse();
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

    fn call_host_fn(&self, fn_index: u32) -> Result<(), EvalError> {
        let mut state = self.state.borrow_mut();
        let fn_name = &state.host_fns[fn_index as usize];
        match &fn_name[..] {
            // fn fd_write(fd: u32, iovs: u32, iovs_len: u32, nwritten: u32): u32;
            "fd_write" => {
                let nwritten = state.pop_i32();
                let iovs_len = state.pop_i32();
                let iovs_ptr = state.pop_i32();
                let fd = state.pop_i32();

                let mut iovs = Vec::new();
                for i in 0..iovs_len {
                    let iov_base = iovs_ptr as usize + (i as usize * 8);
                    let str_ptr = state.memory.load_i32(iov_base);
                    let str_len = state.memory.load_i32(iov_base + 4);

                    let buf = (&mut state.memory.bytes[str_ptr as usize]) as *const u8;
                    iovs.push(wasi::Ciovec {
                        buf,
                        buf_len: str_len as usize,
                    });
                }

                match unsafe { wasi::fd_write(fd as u32, &iovs) } {
                    Ok(nwritten_val) => {
                        state.stack.push(WasmValue::I32 { value: 0 });
                        state
                            .memory
                            .store_i32(nwritten as usize, nwritten_val as i32);
                    }
                    Err(err) => {
                        state.stack.push(WasmValue::I32 {
                            value: err.raw() as i32,
                        });
                    }
                };
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn eval_expr(&self, expr: &WasmExpr) -> Result<(), EvalError> {
        let mut blocks = Vec::<BlockState>::new();

        let mut loc = 0;
        while loc < expr.instrs.len() {
            let instr = &expr.instrs[loc];

            match instr {
                WasmInstr::BlockStart {
                    block_kind,
                    block_type: _,
                } => {
                    if blocks.last().is_some() && blocks.last().unwrap().should_skip {
                        blocks.push(BlockState {
                            loc,
                            kind: WasmBlockKind::Block,
                            should_skip: true,
                        });
                        loc += 1;
                        continue;
                    }

                    let mut should_skip = false;
                    if let WasmBlockKind::If = block_kind {
                        let cond = self.state.borrow_mut().pop_i32();

                        should_skip = cond == 0;
                    }

                    blocks.push(BlockState {
                        loc,
                        kind: block_kind.clone(),
                        should_skip,
                    });
                }
                WasmInstr::Else => {
                    let block = blocks.last_mut().unwrap();
                    if block.kind == WasmBlockKind::If {
                        block.should_skip = !block.should_skip;
                    }
                }
                WasmInstr::BlockEnd => {
                    blocks.pop().unwrap();
                }
                _ if blocks.last().is_some() && blocks.last().unwrap().should_skip => {
                    loc += 1;
                    continue;
                }
                WasmInstr::Branch { label_index } => {
                    let block_len = blocks.len();
                    let block_to_branch = &mut blocks[block_len - 1 - *label_index as usize];

                    if block_to_branch.kind == WasmBlockKind::Loop {
                        block_to_branch.should_skip = false;
                        loc = block_to_branch.loc + 1;
                        for _ in 0..*label_index {
                            blocks.pop();
                        }
                        continue;
                    }

                    for i in 0..(*label_index + 1) {
                        blocks[block_len - 1 - i as usize].should_skip = true;
                    }
                }
                WasmInstr::Return => {
                    break;
                }
                WasmInstr::Call { fn_index } => {
                    self.call_fn(*fn_index)?;
                }

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
                WasmInstr::GlobalGet { global_index } => {
                    let mut state = self.state.borrow_mut();
                    let value = state.globals[*global_index as usize].clone();
                    state.stack.push(value);
                }
                WasmInstr::GlobalSet { global_index } => {
                    let mut state = self.state.borrow_mut();
                    let value = state.stack.pop().unwrap();
                    state.globals[*global_index as usize] = value;
                }
                WasmInstr::Load {
                    kind,
                    align: _,
                    offset,
                } => {
                    let WasmLoadKind::I32 = kind else {
                        todo!("load {kind:?}")
                    };

                    let addr = self.state.borrow_mut().pop_i32();

                    let full_addr = addr as usize + *offset as usize;
                    let value = self.state.borrow_mut().memory.load_i32(full_addr);
                    self.state.borrow_mut().stack.push(WasmValue::I32 { value });
                }
                WasmInstr::Store {
                    kind,
                    align: _,
                    offset,
                } => {
                    let WasmStoreKind::I32 = kind else {
                        todo!("store {kind:?}")
                    };

                    let value = self.state.borrow_mut().pop_i32();
                    let addr = self.state.borrow_mut().pop_i32();

                    let full_addr = addr as usize + *offset as usize;
                    self.state.borrow_mut().memory.store_i32(full_addr, value);
                }

                WasmInstr::Drop => {
                    let mut state = self.state.borrow_mut();
                    let _ = state.stack.pop().unwrap();
                }
                WasmInstr::Unreachable => todo!("{instr:?}"),
                WasmInstr::MemorySize => todo!("{instr:?}"),
                WasmInstr::MemoryGrow => todo!("{instr:?}"),
                WasmInstr::MemoryCopy => todo!("{instr:?}"),

                WasmInstr::I64ExtendI32u => todo!("{instr:?}"),
                WasmInstr::I64ExtendI32s => todo!("{instr:?}"),
                WasmInstr::I32WrapI64 => todo!("{instr:?}"),
                WasmInstr::BinaryOp { kind } => {
                    let rhs = self.state.borrow_mut().pop_i32();
                    let lhs = self.state.borrow_mut().pop_i32();
                    let stack = &mut self.state.borrow_mut().stack;

                    match kind {
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
                        WasmBinaryOpKind::I32_AND => {
                            let value = lhs & rhs;
                            stack.push(WasmValue::I32 { value })
                        }
                        WasmBinaryOpKind::I32_SHR_U => {
                            let value = ((lhs as u32) >> (rhs as u32)) as i32;
                            stack.push(WasmValue::I32 { value });
                        }
                        WasmBinaryOpKind::I32_REM_U => {
                            let value = ((lhs as u32) % (rhs as u32)) as i32;
                            stack.push(WasmValue::I32 { value });
                        }
                        WasmBinaryOpKind::I32_EQ => {
                            let value = if lhs == rhs { 1 } else { 0 };
                            stack.push(WasmValue::I32 { value })
                        }
                        WasmBinaryOpKind::I32_LT_U => {
                            let value = if (lhs as u32) < (rhs as u32) { 1 } else { 0 };
                            stack.push(WasmValue::I32 { value })
                        }
                        WasmBinaryOpKind::I32_NE
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
                        | WasmBinaryOpKind::I32_OR
                        | WasmBinaryOpKind::I32_SHL
                        | WasmBinaryOpKind::I32_SHR_S => todo!("{kind:?}"),

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
                        | WasmBinaryOpKind::I64_SHR_U => todo!("{kind:?}"),

                        WasmBinaryOpKind::F32_EQ
                        | WasmBinaryOpKind::F32_NE
                        | WasmBinaryOpKind::F32_LT
                        | WasmBinaryOpKind::F32_GT
                        | WasmBinaryOpKind::F32_LE
                        | WasmBinaryOpKind::F32_GE
                        | WasmBinaryOpKind::F32_ADD
                        | WasmBinaryOpKind::F32_SUB
                        | WasmBinaryOpKind::F32_MUL
                        | WasmBinaryOpKind::F32_DIV => todo!("{kind:?}"),

                        WasmBinaryOpKind::F64_EQ
                        | WasmBinaryOpKind::F64_NE
                        | WasmBinaryOpKind::F64_LT
                        | WasmBinaryOpKind::F64_GT
                        | WasmBinaryOpKind::F64_LE
                        | WasmBinaryOpKind::F64_GE
                        | WasmBinaryOpKind::F64_ADD
                        | WasmBinaryOpKind::F64_SUB
                        | WasmBinaryOpKind::F64_MUL
                        | WasmBinaryOpKind::F64_DIV => todo!("{kind:?}"),
                    }
                }
            }

            loc += 1;
        }

        Ok(())
    }

    fn get_fn_info(&self, fn_index: u32) -> Result<(&WasmFnType, &WasmFn), EvalError> {
        let resolved_fn_index = fn_index as usize - self.fn_imports_len;
        let type_index = self.wasm_module.functions.get(resolved_fn_index).unwrap();
        let fn_type = self.wasm_module.types.get(*type_index as usize).unwrap();
        let code = self.wasm_module.codes.get(resolved_fn_index).unwrap();
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
    globals: Vec<WasmValue>,
    stack: Vec<WasmValue>,
    call_stack: Vec<CallFrame>,
    memory: LinearMemory,
    host_fns: Vec<String>,
}

impl EvalState {
    fn pop_i32(&mut self) -> i32 {
        let wasm_value = self.stack.pop().unwrap();
        let WasmValue::I32 { value } = wasm_value else {
            unreachable!();
        };

        value
    }
}

#[derive(Default, Debug)]
struct LinearMemory {
    bytes: Vec<u8>,
}

impl LinearMemory {
    fn load_i32(&self, addr: usize) -> i32 {
        i32::from_le_bytes(self.bytes[addr..addr + 4].try_into().unwrap())
    }

    fn store_i32(&mut self, addr: usize, value: i32) {
        self.bytes[addr..addr + 4].copy_from_slice(&value.to_le_bytes())
    }
}

#[derive(Default, Debug)]
struct CallFrame {
    locals: Vec<WasmValue>,
}

struct BlockState {
    loc: usize,
    kind: WasmBlockKind,
    should_skip: bool,
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
