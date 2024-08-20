#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

mod ast;
mod core;
mod ir;
mod ir_generator;
mod lexer;
mod parser;
mod parser_v2;
mod printer;
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

static USAGE: &str = "\
Usage: lo <file> [mode]
  where [mode] is either:
    --compile-v2 (temporary)
    --inspect
    --pretty-print
    --print-c
  No [mode] means compilation to wasm\
";

mod wasi_api {
    use crate::{core::*, ir_generator::*, lexer::*, parser, parser_v2::*, printer::*, USAGE};
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
            return Err(format!("{}", USAGE));
        }

        let mut file_name = args.get(1).unwrap();
        if file_name == "-i" {
            file_name = "<stdin>";
        }

        let compiler_mode = match args.get(2) {
            None => CompilerMode::Compile,
            Some("--compile-v2") => CompilerMode::CompileV2,
            Some("--inspect") => CompilerMode::Inspect,
            Some("--pretty-print") => CompilerMode::PrettyPrint,
            Some("--print-c") => CompilerMode::PrintC,
            Some(unknown_mode) => {
                return Err(format!("Unknown compiler mode: {unknown_mode}\n{}", USAGE));
            }
        };

        if compiler_mode == CompilerMode::CompileV2 {
            let mut files = Vec::new();
            parse_file_and_deps(&mut files, file_name, &LoLocation::internal())?;

            let mut ir_generator = IRGenerator::default();
            for file in &files {
                ir_generator.process_file(file)?;
            }
            let wasm_module = ir_generator.generate()?;

            let mut binary = Vec::new();
            wasm_module.dump(&mut binary);
            fputs(wasi::FD_STDOUT, binary.as_slice());

            return Ok(());
        }

        if compiler_mode == CompilerMode::PrettyPrint || compiler_mode == CompilerMode::PrintC {
            let chars = file_read_utf8(file_name)?;
            let tokens = Lexer::lex(file_name, &chars)?;
            let ast = ParserV2::parse(tokens)?;

            let print_format = if compiler_mode == CompilerMode::PrettyPrint {
                PrintFormat::PrettyPrint
            } else {
                PrintFormat::TranspileToC
            };
            Printer::print(ast, print_format);

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
