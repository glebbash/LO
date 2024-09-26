use crate::wasm::*;
use alloc::{format, string::String, vec::Vec};
use core::str;

#[derive(Default)]
pub struct WasmParser {
    file_name: String,
    file_bytes: Vec<u8>,
    offset: usize,
    module: WasmModule,
}

impl WasmParser {
    pub fn parse(file_name: String, module_bytes: Vec<u8>) -> Result<WasmModule, String> {
        let mut parser = WasmParser {
            file_name,
            file_bytes: module_bytes,
            ..Default::default()
        };

        parser.parse_module()?;

        Ok(parser.module)
    }

    fn parse_module(&mut self) -> Result<(), String> {
        self.parse_magic_and_version()?;

        loop {
            let Some(section_id) = self.eat_any() else {
                break;
            };

            match section_id {
                0x01 => self.parse_type_section()?,
                0x02 => self.parse_import_section()?,
                0x03 => self.parse_function_section()?,
                0x04 => self.parse_table_section()?,
                0x05 => self.parse_memory_section()?,
                0x06 => self.parse_global_section()?,
                0x07 => self.parse_export_section()?,
                0x09 => self.parse_element_section()?,
                0x0A => self.parse_code_section()?,
                0x0B => self.parse_data_section()?,
                0x0C => self.parse_data_count_section()?,
                0x00 => self.parse_custom_section()?,
                byte => {
                    return Err(format!(
                        "{} Unknown section id: '0x{byte:02X}'",
                        self.loc_at(self.offset - 1)
                    ));
                }
            };
        }

        Ok(())
    }

    fn parse_magic_and_version(&mut self) -> Result<(), String> {
        self.expect(0x00)?;
        self.expect('a' as u8)?;
        self.expect('s' as u8)?;
        self.expect('m' as u8)?;

        self.expect(0x01)?;
        self.expect(0x00)?;
        self.expect(0x00)?;
        self.expect(0x00)?;

        Ok(())
    }

    fn parse_type_section(&mut self) -> Result<(), String> {
        let _section_size = self.parse_u32()?;

        let types_len = self.parse_u32()?;
        for _ in 0..types_len {
            self.expect(0x60)?; // func type

            let mut fn_type = WasmFnType {
                inputs: Vec::new(),
                outputs: Vec::new(),
            };

            let inputs_len = self.parse_u32()?;
            for _ in 0..inputs_len {
                fn_type.inputs.push(self.parse_type()?);
            }

            let outputs_len = self.parse_u32()?;
            for _ in 0..outputs_len {
                fn_type.outputs.push(self.parse_type()?);
            }

            self.module.types.push(fn_type);
        }

        Ok(())
    }

    fn parse_import_section(&mut self) -> Result<(), String> {
        let _section_size = self.parse_u32()?;

        let imports_len = self.parse_u32()?;
        for _ in 0..imports_len {
            let module_name = self.parse_string()?;
            let item_name = self.parse_string()?;

            let item_desc: WasmImportDesc;
            match self.expect_any()? {
                0x00 => {
                    let type_index = self.parse_u32()?;
                    item_desc = WasmImportDesc::Func { type_index };
                }
                0x02 => {
                    let limits = self.parse_memory_limits()?;
                    item_desc = WasmImportDesc::Memory(limits);
                }
                byte => {
                    return Err(format!(
                        "{} Unknown import item kind '0x{byte:02X}'",
                        self.loc_at(self.offset - 1)
                    ));
                }
            }

            self.module.imports.push(WasmImport {
                module_name,
                item_name,
                item_desc,
            });
        }

        Ok(())
    }

    fn parse_function_section(&mut self) -> Result<(), String> {
        let _section_size = self.parse_u32()?;

        let fns_len = self.parse_u32()?;
        for _ in 0..fns_len {
            let type_index = self.parse_u32()?;
            self.module.functions.push(type_index);
        }

        Ok(())
    }

    // TODO: support parsing table section
    fn parse_table_section(&mut self) -> Result<(), String> {
        let _section_size = self.parse_u32()?;

        self.expect_many(_section_size as usize)?;

        Ok(())
    }

