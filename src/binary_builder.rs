use crate::wasm_module::{WasmData, WasmExpr, WasmImportDesc, WasmInstr, WasmModule, WasmSetBind};
use alloc::vec::Vec;

const SECTION_TYPE: u8 = 0x01;
const SECTION_IMPORT: u8 = 0x02;
const SECTION_FUNC: u8 = 0x03;
const SECTION_MEMORY: u8 = 0x05;
const SECTION_GLOBAL: u8 = 0x06;
const SECTION_EXPORT: u8 = 0x07;
const SECTION_CODE: u8 = 0x0a;
const SECTION_DATA: u8 = 0x0b;

pub struct BinaryBuilder<'a> {
    module: &'a WasmModule,
    data: Vec<u8>,
}

// TODO(perf): Where temporary section buffer is needed one buffer can be shared
impl<'a> BinaryBuilder<'a> {
    pub fn new(module: &'a WasmModule) -> Self {
        let data = Vec::new();
        Self { module, data }
    }

    pub fn build(mut self) -> Vec<u8> {
        self.emit_magic_and_version();
        self.emit_type_section();
        self.emit_import_section();
        self.emit_function_section();
        self.emit_memory_section();
        self.emit_global_section();
        self.emit_export_section();
        self.emit_code_section();
        self.emit_data_section();
        self.data
    }

    fn emit_magic_and_version(&mut self) {
        // wasm magic number
        self.data.extend_from_slice(b"\0asm");

        // version
        self.data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);
    }

    fn emit_type_section(&mut self) {
        self.data.push(SECTION_TYPE);

        let mut type_section = Vec::new();

        {
            write_u32(&mut type_section, self.module.types.len() as u32);
            for fn_type in &self.module.types {
                type_section.push(0x60); // func type

                write_u32(&mut type_section, fn_type.inputs.len() as u32);
                for &fn_input in &fn_type.inputs {
                    type_section.push(fn_input as u8);
                }

                write_u32(&mut type_section, fn_type.outputs.len() as u32);
                for &fn_output in &fn_type.outputs {
                    type_section.push(fn_output as u8);
                }
            }
        }

        write_u32(&mut self.data, type_section.len() as u32);
        self.data.append(&mut type_section);
    }

    fn emit_import_section(&mut self) {
        self.data.push(SECTION_IMPORT);

        let mut import_section = Vec::new();

        {
            write_u32(&mut import_section, self.module.imports.len() as u32);
            for import in &self.module.imports {
                write_u32(&mut import_section, import.module_name.len() as u32);
                import_section.extend_from_slice(import.module_name.as_bytes());

                write_u32(&mut import_section, import.item_name.len() as u32);
                import_section.extend_from_slice(import.item_name.as_bytes());

                match import.item_desc {
                    WasmImportDesc::Func { type_index } => {
                        write_u32(&mut import_section, 0x00); // fn
                        write_u32(&mut import_section, type_index);
                    }
                }
            }
        }

        write_u32(&mut self.data, import_section.len() as u32);
        self.data.append(&mut import_section);
    }

    // TODO(optimize): Functions with equivalent types can point to the same type
    fn emit_function_section(&mut self) {
        self.data.push(SECTION_FUNC);

        write_u32(&mut self.data, (self.module.functions.len() + 1) as u32);

        write_u32(&mut self.data, self.module.functions.len() as u32);
        for type_index in &self.module.functions {
            write_u32(&mut self.data, *type_index);
        }
    }

    fn emit_memory_section(&mut self) {
        self.data.push(SECTION_MEMORY);

        let mut memory_section = Vec::new();

        {
            write_u32(&mut memory_section, self.module.memories.len() as u32);
            for memory in &self.module.memories {
                if let Some(memory_max) = memory.max {
                    memory_section.push(0x01);
                    write_u32(&mut memory_section, memory.min as u32);
                    write_u32(&mut memory_section, memory_max as u32);
                } else {
                    memory_section.push(0x00);
                    write_u32(&mut memory_section, memory.min as u32);
                }
            }
        }

        write_u32(&mut self.data, memory_section.len() as u32);
        self.data.append(&mut memory_section);
    }

    fn emit_global_section(&mut self) {
        self.data.push(SECTION_GLOBAL);

        let mut global_section = Vec::new();

        {
            write_u32(&mut global_section, self.module.globals.len() as u32);
            for global in &self.module.globals {
                global_section.push(global.kind.value_type as u8);

                if global.kind.mutable {
                    global_section.push(0x01);
                } else {
                    global_section.push(0x00);
                }

                write_expr(&mut global_section, &global.initial_value);
            }
        }

        write_u32(&mut self.data, global_section.len() as u32);
        self.data.append(&mut global_section);
    }

    fn emit_export_section(&mut self) {
        self.data.push(SECTION_EXPORT);

        let mut export_section = Vec::new();

        {
            write_u32(&mut export_section, self.module.exports.len() as u32);
            for export in &self.module.exports {
                write_u32(&mut export_section, export.export_name.len() as u32);
                export_section.extend_from_slice(export.export_name.as_bytes());

                export_section.push(export.export_type as u8);

                write_u32(&mut export_section, export.exported_item_index);
            }
        }

        write_u32(&mut self.data, export_section.len() as u32);
        self.data.append(&mut export_section);
    }

    fn emit_code_section(&mut self) {
        self.data.push(SECTION_CODE);

        let mut code_section = Vec::new();

        {
            let mut fn_section = Vec::new();

            write_u32(&mut code_section, self.module.codes.len() as u32);
            for fn_code in &self.module.codes {
                {
                    write_u32(&mut fn_section, fn_code.locals.len() as u32);
                    for locals_of_some_type in &fn_code.locals {
                        write_u32(&mut fn_section, locals_of_some_type.count as u32);
                        fn_section.push(locals_of_some_type.value_type as u8);
                    }

                    write_expr(&mut fn_section, &fn_code.expr);
                }

                write_u32(&mut code_section, fn_section.len() as u32);
                code_section.append(&mut fn_section);
            }
        }

        write_u32(&mut self.data, code_section.len() as u32);
        self.data.append(&mut code_section);
    }

    fn emit_data_section(&mut self) {
        self.data.push(SECTION_DATA);

        let mut data_section = Vec::new();

        {
            write_u32(&mut data_section, self.module.datas.borrow().len() as u32);
            for data in self.module.datas.borrow().iter() {
                let WasmData::Active { offset, bytes } = data;
                write_u32(&mut data_section, 0);
                write_expr(&mut data_section, offset);
                write_u32(&mut data_section, bytes.len() as u32);
                data_section.extend(bytes);
            }
        }

        write_u32(&mut self.data, data_section.len() as u32);
        self.data.append(&mut data_section);
    }
}

