#![cfg_attr(not(test), no_std)]
#![feature(alloc_error_handler)]
#![feature(vec_into_raw_parts)]

extern crate alloc;

mod ast;
mod binary_writer;
mod compiler;
mod ir;
mod parser;
mod type_checker;
mod wasi_io;
mod wasm;

use alloc::{format, string::String, vec::Vec};
use ast::*;
use binary_writer::*;
use compiler::*;
use core::{alloc::Layout, mem, slice, str};
use parser::*;

#[cfg(target_arch = "wasm32")]
mod wasm_target {
    use lol_alloc::{FreeListAllocator, LockedAllocator};

    #[global_allocator]
    static ALLOCATOR: LockedAllocator<FreeListAllocator> =
        LockedAllocator::new(FreeListAllocator::new());

    #[alloc_error_handler]
    fn oom(_: core::alloc::Layout) -> ! {
        core::arch::wasm32::unreachable()
    }

    #[panic_handler]
    fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
        core::arch::wasm32::unreachable()
    }
}

#[no_mangle]
pub unsafe extern "C" fn mem_alloc(length: usize) -> *mut u8 {
    alloc::alloc::alloc_zeroed(Layout::from_size_align(length, 8).unwrap())
}

#[no_mangle]
pub unsafe extern "C" fn mem_free(ptr: *mut u8, length: usize) {
    alloc::alloc::dealloc(ptr, Layout::from_size_align(length, 8).unwrap());
}

#[repr(C)]
pub struct ParseResult {
    ok: u32, // 0 | 1
    data: *mut u8,
    size: usize,
}

#[no_mangle]
pub extern "C" fn _start() {
    let source = wasi_io::fd_read_all(wasi::FD_STDIN);

    match compile_str(str::from_utf8(&source).unwrap()) {
        Ok(binary) => {
            wasi_io::stdout_write(binary.as_slice());
        }
        Err(mut message) => {
            message.push('\n');
            wasi_io::stderr_write(message.as_bytes());
            wasi_io::exit(1);
        }
    };
}

#[no_mangle]
pub extern "C" fn compile(script_ptr: *const u8, script_len: usize) -> ParseResult {
    let Some(script) = ptr_to_str(script_ptr, script_len) else {
        return ParseResult::err(format!("ParseError: Cannot process input"));
    };

    match compile_str(script) {
        Ok(binary) => ParseResult::ok(binary),
        Err(message) => ParseResult::err(message),
    }
}

pub fn parse_file(file_name: &str, source: &str) -> Result<Vec<SExpr>, CompileError> {
    parse(file_name, source).map_err(|err| {
        return CompileError {
            message: err.message,
            loc: err.loc.clone(),
        };
    })
}

fn compile_str(script: &str) -> Result<Vec<u8>, String> {
    let module = parse_file("<input>", script)
        .and_then(compile_module)
        .map_err(|err| {
            let Location {
                file_name,
                line,
                col,
                ..
            } = err.loc;

            return format!(
                "{msg} in {file_name} at line {line} col {col}",
                msg = err.message,
            );
        })?;

    let mut wasm_binary = Vec::new();
    write_binary(&mut wasm_binary, &module);
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
