#![no_std]
#![feature(alloc_error_handler, thread_local, let_chains)]

extern crate alloc;

mod ast;
mod compiler;
mod core;
mod lexer;
mod lol_alloc;
mod parser;
mod printer;
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

#[no_mangle]
pub extern "C" fn _start() {
    let args = WasiArgs::load().unwrap();
    if args.len() < 3 {
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

        catch!(WasmEval::eval(wasm_module), err, {
            stderr_writeln(err.message);
            proc_exit(1);
        });

        return;
    }

    // for debug purposes only, not public api
    if command == "lex" {
        let mut compiler = Compiler::new();
        let Some(module) = compiler
            .relax_mut()
            .include(file_name, &Loc::internal())
        else {
            proc_exit(1)
        };

        let file_path = &compiler.fm.files[module.parser.lexer.file_index].absolute_path;
        stdout_writeln(format!("file_path: {file_path}"));

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

        return;
    }

    let mut compiler = Compiler::new();

    if command == "format" {
        compiler.in_single_file_mode = true;

        let Some(module) = compiler.include(file_name, &Loc::internal()) else {
            proc_exit(1);
        };

        Printer::print(module.parser.relax());

        return;
    }

    if command == "inspect" {
        compiler.begin_inspection();
    }

    compiler.include(file_name, &Loc::internal());

    compiler.run_passes();

    let mut wasm_module = WasmModule::default();
    compiler.generate(&mut wasm_module);

    if compiler.in_inspection_mode {
        compiler.end_inspection();

        if *compiler.error_count.borrow() == 0 {
            return;
        }
    }

    if *compiler.error_count.borrow() > 0 {
        proc_exit(1);
    }

    if command == "compile" {
        let mut binary = Vec::new();
        wasm_module.dump(&mut binary);
        stdout_write(binary.as_slice());
        return;
    }

    if command == "eval" {
        catch!(WasmEval::eval(wasm_module), err, {
            stderr_writeln(err.message);
            proc_exit(1);
        });
        return;
    }

    stderr_writeln(format!("Unknown command: {command}\n{}", USAGE));
    proc_exit(1);
}
