use alloc::{string::String, vec::Vec};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct WasmModule {
    pub types: Vec<WasmFnType>,
    pub imports: Vec<WasmImport>,
    pub functions: Vec<u32>,
    pub tables: Vec<WasmTable>,
    pub elements: Vec<WasmElement>,
    pub memories: Vec<WasmLimits>,
    pub globals: Vec<WasmGlobal>,
    pub exports: Vec<WasmExport>,
    pub codes: Vec<WasmFn>,
    pub datas: Vec<WasmData>,

    // debug info
    pub function_names: Vec<WasmFnNameItem>,
    pub local_names: Vec<WasmLocalNameItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WasmFnType {
    pub inputs: Vec<WasmType>,
    pub outputs: Vec<WasmType>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WasmImport {
    pub module_name: String,
    pub item_name: String,
    pub item_desc: WasmImportDesc,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WasmImportDesc {
    Func { type_index: u32 },
    Memory(WasmLimits),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WasmTable {
    pub limits: WasmLimits,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WasmElement {
    Passive { expr: WasmExpr, fn_idx: Vec<u32> },
}

#[derive(Clone, Debug, PartialEq)]
pub struct WasmFn {
    pub locals: Vec<WasmLocals>,
    pub expr: WasmExpr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WasmLocals {
    pub count: u32,
    pub value_type: WasmType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WasmExpr {
    pub instrs: Vec<WasmInstr>,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum WasmUnaryOpKind {
    I32_EQZ = 0x45,
    I64_EQZ = 0x50,
    F32_NEG = 0x8C,
    F64_NEG = 0x9A,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum WasmBinaryOpKind {
    I32_EQ = 0x46,
    I32_NE = 0x47,
    I32_LT_S = 0x48,
    I32_LT_U = 0x49,
    I32_GT_S = 0x4A,
    I32_GT_U = 0x4B,
    I32_LE_S = 0x4C,
    I32_LE_U = 0x4D,
    I32_GE_S = 0x4E,
    I32_GE_U = 0x4F,

    I64_EQ = 0x51,
    I64_NE = 0x52,
    I64_LT_S = 0x53,
    I64_LT_U = 0x54,
    I64_GT_S = 0x55,
    I64_GT_U = 0x56,
    I64_LE_S = 0x57,
    I64_LE_U = 0x58,
    I64_GE_S = 0x59,
    I64_GE_U = 0x5A,

    F32_EQ = 0x5B,
    F32_NE = 0x5C,
    F32_LT = 0x5D,
    F32_GT = 0x5E,
    F32_LE = 0x5F,
    F32_GE = 0x60,

    F64_EQ = 0x61,
    F64_NE = 0x62,
    F64_LT = 0x63,
    F64_GT = 0x64,
    F64_LE = 0x65,
    F64_GE = 0x66,

    I32_ADD = 0x6A,
    I32_SUB = 0x6B,
    I32_MUL = 0x6C,
    I32_DIV_S = 0x6D,
    I32_DIV_U = 0x6E,
    I32_REM_S = 0x6F,
    I32_REM_U = 0x70,
    I32_AND = 0x71,
    I32_OR = 0x72,
    I32_XOR = 0x73,
    I32_SHL = 0x74,
    I32_SHR_S = 0x75,
    I32_SHR_U = 0x76,

    I64_ADD = 0x7C,
    I64_SUB = 0x7D,
    I64_MUL = 0x7E,
    I64_DIV_S = 0x7F,
    I64_DIV_U = 0x80,
    I64_REM_S = 0x81,
    I64_REM_U = 0x82,
    I64_AND = 0x83,
    I64_OR = 0x84,
    I64_SHL = 0x86,
    I64_SHR_S = 0x87,
    I64_SHR_U = 0x88,

    F32_ADD = 0x92,
    F32_SUB = 0x93,
    F32_MUL = 0x94,
    F32_DIV = 0x95,

    F64_ADD = 0xA0,
    F64_SUB = 0xA1,
    F64_MUL = 0xA2,
    F64_DIV = 0xA3,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
pub enum WasmStoreKind {
    I32 = 0x36,
    I64 = 0x37,
    F32 = 0x38,
    F64 = 0x39,
    I32_8 = 0x3A,
    I32_16 = 0x3B,
    I64_8 = 0x3C,
    I64_32 = 0x3E,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WasmBlockType {
    NoOut,
    SingleOut { wasm_type: WasmType },
    InOut { type_index: u32 },
}

#[derive(Clone, Debug, PartialEq)]
pub enum WasmInstr {
    Unreachable,
    Drop,
    Select,
    UnaryOp {
        kind: WasmUnaryOpKind,
    },
    BinaryOp {
        kind: WasmBinaryOpKind,
    },
    MemorySize,
    MemoryGrow,
    MemoryCopy,
    MemoryFill,
    I32Const {
        value: i32,
    },
    I64Const {
        value: i64,
    },
    F32Const {
        value: f32,
    },
    F64Const {
        value: f64,
    },
    I32WrapI64,
    I64ExtendI32s,
    I64ExtendI32u,
    I64ReinterpretF64,
    F64ReinterpretI64,
    LocalGet {
        local_index: u32,
    },
    LocalSet {
        local_index: u32,
    },
    LocalTee {
        local_index: u32,
    },
    GlobalGet {
        global_index: u32,
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
        block_kind: WasmBlockKind,
        block_type: WasmBlockType,
    },
    Else,
    BlockEnd,
    Branch {
        label_index: u32,
    },
    BranchIndirect {
        label_idx: Vec<u32>,
        default_label_index: u32,
    },
    BranchIf {
        label_index: u32,
    },
    Call {
        fn_index: u32,
    },
    CallIndirect {
        type_index: u32,
        table_index: u32,
    },
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum WasmBlockKind {
    Block = 0x02,
    Loop = 0x03,
    If = 0x04,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum WasmType {
    FuncRef = 0x70,
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WasmLimits {
    pub min: u32,
    pub max: Option<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WasmGlobal {
    pub mutable: bool,
    pub value_type: WasmType,
    pub initial_value: WasmExpr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WasmExport {
    pub export_type: WasmExportType,
    pub export_name: String,
    pub exported_item_index: u32,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum WasmExportType {
    Func = 0x00,
    Mem = 0x02,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WasmData {
    Active { offset: WasmExpr, bytes: Vec<u8> },
}

#[derive(Clone, Debug, PartialEq)]
pub struct WasmFnNameItem {
    pub fn_index: u32,
    pub fn_name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WasmLocalNameItem {
    pub fn_index: u32,
    pub locals: Vec<WasmLocalInfo>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WasmLocalInfo {
    pub local_index: u32,
    pub local_name: String,
}

impl WasmModule {
    pub fn dump(&self, output: &mut Vec<u8>) {
        self.dump_using_buffer(output, &mut Vec::new());
    }

    pub fn dump_using_buffer(&self, output: &mut Vec<u8>, section_buffer: &mut Vec<u8>) {
        write_magic_and_version(output);

        self.write_type_section(output, section_buffer);

        self.write_import_section(output, section_buffer);

        self.write_function_section(output, section_buffer);

        self.write_memory_section(output, section_buffer);

        self.write_global_section(output, section_buffer);

        self.write_export_section(output, section_buffer);

        self.write_code_section(output, section_buffer);

        self.write_data_section(output, section_buffer);

        self.write_names_custom_section(output, section_buffer);
    }

    fn write_type_section(&self, output: &mut Vec<u8>, section: &mut Vec<u8>) {
        if self.types.len() == 0 {
            return;
        }

        write_u32(section, self.types.len() as u32);
        for fn_type in &self.types {
            write_u8(section, 0x60); // func type

            write_u32(section, fn_type.inputs.len() as u32);
            for fn_input in &fn_type.inputs {
                write_u8(section, fn_input.clone() as u8);
            }

            write_u32(section, fn_type.outputs.len() as u32);
            for fn_output in &fn_type.outputs {
                write_u8(section, fn_output.clone() as u8);
            }
        }

        write_section(output, section, 0x01);
    }

    fn write_import_section(&self, output: &mut Vec<u8>, section: &mut Vec<u8>) {
        if self.imports.len() == 0 {
            return;
        }

        write_u32(section, self.imports.len() as u32);
        for import in &self.imports {
            write_u32(section, import.module_name.len() as u32);
            write_all(section, import.module_name.as_bytes());

            write_u32(section, import.item_name.len() as u32);
            write_all(section, import.item_name.as_bytes());

            match import.item_desc {
                WasmImportDesc::Func { type_index } => {
                    write_u8(section, 0x00); // fn
                    write_u32(section, type_index);
                }
                WasmImportDesc::Memory(ref memory) => {
                    write_u8(section, 0x02); // memory
                    write_limits(section, memory);
                }
            }
        }

        write_section(output, section, 0x02);
    }

    fn write_function_section(&self, output: &mut Vec<u8>, section: &mut Vec<u8>) {
        if self.functions.len() == 0 {
            return;
        }

        write_u32(section, self.functions.len() as u32);
        for type_index in &self.functions {
            write_u32(section, *type_index);
        }

        write_section(output, section, 0x03);
    }

    fn write_memory_section(&self, output: &mut Vec<u8>, section: &mut Vec<u8>) {
        if self.memories.len() == 0 {
            return;
        }

        write_u32(section, self.memories.len() as u32);
        for memory in &self.memories {
            write_limits(section, memory);
        }

        write_section(output, section, 0x05);
    }

    fn write_global_section(&self, output: &mut Vec<u8>, section: &mut Vec<u8>) {
        if self.globals.len() == 0 {
            return;
        }

        write_u32(section, self.globals.len() as u32);
        for global in &self.globals {
            write_u8(section, global.value_type.clone() as u8);

            if global.mutable {
                write_u8(section, 0x01);
            } else {
                write_u8(section, 0x00);
            }

            write_expr(section, &global.initial_value);
        }

        write_section(output, section, 0x06);
    }

    fn write_export_section(&self, output: &mut Vec<u8>, section: &mut Vec<u8>) {
        if self.exports.len() == 0 {
            return;
        }

        write_u32(section, self.exports.len() as u32);
        for export in &self.exports {
            write_u32(section, export.export_name.len() as u32);
            write_all(section, export.export_name.as_bytes());

            write_u8(section, export.export_type.clone() as u8);

            write_u32(section, export.exported_item_index);
        }

        write_section(output, section, 0x07);
    }

    fn write_code_section(&self, output: &mut Vec<u8>, section: &mut Vec<u8>) {
        if self.codes.len() == 0 {
            return;
        }

        let mut fn_section = Vec::new();

        write_u32(section, self.codes.len() as u32);
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

            write_u32(section, fn_section.len() as u32);
            section.append(&mut fn_section);
        }

        write_section(output, section, 0x0A);
    }

    fn write_data_section(&self, output: &mut Vec<u8>, section: &mut Vec<u8>) {
        if self.datas.len() == 0 {
            return;
        }

        write_u32(section, self.datas.len() as u32);
        for data in self.datas.iter() {
            let WasmData::Active { offset, bytes } = data;
            write_u32(section, 0);
            write_expr(section, offset);
            write_u32(section, bytes.len() as u32);
            write_all(section, bytes);
        }

        write_section(output, section, 0x0B);
    }

    fn write_names_custom_section(&self, output: &mut Vec<u8>, section: &mut Vec<u8>) {
        if self.function_names.len() == 0 || self.local_names.len() == 0 {
            return;
        }

        let section_name = "name";
        write_u32(section, section_name.len() as u32);
        write_all(section, section_name.as_bytes());

        // function names:
        {
            let mut subsection_buf = Vec::new();
            write_u32(&mut subsection_buf, self.function_names.len() as u32);
            for fn_info in &self.function_names {
                write_u32(&mut subsection_buf, fn_info.fn_index);
                write_u32(&mut subsection_buf, fn_info.fn_name.len() as u32);
                write_all(&mut subsection_buf, fn_info.fn_name.as_bytes());
            }
            write_section(section, &mut subsection_buf, 1);
        }

        // local names:
        {
            let mut subsection_buf = Vec::new();
            write_u32(&mut subsection_buf, self.local_names.len() as u32);
            for fn_info in &self.local_names {
                write_u32(&mut subsection_buf, fn_info.fn_index);

                write_u32(&mut subsection_buf, fn_info.locals.len() as u32);
                for local_info in &fn_info.locals {
                    write_u32(&mut subsection_buf, local_info.local_index);
                    write_u32(&mut subsection_buf, local_info.local_name.len() as u32);
                    write_all(&mut subsection_buf, local_info.local_name.as_bytes());
                }
            }
            write_section(section, &mut subsection_buf, 2);
        }

        write_section(output, section, 0x00);
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
        WasmInstr::BlockStart {
            block_kind,
            block_type,
        } => {
            write_u8(out, block_kind.clone() as u8);
            match block_type {
                WasmBlockType::NoOut => {
                    write_u8(out, 0x40); // no value
                }
                WasmBlockType::SingleOut { wasm_type } => {
                    write_u8(out, wasm_type.clone() as u8);
                }
                WasmBlockType::InOut { type_index } => {
                    write_i32(out, *type_index as i32);
                }
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
        WasmInstr::BranchIf { label_index } => {
            write_u8(out, 0x0D);
            write_u32(out, *label_index);
        }
        WasmInstr::BranchIndirect {
            label_idx,
            default_label_index,
        } => {
            write_u8(out, 0x0E);
            write_u32(out, label_idx.len() as u32);
            for label_index in label_idx {
                write_u32(out, *label_index);
            }
            write_u32(out, *default_label_index);
        }
        WasmInstr::Return => {
            write_u8(out, 0x0F);
        }
        WasmInstr::Call { fn_index } => {
            write_u8(out, 0x10);
            write_u32(out, *fn_index);
        }
        WasmInstr::CallIndirect {
            type_index,
            table_index,
        } => {
            write_u8(out, 0x11);
            write_u32(out, *type_index);
            write_u32(out, *table_index);
        }
        WasmInstr::Drop => {
            write_u8(out, 0x1A);
        }
        WasmInstr::Select => {
            write_u8(out, 0x1B);
        }
        WasmInstr::LocalGet { local_index } => {
            write_u8(out, 0x20);
            write_u32(out, *local_index);
        }
        WasmInstr::LocalSet { local_index } => {
            write_u8(out, 0x21);
            write_u32(out, *local_index);
        }
        WasmInstr::LocalTee { local_index } => {
            write_u8(out, 0x22);
            write_u32(out, *local_index);
        }
        WasmInstr::GlobalGet { global_index } => {
            write_u8(out, 0x23);
            write_u32(out, *global_index);
        }
        WasmInstr::GlobalSet { global_index } => {
            write_u8(out, 0x24);
            write_u32(out, *global_index);
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
        WasmInstr::Store {
            align,
            offset,
            kind,
        } => {
            write_u8(out, kind.clone() as u8);
            write_u32(out, *align);
            write_u32(out, *offset);
        }
        WasmInstr::MemorySize => {
            write_u8(out, 0x3F);
            write_u8(out, 0x00);
        }
        WasmInstr::MemoryGrow => {
            write_u8(out, 0x40);
            write_u8(out, 0x00);
        }
        WasmInstr::I32Const { value } => {
            write_u8(out, 0x41);
            write_i32(out, *value);
        }
        WasmInstr::I64Const { value } => {
            write_u8(out, 0x42);
            write_i64(out, *value);
        }
        WasmInstr::F32Const { value } => {
            write_u8(out, 0x43);
            out.extend_from_slice(&value.to_le_bytes());
        }
        WasmInstr::F64Const { value } => {
            write_u8(out, 0x44);
            out.extend_from_slice(&value.to_le_bytes());
        }
        WasmInstr::UnaryOp { kind } => {
            write_u8(out, kind.clone() as u8);
        }
        WasmInstr::BinaryOp { kind } => {
            write_u8(out, kind.clone() as u8);
        }
        WasmInstr::I32WrapI64 => {
            write_u8(out, 0xA7);
        }
        WasmInstr::I64ExtendI32s => {
            write_u8(out, 0xAC);
        }
        WasmInstr::I64ExtendI32u => {
            write_u8(out, 0xAD);
        }
        WasmInstr::I64ReinterpretF64 => {
            write_u8(out, 0xBD);
        }
        WasmInstr::F64ReinterpretI64 => {
            write_u8(out, 0xBF);
        }
        WasmInstr::MemoryCopy => {
            write_u8(out, 0xFC);
            write_u32(out, 10);
            write_u8(out, 0x00);
            write_u8(out, 0x00);
        }
        WasmInstr::MemoryFill => {
            write_u8(out, 0xFC);
            write_u32(out, 11);
            write_u8(out, 0x00);
        }
    }
}

fn write_limits(out: &mut Vec<u8>, limits: &WasmLimits) {
    if let Some(limits_max) = limits.max {
        write_u8(out, 0x01);
        write_u32(out, limits.min as u32);
        write_u32(out, limits_max as u32);
    } else {
        write_u8(out, 0x00);
        write_u32(out, limits.min as u32);
    }
}

fn write_u8(out: &mut Vec<u8>, value: u8) {
    out.push(value);
}

pub fn write_u32(out: &mut Vec<u8>, value: u32) {
    leb128_write_unsigned(out, value as u64);
}

fn write_i32(out: &mut Vec<u8>, value: i32) {
    leb128_write_signed(out, value as i64);
}

fn write_i64(out: &mut Vec<u8>, value: i64) {
    leb128_write_signed(out, value);
}

pub fn write_all(out: &mut Vec<u8>, value: &[u8]) {
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
