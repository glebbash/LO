#![cfg_attr(not(test), no_std)]
#![feature(alloc_error_handler)]

extern crate alloc;

mod ast;
mod codegen;
mod compiler;
mod expand;
mod ir;
mod lowering;
mod parser;
mod type_checker;
mod wasi_io;
mod wasm;

use alloc::{string::String, vec::Vec};

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

fn exec_pipeline(script: &str) -> Result<Vec<u8>, String> {
    let raw_exprs = parser::parse("<input>", script)?;
    let exprs = expand::expand(raw_exprs)?;
    let module = compiler::compile(&exprs)?;
    let binary = codegen::codegen(&module);
    Ok(binary)
}

mod wasi_api {
    use super::exec_pipeline;
    use super::wasi_io::*;
    use core::str;

    #[no_mangle]
    pub extern "C" fn _start() {
        let source = stdin_read();

        match exec_pipeline(str::from_utf8(&source).unwrap()) {
            Ok(binary) => {
                stdout_write(binary.as_slice());
            }
            Err(mut message) => {
                message.push('\n');
                stderr_write(message.as_bytes());
                exit(1);
            }
        };
    }
}

mod fn_api {
    use super::exec_pipeline;
    use alloc::{format, string::String, vec::Vec};
    use core::{alloc::Layout, mem::ManuallyDrop, slice, str};

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
    pub extern "C" fn compile(script_ptr: *const u8, script_len: usize) -> ParseResult {
        let bytes = unsafe { slice::from_raw_parts(script_ptr, script_len) };
        let Ok(script) = str::from_utf8(bytes) else {
            return ParseResult::from(Err(format!("ParseError: Cannot process input")));
        };

        ParseResult::from(exec_pipeline(script))
    }

    impl ParseResult {
        fn from(res: Result<Vec<u8>, String>) -> Self {
            match res {
                Ok(binary) => Self::new(1, binary),
                Err(message) => Self::new(0, message.into()),
            }
        }

        fn new(ok: u32, vec: Vec<u8>) -> Self {
            let mut vec = ManuallyDrop::new(vec);

            vec.shrink_to_fit();
            assert!(vec.len() == vec.capacity());

            Self {
                ok,
                data: vec.as_mut_ptr(),
                size: vec.len(),
            }
        }
    }
}
