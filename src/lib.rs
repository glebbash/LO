#![cfg_attr(not(test), no_std)]
#![feature(alloc_error_handler)]
#![feature(vec_into_raw_parts)]

extern crate alloc;

mod binary_builder;
mod compiler;
mod parser;
mod runtime;
mod type_checker;
mod wasi_io;
mod wasm_module;

use alloc::{
    alloc::{alloc_zeroed, dealloc},
    format,
    string::String,
    vec::Vec,
};
use binary_builder::BinaryBuilder;
use compiler::compile_module;
use core::{alloc::Layout, mem, slice, str};
use parser::parse;

#[no_mangle]
pub unsafe extern "C" fn mem_alloc(length: usize) -> *mut u8 {
    alloc_zeroed(Layout::from_size_align(length, 8).unwrap())
}

#[no_mangle]
pub unsafe extern "C" fn mem_free(ptr: *mut u8, length: usize) {
    dealloc(ptr, Layout::from_size_align(length, 8).unwrap());
}

#[repr(C)]
pub struct ParseResult {
    ok: u32, // 0 | 1
    data: *mut u8,
    size: usize,
}

#[no_mangle]
pub extern "C" fn _start() {
    let source = wasi_io::fd_read(wasi::FD_STDIN);

    match compile_str(str::from_utf8(&source).unwrap()) {
        Ok(binary) => {
            wasi_io::fd_write(wasi::FD_STDOUT, binary.as_slice());
        }
        Err(mut message) => {
            message.push('\n');
            wasi_io::fd_write(wasi::FD_STDERR, message.as_bytes());

            unsafe { wasi::proc_exit(1) };
        }
    };
}

#[no_mangle]
pub extern "C" fn compile(script_ptr: *const u8, script_len: usize) -> ParseResult {
    let Some(script) = ptr_to_str(script_ptr, script_len) else {
        return ParseResult::err(String::from("ParseError: Cannot process input"));
    };

    match compile_str(script) {
        Ok(binary) => ParseResult::ok(binary),
        Err(message) => ParseResult::err(message),
    }
}

fn compile_str(script: &str) -> Result<Vec<u8>, String> {
    let exprs = match parse(script) {
        Ok(exprs) => exprs,
        Err(err) => {
            let (line, col) = err.loc().position_in(script);

            return Err(format!("ParseError: {err} at line {line} col {col}"));
        }
    };

    let module = match compile_module(&exprs) {
        Ok(module) => module,
        Err(err) => {
            let (line, col) = err.loc.position_in(script);

            return Err(format!(
                "CompilerError: {msg} at line {line} col {col}",
                msg = err.message
            ));
        }
    };

    let wasm_binary = BinaryBuilder::new(&module).build();

    Ok(wasm_binary)
}

fn ptr_to_str<'a>(chars: *const u8, chars_len: usize) -> Option<&'a str> {
    let slice = unsafe { slice::from_raw_parts(chars, chars_len) };
    str::from_utf8(slice).ok()
}

impl ParseResult {
    fn ok(result: Vec<u8>) -> Self {
        Self::new(1, result)
    }

    fn err(err_message: String) -> Self {
        Self::new(0, err_message.into())
    }

    fn new(ok: u32, mut vec: Vec<u8>) -> Self {
        vec.shrink_to_fit();
        assert!(vec.len() == vec.capacity());

        let data = vec.as_mut_ptr();
        let size = vec.len();

        mem::forget(vec);

        Self { ok, data, size }
    }
}
