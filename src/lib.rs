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
        return finalize_and_exit(1);
    }

    let command = match args.get(1).unwrap() {
        "compile" => LoCommand::Compile,
        "inspect" => LoCommand::Inspect,
        "format" => LoCommand::Format,
        "eval" => LoCommand::Eval,
        "wasi" => LoCommand::Wasi,
        unknown_command => {
            stderr_writeln(format!("Unknown command: {unknown_command}\n{}", USAGE));
            return finalize_and_exit(1);
        }
    };

    let mut file_name = args.get(2).unwrap();
    if file_name == "-i" {
        file_name = "<stdin>";
    }

    if command == LoCommand::Format {
        let mut compiler = Compiler::new(command);
        let Some(module) = compiler.import(file_name, &LoLocation::internal()) else {
            return finalize_and_exit(1);
        };

        Printer::print(UBox::new(&module.ast), module.source);

        return finalize_and_exit(0);
    }

    if command == LoCommand::Wasi {
        let module_bytes = catch!(file_read(file_name), err, {
            stderr_writeln(err);
            return finalize_and_exit(1);
        });

        let wasm_module = WasmParser::parse(String::from(file_name), module_bytes);
        let wasm_module = catch!(wasm_module, err, {
            stderr_writeln(err);
            return finalize_and_exit(1);
        });

        catch!(WasmEval::eval(wasm_module), err, {
            stderr_writeln(err.message);
            return finalize_and_exit(1);
        });

        return finalize_and_exit(0);
    }

    if command == LoCommand::Inspect {
        stdout_enable_buffering();
    }

    let mut compiler = Compiler::new(command);
    compiler.import(file_name, &LoLocation::internal());

    // safety: passes won't change size of modules
    let modules = unsafe_borrow(&compiler.modules);

    for module in modules {
        compiler.pass_collect_typedefs(&module);
    }

    for module in modules {
        compiler.pass_build_structs(&module);
    }

    for module in modules {
        compiler.pass_main(&module);
    }

    let mut wasm_module = WasmModule::default();
    compiler.generate(&mut wasm_module);

    compiler.end_inspection();
    if *compiler.error_count.borrow() > 0 {
        return finalize_and_exit(1);
    }

    if command == LoCommand::Inspect {
        return finalize_and_exit(0);
    }

    if command == LoCommand::Compile {
        let mut binary = Vec::new();
        wasm_module.dump(&mut binary);
        stdout_write(binary.as_slice());
        return finalize_and_exit(0);
    }

    if command == LoCommand::Eval {
        catch!(WasmEval::eval(wasm_module), err, {
            stderr_writeln(err.message);
            return finalize_and_exit(1);
        });
        return finalize_and_exit(0);
    }

    unreachable!();
}

fn finalize_and_exit(exit_code: u32) {
    stdout_disable_buffering();
    proc_exit(exit_code);
}
