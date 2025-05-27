#![no_std]
#![feature(alloc_error_handler, thread_local, let_chains)]

extern crate alloc;

mod ast;
mod codegen;
mod core;
mod lexer;
mod parser;
mod printer;
mod wasm;
mod wasm_eval;
mod wasm_parser;

mod wasm_target {
    use lol_alloc::{FreeListAllocator, LockedAllocator};

    #[global_allocator]
    static ALLOCATOR: LockedAllocator<FreeListAllocator> =
        LockedAllocator::new(FreeListAllocator::new());

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

use crate::{
    ast::*, codegen::*, core::*, lexer::*, parser::*, printer::*, wasm::*, wasm_eval::*,
    wasm_parser::*,
};
use alloc::{format, rc::Rc, string::String, vec::Vec};

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
        finalize_and_exit(1);
    }

    let command = match args.get(1).unwrap() {
        "compile" => LoCommand::Compile,
        "inspect" => LoCommand::Inspect,
        "format" => LoCommand::Format,
        "eval" => LoCommand::Eval,
        "wasi" => LoCommand::Wasi,
        unknown_command => {
            stderr_writeln(format!("Unknown command: {unknown_command}\n{}", USAGE));
            finalize_and_exit(1);
        }
    };

    let mut file_name = args.get(2).unwrap();
    if file_name == "-i" {
        file_name = "<stdin>";
    }

    if command == LoCommand::Format {
        let mut fm = FileManager::default();
        let included_file = catch!(fm.include_file(file_name, &LoLocation::internal()), err, {
            stderr_writeln(err.to_string(&fm));
            finalize_and_exit(1);
        });

        let tokens = Lexer::lex(
            included_file.file_index,
            &included_file.file_contents.unwrap(),
        );
        let tokens = catch!(tokens, err, {
            stderr_writeln(err.to_string(&fm));
            finalize_and_exit(1);
        });
        let ast = catch!(Parser::parse(tokens), err, {
            stderr_writeln(err.to_string(&fm));
            finalize_and_exit(1);
        });

        stdout_enable_buffering();
        Printer::print(Rc::new(ast));

        finalize_and_exit(0);
    }

    if command == LoCommand::Wasi {
        let module_bytes = catch!(file_read(file_name), err, {
            stderr_writeln(err);
            finalize_and_exit(1);
        });

        let wasm_module = WasmParser::parse(String::from(file_name), module_bytes);
        let wasm_module = catch!(wasm_module, err, {
            stderr_writeln(err);
            finalize_and_exit(1);
        });

        catch!(WasmEval::eval(wasm_module), err, {
            stderr_writeln(err.message);
            finalize_and_exit(1);
        });

        finalize_and_exit(0);
    }

    if command == LoCommand::Inspect {
        stdout_enable_buffering();
    }

    let mut codegen = CodeGen::new(command);

    let included_file = codegen.fm.include_file(file_name, &LoLocation::internal());
    let included_file = catch!(included_file, err, {
        stderr_writeln(err.to_string(&codegen.fm));
        finalize_and_exit(1);
    });

    let mut asts = Vec::new();
    parse_file_tree(
        &mut codegen,
        &mut asts,
        included_file.file_index,
        included_file.file_contents.unwrap(),
    );
    for ast in &asts {
        codegen.pass_collect_typedefs(ast);
    }
    for ast in &asts {
        codegen.pass_build_structs(ast);
    }
    for ast in asts {
        codegen.pass_main(ast);
    }

    let mut wasm_module = WasmModule::default();
    codegen.generate(&mut wasm_module);

    codegen.end_inspection();
    if *codegen.error_count.borrow() > 0 {
        finalize_and_exit(1);
    }

    if command == LoCommand::Inspect {
        finalize_and_exit(0);
    }

    if command == LoCommand::Compile {
        let mut binary = Vec::new();
        wasm_module.dump(&mut binary);
        stdout_write(binary.as_slice());
        finalize_and_exit(0);
    }

    if command == LoCommand::Eval {
        catch!(WasmEval::eval(wasm_module), err, {
            stderr_writeln(err.message);
            finalize_and_exit(1);
        });
        finalize_and_exit(0);
    }

    unreachable!();
}

fn parse_file_tree(
    codegen: &mut CodeGen,
    asts: &mut Vec<AST>,
    file_index: u32,
    file_contents: String,
) {
    if codegen.command == LoCommand::Inspect {
        let file_path = codegen.fm.get_file_path(file_index);
        stdout_writeln(format!(
            "{{ \"type\": \"file\", \
                \"index\": {file_index}, \
                \"path\": \"{file_path}\" }},",
        ));
    }

    let tokens = catch!(Lexer::lex(file_index, &file_contents), err, {
        return codegen.report_error(err);
    });

    let ast = catch!(Parser::parse(tokens), err, {
        return codegen.report_error(err);
    });

    // pass 0: parse all included files (recursive)
    for expr in &ast.exprs {
        let TopLevelExpr::Include(include) = expr else {
            continue;
        };

        let included_file = codegen
            .fm
            .include_file(&include.file_path.unescape(), &include.loc);
        let included_file = catch!(included_file, err, {
            codegen.report_error(err);
            continue;
        });

        if let Some(file_contents) = included_file.file_contents {
            parse_file_tree(codegen, asts, included_file.file_index, file_contents);
        }

        if codegen.command == LoCommand::Inspect {
            let source_index = file_index;
            let source_range = RangeDisplay(&include.loc);
            let target_index = included_file.file_index;
            let target_range = "1:1-1:1";

            stdout_writeln(format!(
                "{{ \"type\": \"info\", \
                    \"link\": \"{target_index}/{target_range}\", \
                    \"loc\": \"{source_index}/{source_range}\" }},",
            ));
        }
    }

    asts.push(ast);
}

fn finalize_and_exit(exit_code: u32) -> ! {
    stdout_disable_buffering();
    proc_exit(exit_code);
}
