#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

mod ast;
mod core;
mod emit_c;
mod ir;
mod lexer;
mod parser;
mod parser_v2;
mod pretty_print;
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
    use crate::{core::*, emit_c, lexer, parser, parser_v2, pretty_print};
    use alloc::{format, string::String, vec::Vec};

    #[no_mangle]
    pub extern "C" fn _start() {
        start().unwrap_or_else(|err_message| {
            stderr_write(err_message);
            stderr_write("\n");
            proc_exit(1);
        });
    }

    fn start() -> Result<(), String> {
        let args = WasiArgs::load().unwrap();
        if args.len() < 2 {
            return Err(format!("Usage: lo <file> [mode]"));
        }

        let mut file_name = args.get(1).unwrap();
        if file_name == "-i" {
            file_name = "<stdin>";
        }

        let compiler_mode = match args.get(2) {
            None => CompilerMode::Compile,
            Some("--inspect") => CompilerMode::Inspect,
            Some("--pretty-print") => CompilerMode::PrettyPrint,
            Some("--emit-c") => CompilerMode::EmitC,
            Some(unknown_mode) => {
                return Err(format!("Unknown compiler mode: {unknown_mode}"));
            }
        };

        if compiler_mode == CompilerMode::PrettyPrint {
            let chars = file_read_utf8(file_name)?;
            let tokens = lexer::lex_all(file_name, &chars)?;
            let ast = parser_v2::parse(tokens)?;
            pretty_print::pretty_print(&ast);
            return Ok(());
        };

        if compiler_mode == CompilerMode::EmitC {
            let chars = file_read_utf8(file_name)?;
            let tokens = lexer::lex_all(file_name, &chars)?;
            let ast = parser_v2::parse(tokens)?;
            emit_c::emit_c(&ast);
            return Ok(());
        };

        let ctx = &mut parser::init(compiler_mode);

        parser::parse_file(ctx, file_name, &LoLocation::internal())?;

        parser::finalize(ctx)?;

        if ctx.mode == CompilerMode::Compile {
            let mut binary = Vec::new();
            ctx.wasm_module.take().dump(&mut binary);
            fputs(wasi::FD_STDOUT, binary.as_slice());
        }

        return Ok(());
    }
}
