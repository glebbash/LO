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
    tables: Vec<WasmFnTable>,
    globals: Vec<WasmValue>,
    stack: Vec<WasmValue>,
    call_stack: Vec<CallFrame>,
    memory: LinearMemory,
    host_fns: Vec<String>,
    jump_tables: Vec<(u32, JumpTable)>,
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
            self.eval_expr(
                &global.initial_value,
                &JumpTable::for_expr(&global.initial_value),
            )?;
            let initial_value = self.stack.pop().unwrap();
            self.globals.push(initial_value);
        }

        for table in &self.wasm_module.tables {
            self.tables.push(WasmFnTable {
                fns: vec![None; table.limits.min as usize],
            });
        }

        for element in unsafe_borrow(&self.wasm_module.elements) {
            match element {
                WasmElement::Passive { expr, fn_idx } => {
                    self.eval_expr(expr, &JumpTable::for_expr(expr))?;
                    let start_index = self.pop_i32() as usize;

                    for (fn_index, i) in fn_idx.iter().zip(0..) {
                        self.tables[0].fns[start_index + i] = Some(*fn_index);
                    }
                }
            }
        }

        if let Some(memory) = self.wasm_module.memories.first() {
            self.memory = LinearMemory {
                size_in_pages: memory.min as usize,
                bytes: vec![0; memory.min as usize * PAGE_SIZE],
            };

            for data in unsafe_borrow(&self.wasm_module.datas) {
                match data {
                    WasmData::Active { offset, bytes } => {
                        self.eval_expr(offset, &JumpTable::for_expr(offset))?;
                        let offset = self.pop_i32() as usize;

                        self.memory.bytes[offset..offset + bytes.len()].copy_from_slice(&bytes);
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
                        self.host_fns.push(full_name);
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

        for (fn_code, i) in self.wasm_module.codes.iter().zip(0..) {
            let fn_index = (self.fn_imports_len + i) as u32;
            let jump_table = JumpTable::for_expr(&fn_code.expr);
            self.jump_tables.push((fn_index, jump_table));
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
            values.push(self.stack.pop().unwrap());
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
            let value = self.stack.pop().unwrap();
            call_frame.locals.push(value);
        }
        call_frame.locals.reverse();
        for local in &code.locals {
            for _ in 0..local.count {
                let value = WasmValue::default_for_type(&local.value_type);
                call_frame.locals.push(value);
            }
        }
        self.call_stack.push(call_frame);

        let jump_table = self.get_jump_table_for_fn(fn_index);
        self.eval_expr(&code.expr, &jump_table)?;

        self.call_stack.pop();

        Ok(())
    }

    fn eval_expr(&mut self, expr: &WasmExpr, jump_table: &JumpTable) -> Result<(), EvalError> {
        let mut loc = 0;
        while loc < expr.instrs.len() {
            let instr = &expr.instrs[loc];

            match instr {
                WasmInstr::BlockStart { block_kind, .. } => {
                    if let WasmBlockKind::If = block_kind {
                        let cond = self.pop_i32();
                        if cond == 0 {
                            loc = jump_table.get_jump_loc(loc);
                            continue;
                        }
                    }
                }
                WasmInstr::Else { .. } | WasmInstr::Branch { .. } => {
                    loc = jump_table.get_jump_loc(loc);
                    continue;
                }
                WasmInstr::BranchIf { .. } => {
                    let cond = self.pop_i32();
                    if cond == 0 {
                        loc = jump_table.get_jump_loc(loc);
                        continue;
                    }
                }
                WasmInstr::BranchIndirect { .. } => {
                    todo!();
                }
                WasmInstr::BlockEnd => {}

                WasmInstr::Return => {
                    break;
                }
                WasmInstr::Call { fn_index } => {
                    self.call_fn(*fn_index)?;
                }
                WasmInstr::CallIndirect {
                    type_index,
                    table_index,
                } => {
                    let fn_index_in_table = self.pop_i32();

                    let Some(table) = self.tables.get(*table_index as usize) else {
                        return Err(EvalError {
                            message: format!("Invalid table index: {table_index}"),
                        });
                    };

                    let Some(func_ref) = table.fns.get(fn_index_in_table as usize) else {
                        return Err(EvalError {
                            message: format!(
                                "Function index out of table bounds: {fn_index_in_table}, table id: {table_index}, table size: {}",
                                table.fns.len()
                            ),
                        });
                    };

                    let Some(fn_index) = func_ref else {
                        return Err(EvalError {
                            message: format!("Trying to call indirect on ref.null",),
                        });
                    };

                    // TODO: type check indirect function calls
                    let _ = type_index;

                    self.call_fn(*fn_index)?;
                }

                WasmInstr::I32Const { value } => {
                    let value = WasmValue::I32 { value: *value };
                    self.stack.push(value);
                }
                WasmInstr::I64Const { value } => {
                    let value = WasmValue::I64 { value: *value };
                    self.stack.push(value);
                }
                WasmInstr::F32Const { value } => {
                    let value = WasmValue::F32 { value: *value };
                    self.stack.push(value);
                }
                WasmInstr::F64Const { value } => {
                    let value = WasmValue::F64 { value: *value };
                    self.stack.push(value);
                }

                WasmInstr::LocalGet { local_index } => {
                    let frame = self.call_stack.last_mut().unwrap();
                    let value = frame.locals[*local_index as usize].clone();
                    self.stack.push(value);
                }
                WasmInstr::LocalSet { local_index } => {
                    let value = self.stack.pop().unwrap();
                    let frame = self.call_stack.last_mut().unwrap();
                    frame.locals[*local_index as usize] = value;
                }
                WasmInstr::LocalTee { local_index } => {
                    let value = self.stack.last().unwrap();
                    let frame = self.call_stack.last_mut().unwrap();
                    frame.locals[*local_index as usize] = value.clone();
                }
                WasmInstr::GlobalGet { global_index } => {
                    let value = self.globals[*global_index as usize].clone();
                    self.stack.push(value);
                }
                WasmInstr::GlobalSet { global_index } => {
                    let value = self.stack.pop().unwrap();
                    self.globals[*global_index as usize] = value;
                }
                WasmInstr::Load {
                    kind,
                    align: _,
                    offset,
                } => match kind {
                    WasmLoadKind::I32 => {
                        let addr = self.pop_i32();
                        let full_addr = addr as usize + *offset as usize;
                        let value = self.memory.load_i32(full_addr);
                        self.stack.push(WasmValue::I32 { value });
                    }
                    WasmLoadKind::I32U8 => {
                        let addr = self.pop_i32();
                        let full_addr = addr as usize + *offset as usize;
                        let Some(value) = self.memory.bytes.get(full_addr).cloned() else {
                            return Err(self.err_with_stack(format!(
                                "Memory read out of bounds: {full_addr}"
                            )));
                        };

                        self.stack.push(WasmValue::I32 {
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
                        let value = self.pop_i32();
                        let addr = self.pop_i32();
                        let full_addr = addr as usize + *offset as usize;
                        self.memory.store_i32(full_addr, value);
                    }
                    WasmStoreKind::I32_8 => {
                        let value = self.pop_i32();
                        let addr = self.pop_i32();
                        let full_addr = addr as usize + *offset as usize;
                        self.memory.bytes[full_addr] = value as u8;
                    }
                    WasmStoreKind::I64_8 => {
                        let value = self.pop_i64();
                        let addr = self.pop_i32();
                        let full_addr = addr as usize + *offset as usize;
                        self.memory.bytes[full_addr] = value as u8;
                    }
                    WasmStoreKind::I64_32 => {
                        let value = self.pop_i64();
                        let addr = self.pop_i32();
                        let full_addr = addr as usize + *offset as usize;
                        self.memory.store_i32(full_addr, value as i32);
                    }
                    _ => todo!("store {kind:?}"),
                },
                WasmInstr::Select => {
                    let cond = self.pop_i32();
                    let rhs = self.stack.pop().unwrap();
                    let lhs = self.stack.pop().unwrap();

                    if cond == 0 {
                        self.stack.push(lhs);
                    } else {
                        self.stack.push(rhs);
                    }
                }

                WasmInstr::Drop => {
                    let _ = self.stack.pop().unwrap();
                }
                WasmInstr::Unreachable => {
                    return Err(self.err_with_stack("Hit unreachable trap"));
                }
                WasmInstr::MemorySize => {
                    let mem_size = self.memory.size_in_pages as i32;
                    self.stack.push(WasmValue::I32 { value: mem_size });
                }
                WasmInstr::MemoryCopy => {
                    let num_bytes = self.pop_i32();
                    let source = self.pop_i32();
                    let destination = self.pop_i32();

                    self.memory.bytes.copy_within(
                        source as usize..source as usize + num_bytes as usize,
                        destination as usize,
                    );
                }
                WasmInstr::MemoryFill => {
                    let num_bytes = self.pop_i32();
                    let value = self.pop_i32();
                    let destination = self.pop_i32();

                    self.memory
                        .bytes
                        .get_mut(destination as usize..destination as usize + num_bytes as usize)
                        .unwrap()
                        .fill_with(|| value as u8);
                }
                WasmInstr::MemoryGrow => todo!("{instr:?}"),

                WasmInstr::I64ExtendI32u => {
                    let value = self.pop_i32();
                    self.stack.push(WasmValue::I64 {
                        value: value as u32 as u64 as i64,
                    })
                }
                WasmInstr::I64ExtendI32s => {
                    let value = self.pop_i32();
                    self.stack.push(WasmValue::I64 {
                        value: value as i64,
                    })
                }
                WasmInstr::I32WrapI64 => {
                    let value = self.pop_i64();
                    self.stack.push(WasmValue::I32 {
                        value: value as i32,
                    })
                }
                WasmInstr::I64ReinterpretF64 => {
                    let value = self.pop_f64();
                    self.stack.push(WasmValue::I64 {
                        value: value as i64,
                    })
                }
                WasmInstr::F64ReinterpretI64 => {
                    let value = self.pop_i64();
                    self.stack.push(WasmValue::F64 {
                        value: value as f64,
                    })
                }
                WasmInstr::UnaryOp { kind } => match kind {
                    WasmUnaryOpKind::I32_EQZ => {
                        let op = self.pop_i32();
                        let value = if op == 0 { 1 } else { 0 };
                        self.stack.push(WasmValue::I32 { value });
                    }
                    WasmUnaryOpKind::I64_EQZ => {
                        let op = self.pop_i64();
                        let value = if op == 0 { 1 } else { 0 };
                        self.stack.push(WasmValue::I32 { value });
                    }
                    WasmUnaryOpKind::F64_NEG => {
                        let op = self.pop_f64();
                        let value = -op;
                        self.stack.push(WasmValue::F64 { value });
                    }
                },
                WasmInstr::BinaryOp { kind } => match kind {
                    WasmBinaryOpKind::I32_ADD => {
                        let rhs = self.pop_i32();
                        let lhs = self.pop_i32();
                        let value = lhs + rhs;
                        self.stack.push(WasmValue::I32 { value });
                    }
                    WasmBinaryOpKind::I32_SUB => {
                        let rhs = self.pop_i32();
                        let lhs = self.pop_i32();
                        let value = lhs - rhs;
                        self.stack.push(WasmValue::I32 { value });
                    }
                    WasmBinaryOpKind::I32_MUL => {
                        let rhs = self.pop_i32();
                        let lhs = self.pop_i32();
                        let value = lhs * rhs;
                        self.stack.push(WasmValue::I32 { value });
                    }
                    WasmBinaryOpKind::I32_AND => {
                        let rhs = self.pop_i32();
                        let lhs = self.pop_i32();
                        let value = lhs & rhs;
                        self.stack.push(WasmValue::I32 { value })
                    }
                    WasmBinaryOpKind::I32_OR => {
                        let rhs = self.pop_i32();
                        let lhs = self.pop_i32();
                        let value = lhs | rhs;
                        self.stack.push(WasmValue::I32 { value })
                    }
                    WasmBinaryOpKind::I32_XOR => {
                        let rhs = self.pop_i32();
                        let lhs = self.pop_i32();
                        let value = lhs ^ rhs;
                        self.stack.push(WasmValue::I32 { value })
                    }
                    WasmBinaryOpKind::I32_DIV_U => {
                        let rhs = self.pop_i32();
                        let lhs = self.pop_i32();
                        let value = ((lhs as u32) / (rhs as u32)) as i32;
                        self.stack.push(WasmValue::I32 { value });
                    }
                    WasmBinaryOpKind::I32_SHR_U => {
                        let rhs = self.pop_i32();
                        let lhs = self.pop_i32();
                        let value = ((lhs as u32) >> (rhs as u32)) as i32;
                        self.stack.push(WasmValue::I32 { value });
                    }
                    WasmBinaryOpKind::I32_REM_U => {
                        let rhs = self.pop_i32();
                        let lhs = self.pop_i32();
                        let value = ((lhs as u32) % (rhs as u32)) as i32;
                        self.stack.push(WasmValue::I32 { value });
                    }
                    WasmBinaryOpKind::I32_EQ => {
                        let rhs = self.pop_i32();
                        let lhs = self.pop_i32();
                        let value = if lhs == rhs { 1 } else { 0 };
                        self.stack.push(WasmValue::I32 { value })
                    }
                    WasmBinaryOpKind::I32_NE => {
                        let rhs = self.pop_i32();
                        let lhs = self.pop_i32();
                        let value = if lhs != rhs { 1 } else { 0 };
                        self.stack.push(WasmValue::I32 { value })
                    }
                    WasmBinaryOpKind::I32_GT_S => {
                        let rhs = self.pop_i32();
                        let lhs = self.pop_i32();
                        let value = if lhs > rhs { 1 } else { 0 };
                        self.stack.push(WasmValue::I32 { value })
                    }
                    WasmBinaryOpKind::I32_LT_U => {
                        let rhs = self.pop_i32();
                        let lhs = self.pop_i32();
                        let value = if (lhs as u32) < (rhs as u32) { 1 } else { 0 };
                        self.stack.push(WasmValue::I32 { value })
                    }
                    WasmBinaryOpKind::I32_GT_U => {
                        let rhs = self.pop_i32();
                        let lhs = self.pop_i32();
                        let value = if (lhs as u32) > (rhs as u32) { 1 } else { 0 };
                        self.stack.push(WasmValue::I32 { value })
                    }
                    WasmBinaryOpKind::I32_GE_U => {
                        let rhs = self.pop_i32();
                        let lhs = self.pop_i32();
                        let value = if (lhs as u32) >= (rhs as u32) { 1 } else { 0 };
                        self.stack.push(WasmValue::I32 { value })
                    }
                    WasmBinaryOpKind::I32_LE_U => {
                        let rhs = self.pop_i32();
                        let lhs = self.pop_i32();
                        let value = if (lhs as u32) <= (rhs as u32) { 1 } else { 0 };
                        self.stack.push(WasmValue::I32 { value })
                    }
                    WasmBinaryOpKind::I32_LT_S
                    | WasmBinaryOpKind::I32_LE_S
                    | WasmBinaryOpKind::I32_GE_S
                    | WasmBinaryOpKind::I32_DIV_S
                    | WasmBinaryOpKind::I32_REM_S
                    | WasmBinaryOpKind::I32_SHL
                    | WasmBinaryOpKind::I32_SHR_S => todo!("{kind:?}"),

                    WasmBinaryOpKind::I64_MUL => {
                        let rhs = self.pop_i64();
                        let lhs = self.pop_i64();
                        let value = lhs * rhs;
                        self.stack.push(WasmValue::I64 { value })
                    }
                    WasmBinaryOpKind::I64_EQ => {
                        let rhs = self.pop_i64();
                        let lhs = self.pop_i64();
                        let value = if lhs == rhs { 1 } else { 0 };
                        self.stack.push(WasmValue::I32 { value })
                    }
                    WasmBinaryOpKind::I64_DIV_U => {
                        let rhs = self.pop_i64();
                        let lhs = self.pop_i64();
                        let value = (lhs as u64 / rhs as u64) as i64;
                        self.stack.push(WasmValue::I64 { value })
                    }
                    WasmBinaryOpKind::I64_REM_U => {
                        let rhs = self.pop_i64();
                        let lhs = self.pop_i64();
                        let value = (lhs as u64 % rhs as u64) as i64;
                        self.stack.push(WasmValue::I64 { value })
                    }
                    WasmBinaryOpKind::I64_NE
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
                    | WasmBinaryOpKind::I64_DIV_S
                    | WasmBinaryOpKind::I64_REM_S
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
                },
            }

            loc += 1;
        }

        Ok(())
    }

    fn get_jump_table_for_fn(&mut self, fn_index: u32) -> &'static JumpTable {
        let jt_index = self
            .jump_tables
            .binary_search_by_key(&fn_index, |jt| jt.0)
            .unwrap();

        unsafe_borrow(&self.jump_tables[jt_index].1)
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

        while let Some(frame) = self.call_stack.pop() {
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

    fn pop_i32(&mut self) -> i32 {
        let wasm_value = self.stack.pop().unwrap();
        let WasmValue::I32 { value } = wasm_value else {
            let err = self.err_with_stack(format!(
                "Trying to pop I32 but got {:?}",
                wasm_value.get_type()
            ));
            stderr_write(format!("Error: {}\n", err.message));
            proc_exit(1);
        };

        value
    }

    fn pop_i64(&mut self) -> i64 {
        let wasm_value = self.stack.pop().unwrap();
        let WasmValue::I64 { value } = wasm_value else {
            let err = self.err_with_stack(format!(
                "Trying to pop I64 but got {:?}",
                wasm_value.get_type()
            ));
            stderr_write(format!("Error: {}\n", err.message));
            proc_exit(1);
        };

        value
    }

    fn pop_f64(&mut self) -> f64 {
        let wasm_value = self.stack.pop().unwrap();
        let WasmValue::F64 { value } = wasm_value else {
            let err = self.err_with_stack(format!(
                "Trying to pop F64 but got {:?}",
                wasm_value.get_type()
            ));
            stderr_write(format!("Error: {}\n", err.message));
            proc_exit(1);
        };

        value
    }
}

// values

#[derive(Debug, Clone)]
pub enum WasmValue {
    I32 { value: i32 },
    I64 { value: i64 },
    F32 { value: f32 },
    F64 { value: f64 },
    FuncRef { fn_index: Option<u32> },
}

impl WasmValue {
    fn default_for_type(wasm_type: &WasmType) -> Self {
        match wasm_type {
            WasmType::I32 => WasmValue::I32 { value: 0 },
            WasmType::I64 => WasmValue::I64 { value: 0 },
            WasmType::F32 => WasmValue::F32 { value: 0.0 },
            WasmType::F64 => WasmValue::F64 { value: 0.0 },
            WasmType::FuncRef => WasmValue::FuncRef { fn_index: None },
        }
    }

    fn get_type(&self) -> WasmType {
        match self {
            WasmValue::I32 { .. } => WasmType::I32,
            WasmValue::I64 { .. } => WasmType::I64,
            WasmValue::F32 { .. } => WasmType::F32,
            WasmValue::F64 { .. } => WasmType::F64,
            WasmValue::FuncRef { .. } => WasmType::FuncRef,
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
            WasmValue::FuncRef { fn_index } => match fn_index {
                Some(fn_index) => write!(f, "<ref.func {fn_index}>"),
                None => write!(f, "<ref.null>"),
            },
        }
    }
}

// state

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

#[derive(Default, Debug)]
struct JumpTable {
    jumps: Vec<(usize, usize)>, // (from, to)
}

impl JumpTable {
    fn for_expr(expr: &WasmExpr) -> Self {
        struct BlockInfo {
            loc: usize,
            kind: WasmBlockKind,
            unresolved_branches: Vec<usize>,
            had_else: bool,
        }

        let mut jump_table = JumpTable::default();

        let mut blocks = Vec::<BlockInfo>::new();
        for (instr, loc) in expr.instrs.iter().zip(0..) {
            match instr {
                WasmInstr::BlockStart { block_kind, .. } => {
                    blocks.push(BlockInfo {
                        loc,
                        kind: block_kind.clone(),
                        unresolved_branches: Vec::new(),
                        had_else: false,
                    });
                }
                WasmInstr::Else => {
                    let block = blocks.last_mut().unwrap();
                    assert_eq!(block.kind, WasmBlockKind::If);
                    jump_table.jumps.push((block.loc, loc + 1));
                    block.had_else = true;
                    block.unresolved_branches.push(loc);
                }
                WasmInstr::Branch { label_index } => {
                    let blocks_len = blocks.len();
                    let target_block = blocks
                        .get_mut(blocks_len - 1 - *label_index as usize)
                        .unwrap();

                    match target_block.kind {
                        WasmBlockKind::Loop => {
                            jump_table.jumps.push((loc, target_block.loc + 1));
                        }
                        _ => {
                            target_block.unresolved_branches.push(loc);
                        }
                    }
                }
                WasmInstr::BranchIndirect { .. } => {
                    todo!()
                }
                WasmInstr::BlockEnd => {
                    let block = blocks.pop().unwrap();

                    for branch_loc in block.unresolved_branches {
                        jump_table.jumps.push((branch_loc, loc));
                    }

                    if let WasmBlockKind::If = block.kind {
                        if !block.had_else {
                            jump_table.jumps.push((block.loc, loc));
                        }
                    }
                }
                _ => {}
            }
        }

        // jumps from branches are created out of order so need to sort
        jump_table.jumps.sort_by_key(|&(from, _)| from);

        jump_table
    }

    fn get_jump_loc(&self, from_loc: usize) -> usize {
        let jump_index = self
            .jumps
            .binary_search_by_key(&from_loc, |(from, _)| *from);
        self.jumps[jump_index.unwrap()].1
    }
}

struct WasmFnTable {
    fns: Vec<Option<u32>>,
}

// host fns

struct SupportedHostFn {
    module_name: &'static str,
    fn_name: &'static str,
    fn_inputs: &'static [WasmType],
    fn_outputs: &'static [WasmType],
}

static SUPPORTED_HOST_FNS: [SupportedHostFn; 13] = [
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
        fn_name: "fd_seek",
        fn_inputs: &[WasmType::I32, WasmType::I64, WasmType::I32, WasmType::I32],
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
    let fn_name = &eval.host_fns[fn_index as usize];
    match &fn_name[..] {
        "utils::debug" => {
            let value = eval.pop_i32() as u32;
            debug(format!("{value}"));
        }
        "utils::debug_str" => {
            let message_len = eval.pop_i32() as u32;
            let message_ptr = eval.pop_i32() as u32;
            let message_bytes = &eval.memory.bytes
                [message_ptr as usize..message_ptr as usize + message_len as usize];

            let message = str::from_utf8(message_bytes).unwrap();
            stderr_write(message);
        }
        "wasi_snapshot_preview1::fd_prestat_get" => {
            let buf = eval.pop_i32();
            let fd = eval.pop_i32();

            match unsafe { wasi::fd_prestat_get(fd as u32) } {
                Ok(prestat) => {
                    let pr_name_len = unsafe { prestat.u.dir.pr_name_len as i32 };
                    eval.memory.store_i32(buf as usize, prestat.tag as i32);
                    eval.memory.store_i32(buf as usize + 4, pr_name_len);

                    eval.stack.push(WasmValue::I32 { value: 0 });
                }
                Err(err) => eval.stack.push(WasmValue::I32 {
                    value: err.raw() as i32,
                }),
            }
        }
        "wasi_snapshot_preview1::fd_prestat_dir_name" => {
            let path_len = eval.pop_i32();
            let path = eval.pop_i32();
            let fd = eval.pop_i32();

            let layout = Layout::array::<u8>(path_len as usize).unwrap();
            let path_buf = unsafe { alloc(layout) };

            match unsafe { wasi::fd_prestat_dir_name(fd as u32, path_buf, path_len as usize) } {
                Ok(()) => {
                    let path_slice = unsafe {
                        ::core::ptr::slice_from_raw_parts(path_buf, path_len as usize)
                            .as_ref()
                            .unwrap()
                    };
                    eval.memory.bytes[path as usize..path as usize + path_len as usize]
                        .copy_from_slice(path_slice);

                    eval.stack.push(WasmValue::I32 { value: 0 })
                }
                Err(err) => eval.stack.push(WasmValue::I32 {
                    value: err.raw() as i32,
                }),
            }

            unsafe {
                dealloc(path_buf, layout);
            }
        }
        "wasi_snapshot_preview1::fd_fdstat_get" => {
            let fdstat_ptr = eval.pop_i32();
            let fd = eval.pop_i32();

            match unsafe { wasi::fd_fdstat_get(fd as u32) } {
                Ok(fdstat) => {
                    eval.memory.bytes[fdstat_ptr as usize] = fdstat.fs_filetype.raw();
                    eval.memory
                        .store_i16(fdstat_ptr as usize + 2, fdstat.fs_flags as i16);
                    eval.memory
                        .store_i64(fdstat_ptr as usize + 8, fdstat.fs_rights_base as i64);
                    eval.memory
                        .store_i64(fdstat_ptr as usize + 16, fdstat.fs_rights_inheriting as i64);

                    eval.stack.push(WasmValue::I32 { value: 0 });
                }
                Err(err) => eval.stack.push(WasmValue::I32 {
                    value: err.raw() as i32,
                }),
            }
        }
        "wasi_snapshot_preview1::path_open" => {
            let fd_ptr = eval.pop_i32();
            let fdflags = eval.pop_i32();
            let fs_rights_inheriting = eval.pop_i64();
            let fs_rights_base = eval.pop_i64();
            let oflags = eval.pop_i32();
            let path_len = eval.pop_i32();
            let path_ptr = eval.pop_i32();
            let dirflags = eval.pop_i32();
            let dirfd = eval.pop_i32();

            let path_bytes =
                &eval.memory.bytes[path_ptr as usize..path_ptr as usize + path_len as usize];

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
                    eval.stack.push(WasmValue::I32 { value: 0 });
                    eval.memory.store_i32(fd_ptr as usize, fd as i32);
                }
                Err(err) => eval.stack.push(WasmValue::I32 {
                    value: err.raw() as i32,
                }),
            }
        }
        "wasi_snapshot_preview1::fd_write" => {
            let nwritten_ptr = eval.pop_i32();
            let iovs_len = eval.pop_i32();
            let iovs_ptr = eval.pop_i32();
            let fd = eval.pop_i32();

            let mut iovs = Vec::new();
            for i in 0..iovs_len {
                let iov_base = iovs_ptr as usize + (i as usize * 8);
                let str_ptr = eval.memory.load_i32(iov_base);
                let str_len = eval.memory.load_i32(iov_base + 4);

                let buf = (&mut eval.memory.bytes[str_ptr as usize]) as *const u8;
                iovs.push(wasi::Ciovec {
                    buf,
                    buf_len: str_len as usize,
                });
            }

            match unsafe { wasi::fd_write(fd as u32, &iovs) } {
                Ok(nwritten) => {
                    eval.stack.push(WasmValue::I32 { value: 0 });
                    eval.memory
                        .store_i32(nwritten_ptr as usize, nwritten as i32);
                }
                Err(err) => eval.stack.push(WasmValue::I32 {
                    value: err.raw() as i32,
                }),
            };
        }
        "wasi_snapshot_preview1::fd_read" => {
            let nread_ptr = eval.pop_i32();
            let iovs_len = eval.pop_i32();
            let iovs_ptr = eval.pop_i32();
            let fd = eval.pop_i32();

            let mut iovs = Vec::new();
            for i in 0..iovs_len {
                let iov_base = iovs_ptr as usize + (i as usize * 8);
                let str_ptr = eval.memory.load_i32(iov_base);
                let str_len = eval.memory.load_i32(iov_base + 4);

                let buf = (&mut eval.memory.bytes[str_ptr as usize]) as *mut u8;
                iovs.push(wasi::Iovec {
                    buf,
                    buf_len: str_len as usize,
                })
            }

            match unsafe { wasi::fd_read(fd as u32, &iovs) } {
                Ok(nread) => {
                    eval.stack.push(WasmValue::I32 { value: 0 });
                    eval.memory.store_i32(nread_ptr as usize, nread as i32);
                }
                Err(err) => eval.stack.push(WasmValue::I32 {
                    value: err.raw() as i32,
                }),
            };
        }
        "wasi_snapshot_preview1::fd_close" => {
            let fd = eval.pop_i32();

            match unsafe { wasi::fd_close(fd as u32) } {
                Ok(()) => eval.stack.push(WasmValue::I32 { value: 0 }),
                Err(err) => eval.stack.push(WasmValue::I32 {
                    value: err.raw() as i32,
                }),
            }
        }
        "wasi_snapshot_preview1::args_sizes_get" => {
            let argv_buf_size_ptr = eval.pop_i32();
            let argc_ptr = eval.pop_i32();

            match unsafe { wasi::args_sizes_get() } {
                Ok((argc, argv_buf_size)) => {
                    eval.memory.store_i32(argc_ptr as usize, argc as i32);
                    eval.memory
                        .store_i32(argv_buf_size_ptr as usize, argv_buf_size as i32);

                    eval.stack.push(WasmValue::I32 { value: 0 });
                }
                Err(err) => eval.stack.push(WasmValue::I32 {
                    value: err.raw() as i32,
                }),
            }
        }
        "wasi_snapshot_preview1::args_get" => {
            let argv_buf_ptr = eval.pop_i32();
            let argv_ptr = eval.pop_i32();

            let argv = &mut eval.memory.bytes[argv_ptr as usize] as *mut u8 as *mut *mut u8;
            let argv_buf = &mut eval.memory.bytes[argv_buf_ptr as usize] as *mut u8;

            match unsafe { wasi::args_get(argv, argv_buf) } {
                Ok(()) => {
                    // fix argv pointers to point to guest memory instead of host memory
                    {
                        let mem_base = (&eval.memory.bytes).as_ptr() as usize;

                        let (argc, _) = unsafe { wasi::args_sizes_get() }.unwrap();
                        for i in 0..argc {
                            unsafe {
                                let argv_i = argv.add(i);
                                *argv_i = (((*argv_i) as usize) - mem_base) as *mut u8;
                            }
                        }
                    };

                    eval.stack.push(WasmValue::I32 { value: 0 });
                }
                Err(err) => eval.stack.push(WasmValue::I32 {
                    value: err.raw() as i32,
                }),
            }
        }
        "wasi_snapshot_preview1::proc_exit" => {
            let exit_code = eval.pop_i32();
            proc_exit(exit_code as u32);
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