    fn parse_memory_section(&mut self) -> Result<(), String> {
        let _section_size = self.parse_u32()?;

        let memories_len = self.parse_u32()?;
        for _ in 0..memories_len {
            let limits = self.parse_memory_limits()?;
            self.module.memories.push(limits);
        }

        Ok(())
    }

    fn parse_global_section(&mut self) -> Result<(), String> {
        let _section_size = self.parse_u32()?;

        let globals_len = self.parse_u32()?;
        for _ in 0..globals_len {
            let value_type = self.parse_type()?;

            let mutable: bool;
            match self.expect_any()? {
                0x00 => {
                    mutable = false;
                }
                0x01 => {
                    mutable = true;
                }
                byte => {
                    return Err(format!(
                        "{} Unknown memory mutability kind '0x{byte:02X}'",
                        self.loc_at(self.offset - 1)
                    ))
                }
            }

            let initial_value = self.parse_expr()?;

            self.module.globals.push(WasmGlobal {
                mutable,
                value_type,
                initial_value,
            })
        }

        Ok(())
    }

    fn parse_export_section(&mut self) -> Result<(), String> {
        let _section_size = self.parse_u32()?;

        let exports_len = self.parse_u32()?;
        for _ in 0..exports_len {
            let export_name = self.parse_string()?;
            let export_type = match self.expect_any()? {
                0x00 => WasmExportType::Func,
                0x02 => WasmExportType::Mem,
                byte => {
                    return Err(format!(
                        "{} Unknown export type '0x{byte:02X}'",
                        self.loc_at(self.offset - 1)
                    ))
                }
            };
            let exported_item_index = self.parse_u32()?;

            let export = WasmExport {
                export_type,
                export_name,
                exported_item_index,
            };
            self.module.exports.push(export);
        }

        Ok(())
    }

    // TODO: support parsing element section
    fn parse_element_section(&mut self) -> Result<(), String> {
        let _section_size = self.parse_u32()?;

        self.expect_many(_section_size as usize)?;

        Ok(())
    }

    fn parse_code_section(&mut self) -> Result<(), String> {
        let _section_size = self.parse_u32()?;

        let codes_len = self.parse_u32()?;
        for _ in 0..codes_len {
            let _subsection_len = self.parse_u32()?;

            let mut locals = Vec::new();

            let locals_len = self.parse_u32()?;
            for _ in 0..locals_len {
                let locals_of_type_count = self.parse_u32()?;
                let local_type = self.parse_type()?;

                locals.push(WasmLocals {
                    count: locals_of_type_count,
                    value_type: local_type,
                });
            }

            let expr = self.parse_expr()?;

            let wasm_fn = WasmFn { locals, expr };
            self.module.codes.push(wasm_fn);
        }

        Ok(())
    }

    fn parse_data_section(&mut self) -> Result<(), String> {
        let _section_size = self.parse_u32()?;

        let datas_len = self.parse_u32()?;
        for _ in 0..datas_len {
            let data_kind = self.parse_u32()?;
            if data_kind != 0 {
                return Err(format!(
                    "{} Only data kind 0 is supported, got: {data_kind}'",
                    self.loc() // sadly this points to u32 end, not start
                ));
            }

            let offset = self.parse_expr()?;

            let bytes_len = self.parse_u32()?;
            let bytes = Vec::from(self.expect_many(bytes_len as usize)?);

            self.module.datas.push(WasmData::Active { offset, bytes });
        }

        Ok(())
    }

    // TODO: support parsing data count section
    fn parse_data_count_section(&mut self) -> Result<(), String> {
        let _section_size = self.parse_u32()?;

        self.expect_many(_section_size as usize)?;

        Ok(())
    }

    // TODO: support parsing custom section
    fn parse_custom_section(&mut self) -> Result<(), String> {
        let _section_size = self.parse_u32()?;

        self.expect_many(_section_size as usize)?;

        Ok(())
    }

    fn parse_memory_limits(&mut self) -> Result<WasmLimits, String> {
        match self.expect_any()? {
            0x00 => {
                let min = self.parse_u32()?;
                Ok(WasmLimits { min, max: None })
            }
            0x01 => {
                let min = self.parse_u32()?;
                let max = self.parse_u32()?;
                Ok(WasmLimits {
                    min,
                    max: Some(max),
                })
            }
            byte => Err(format!(
                "{} Unknown memory limit kind '0x{byte:02X}'",
                self.loc_at(self.offset - 1)
            )),
        }
    }

