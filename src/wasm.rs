use ::alloc::{string::String, vec::Vec};

#[derive(Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WasmModule {
    pub types: Vec<WasmFnType>,
    pub imports: Vec<WasmImport>,
    pub functions: Vec<u32>,
    pub memories: Vec<WasmLimits>,
    pub globals: Vec<WasmGlobal>,
    pub exports: Vec<WasmExport>,
    pub codes: Vec<WasmFn>,
    pub datas: Vec<WasmData>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WasmFnType {
    pub inputs: Vec<WasmType>,
    pub outputs: Vec<WasmType>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WasmImport {
    pub module_name: String,
    pub item_name: String,
    pub item_desc: WasmImportDesc,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WasmImportDesc {
    Func { type_index: u32 },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WasmFn {
    pub locals: Vec<WasmLocals>,
    pub expr: WasmExpr,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WasmLocals {
    pub count: u32,
    pub value_type: WasmType,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WasmExpr {
    pub instrs: Vec<WasmInstr>,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WasmBinaryOpKind {
    I32Equal = 0x46,
    I32LessThenUnsigned = 0x49,
    I32GreaterThanUnsigned = 0x4B,
    I32GreaterEqualUnsigned = 0x4F,
    I32NotEqual = 0x47,
    I32Add = 0x6A,
    I32Sub = 0x6B,
    I32Mul = 0x6C,
    I32DivUnsigned = 0x6E,
    I32RemUnsigned = 0x70,
    I32And = 0x71,
    I32Or = 0x72,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WasmLoadKind {
    I32 = 0x28,
    I64 = 0x29,
    F32 = 0x2A,
    F64 = 0x2B,
    I32I8 = 0x2C,
    I32U8 = 0x2D,
    I32I16 = 0x2E,
    I32U16 = 0x2F,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WasmStoreKind {
    I32 = 0x36,
    I64 = 0x37,
    F32 = 0x38,
    F64 = 0x39,
    I32U8 = 0x3A,
    I32U16 = 0x3B,
}

impl WasmStoreKind {
    pub fn from_load_kind(kind: &WasmLoadKind) -> Self {
        match kind {
            WasmLoadKind::I32 => Self::I32,
            WasmLoadKind::I64 => Self::I64,
            WasmLoadKind::F32 => Self::F32,
            WasmLoadKind::F64 => Self::F64,
            WasmLoadKind::I32I8 => Self::I32U8,
            WasmLoadKind::I32U8 => Self::I32U8,
            WasmLoadKind::I32I16 => Self::I32U16,
            WasmLoadKind::I32U16 => Self::I32U16,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WasmInstr {
    Unreachable,
    Drop,
    BinaryOp {
        kind: WasmBinaryOpKind,
    },
    MemorySize,
    MemoryGrow,
    I32Const {
        value: i32,
    },
    I64Const {
        value: i64,
    },
    LocalGet {
        local_index: u32,
    },
    GlobalGet {
        global_index: u32,
    },
    LocalSet {
        local_index: u32,
    },
    GlobalSet {
        global_index: u32,
    },
    Load {
        kind: WasmLoadKind,
        align: u32,
        offset: u32,
    },
    Store {
        kind: WasmStoreKind,
        align: u32,
        offset: u32,
    },
    Return,
    BlockStart {
        block_type: WasmBlockType,
        return_type: Option<WasmType>,
    },
    Else,
    BlockEnd,
    Branch {
        label_index: u32,
    },
    Call {
        fn_index: u32,
    },
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WasmBlockType {
    Block = 0x02,
    Loop = 0x03,
    If = 0x04,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WasmType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WasmLimits {
    pub min: u32,
    pub max: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WasmGlobal {
    pub kind: WasmGlobalKind,
    pub initial_value: WasmExpr,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WasmGlobalKind {
    pub value_type: WasmType,
    pub mutable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WasmExport {
    pub export_type: WasmExportType,
    pub export_name: String,
    pub exported_item_index: u32,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WasmExportType {
    Func = 0x00,
    Mem = 0x02,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WasmData {
    Active { offset: WasmExpr, bytes: Vec<u8> },
}

impl WasmModule {
    pub fn dump(&self, output: &mut Vec<u8>) {
        self.dump_using_buffer(output, &mut Vec::new());
    }

    pub fn dump_using_buffer(&self, output: &mut Vec<u8>, section_buffer: &mut Vec<u8>) {
        write_magic_and_version(output);

        self.write_type_section(section_buffer);
        write_section(output, section_buffer, 0x01);

        self.write_import_section(section_buffer);
        write_section(output, section_buffer, 0x02);

        self.write_function_section(section_buffer);
        write_section(output, section_buffer, 0x03);

        self.write_memory_section(section_buffer);
        write_section(output, section_buffer, 0x05);

        self.write_global_section(section_buffer);
        write_section(output, section_buffer, 0x06);

        self.write_export_section(section_buffer);
        write_section(output, section_buffer, 0x07);

        self.write_code_section(section_buffer);
        write_section(output, section_buffer, 0x0A);

        self.write_data_section(section_buffer);
        write_section(output, section_buffer, 0x0B);
    }

    fn write_type_section(&self, out: &mut Vec<u8>) {
        write_u32(out, self.types.len() as u32);
        for fn_type in &self.types {
            write_u8(out, 0x60); // func type

            write_u32(out, fn_type.inputs.len() as u32);
            for fn_input in &fn_type.inputs {
                write_u8(out, fn_input.clone() as u8);
            }

            write_u32(out, fn_type.outputs.len() as u32);
            for fn_output in &fn_type.outputs {
                write_u8(out, fn_output.clone() as u8);
            }
        }
    }

    fn write_import_section(&self, out: &mut Vec<u8>) {
        write_u32(out, self.imports.len() as u32);
        for import in &self.imports {
            write_u32(out, import.module_name.len() as u32);
            write_all(out, import.module_name.as_bytes());

            write_u32(out, import.item_name.len() as u32);
            write_all(out, import.item_name.as_bytes());

            match import.item_desc {
                WasmImportDesc::Func { type_index } => {
                    write_u32(out, 0x00); // fn
                    write_u32(out, type_index);
                }
            }
        }
    }

    fn write_function_section(&self, out: &mut Vec<u8>) {
        write_u32(out, self.functions.len() as u32);
        for type_index in &self.functions {
            write_u32(out, *type_index);
        }
    }

    fn write_memory_section(&self, out: &mut Vec<u8>) {
        write_u32(out, self.memories.len() as u32);
        for memory in &self.memories {
            if let Some(memory_max) = memory.max {
                write_u8(out, 0x01);
                write_u32(out, memory.min as u32);
                write_u32(out, memory_max as u32);
            } else {
                write_u8(out, 0x00);
                write_u32(out, memory.min as u32);
            }
        }
    }

    fn write_global_section(&self, out: &mut Vec<u8>) {
        write_u32(out, self.globals.len() as u32);
        for global in &self.globals {
            write_u8(out, global.kind.value_type.clone() as u8);

            if global.kind.mutable {
                write_u8(out, 0x01);
            } else {
                write_u8(out, 0x00);
            }

            write_expr(out, &global.initial_value);
        }
    }

    fn write_export_section(&self, out: &mut Vec<u8>) {
        write_u32(out, self.exports.len() as u32);
        for export in &self.exports {
            write_u32(out, export.export_name.len() as u32);
            write_all(out, export.export_name.as_bytes());

            write_u8(out, export.export_type.clone() as u8);

            write_u32(out, export.exported_item_index);
        }
    }

    fn write_code_section(&self, out: &mut Vec<u8>) {
        let mut fn_section = Vec::new();

        write_u32(out, self.codes.len() as u32);
        for fn_code in &self.codes {
            write_u32(&mut fn_section, fn_code.locals.len() as u32);
            for locals_of_some_type in &fn_code.locals {
                write_u32(&mut fn_section, locals_of_some_type.count as u32);
                write_u8(
                    &mut fn_section,
                    locals_of_some_type.value_type.clone() as u8,
                );
            }
            write_expr(&mut fn_section, &fn_code.expr);

            write_u32(out, fn_section.len() as u32);
            out.append(&mut fn_section);
        }
    }

    fn write_data_section(&self, out: &mut Vec<u8>) {
        write_u32(out, self.datas.len() as u32);
        for data in self.datas.iter() {
            let WasmData::Active { offset, bytes } = data;
            write_u32(out, 0);
            write_expr(out, offset);
            write_u32(out, bytes.len() as u32);
            write_all(out, bytes);
        }
    }
}

fn write_section(out: &mut Vec<u8>, section: &mut Vec<u8>, section_code: u8) {
    write_u8(out, section_code);
    write_u32(out, section.len() as u32);
    out.append(section);
}

fn write_magic_and_version(out: &mut Vec<u8>) {
    // wasm magic number
    write_all(out, b"\0asm");

    // version
    write_all(out, &[0x01, 0x00, 0x00, 0x00]);
}

fn write_expr(out: &mut Vec<u8>, expr: &WasmExpr) {
    for instr in &expr.instrs {
        write_instr(out, instr);
    }

    write_u8(out, 0x0B); // end
}

fn write_instr(out: &mut Vec<u8>, instr: &WasmInstr) {
    match instr {
        WasmInstr::Unreachable => {
            write_u8(out, 0x00);
        }
        WasmInstr::BinaryOp { kind } => {
            write_u8(out, kind.clone() as u8);
        }
        WasmInstr::MemorySize => {
            write_u8(out, 0x3F);
            write_u8(out, 0x00);
        }
        WasmInstr::MemoryGrow => {
            write_u8(out, 0x40);
            write_u8(out, 0x00);
        }
        WasmInstr::Load {
            kind,
            align,
            offset,
        } => {
            write_u8(out, kind.clone() as u8);
            write_u32(out, *align);
            write_u32(out, *offset);
        }
        WasmInstr::I32Const { value } => {
            write_u8(out, 0x41);
            write_i32(out, *value);
        }
        WasmInstr::I64Const { value } => {
            write_u8(out, 0x42);
            write_i64(out, *value);
        }
        WasmInstr::LocalGet { local_index } => {
            write_u8(out, 0x20);
            write_u32(out, *local_index);
        }
        WasmInstr::GlobalGet { global_index } => {
            write_u8(out, 0x23);
            write_u32(out, *global_index);
        }
        WasmInstr::LocalSet { local_index } => {
            write_u8(out, 0x21);
            write_u32(out, *local_index);
        }
        WasmInstr::GlobalSet { global_index } => {
            write_u8(out, 0x24);
            write_u32(out, *global_index);
        }
        WasmInstr::Store {
            align,
            offset,
            kind,
        } => {
            write_u8(out, kind.clone() as u8);
            write_u32(out, *align);
            write_u32(out, *offset);
        }
        WasmInstr::Drop => {
            write_u8(out, 0x1A);
        }
        WasmInstr::Return => {
            write_u8(out, 0x0F);
        }
        WasmInstr::Call { fn_index } => {
            write_u8(out, 0x10);
            write_u32(out, *fn_index);
        }
        WasmInstr::BlockStart {
            block_type,
            return_type,
        } => {
            write_u8(out, block_type.clone() as u8);
            if let Some(return_type) = return_type {
                write_u8(out, return_type.clone() as u8);
            } else {
                write_u8(out, 0x40); // no value
            }
        }
        WasmInstr::Else => {
            write_u8(out, 0x05);
        }
        WasmInstr::BlockEnd => {
            write_u8(out, 0x0B);
        }
        WasmInstr::Branch { label_index } => {
            write_u8(out, 0x0C);
            write_u32(out, *label_index);
        }
    }
}

fn write_u8(out: &mut Vec<u8>, value: u8) {
    out.push(value);
}

fn write_u32(out: &mut Vec<u8>, value: u32) {
    leb128_write_unsigned(out, value as u64);
}

fn write_i32(out: &mut Vec<u8>, value: i32) {
    leb128_write_signed(out, value as i64);
}

fn write_i64(out: &mut Vec<u8>, value: i64) {
    leb128_write_signed(out, value);
}

fn write_all(out: &mut Vec<u8>, value: &[u8]) {
    out.extend_from_slice(value);
}

// LEB128

const CONTINUATION_BIT: u8 = 1 << 7;

fn leb128_write_signed(output: &mut Vec<u8>, mut val: i64) {
    while val != 0 && val != -1 {
        let byte = (val as u8) & !CONTINUATION_BIT;
        val >>= 7;
        output.push(byte | CONTINUATION_BIT);
    }
    output.push((val as u8) & !CONTINUATION_BIT);
}

fn leb128_write_unsigned(output: &mut Vec<u8>, mut val: u64) {
    while val > 0 {
        let byte = (val as u8) & !CONTINUATION_BIT;
        val >>= 7;
        output.push(byte | CONTINUATION_BIT);
    }
    output.push((val as u8) & !CONTINUATION_BIT);
}
