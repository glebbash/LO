use crate::{core::*, wasm::*};
use alloc::{
    alloc::{alloc, dealloc, Layout},
    format, str,
    string::String,
    vec,
    vec::Vec,
};

const PAGE_SIZE: usize = 65_536;

#[derive(Debug)]
pub struct EvalError {
    pub message: String,
}

#[derive(Default)]
pub struct WasmEval {
    wasm_module: WasmModule,
    fn_imports_len: usize,
    state: EvalState,
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
        for global in unsafe_borrow(&self.wasm_module.globals) {
            self.eval_expr(&global.initial_value)?;
            let initial_value = self.state.stack.pop().unwrap();
            self.state.globals.push(initial_value);
        }

        if let Some(memory) = self.wasm_module.memories.first() {
            self.state.memory = LinearMemory {
                size_in_pages: memory.min as usize,
                bytes: vec![0; memory.min as usize * PAGE_SIZE],
            };

            for data in unsafe_borrow(&self.wasm_module.datas) {
                match data {
                    WasmData::Active { offset, bytes } => {
                        self.eval_expr(offset)?;
                        let offset = self.state.pop_i32() as usize;

                        self.state.memory.bytes[offset..offset + bytes.len()]
                            .copy_from_slice(&bytes);
                    }
                }
            }
        }