    fn parse_expr(&mut self) -> Result<WasmExpr, String> {
        let mut expr = WasmExpr { instrs: Vec::new() };
        let mut block_depth = 1;

        loop {
            match self.expect_any()? {
                0x00 => {
                    expr.instrs.push(WasmInstr::Unreachable);
                }
                op_code @ (0x02 | 0x03 | 0x04) => {
                    block_depth += 1;

                    let block_kind = match op_code {
                        0x02 => WasmBlockKind::Block,
                        0x03 => WasmBlockKind::Loop,
                        0x04 => WasmBlockKind::If,
                        _ => unreachable!(),
                    };

                    let block_type = if self.eat(0x40) {
                        WasmBlockType::NoOut
                    } else if let Some(wasm_type) = self.try_parse_type() {
                        WasmBlockType::SingleOut { wasm_type }
                    } else {
                        let type_index = self.parse_u32()?;
                        WasmBlockType::InOut { type_index }
                    };

                    expr.instrs.push(WasmInstr::BlockStart {
                        block_kind,
                        block_type,
                    });
                }
                0x05 => {
                    expr.instrs.push(WasmInstr::Else);
                }
                0x0B => {
                    block_depth -= 1;

                    if block_depth == 0 {
                        break;
                    }

                    expr.instrs.push(WasmInstr::BlockEnd);
                }
                0x0C => {
                    let label_index = self.parse_u32()?;
                    expr.instrs.push(WasmInstr::Branch { label_index });
                }
                0x0F => {
                    expr.instrs.push(WasmInstr::Return);
                }
                0x10 => {
                    let fn_index = self.parse_u32()?;
                    expr.instrs.push(WasmInstr::Call { fn_index });
                }
                0x1A => {
                    expr.instrs.push(WasmInstr::Drop);
                }
                0x20 => {
                    let local_index = self.parse_u32()?;
                    expr.instrs.push(WasmInstr::LocalGet { local_index });
                }
                0x21 => {
                    let local_index = self.parse_u32()?;
                    expr.instrs.push(WasmInstr::LocalSet { local_index });
                }
                0x23 => {
                    let global_index = self.parse_u32()?;
                    expr.instrs.push(WasmInstr::GlobalGet { global_index });
                }
                0x24 => {
                    let global_index = self.parse_u32()?;
                    expr.instrs.push(WasmInstr::GlobalSet { global_index });
                }
                op_code @ (0x28 | 0x29 | 0x2A | 0x2B | 0x2C | 0x2D | 0x2E | 0x2F) => {
                    let load_kind = match op_code {
                        0x28 => WasmLoadKind::I32,
                        0x29 => WasmLoadKind::I64,
                        0x2A => WasmLoadKind::F32,
                        0x2B => WasmLoadKind::F64,
                        0x2C => WasmLoadKind::I32I8,
                        0x2D => WasmLoadKind::I32U8,
                        0x2E => WasmLoadKind::I32I16,
                        0x2F => WasmLoadKind::I32U16,
                        _ => unreachable!(),
                    };
                    let align = self.parse_u32()?;
                    let offset = self.parse_u32()?;

                    expr.instrs.push(WasmInstr::Load {
                        kind: load_kind,
                        align,
                        offset,
                    });
                }
                op_code @ (0x36 | 0x37 | 0x38 | 0x39 | 0x3A | 0x3B) => {
                    let store_kind = match op_code {
                        0x36 => WasmStoreKind::I32,
                        0x37 => WasmStoreKind::I64,
                        0x38 => WasmStoreKind::F32,
                        0x39 => WasmStoreKind::F64,
                        0x3A => WasmStoreKind::I32U8,
                        0x3B => WasmStoreKind::I32U16,
                        _ => unreachable!(),
                    };
                    let align = self.parse_u32()?;
                    let offset = self.parse_u32()?;

                    expr.instrs.push(WasmInstr::Store {
                        kind: store_kind,
                        align,
                        offset,
                    });
                }
                0x3F => {
                    if !self.eat(0x00) {
                        return Err(format!("{} Only single memory is supported", self.loc()));
                    }

                    expr.instrs.push(WasmInstr::MemorySize);
                }
                0x40 => {
                    if !self.eat(0x00) {
                        return Err(format!("{} Only single memory is supported", self.loc()));
                    }

                    expr.instrs.push(WasmInstr::MemoryGrow);
                }
                0x41 => {
                    let value = self.parse_i32()?;
                    expr.instrs.push(WasmInstr::I32Const { value });
                }
                0x42 => {
                    let value = self.parse_i64()?;
                    expr.instrs.push(WasmInstr::I64Const { value });
                }
                0x43 => {
                    let bytes = self.expect_many(4)?;
                    let value = f32::from_le_bytes(bytes.try_into().unwrap());
                    expr.instrs.push(WasmInstr::F32Const { value });
                }
                0x44 => {
                    let bytes = self.expect_many(8)?;
                    let value = f64::from_le_bytes(bytes.try_into().unwrap());
                    expr.instrs.push(WasmInstr::F64Const { value });
                }
                op_code @ (0x46 | 0x47 | 0x48 | 0x49 | 0x4A | 0x4B | 0x4C | 0x4D | 0x4E | 0x4F
                | 0x51 | 0x52 | 0x53 | 0x54 | 0x55 | 0x56 | 0x57 | 0x58 | 0x59
                | 0x5A | 0x5B | 0x5C | 0x5D | 0x5E | 0x5F | 0x60 | 0x61 | 0x62
                | 0x63 | 0x64 | 0x65 | 0x66 | 0x6A | 0x6B | 0x6C | 0x6D | 0x6E
                | 0x6F | 0x70 | 0x71 | 0x72 | 0x74 | 0x75 | 0x76 | 0x7C | 0x7D
                | 0x7E | 0x7F | 0x80 | 0x81 | 0x82 | 0x83 | 0x84 | 0x86 | 0x87
                | 0x88 | 0x92 | 0x93 | 0x94 | 0x95 | 0xA0 | 0xA1 | 0xA2 | 0xA3) => {
                    let binary_op_kind = match op_code {
                        0x46 => WasmBinaryOpKind::I32_EQ,
                        0x47 => WasmBinaryOpKind::I32_NE,
                        0x48 => WasmBinaryOpKind::I32_LT_S,
                        0x49 => WasmBinaryOpKind::I32_LT_U,
                        0x4A => WasmBinaryOpKind::I32_GT_S,
                        0x4B => WasmBinaryOpKind::I32_GT_U,
                        0x4C => WasmBinaryOpKind::I32_LE_S,
                        0x4D => WasmBinaryOpKind::I32_LE_U,
                        0x4E => WasmBinaryOpKind::I32_GE_S,
                        0x4F => WasmBinaryOpKind::I32_GE_U,
                        0x51 => WasmBinaryOpKind::I64_EQ,
                        0x52 => WasmBinaryOpKind::I64_NE,
                        0x53 => WasmBinaryOpKind::I64_LT_S,
                        0x54 => WasmBinaryOpKind::I64_LT_U,
                        0x55 => WasmBinaryOpKind::I64_GT_S,
                        0x56 => WasmBinaryOpKind::I64_GT_U,
                        0x57 => WasmBinaryOpKind::I64_LE_S,
                        0x58 => WasmBinaryOpKind::I64_LE_U,
                        0x59 => WasmBinaryOpKind::I64_GE_S,
                        0x5A => WasmBinaryOpKind::I64_GE_U,
                        0x5B => WasmBinaryOpKind::F32_EQ,
                        0x5C => WasmBinaryOpKind::F32_NE,
                        0x5D => WasmBinaryOpKind::F32_LT,
                        0x5E => WasmBinaryOpKind::F32_GT,
                        0x5F => WasmBinaryOpKind::F32_LE,
                        0x60 => WasmBinaryOpKind::F32_GE,
                        0x61 => WasmBinaryOpKind::F64_EQ,
                        0x62 => WasmBinaryOpKind::F64_NE,
                        0x63 => WasmBinaryOpKind::F64_LT,
                        0x64 => WasmBinaryOpKind::F64_GT,
                        0x65 => WasmBinaryOpKind::F64_LE,
                        0x66 => WasmBinaryOpKind::F64_GE,
                        0x6A => WasmBinaryOpKind::I32_ADD,
                        0x6B => WasmBinaryOpKind::I32_SUB,
                        0x6C => WasmBinaryOpKind::I32_MUL,
                        0x6D => WasmBinaryOpKind::I32_DIV_S,
                        0x6E => WasmBinaryOpKind::I32_DIV_U,
                        0x6F => WasmBinaryOpKind::I32_REM_S,
                        0x70 => WasmBinaryOpKind::I32_REM_U,
                        0x71 => WasmBinaryOpKind::I32_AND,
                        0x72 => WasmBinaryOpKind::I32_OR,
                        0x74 => WasmBinaryOpKind::I32_SHL,
                        0x75 => WasmBinaryOpKind::I32_SHR_S,
                        0x76 => WasmBinaryOpKind::I32_SHR_U,
                        0x7C => WasmBinaryOpKind::I64_ADD,
                        0x7D => WasmBinaryOpKind::I64_SUB,
                        0x7E => WasmBinaryOpKind::I64_MUL,
                        0x7F => WasmBinaryOpKind::I64_DIV_S,
                        0x80 => WasmBinaryOpKind::I64_DIV_U,
                        0x81 => WasmBinaryOpKind::I64_REM_S,
                        0x82 => WasmBinaryOpKind::I64_REM_U,
                        0x83 => WasmBinaryOpKind::I64_AND,
                        0x84 => WasmBinaryOpKind::I64_OR,
                        0x86 => WasmBinaryOpKind::I64_SHL,
                        0x87 => WasmBinaryOpKind::I64_SHR_S,
                        0x88 => WasmBinaryOpKind::I64_SHR_U,
                        0x92 => WasmBinaryOpKind::F32_ADD,
                        0x93 => WasmBinaryOpKind::F32_SUB,
                        0x94 => WasmBinaryOpKind::F32_MUL,
                        0x95 => WasmBinaryOpKind::F32_DIV,
                        0xA0 => WasmBinaryOpKind::F64_ADD,
                        0xA1 => WasmBinaryOpKind::F64_SUB,
                        0xA2 => WasmBinaryOpKind::F64_MUL,
                        0xA3 => WasmBinaryOpKind::F64_DIV,
                        _ => unreachable!(),
                    };
                    expr.instrs.push(WasmInstr::BinaryOp {
                        kind: binary_op_kind,
                    });
                }
                0xA7 => {
                    expr.instrs.push(WasmInstr::I32WrapI64);
                }
                0xAC => {
                    expr.instrs.push(WasmInstr::I64ExtendI32s);
                }
                0xAD => {
                    expr.instrs.push(WasmInstr::I64ExtendI32u);
                }
                0xFC => {
                    if self.parse_u32()? != 10 {
                        return Err(format!("Only memory.copy is supported from 0xFC family"));
                    }

                    if !self.eat(0x00) {
                        return Err(format!("{} Only single memory is supported", self.loc()));
                    }

                    if !self.eat(0x00) {
                        return Err(format!("{} Only single memory is supported", self.loc()));
                    }

                    expr.instrs.push(WasmInstr::MemoryCopy);
                }
                byte => {
                    return Err(format!(
                        "{} Unknown instruction '0x{byte:02X}'",
                        self.loc_at(self.offset - 1)
                    ))
                }
            }
        }

        Ok(expr)
    }

