use crate::wasm_module::{Instr, WasmModule};
use alloc::vec::Vec;
use mini_leb128::{write_i32, write_u32};

const FUNC_TYPE: u8 = 0x60;

const SECTION_TYPE: u8 = 0x01;
const SECTION_FUNC: u8 = 0x03;
const SECTION_EXPORT: u8 = 0x07;
const SECTION_CODE: u8 = 0x0a;

const EXPR_END_OPCODE: u8 = 0x0b;

pub struct BinaryBuilder<'a> {
    module: &'a WasmModule,
    data: Vec<u8>,
}

// TODO(optimize): Where temporary section buffer is needed one buffer can be shared
impl<'a> BinaryBuilder<'a> {
    pub fn new(module: &'a WasmModule) -> Self {
        let data = Vec::new();
        Self { module, data }
    }

    pub fn build(mut self) -> Vec<u8> {
        self.emit_magic_and_version();
        self.emit_type_section();
        self.emit_func_section();
        self.emit_export_section();
        self.emit_code_section();
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
            write_u32(&mut type_section, self.module.fn_types.len() as u32).unwrap();
            for fn_type in &self.module.fn_types {
                type_section.push(FUNC_TYPE);

                write_u32(&mut type_section, fn_type.inputs.len() as u32).unwrap();
                for &fn_input in &fn_type.inputs {
                    type_section.push(fn_input as u8);
                }

                write_u32(&mut type_section, fn_type.outputs.len() as u32).unwrap();
                for &fn_output in &fn_type.outputs {
                    type_section.push(fn_output as u8);
                }
            }
        }

        write_u32(&mut self.data, type_section.len() as u32).unwrap();
        self.data.append(&mut type_section);
    }

    /**
    Currently functions and their types map 1 to 1.

    TODO(optimize): Functions with equivalent types can point to the same type
    */
    fn emit_func_section(&mut self) {
        self.data.push(SECTION_FUNC);

        write_u32(&mut self.data, (self.module.fn_types.len() + 1) as u32).unwrap();

        write_u32(&mut self.data, self.module.fn_types.len() as u32).unwrap();
        for i in 0..self.module.fn_types.len() {
            write_u32(&mut self.data, i as u32).unwrap();
        }
    }

    fn emit_export_section(&mut self) {
        self.data.push(SECTION_EXPORT);

        let mut export_section = Vec::new();

        {
            write_u32(&mut export_section, self.module.exports.len() as u32).unwrap();
            for export in &self.module.exports {
                write_u32(&mut export_section, export.export_name.len() as u32).unwrap();
                export_section.extend_from_slice(export.export_name.as_bytes());

                export_section.push(export.export_type as u8);

                write_u32(&mut export_section, export.exported_item_index as u32).unwrap();
            }
        }

        write_u32(&mut self.data, export_section.len() as u32).unwrap();
        self.data.append(&mut export_section);
    }

    fn emit_code_section(&mut self) {
        self.data.push(SECTION_CODE);

        let mut code_section = Vec::new();

        {
            let mut fn_section = Vec::new();

            write_u32(&mut code_section, self.module.fn_codes.len() as u32).unwrap();
            for fn_code in &self.module.fn_codes {
                {
                    write_u32(&mut fn_section, fn_code.locals.len() as u32).unwrap();
                    for locals_of_some_type in &fn_code.locals {
                        write_u32(&mut fn_section, locals_of_some_type.count as u32).unwrap();
                        fn_section.push(locals_of_some_type.value_type as u8);
                    }

                    for instr in &fn_code.expr.instrs {
                        write_instr(&mut fn_section, instr);
                    }

                    fn_section.push(EXPR_END_OPCODE);
                }

                write_u32(&mut code_section, fn_section.len() as u32).unwrap();
                code_section.append(&mut fn_section);
            }
        }

        write_u32(&mut self.data, code_section.len() as u32).unwrap();
        self.data.append(&mut code_section);
    }
}

fn write_instr(output: &mut Vec<u8>, instr: &Instr) {
    match instr {
        Instr::Return => output.push(0x0f),
        Instr::I32Const(value) => {
            output.push(0x41);
            write_i32(output, *value).unwrap();
        }
    }
}
