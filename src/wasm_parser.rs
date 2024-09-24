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
            let Some(section_id) = self.eat() else {
                break;
            };

            match section_id {
                0x01 => self.parse_type_section()?,
                0x03 => self.parse_function_section()?,
                0x07 => self.parse_export_section()?,
                0x0A => self.parse_code_section()?,
                _ => {
                    return Err(format!("{} Unknown section id: {section_id}", self.loc()));
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

    fn parse_function_section(&mut self) -> Result<(), String> {
        let _section_size = self.parse_u32()?;

        let fns_len = self.parse_u32()?;
        for _ in 0..fns_len {
            let type_index = self.parse_u32()?;
            self.module.functions.push(type_index);
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
                        "{} Unknown export type '{byte:02X}'",
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

    fn parse_expr(&mut self) -> Result<WasmExpr, String> {
        let mut expr = WasmExpr { instrs: Vec::new() };

        loop {
            match self.expect_any()? {
                0x0B => break,
                0x41 => {
                    let value = self.parse_i32()?;
                    expr.instrs.push(WasmInstr::I32Const { value });
                }
                0x0F => {
                    expr.instrs.push(WasmInstr::Return);
                }
                byte => {
                    return Err(format!(
                        "{} Unknown instruction {byte:02X}",
                        self.loc_at(self.offset - 1)
                    ))
                }
            }
        }

        Ok(expr)
    }

    fn parse_type(&mut self) -> Result<WasmType, String> {
        match self.expect_any()? {
            0x7F => Ok(WasmType::I32),
            0x7E => Ok(WasmType::I64),
            0x7D => Ok(WasmType::F32),
            0x7C => Ok(WasmType::F64),
            byte => Err(format!(
                "{} Unknown type '{byte:02X}'",
                self.loc_at(self.offset - 1)
            )),
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

    fn expect(&mut self, expected_byte: u8) -> Result<(), String> {
        let Some(byte) = self.peek() else {
            return Err(format!(
                "{} Unexpected EOF, wanted '{expected_byte}'",
                self.loc()
            ));
        };

        if byte != expected_byte {
            return Err(format!(
                "{} Unexpected byte '{byte:02X}', wanted '{expected_byte:02X}'",
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

    fn eat(&mut self) -> Option<u8> {
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