    fn parse_type(&mut self) -> Result<WasmType, String> {
        let Some(wasm_type) = self.try_parse_type() else {
            let byte = self.expect_any()?;
            return Err(format!(
                "{} Unknown type '0x{byte:02X}'",
                self.loc_at(self.offset - 1)
            ));
        };

        Ok(wasm_type)
    }

    fn try_parse_type(&mut self) -> Option<WasmType> {
        let Ok(byte) = self.expect_any() else {
            return None;
        };

        match byte {
            0x7F => Some(WasmType::I32),
            0x7E => Some(WasmType::I64),
            0x7D => Some(WasmType::F32),
            0x7C => Some(WasmType::F64),
            _ => {
                self.offset -= 1;
                None
            }
        }
    }

    fn parse_string(&mut self) -> Result<String, String> {
        let string_size = self.parse_u32()?;
        let bytes = self.expect_many(string_size as usize)?;

        let Ok(string) = str::from_utf8(bytes) else {
            return Err(format!(
                "{} String is not UTF-8 encoded",
                self.loc_at(self.offset - string_size as usize)
            ));
        };

        Ok(String::from(string))
    }

    fn parse_u32(&mut self) -> Result<u32, String> {
        let mut result = 0;
        let mut shift = 0;

        loop {
            let byte = self.expect_any()?;

            result |= ((byte & 0x7F) as u32) << shift;

            if byte & 0x80 == 0 {
                break;
            }

            shift += 7;
            if shift >= 32 {
                return Err(format!(
                    "{} LEB128 u32 overflow",
                    self.loc_at(self.offset - 1)
                ));
            }
        }

        Ok(result)
    }

