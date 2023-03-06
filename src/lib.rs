#![cfg_attr(not(test), no_std)]
#![feature(alloc_error_handler)]
#![feature(vec_into_raw_parts)]

extern crate alloc;

mod binary_builder;
mod compiler;
mod parser;
mod runtime;
mod wasm_module;

use crate::parser::index_to_position;
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
    ok: u32,
    data: *mut u8,
    size: usize,
}

#[no_mangle]
pub extern "C" fn compile(script_ptr: *const u8, script_len: usize) -> ParseResult {
    let Some(script) = ptr_to_str(script_ptr, script_len) else {
        return ParseResult::err(String::from("ParseError: Cannot process input"));
    };

    let exprs = match parse(script) {
        Err(err) => {
            let (line, col) = index_to_position(script, err.index);

            return ParseResult::err(format!(
                "ParseError: {error_message} at line {line} col {col}",
                error_message = err.data
            ));
        }
        Ok(exprs) => exprs,
    };

    let module = match compile_module(&exprs) {
        Ok(module) => module,
        Err(err) => {
            return ParseResult::err(format!("CompilerError: {err}"));
        }
    };

    let wasm_binary = BinaryBuilder::new(&module).build();

    ParseResult::ok(wasm_binary)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use std::fs::File;
        use std::io::Write;

        let script = include_str!("../examples/42.lole");
        let script_len = script.len();

        let script_ptr = unsafe { mem_alloc(script_len) };
        unsafe { core::ptr::copy(script.as_ptr(), script_ptr, script_len) };

        let ParseResult { ok, data, size } = compile(script_ptr, script_len);

        // TODO: add assertions
        if ok == 1 {
            let wasm_binary = unsafe { std::slice::from_raw_parts(data, size) };

            let mut out = File::create("tmp/main.wasm").unwrap();
            out.write_all(wasm_binary).unwrap();
            out.flush().unwrap();
        } else {
            let error_msg = unsafe { std::slice::from_raw_parts(data, size) };
            let error_msg = str::from_utf8(error_msg).unwrap();

            panic!("{error_msg}");
        }

        unsafe { mem_free(data, size) };
        unsafe { mem_free(script_ptr, script_len) };
    }
}