fn write_expr(output: &mut Vec<u8>, expr: &WasmExpr) {
    for instr in &expr.instrs {
        write_instr(output, instr);
    }

    output.push(0x0b); // end
}

fn write_instr(output: &mut Vec<u8>, instr: &WasmInstr) {
    match instr {
        WasmInstr::NoEmit { instr: _ } => {}
        WasmInstr::NoTypeCheck { instr } => {
            write_instr(output, instr);
        }
        WasmInstr::Unreachable => {
            output.push(0x00);
        }
        WasmInstr::BinaryOp { kind, lhs, rhs } => {
            write_instr(output, lhs);
            write_instr(output, rhs);
            output.push(*kind as u8);
        }
        WasmInstr::MemorySize => {
            output.push(0x3F);
            output.push(0x00);
        }
        WasmInstr::MemoryGrow { size } => {
            write_instr(output, size);
            output.push(0x40);
            output.push(0x00);
        }
        WasmInstr::Load {
            kind,
            align,
            offset,
            address_instr,
        } => {
            write_instr(output, address_instr);
            output.push(*kind as u8);
            write_u32(output, *align);
            write_u32(output, *offset);
        }
        WasmInstr::I32Const { value } => {
            output.push(0x41);
            write_i32(output, *value);
        }
        WasmInstr::I32ConstLazy { value } => {
            output.push(0x41);
            write_i32(output, *value.borrow());
        }
        WasmInstr::I64Const { value } => {
            output.push(0x42);
            write_i64(output, *value);
        }
        WasmInstr::LocalGet { local_index } => {
            output.push(0x20);
            write_u32(output, *local_index);
        }
        WasmInstr::GlobalGet { global_index } => {
            output.push(0x23);
            write_u32(output, *global_index);
        }
        WasmInstr::Set { bind } => match bind {
            WasmSetBind::Local { index } => {
                output.push(0x21);
                write_u32(output, *index);
            }
            WasmSetBind::Global { index } => {
                output.push(0x24);
                write_u32(output, *index);
            }
            WasmSetBind::Memory {
                align,
                offset,
                kind,
                address_instr,
                value_local_index,
            } => {
                // set_local(tmp, <value on top of the stack>)
                output.push(0x21);
                write_u32(output, *value_local_index);

                write_instr(output, address_instr);

                // get_local(tmp)
                output.push(0x20);
                write_u32(output, *value_local_index);

                output.push(*kind as u8);

                write_u32(output, *align);
                write_u32(output, *offset);
            }
        },
        WasmInstr::StructGet {
            base_index: _,
            struct_name: _,
            primitive_gets,
        } => {
            for value in primitive_gets {
                write_instr(output, value);
            }
        }
        WasmInstr::StructLoad {
            struct_name: _,
            address_instr,
            address_local_index,
            base_byte_offset: _,
            primitive_loads,
        } => {
            write_instr(output, address_instr);
            write_instr(
                output,
                &WasmInstr::Set {
                    bind: WasmSetBind::Local {
                        index: *address_local_index,
                    },
                },
            );

            for value in primitive_loads {
                write_instr(output, value);
            }
        }
        WasmInstr::MultiValueEmit { values } => {
            for value in values {
                write_instr(output, value);
            }
        }
        WasmInstr::Drop { value, drop_count } => {
            write_instr(output, value);
            for _ in 0..*drop_count {
                output.push(0x1A);
            }
        }
        WasmInstr::Return { value } => {
            write_instr(output, value);
            output.push(0x0f);
        }
        WasmInstr::Loop { instrs } => {
            output.push(0x02); // begin block
            output.push(0x40); // no value

            {
                output.push(0x03); // begin loop
                output.push(0x40); // no value

                {
                    for instr in instrs {
                        write_instr(output, instr);
                    }

                    // loop implicitly
                    output.push(0x0c); // br
                    write_u32(output, 0);
                }

                output.push(0x0b); // end loop
            }

            output.push(0x0b); // end block
        }
        // to break the loop we need to:
        // 1. break out of if branch (br 0)
        // 2. end loop iteration with (br 1)
        // 3. end surrounding loop block (br 2)
        // NOTE: calling break or continue outside of if branch is undefined
        WasmInstr::LoopBreak => {
            output.push(0x0c); // br
            write_u32(output, 2);
        }
        // to `continue` in the loop we need to:
        // 1. break out of if branch (br 0)
        // 2. end loop iteration with (br 1)
        // NOTE: calling break or continue outside of if branch is undefined
        WasmInstr::LoopContinue => {
            output.push(0x0c); // br
            write_u32(output, 1);
        }
        WasmInstr::Call {
            fn_index,
            fn_type_index: _,
            args,
        } => {
            for arg in args {
                write_instr(output, arg);
            }
            output.push(0x10);
            write_u32(output, *fn_index);
        }
        WasmInstr::If {
            block_type,
            cond,
            then_branch,
            else_branch,
        } => {
            write_instr(output, cond);
            output.push(0x04); // if
            output.push((*block_type) as u8);
            write_instr(output, then_branch);
            output.push(0x05); // then
            write_instr(output, else_branch);
            output.push(0x0b); // end
        }
        WasmInstr::IfSingleBranch { cond, then_branch } => {
            write_instr(output, cond);
            output.push(0x04); // if
            output.push(0x40); // no value
            write_instr(output, then_branch);
            output.push(0x0b); // end
        }
    }
}

fn write_u32(output: &mut Vec<u8>, value: u32) {
    mini_leb128::write_u32(output, value).unwrap();
}

fn write_i32(output: &mut Vec<u8>, value: i32) {
    mini_leb128::write_i32(output, value).unwrap();
}

fn write_i64(output: &mut Vec<u8>, value: i64) {
    mini_leb128::write_i64(output, value).unwrap();
}