    fn parse_i32(&mut self) -> Result<i32, String> {
        let mut result = 0;
        let mut shift = 0;
        let mut byte;

        loop {
            byte = self.expect_any()?;

            result |= ((byte & 0x7F) as i32) << shift;

            if byte & 0x80 == 0 {
                break;
            }

            shift += 7;
            if shift >= 32 {
                return Err(format!(
                    "{} LEB128 i32 overflow",
                    self.loc_at(self.offset - 1)
                ));
            }
        }

        if (shift < 32) && (byte & 0x40 != 0) {
            result |= !0 << shift;
        }

        Ok(result)
    }

    fn parse_i64(&mut self) -> Result<i64, String> {
        let mut result = 0;
        let mut shift = 0;
        let mut byte;

        loop {
            byte = self.expect_any()?;

            result |= ((byte & 0x7F) as i64) << shift;

            if byte & 0x80 == 0 {
                break;
            }

            shift += 7;
            if shift >= 64 {
                return Err(format!(
                    "{} LEB128 i64 overflow",
                    self.loc_at(self.offset - 1)
                ));
            }
        }

        if (shift < 32) && (byte & 0x40 != 0) {
            result |= !0 << shift;
        }

        Ok(result)
    }

    fn expect(&mut self, expected_byte: u8) -> Result<(), String> {
        let Some(byte) = self.peek() else {
            return Err(format!(
                "{} Unexpected EOF, wanted '{expected_byte}'",
                self.loc()
            ));
        };

        if byte != expected_byte {
            return Err(format!(
                "{} Unexpected byte '0x{byte:02X}', wanted '0x{expected_byte:02X}'",
                self.loc()
            ));
        }

        self.next();

        Ok(())
    }

