#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

mod ir;
mod lexer;
mod parser;
mod utils;
mod wasm;

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

mod wasi_api {
    use super::{ir, parser, utils::*};
    use alloc::{format, string::String, vec::Vec};

    #[no_mangle]
    pub extern "C" fn _start() {
        start().unwrap_or_else(|mut err_message| {
            err_message.push('\n');
            stderr_write(err_message.as_bytes());
            proc_exit(1);
        });
    }

    fn start() -> Result<(), String> {
        let args = WasiArgs::load().unwrap();
        if args.len() < 2 {
            return Err(format!("Usage: lo <file> [options]"));
        }

        let ctx = &mut ir::ModuleContext::default();
        ctx.inspect_mode = args.get(2) == Some("--inspect");

        let file_name = args.get(1).unwrap();
        if file_name == "-i" {
            parser::parse_file_with_contents(ctx, "<stdin>", &stdin_read())?;
        } else {
            do_cwd_extra_steps().unwrap();
            parser::parse_file(ctx, file_name, &LoLocation::internal())?;
        }

        parser::finalize(ctx)?;

        if ctx.inspect_mode {
            return Ok(());
        }

        let mut binary = Vec::new();
        ctx.wasm_module.take().dump(&mut binary);
        stdout_write(binary.as_slice());
        return Ok(());
    }
}

mod fn_api {
    use super::{ir, parser};
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
        file_contents_ptr: *const u8,
        file_contents_len: usize,
    ) -> ParseResult {
        match _compile(
            file_name_ptr,
            file_name_len,
            file_contents_ptr,
            file_contents_len,
        ) {
            Ok(bytes) => ParseResult::new(true, bytes),
            Err(message) => ParseResult::new(false, message.into_bytes()),
        }
    }

    fn _compile(
        file_name_ptr: *const u8,
        file_name_len: usize,
        file_contents_ptr: *const u8,
        file_contents_len: usize,
    ) -> Result<Vec<u8>, String> {
        let file_name_bytes = unsafe { slice::from_raw_parts(file_name_ptr, file_name_len) };
        let Ok(file_name) = str::from_utf8(file_name_bytes) else {
            return Err(format!("ParseError: file_name is not a valid UTF-8 string"));
        };

        let file_contents = unsafe { slice::from_raw_parts(file_contents_ptr, file_contents_len) };

        let ctx = &mut ir::ModuleContext::default();
        parser::parse_file_with_contents(ctx, file_name, file_contents)?;
        parser::finalize(ctx)?;

        let mut binary = Vec::new();
        ctx.wasm_module.take().dump(&mut binary);
        Ok(binary)
    }

    impl ParseResult {
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
