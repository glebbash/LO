#![cfg_attr(not(test), no_std)]
#![feature(alloc_error_handler)]
#![feature(vec_into_raw_parts)]

// TODO: store panic message in global variable and provide access to it
//       or do not panic at all

extern crate alloc;

mod binary_builder;
mod compiler;
mod parser;
mod runtime;
mod wasm_module;

use crate::parser::index_to_position;
use alloc::vec::Vec;
use binary_builder::BinaryBuilder;
use compiler::compile_module;
use parser::parse;

#[repr(C)]
pub struct RawVec {
    data: *const u8,
    length: usize,
    capacity: usize,
}

#[no_mangle]
pub extern "C" fn compile(script_ptr: *const u8, script_len: usize) -> RawVec {
    let script = read_ascii_str(script_ptr, script_len).unwrap();

    let exprs = match parse(script) {
        Err(err) => {
            let (line, col) = index_to_position(script, err.index);

            panic!(
                "ParseError: {error_message} at line {line} col {col}",
                error_message = err.data
            );
        }
        Ok(exprs) => exprs,
    };

    let module = compile_module(&exprs);

    let wasm_binary = BinaryBuilder::new(&module).build();

    let (data, length, capacity) = wasm_binary.into_raw_parts();

    RawVec {
        data,
        length,
        capacity,
    }
}

#[no_mangle]
pub extern "C" fn free_binary(wasm_binary: RawVec) {
    let _vec = unsafe {
        Vec::<u8>::from_raw_parts(
            wasm_binary.data as *mut u8,
            wasm_binary.length,
            wasm_binary.capacity,
        )
    };
}

fn read_ascii_str<'a>(chars: *const u8, chars_len: usize) -> Option<&'a str> {
    unsafe { core::str::from_utf8(core::slice::from_raw_parts(chars, chars_len)).ok() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use std::io::Write;

        let (script, script_len) = make_ascii_str(include_str!("../examples/42.lole"));

        let result = compile(script, script_len);

        let wasm_binary = unsafe { std::slice::from_raw_parts(result.data, result.length) };

        let mut f1 = std::fs::File::create("tmp/main.wasm").unwrap();
        f1.write_all(wasm_binary).unwrap();
        f1.flush().unwrap();

        // TODO: add assertions
    }

    fn make_ascii_str(data: &str) -> (*const u8, usize) {
        return (data.as_bytes().as_ptr(), data.as_bytes().len());
    }
}