    fn expect_any(&mut self) -> Result<u8, String> {
        let Some(byte) = self.peek() else {
            return Err(format!("{} Unexpected EOF", self.loc()));
        };

        self.next();

        Ok(byte)
    }

    fn expect_many(&mut self, num_bytes: usize) -> Result<&[u8], String> {
        let Some(bytes) = self
            .file_bytes
            .get(self.offset..self.offset + num_bytes as usize)
        else {
            return Err(format!(
                "{} Expected {num_bytes} bytes but hit EOF",
                self.loc()
            ));
        };

        self.offset += num_bytes;

        Ok(bytes)
    }

    fn eat(&mut self, expected_byte: u8) -> bool {
        if let Some(byte) = self.peek() {
            if byte == expected_byte {
                self.next();
                return true;
            }
        }

        false
    }

    fn eat_any(&mut self) -> Option<u8> {
        if let Some(byte) = self.peek() {
            self.next();
            return Some(byte);
        }

        None
    }

    fn peek(&mut self) -> Option<u8> {
        self.file_bytes.get(self.offset).cloned()
    }

    fn next(&mut self) {
        self.offset += 1;
    }

    fn loc(&self) -> WasmParserLoc {
        WasmParserLoc(&self.file_name, self.offset)
    }

    fn loc_at(&self, offset: usize) -> WasmParserLoc {
        WasmParserLoc(&self.file_name, offset)
    }
}

struct WasmParserLoc<'a>(&'a str, usize);
impl<'a> core::fmt::Display for WasmParserLoc<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{} -", self.0, self.1)?;
        Ok(())
    }
}
