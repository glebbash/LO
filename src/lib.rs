#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

mod ir;
mod lexer;
mod parser;
mod utils;
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

fn exec_pipeline(file_name: &str, script: &str) -> Result<Vec<u8>, String> {
    let tokens = lexer::lex_all(file_name, script)?;
    let module = parser::parse(tokens)?;
    let mut binary = Vec::new();
    module.dump(&mut binary);
    Ok(binary)
}

mod wasi_api {
    use super::exec_pipeline;
    use super::utils::*;

    #[no_mangle]
    pub extern "C" fn _start() {
        let args = WasiArgs::load().unwrap();
        let (file_name, source) = if args.len() == 2 {
            let file_name = args.get(1).unwrap();
            let fd = fd_open(file_name).unwrap_or_else(|err| {
                let msg = alloc::format!("Error: cannot open file {file_name}: {err}\n");
                stderr_write(msg.as_bytes());
                proc_exit(1);
            });
            (file_name, fd_read_all_and_close(fd))
        } else {
            ("<stdin>.lo", stdin_read())
        };

        match exec_pipeline(file_name, core::str::from_utf8(&source).unwrap()) {
            Ok(binary) => {
                stdout_write(binary.as_slice());
            }
            Err(mut message) => {
                message.push('\n');
                stderr_write(message.as_bytes());
                proc_exit(1);
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
        ok: bool,
        data: *mut u8,
        size: usize,
    }

    #[no_mangle]
    pub extern "C" fn compile(
        file_name_ptr: *const u8,
        file_name_len: usize,
        script_ptr: *const u8,
        script_len: usize,
    ) -> ParseResult {
        let file_name_bytes = unsafe { slice::from_raw_parts(file_name_ptr, file_name_len) };
        let Ok(file_name) = str::from_utf8(file_name_bytes) else {
            return ParseResult::from(Err(format!(
                "ParseError: file_name is not a valid utf8 string"
            )));
        };

        let script_bytes = unsafe { slice::from_raw_parts(script_ptr, script_len) };
        let Ok(script) = str::from_utf8(script_bytes) else {
            return ParseResult::from(Err(format!(
                "ParseError: script is not a valid utf8 string"
            )));
        };

        ParseResult::from(exec_pipeline(file_name, script))
    }

    impl ParseResult {
        fn from(res: Result<Vec<u8>, String>) -> Self {
            match res {
                Ok(binary) => Self::new(true, binary),
                Err(message) => Self::new(false, message.into()),
            }
        }

        fn new(ok: bool, vec: Vec<u8>) -> Self {
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
