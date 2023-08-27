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

// TODO(perf): Where temporary section buffer is needed one buffer can be shared
pub fn write_binary(out: &mut Vec<u8>, module: &WasmModule) {
    write_magic_and_version(out, module);
    write_type_section(out, module);
    write_import_section(out, module);
    write_function_section(out, module);
    write_memory_section(out, module);
    write_global_section(out, module);
    write_export_section(out, module);
    write_code_section(out, module);
    write_data_section(out, module);
}

fn write_magic_and_version(out: &mut Vec<u8>, _: &WasmModule) {
    // wasm magic number
    out.extend_from_slice(b"\0asm");

    // version
    out.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);
}

fn write_type_section(out: &mut Vec<u8>, module: &WasmModule) {
    out.push(SECTION_TYPE);

    let mut type_section = Vec::new();

    {
        write_u32(&mut type_section, module.types.len() as u32);
        for fn_type in &module.types {
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

    write_u32(out, type_section.len() as u32);
    out.append(&mut type_section);
}

fn write_import_section(out: &mut Vec<u8>, module: &WasmModule) {
    out.push(SECTION_IMPORT);

    let mut import_section = Vec::new();

    {
        write_u32(&mut import_section, module.imports.len() as u32);
        for import in &module.imports {
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

    write_u32(out, import_section.len() as u32);
    out.append(&mut import_section);
}

fn write_function_section(out: &mut Vec<u8>, module: &WasmModule) {
    out.push(SECTION_FUNC);

    write_u32(out, (module.functions.len() + 1) as u32);

    write_u32(out, module.functions.len() as u32);
    for type_index in &module.functions {
        write_u32(out, *type_index);
    }
}

fn write_memory_section(out: &mut Vec<u8>, module: &WasmModule) {
    out.push(SECTION_MEMORY);

    let mut memory_section = Vec::new();

    {
        write_u32(&mut memory_section, module.memories.len() as u32);
        for memory in &module.memories {
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

    write_u32(out, memory_section.len() as u32);
    out.append(&mut memory_section);
}

fn write_global_section(out: &mut Vec<u8>, module: &WasmModule) {
    out.push(SECTION_GLOBAL);

    let mut global_section = Vec::new();

    {
        write_u32(&mut global_section, module.globals.len() as u32);
        for global in &module.globals {
            global_section.push(global.kind.value_type as u8);

            if global.kind.mutable {
                global_section.push(0x01);
            } else {
                global_section.push(0x00);
            }

            write_expr(&mut global_section, &global.initial_value);
        }
    }

    write_u32(out, global_section.len() as u32);
    out.append(&mut global_section);
}

fn write_export_section(out: &mut Vec<u8>, module: &WasmModule) {
    out.push(SECTION_EXPORT);

    let mut export_section = Vec::new();

    {
        write_u32(&mut export_section, module.exports.len() as u32);
        for export in &module.exports {
            write_u32(&mut export_section, export.export_name.len() as u32);
            export_section.extend_from_slice(export.export_name.as_bytes());

            export_section.push(export.export_type as u8);

            write_u32(&mut export_section, export.exported_item_index);
        }
    }

    write_u32(out, export_section.len() as u32);
    out.append(&mut export_section);
}

fn write_code_section(out: &mut Vec<u8>, module: &WasmModule) {
    out.push(SECTION_CODE);

    let mut code_section = Vec::new();

    {
        let mut fn_section = Vec::new();

        write_u32(&mut code_section, module.codes.len() as u32);
        for fn_code in &module.codes {
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

    write_u32(out, code_section.len() as u32);
    out.append(&mut code_section);
}

fn write_data_section(out: &mut Vec<u8>, module: &WasmModule) {
    out.push(SECTION_DATA);

    let mut data_section = Vec::new();

    {
        write_u32(&mut data_section, module.datas.borrow().len() as u32);
        for data in module.datas.borrow().iter() {
            let WasmData::Active { offset, bytes } = data;
            write_u32(&mut data_section, 0);
            write_expr(&mut data_section, offset);
            write_u32(&mut data_section, bytes.len() as u32);
            data_section.extend(bytes);
        }
    }

    write_u32(out, data_section.len() as u32);
    out.append(&mut data_section);
}

fn write_expr(out: &mut Vec<u8>, expr: &WasmExpr) {
    for instr in &expr.instrs {
        write_instr(out, instr);
    }

    out.push(0x0b); // end
}

fn write_instr(out: &mut Vec<u8>, instr: &WasmInstr) {
    match instr {
        WasmInstr::NoEmit { instr: _ } => {}
        WasmInstr::NoTypeCheck { instr } => {
            write_instr(out, instr);
        }
        WasmInstr::Unreachable => {
            out.push(0x00);
        }
        WasmInstr::BinaryOp { kind, lhs, rhs } => {
            write_instr(out, lhs);
            write_instr(out, rhs);
            out.push(*kind as u8);
        }
        WasmInstr::MemorySize => {
            out.push(0x3F);
            out.push(0x00);
        }
        WasmInstr::MemoryGrow { size } => {
            write_instr(out, size);
            out.push(0x40);
            out.push(0x00);
        }
        WasmInstr::Load {
            kind,
            align,
            offset,
            address_instr,
        } => {
            write_instr(out, address_instr);
            out.push(*kind as u8);
            write_u32(out, *align);
            write_u32(out, *offset);
        }
        WasmInstr::I32Const { value } => {
            out.push(0x41);
            write_i32(out, *value);
        }
        WasmInstr::I32ConstLazy { value } => {
            out.push(0x41);
            write_i32(out, *value.borrow());
        }
        WasmInstr::I64Const { value } => {
            out.push(0x42);
            write_i64(out, *value);
        }
        WasmInstr::LocalGet { local_index } => {
            out.push(0x20);
            write_u32(out, *local_index);
        }
        WasmInstr::GlobalGet { global_index } => {
            out.push(0x23);
            write_u32(out, *global_index);
        }
        WasmInstr::Set { bind } => match bind {
            WasmSetBind::Local { index } => {
                out.push(0x21);
                write_u32(out, *index);
            }
            WasmSetBind::Global { index } => {
                out.push(0x24);
                write_u32(out, *index);
            }
            WasmSetBind::Memory {
                align,
                offset,
                kind,
                address_instr,
                value_local_index,
            } => {
                // set_local(tmp, <value on top of the stack>)
                out.push(0x21);
                write_u32(out, *value_local_index);

                write_instr(out, address_instr);

                // get_local(tmp)
                out.push(0x20);
                write_u32(out, *value_local_index);

                out.push(*kind as u8);

                write_u32(out, *align);
                write_u32(out, *offset);
            }
        },
        WasmInstr::StructGet {
            base_index: _,
            struct_name: _,
            primitive_gets,
        } => {
            for value in primitive_gets {
                write_instr(out, value);
            }
        }
        WasmInstr::StructLoad {
            struct_name: _,
            address_instr,
            address_local_index,
            base_byte_offset: _,
            primitive_loads,
        } => {
            write_instr(out, address_instr);
            write_instr(
                out,
                &WasmInstr::Set {
                    bind: WasmSetBind::Local {
                        index: *address_local_index,
                    },
                },
            );

            for value in primitive_loads {
                write_instr(out, value);
            }
        }
        WasmInstr::MultiValueEmit { values } => {
            for value in values {
                write_instr(out, value);
            }
        }
        WasmInstr::Drop { value, drop_count } => {
            write_instr(out, value);
            for _ in 0..*drop_count {
                out.push(0x1A);
            }
        }
        WasmInstr::Return { value } => {
            write_instr(out, value);
            out.push(0x0f);
        }
        WasmInstr::Loop { instrs } => {
            out.push(0x02); // begin block
            out.push(0x40); // no value

            {
                out.push(0x03); // begin loop
                out.push(0x40); // no value

                {
                    for instr in instrs {
                        write_instr(out, instr);
                    }

                    // loop implicitly
                    out.push(0x0c); // br
                    write_u32(out, 0);
                }

                out.push(0x0b); // end loop
            }

            out.push(0x0b); // end block
        }
        // to break the loop we need to:
        // 1. break out of if branch (br 0)
        // 2. end loop iteration with (br 1)
        // 3. end surrounding loop block (br 2)
        // NOTE: calling break or continue outside of if branch is undefined
        WasmInstr::LoopBreak => {
            out.push(0x0c); // br
            write_u32(out, 2);
        }
        // to `continue` in the loop we need to:
        // 1. break out of if branch (br 0)
        // 2. end loop iteration with (br 1)
        // NOTE: calling break or continue outside of if branch is undefined
        WasmInstr::LoopContinue => {
            out.push(0x0c); // br
            write_u32(out, 1);
        }
        WasmInstr::Call {
            fn_index,
            fn_type_index: _,
            args,
        } => {
            for arg in args {
                write_instr(out, arg);
            }
            out.push(0x10);
            write_u32(out, *fn_index);
        }
        WasmInstr::If {
            block_type,
            cond,
            then_branch,
            else_branch,
        } => {
            write_instr(out, cond);
            out.push(0x04); // if
            out.push((*block_type) as u8);
            write_instr(out, then_branch);
            out.push(0x05); // then
            write_instr(out, else_branch);
            out.push(0x0b); // end
        }
        WasmInstr::IfSingleBranch { cond, then_branch } => {
            write_instr(out, cond);
            out.push(0x04); // if
            out.push(0x40); // no value
            write_instr(out, then_branch);
            out.push(0x0b); // end
        }
    }
}

fn write_u32(out: &mut Vec<u8>, value: u32) {
    mini_leb128::write_u32(out, value).unwrap();
}

fn write_i32(out: &mut Vec<u8>, value: i32) {
    mini_leb128::write_i32(out, value).unwrap();
}

fn write_i64(out: &mut Vec<u8>, value: i64) {
    mini_leb128::write_i64(out, value).unwrap();
}
