#![no_std]
#![feature(alloc_error_handler, thread_local)]

extern crate alloc;

mod ast;
mod codegen;
mod core;
mod ir;
mod lexer;
mod parser;
mod parser_v2;
mod printer;
mod wasm;
mod wasm_eval;
mod wasm_parser;

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
    fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
        crate::core::stderr_write(alloc::format!("{info}\n"));
        core::arch::wasm32::unreachable();
    }
}

static USAGE: &str = "\
Usage: lo <file> [mode]
  where [mode] is either:
    --compile-v2 (temporary)
    --inspect
    --pretty-print
    --eval (experimental)
    --eval-wasm (experimental)
  No [mode] means compilation to wasm\
";

mod wasi_api {
    use crate::{
        codegen::*, core::*, lexer::*, parser, parser_v2::*, printer::*, wasm_eval::*,
        wasm_parser::*, USAGE,
    };
    use alloc::{format, rc::Rc, string::String, vec::Vec};

    #[no_mangle]
    pub extern "C" fn _start() {
        start().unwrap_or_else(|err_message| {
            stdout_disable_bufferring();

            stderr_write(err_message);
            stderr_write("\n");
            proc_exit(1);
        });

        stdout_disable_bufferring();
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
            Some("--eval") => CompilerMode::Eval,
            Some("--eval-wasm") => CompilerMode::EvalWasm,
            Some(unknown_mode) => {
                return Err(format!("Unknown compiler mode: {unknown_mode}\n{}", USAGE));
            }
        };

        if compiler_mode == CompilerMode::CompileV2 {
            let mut files = Vec::new();
            parse_file_and_deps(&mut files, file_name, &LoLocation::internal())?;

            let mut codegen = CodeGen::with_default_types();
            for file in files {
                codegen.add_file(file)?;
            }
            codegen.errors.print_all()?;
            let wasm_module = codegen.generate()?;

            let mut binary = Vec::new();
            wasm_module.dump(&mut binary);
            fputs(wasi::FD_STDOUT, binary.as_slice());

            return Ok(());
        }

        if compiler_mode == CompilerMode::PrettyPrint {
            let chars = file_read_utf8(file_name)?;
            let tokens = Lexer::lex(file_name, &chars)?;
            let ast = ParserV2::parse(tokens)?;

            stdout_enable_bufferring();
            Printer::print(Rc::new(ast));

            return Ok(());
        };

        if compiler_mode == CompilerMode::EvalWasm {
            let module_bytes = file_read(file_name)?;

            let wasm_module = WasmParser::parse(String::from(file_name), module_bytes)?;

            WasmEval::eval(wasm_module).map_err(|err| err.message)?;

            return Ok(());
        }

        if compiler_mode == CompilerMode::Inspect {
            stdout_enable_bufferring();
        }

        let ctx = &mut parser::init(compiler_mode);

        parser::parse_file(ctx, file_name, &LoLocation::internal())?;

        parser::finalize(ctx)?;

        if ctx.mode == CompilerMode::Compile {
            let mut binary = Vec::new();
            ctx.wasm_module.take().dump(&mut binary);
            fputs(wasi::FD_STDOUT, binary.as_slice());
        }

        if ctx.mode == CompilerMode::Eval {
            let wasm_module = ctx.wasm_module.take();

            WasmEval::eval(wasm_module).map_err(|err| err.message)?;
        }

        return Ok(());
    }
}
