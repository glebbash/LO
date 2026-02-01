#![no_std]
#![feature(alloc_error_handler, thread_local)]

extern crate alloc;

mod ast;
mod compiler;
mod core;
mod lexer;
mod lol_alloc;
mod parser;
mod printer;
mod type_checker;
mod wasi;
mod wasm;
mod wasm_eval;
mod wasm_parser;

mod wasm_target {
    use crate::lol_alloc::LolAlloc;

    #[global_allocator]
    static ALLOCATOR: LolAlloc = LolAlloc::new();

    #[alloc_error_handler]
    fn oom(_: core::alloc::Layout) -> ! {
        core::arch::wasm32::unreachable()
    }

    #[cfg(not(test))]
    #[panic_handler]
    fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
        crate::core::stderr_write(alloc::format!("{info}\n"));
        core::arch::wasm32::unreachable();
    }
}

use crate::{compiler::*, core::*, printer::*, wasm::*, wasm_eval::*, wasm_parser::*};
use alloc::{format, string::String, vec::Vec};

static USAGE: &str = "Usage:
  lo compile <input.lo>
  lo inspect <input.lo>
  lo format <input.lo>
  lo eval <input.lo> (experimental)
  lo wasi <input.lo> (experimental)";

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    let args = WasiArgs::load().unwrap();
    if args.size < 3 {
        stderr_writeln(USAGE);
        proc_exit(1);
    }

    let command = args.get(1).unwrap();
    let mut file_name = args.get(2).unwrap();
    if file_name == "-i" {
        file_name = "<stdin>";
    }

    if command == "wasi" {
        let module_bytes = catch!(file_read(file_name), err, {
            stderr_writeln(err);
            proc_exit(1);
        });

        let wasm_module = WasmParser::parse(String::from(file_name), module_bytes);
        let wasm_module = catch!(wasm_module, err, {
            stderr_writeln(err);
            proc_exit(1);
        });

        let mut eval = WasmEval::new(wasm_module);
        eval.wasi_args = Some(args);
        eval.wasi_args_skip = 2;

        catch!(eval.eval(), err, {
            stderr_writeln(err.message);
            proc_exit(1);
        });

        return;
    }

    let mut compiler = Compiler::new();

    // for debug purposes only, not public api
    if command == "lex" {
        compiler.in_single_file_mode = true;
        compiler.in_lex_only_mode = true;

        let Some(module) = compiler.relax_mut().include(file_name, &Loc::internal()) else {
            proc_exit(1)
        };

        let file_info = &compiler.fm.files[module.parser.lexer.file_index];
        stdout_writeln(format!("file_path: {}", file_info.absolute_path));

        stdout_enable_buffering();

        let source = module.parser.lexer.source;
        for token in &module.parser.lexer.tokens {
            stdout_writeln(format!(
                "{}:{}-{}:{} ({}-{}) {:?} >> {} <<",
                token.loc.pos.line,
                token.loc.pos.col,
                token.loc.end_pos.line,
                token.loc.end_pos.col,
                token.loc.pos.offset,
                token.loc.end_pos.offset,
                token.type_,
                token.loc.read_span(source).replace("\n", "\\n"),
            ));
        }

        stdout_disable_buffering();

        return;
    }

    if command == "format" {
        compiler.in_single_file_mode = true;

        let Some(module) = compiler.include(file_name, &Loc::internal()) else {
            proc_exit(1);
        };

        let mut printer = Printer::new(&module.parser.relax());
        printer.print_file();

        return;
    }

    if command == "inspect" {
        compiler.reporter.begin_inspection();
    }

    compiler.include(file_name, &Loc::internal());

    compiler.run_passes();

    let mut wasm_module = WasmModule::default();
    compiler.generate(&mut wasm_module);

    if compiler.reporter.in_inspection_mode {
        compiler.reporter.end_inspection();

        if *compiler.reporter.error_count == 0 {
            return;
        }
    }

    if *compiler.reporter.error_count > 0 {
        proc_exit(1);
    }

    if command == "compile" {
        let mut binary = Vec::new();
        wasm_module.dump(&mut binary);
        stdout_write(binary.as_slice());
        return;
    }

    if command == "eval" {
        let mut eval = WasmEval::new(wasm_module);
        eval.wasi_args = Some(args);
        eval.wasi_args_skip = 2;

        catch!(eval.eval(), err, {
            stderr_writeln(err.message);
            proc_exit(1);
        });
        return;
    }

    stderr_writeln(format!("Unknown command: {command}\n{}", USAGE));
    proc_exit(1);
}
