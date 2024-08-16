#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

mod emit_c;
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
    use super::{ir::*, parser, utils::*};
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

        let mut compiler_mode = CompilerMode::Compile;
        if args.get(2) == Some("--inspect") {
            compiler_mode = CompilerMode::Inspect;
        } else if args.get(2) == Some("--emit-c") {
            compiler_mode = CompilerMode::EmitC;
        }

        let ctx = &mut parser::init(compiler_mode);

        let file_name = args.get(1).unwrap();
        if file_name == "-i" {
            parser::parse_file_contents(ctx, String::from("<stdin>"), &stdin_read())?;
        } else {
            if let Err(err) = unlock_fs() {
                return Err(format!("Error unlocking fs: {err}"));
            }
            parser::parse_file(ctx, file_name, &LoLocation::internal())?;
        }

        parser::finalize(ctx)?;

        if ctx.mode != CompilerMode::Compile {
            return Ok(());
        }

        let mut binary = Vec::new();
        ctx.wasm_module.take().dump(&mut binary);
        stdout_write(binary.as_slice());
        return Ok(());
    }
}
