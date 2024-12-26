#![no_std]
#![feature(alloc_error_handler, thread_local, let_chains)]

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

// TODO: add tests for --inspect mode
// TODO: add --inspect functionality for v2
// TODO: add debug section emitting for v2

static USAGE: &str = "\
Usage: lo <file> [<mode>] [--v2]
  Where <mode> is either:
    --compile (default if not provided)
    --inspect
    --pretty-print
    --eval (experimental)
    --eval-wasm (experimental)\
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
            Some("--compile") => CompilerMode::Compile,
            Some("--inspect") => CompilerMode::Inspect,
            Some("--pretty-print") => CompilerMode::PrettyPrint,
            Some("--eval") => CompilerMode::Eval,
            Some("--eval-wasm") => CompilerMode::EvalWasm,
            Some(unknown_mode) => {
                return Err(format!("Unknown compiler mode: {unknown_mode}\n{}", USAGE));
            }
        };

        let mut is_v2 = false;
        if let Some(version_arg) = args.get(3) {
            if version_arg == "--v2" {
                is_v2 = true;
            } else {
                return Err(format!("Unknown version: {version_arg}\n{}", USAGE));
            }
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

        let wasm_module = if is_v2 {
            let mut files = Vec::new();
            parse_file_and_deps(
                &mut files,
                file_name,
                &mut Vec::new(),
                &LoLocation::internal(),
            )?;

            let mut codegen = CodeGen::with_default_types();
            for file in files {
                codegen.add_file(file)?;
            }
            codegen.errors.print_all()?;

            codegen.generate()?
        } else {
            let ctx = &mut parser::init(compiler_mode.clone());
            parser::parse_file(ctx, file_name, &LoLocation::internal())?;
            parser::finalize(ctx)?;

            ctx.wasm_module.take()
        };

        if compiler_mode == CompilerMode::Compile {
            let mut binary = Vec::new();
            wasm_module.dump(&mut binary);
            fputs(wasi::FD_STDOUT, binary.as_slice());
        }

        if compiler_mode == CompilerMode::Eval {
            WasmEval::eval(wasm_module).map_err(|err| err.message)?;
        }

        return Ok(());
    }
}