        'import_loop: for (import, i) in self.wasm_module.imports.iter().zip(0..) {
            if let WasmImportDesc::Func { type_index } = import.item_desc {
                let fn_type = &self.wasm_module.types[type_index as usize];

                for host_fn in &SUPPORTED_HOST_FNS {
                    if import.module_name == host_fn.module_name
                        && import.item_name == host_fn.fn_name
                        && &fn_type.inputs[..] == host_fn.fn_inputs
                        && &fn_type.outputs[..] == host_fn.fn_outputs
                    {
                        let full_name = format!("{}::{}", import.module_name, import.item_name);
                        self.state.host_fns.push(full_name);
                        self.fn_imports_len += 1;
                        continue 'import_loop;
                    }
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

    fn eval_main(&mut self) -> Result<(), EvalError> {
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
            values.push(self.state.stack.pop().unwrap());
        }
        values.reverse();

        stdout_write(format!("result of `main` is: {}\n", ListDisplay(&values)));
        Ok(())
    }

    fn call_fn(&mut self, fn_index: u32) -> Result<(), EvalError> {
        if fn_index < self.fn_imports_len as u32 {
            return call_host_fn(self, fn_index);
        }

        let (fn_type, code) = unsafe_borrow(self).get_fn_info(fn_index)?;

        let mut call_frame = CallFrame {
            fn_index,
            ..Default::default()
        };
        for _ in 0..fn_type.inputs.len() {
            let value = self.state.stack.pop().unwrap();
            call_frame.locals.push(value);
        }
        call_frame.locals.reverse();
        for local in &code.locals {
            for _ in 0..local.count {
                let value = WasmValue::default_for_type(&local.value_type);
                call_frame.locals.push(value);
            }
        }
        self.state.call_stack.push(call_frame);

        self.eval_expr(&code.expr)?;

        self.state.call_stack.pop();

        Ok(())
    }

    fn eval_expr(&mut self, expr: &WasmExpr) -> Result<(), EvalError> {
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
                        let cond = self.state.pop_i32();

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
                    self.state.stack.push(value);
                }
                WasmInstr::I64Const { value } => {
                    let value = WasmValue::I64 { value: *value };
                    self.state.stack.push(value);
                }
                WasmInstr::F32Const { value } => {
                    let value = WasmValue::F32 { value: *value };
                    self.state.stack.push(value);
                }
                WasmInstr::F64Const { value } => {
                    let value = WasmValue::F64 { value: *value };
                    self.state.stack.push(value);
                }

                WasmInstr::LocalGet { local_index } => {
                    let frame = self.state.call_stack.last_mut().unwrap();
                    let value = frame.locals[*local_index as usize].clone();
                    self.state.stack.push(value);
                }
                WasmInstr::LocalSet { local_index } => {
                    let value = self.state.stack.pop().unwrap();
                    let frame = self.state.call_stack.last_mut().unwrap();
                    frame.locals[*local_index as usize] = value;
                }
                WasmInstr::GlobalGet { global_index } => {
                    let value = self.state.globals[*global_index as usize].clone();
                    self.state.stack.push(value);
                }
                WasmInstr::GlobalSet { global_index } => {
                    let value = self.state.stack.pop().unwrap();
                    self.state.globals[*global_index as usize] = value;
                }
                WasmInstr::Load {
                    kind,
                    align: _,
                    offset,
                } => match kind {
                    WasmLoadKind::I32 => {
                        let addr = self.state.pop_i32();
                        let full_addr = addr as usize + *offset as usize;
                        let value = self.state.memory.load_i32(full_addr);
                        self.state.stack.push(WasmValue::I32 { value });
                    }
                    WasmLoadKind::I32U8 => {
                        let addr = self.state.pop_i32();
                        let full_addr = addr as usize + *offset as usize;
                        let Some(value) = self.state.memory.bytes.get(full_addr).cloned() else {
                            return Err(self.err_with_stack(format!(
                                "Memory read out of bounds: {full_addr}"
                            )));
                        };

                        self.state.stack.push(WasmValue::I32 {
                            value: value as i32,
                        });
                    }
                    _ => todo!("load {kind:?}"),
                },
                WasmInstr::Store {
                    kind,
                    align: _,
                    offset,
                } => match kind {
                    WasmStoreKind::I32 => {
                        let value = self.state.pop_i32();
                        let addr = self.state.pop_i32();
                        let full_addr = addr as usize + *offset as usize;
                        self.state.memory.store_i32(full_addr, value);
                    }
                    WasmStoreKind::I32U8 => {
                        let value = self.state.pop_i32();
                        let addr = self.state.pop_i32();
                        let full_addr = addr as usize + *offset as usize;
                        self.state.memory.bytes[full_addr] = value as u8;
                    }
                    _ => todo!("store {kind:?}"),
                },

                WasmInstr::Drop => {
                    let _ = self.state.stack.pop().unwrap();
                }
                WasmInstr::Unreachable => {
                    return Err(self.err_with_stack("Hit unreachable trap"));
                }
                WasmInstr::MemorySize => {
                    let mem_size = self.state.memory.size_in_pages as i32;
                    self.state.stack.push(WasmValue::I32 { value: mem_size });
                }
                WasmInstr::MemoryCopy => {
                    let num_bytes = self.state.pop_i32();
                    let source = self.state.pop_i32();
                    let destination = self.state.pop_i32();

                    self.state.memory.bytes.copy_within(
                        source as usize..source as usize + num_bytes as usize,
                        destination as usize,
                    );
                }
                WasmInstr::MemoryGrow => todo!("{instr:?}"),

                WasmInstr::I64ExtendI32u => {
                    crate::core::debug(format!("here 2?"));
                    let value = self.state.pop_i32();
                    crate::core::debug(format!("no 2"));
                    self.state.stack.push(WasmValue::I64 {
                        value: value as u32 as u64 as i64,
                    })
                }
                WasmInstr::I64ExtendI32s => {
                    crate::core::debug(format!("here?"));
                    let value = self.state.pop_i32();
                    crate::core::debug(format!("no"));
                    self.state.stack.push(WasmValue::I64 {
                        value: value as i64,
                    })
                }
                WasmInstr::I32WrapI64 => todo!("{instr:?}"),
                WasmInstr::BinaryOp { kind } => {
                    let rhs = self.state.pop_i32();
                    let lhs = self.state.pop_i32();
                    let stack = &mut self.state.stack;

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
                        WasmBinaryOpKind::I32_OR => {
                            let value = lhs | rhs;
                            stack.push(WasmValue::I32 { value })
                        }
                        WasmBinaryOpKind::I32_DIV_U => {
                            let value = ((lhs as u32) / (rhs as u32)) as i32;
                            stack.push(WasmValue::I32 { value });
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
                        WasmBinaryOpKind::I32_NE => {
                            let value = if lhs != rhs { 1 } else { 0 };
                            stack.push(WasmValue::I32 { value })
                        }
                        WasmBinaryOpKind::I32_GT_S => {
                            let value = if lhs > rhs { 1 } else { 0 };
                            stack.push(WasmValue::I32 { value })
                        }
                        WasmBinaryOpKind::I32_LT_U => {
                            let value = if (lhs as u32) < (rhs as u32) { 1 } else { 0 };
                            stack.push(WasmValue::I32 { value })
                        }
                        WasmBinaryOpKind::I32_GT_U => {
                            let value = if (lhs as u32) > (rhs as u32) { 1 } else { 0 };
                            stack.push(WasmValue::I32 { value })
                        }
                        WasmBinaryOpKind::I32_GE_U => {
                            let value = if (lhs as u32) >= (rhs as u32) { 1 } else { 0 };
                            stack.push(WasmValue::I32 { value })
                        }
                        WasmBinaryOpKind::I32_LE_U => {
                            let value = if (lhs as u32) <= (rhs as u32) { 1 } else { 0 };
                            stack.push(WasmValue::I32 { value })
                        }
                        WasmBinaryOpKind::I32_LT_S
                        | WasmBinaryOpKind::I32_LE_S
                        | WasmBinaryOpKind::I32_GE_S
                        | WasmBinaryOpKind::I32_DIV_S
                        | WasmBinaryOpKind::I32_REM_S
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

    fn err_with_stack(&mut self, message: impl AsRef<str>) -> EvalError {
        use core::fmt::Write;

        let mut message = String::from(message.as_ref());

        while let Some(frame) = self.state.call_stack.pop() {
            write!(&mut message, "\n  at ").unwrap();
            if let Some(fn_name) = self.get_fn_name(frame.fn_index) {
                write!(&mut message, "{fn_name}").unwrap();
            } else {
                write!(&mut message, "<unnamed-fn> #{}", frame.fn_index).unwrap();
            }
        }

        EvalError { message }
    }

    fn get_fn_name(&self, fn_index: u32) -> Option<&str> {
        for fn_info in &self.wasm_module.debug_fn_info {
            if fn_info.fn_index == fn_index {
                return Some(&fn_info.fn_name);
            }
        }

        None
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

    fn pop_i64(&mut self) -> i64 {
        let wasm_value = self.stack.pop().unwrap();
        let WasmValue::I64 { value } = wasm_value else {
            unreachable!();
        };

        value
    }
}

#[derive(Default, Debug)]
struct LinearMemory {
    size_in_pages: usize,
    bytes: Vec<u8>,
}

impl LinearMemory {
    fn load_i32(&self, addr: usize) -> i32 {
        i32::from_le_bytes(self.bytes[addr..addr + 4].try_into().unwrap())
    }

    fn store_i16(&mut self, addr: usize, value: i16) {
        self.bytes[addr..addr + 2].copy_from_slice(&value.to_le_bytes());
    }

    fn store_i32(&mut self, addr: usize, value: i32) {
        self.bytes[addr..addr + 4].copy_from_slice(&value.to_le_bytes());
    }

    fn store_i64(&mut self, addr: usize, value: i64) {
        self.bytes[addr..addr + 8].copy_from_slice(&value.to_le_bytes());
    }
}

#[derive(Default, Debug)]
struct CallFrame {
    fn_index: u32,
    locals: Vec<WasmValue>,
}

struct BlockState {
    loc: usize,
    kind: WasmBlockKind,
    should_skip: bool,
}

// host fns

struct SupportedHostFn {
    module_name: &'static str,
    fn_name: &'static str,
    fn_inputs: &'static [WasmType],
    fn_outputs: &'static [WasmType],
}

static SUPPORTED_HOST_FNS: [SupportedHostFn; 12] = [
    SupportedHostFn {
        module_name: "utils",
        fn_name: "debug",
        fn_inputs: &[WasmType::I32],
        fn_outputs: &[],
    },
    SupportedHostFn {
        module_name: "utils",
        fn_name: "debug_str",
        fn_inputs: &[WasmType::I32, WasmType::I32],
        fn_outputs: &[],
    },
    SupportedHostFn {
        module_name: "wasi_snapshot_preview1",
        fn_name: "path_open",
        fn_inputs: &[
            WasmType::I32,
            WasmType::I32,
            WasmType::I32,
            WasmType::I32,
            WasmType::I32,
            WasmType::I64,
            WasmType::I64,
            WasmType::I32,
            WasmType::I32,
        ],
        fn_outputs: &[WasmType::I32],
    },
    SupportedHostFn {
        module_name: "wasi_snapshot_preview1",
        fn_name: "fd_read",
        fn_inputs: &[WasmType::I32, WasmType::I32, WasmType::I32, WasmType::I32],
        fn_outputs: &[WasmType::I32],
    },
    SupportedHostFn {
        module_name: "wasi_snapshot_preview1",
        fn_name: "fd_write",
        fn_inputs: &[WasmType::I32, WasmType::I32, WasmType::I32, WasmType::I32],
        fn_outputs: &[WasmType::I32],
    },
    SupportedHostFn {
        module_name: "wasi_snapshot_preview1",
        fn_name: "fd_close",
        fn_inputs: &[WasmType::I32],
        fn_outputs: &[WasmType::I32],
    },
    SupportedHostFn {
        module_name: "wasi_snapshot_preview1",
        fn_name: "args_sizes_get",
        fn_inputs: &[WasmType::I32, WasmType::I32],
        fn_outputs: &[WasmType::I32],
    },
    SupportedHostFn {
        module_name: "wasi_snapshot_preview1",
        fn_name: "args_get",
        fn_inputs: &[WasmType::I32, WasmType::I32],
        fn_outputs: &[WasmType::I32],
    },
    SupportedHostFn {
        module_name: "wasi_snapshot_preview1",
        fn_name: "proc_exit",
        fn_inputs: &[WasmType::I32],
        fn_outputs: &[],
    },
    SupportedHostFn {
        module_name: "wasi_snapshot_preview1",
        fn_name: "fd_prestat_get",
        fn_inputs: &[WasmType::I32, WasmType::I32],
        fn_outputs: &[WasmType::I32],
    },
    SupportedHostFn {
        module_name: "wasi_snapshot_preview1",
        fn_name: "fd_prestat_dir_name",
        fn_inputs: &[WasmType::I32, WasmType::I32, WasmType::I32],
        fn_outputs: &[WasmType::I32],
    },
    SupportedHostFn {
        module_name: "wasi_snapshot_preview1",
        fn_name: "fd_fdstat_get",
        fn_inputs: &[WasmType::I32, WasmType::I32],
        fn_outputs: &[WasmType::I32],
    },
];

fn call_host_fn(eval: &mut WasmEval, fn_index: u32) -> Result<(), EvalError> {
    let state = &mut eval.state;
    let fn_name = &state.host_fns[fn_index as usize];
    match &fn_name[..] {
        "utils::debug" => {
            let value = state.pop_i32() as u32;
            debug(format!("{value}"));
        }
        "utils::debug_str" => {
            let message_len = state.pop_i32() as u32;
            let message_ptr = state.pop_i32() as u32;
            let message_bytes = &state.memory.bytes
                [message_ptr as usize..message_ptr as usize + message_len as usize];

            let message = str::from_utf8(message_bytes).unwrap();
            stderr_write(message);
        }
        "wasi_snapshot_preview1::fd_prestat_get" => {
            let buf = state.pop_i32();
            let fd = state.pop_i32();

            match unsafe { wasi::fd_prestat_get(fd as u32) } {
                Ok(prestat) => {
                    let pr_name_len = unsafe { prestat.u.dir.pr_name_len as i32 };
                    state.memory.store_i32(buf as usize, prestat.tag as i32);
                    state.memory.store_i32(buf as usize + 4, pr_name_len);

                    state.stack.push(WasmValue::I32 { value: 0 });
                }
                Err(err) => state.stack.push(WasmValue::I32 {
                    value: err.raw() as i32,
                }),
            }
        }
        "wasi_snapshot_preview1::fd_prestat_dir_name" => {
            let path_len = state.pop_i32();
            let path = state.pop_i32();
            let fd = state.pop_i32();

            let layout = Layout::array::<u8>(path_len as usize).unwrap();
            let path_buf = unsafe { alloc(layout) };

            match unsafe { wasi::fd_prestat_dir_name(fd as u32, path_buf, path_len as usize) } {
                Ok(()) => {
                    let path_slice = unsafe {
                        ::core::ptr::slice_from_raw_parts(path_buf, path_len as usize)
                            .as_ref()
                            .unwrap()
                    };
                    state.memory.bytes[path as usize..path as usize + path_len as usize]
                        .copy_from_slice(path_slice);

                    state.stack.push(WasmValue::I32 { value: 0 })
                }
                Err(err) => state.stack.push(WasmValue::I32 {
                    value: err.raw() as i32,
                }),
            }

            unsafe {
                dealloc(path_buf, layout);
            }
        }
        "wasi_snapshot_preview1::fd_fdstat_get" => {
            let fdstat_ptr = state.pop_i32();
            let fd = state.pop_i32();

            match unsafe { wasi::fd_fdstat_get(fd as u32) } {
                Ok(fdstat) => {
                    state.memory.bytes[fdstat_ptr as usize] = fdstat.fs_filetype.raw();
                    state
                        .memory
                        .store_i16(fdstat_ptr as usize + 2, fdstat.fs_flags as i16);
                    state
                        .memory
                        .store_i64(fdstat_ptr as usize + 8, fdstat.fs_rights_base as i64);
                    state
                        .memory
                        .store_i64(fdstat_ptr as usize + 16, fdstat.fs_rights_inheriting as i64);

                    state.stack.push(WasmValue::I32 { value: 0 });
                }
                Err(err) => state.stack.push(WasmValue::I32 {
                    value: err.raw() as i32,
                }),
            }
        }
        "wasi_snapshot_preview1::path_open" => {
            let fd_ptr = state.pop_i32();
            let fdflags = state.pop_i32();
            let fs_rights_inheriting = state.pop_i64();
            let fs_rights_base = state.pop_i64();
            let oflags = state.pop_i32();
            let path_len = state.pop_i32();
            let path_ptr = state.pop_i32();
            let dirflags = state.pop_i32();
            let dirfd = state.pop_i32();

            let path_bytes =
                &state.memory.bytes[path_ptr as usize..path_ptr as usize + path_len as usize];

            match unsafe {
                wasi::path_open(
                    dirfd as u32,
                    dirflags as u32,
                    str::from_utf8(path_bytes).unwrap(),
                    oflags as u16,
                    fs_rights_base as u64,
                    fs_rights_inheriting as u64,
                    fdflags as u16,
                )
            } {
                Ok(fd) => {
                    state.stack.push(WasmValue::I32 { value: 0 });
                    state.memory.store_i32(fd_ptr as usize, fd as i32);
                }
                Err(err) => state.stack.push(WasmValue::I32 {
                    value: err.raw() as i32,
                }),
            }
        }
        "wasi_snapshot_preview1::fd_write" => {
            let nwritten_ptr = state.pop_i32();
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
                Ok(nwritten) => {
                    state.stack.push(WasmValue::I32 { value: 0 });
                    state
                        .memory
                        .store_i32(nwritten_ptr as usize, nwritten as i32);
                }
                Err(err) => state.stack.push(WasmValue::I32 {
                    value: err.raw() as i32,
                }),
            };
        }
        "wasi_snapshot_preview1::fd_read" => {
            let nread_ptr = state.pop_i32();
            let iovs_len = state.pop_i32();
            let iovs_ptr = state.pop_i32();
            let fd = state.pop_i32();

            let mut iovs = Vec::new();
            for i in 0..iovs_len {
                let iov_base = iovs_ptr as usize + (i as usize * 8);
                let str_ptr = state.memory.load_i32(iov_base);
                let str_len = state.memory.load_i32(iov_base + 4);

                let buf = (&mut state.memory.bytes[str_ptr as usize]) as *mut u8;
                iovs.push(wasi::Iovec {
                    buf,
                    buf_len: str_len as usize,
                })
            }

            match unsafe { wasi::fd_read(fd as u32, &iovs) } {
                Ok(nread) => {
                    state.stack.push(WasmValue::I32 { value: 0 });
                    state.memory.store_i32(nread_ptr as usize, nread as i32);
                }
                Err(err) => state.stack.push(WasmValue::I32 {
                    value: err.raw() as i32,
                }),
            };
        }
        "wasi_snapshot_preview1::fd_close" => {
            let fd = state.pop_i32();

            match unsafe { wasi::fd_close(fd as u32) } {
                Ok(()) => state.stack.push(WasmValue::I32 { value: 0 }),
                Err(err) => state.stack.push(WasmValue::I32 {
                    value: err.raw() as i32,
                }),
            }
        }
        _ => {
            return Err(EvalError {
                message: format!("Host fn '{fn_name}' is not implemented"),
            })
        }
    }

    Ok(())
}

fn unsafe_borrow<T>(x: &T) -> &'static T {
    unsafe { &*(x as *const T) }
}
