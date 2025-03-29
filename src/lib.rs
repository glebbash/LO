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
Usage:
  lo compile <input.lo>
  lo inspect <input.lo>
  lo format <input.lo>
  lo eval <input.lo> (experimental)
  lo wasi <input.lo> (experimental)\
";

mod wasi_api {
    use crate::{
        ast::*, codegen::*, core::*, lexer::*, parser::*, printer::*, wasm_eval::*, wasm_parser::*,
        USAGE,
    };
    use alloc::{format, rc::Rc, string::String, vec::Vec};

    #[no_mangle]
    pub extern "C" fn _start() {
        let err = start().err();

        stdout_disable_buffering();

        if let Some(err_message) = err {
            stderr_write(err_message);
            stderr_write("\n");
            proc_exit(1);
        }
    }

    fn start() -> Result<(), String> {
        let args = WasiArgs::load().unwrap();
        if args.len() < 3 {
            return Err(format!("{}", USAGE));
        }

        let command = match args.get(1).unwrap() {
            "compile" => LoCommand::Compile,
            "inspect" => LoCommand::Inspect,
            "format" => LoCommand::Format,
            "eval" => LoCommand::Eval,
            "wasi" => LoCommand::Wasi,
            unknown_command => {
                return Err(format!("Unknown command: {unknown_command}\n{}", USAGE));
            }
        };

        let mut file_name = args.get(2).unwrap();
        if file_name == "-i" {
            file_name = "<stdin>";
        }

        if command == LoCommand::Format {
            let mut fm = FileManager::default();
            let included_file = fm
                .include_file(file_name, &LoLocation::internal())
                .map_err(|err| err.to_string(&fm))?;

            let tokens = Lexer::lex(
                included_file.file_index,
                &included_file.file_contents.unwrap(),
            )
            .map_err(|err| err.to_string(&fm))?;
            let ast = Parser::parse(tokens).map_err(|err| err.to_string(&fm))?;

            stdout_enable_buffering();
            Printer::print(Rc::new(ast));

            return Ok(());
        }

        if command == LoCommand::Wasi {
            let module_bytes = file_read(file_name)?;

            let wasm_module = WasmParser::parse(String::from(file_name), module_bytes)?;

            WasmEval::eval(wasm_module).map_err(|err| err.message)?;

            return Ok(());
        }

        if command == LoCommand::Inspect {
            stdout_enable_buffering();
        }

        let mut codegen = CodeGen::new(command);

        let included_file = codegen
            .fm
            .include_file(file_name, &LoLocation::internal())
            .map_err(|err| err.to_string(&codegen.fm))?;

        let mut asts = Vec::new();
        parse_file_tree(
            command,
            &mut codegen.fm,
            &mut asts,
            included_file.file_index,
            included_file.file_contents.unwrap(),
        )
        .map_err(|err| err.to_string(&codegen.fm))?;

        for ast in &asts {
            codegen
                .process_file_pass1(ast)
                .map_err(|err| err.to_string(&codegen.fm))?;
        }
        for ast in asts {
            codegen
                .process_file_pass2(ast)
                .map_err(|err| err.to_string(&codegen.fm))?;
        }
        codegen.errors.print_all(&codegen.fm)?;

        let wasm_module = codegen
            .generate()
            .map_err(|err| err.to_string(&codegen.fm))?;

        if command == LoCommand::Compile {
            let mut binary = Vec::new();
            wasm_module.dump(&mut binary);
            stdout_write(binary.as_slice());
        }

        if command == LoCommand::Eval {
            WasmEval::eval(wasm_module).map_err(|err| err.message)?;
        }

        return Ok(());
    }

    fn parse_file_tree(
        mode: LoCommand,
        fm: &mut FileManager,
        asts: &mut Vec<AST>,
        file_index: u32,
        file_contents: String,
    ) -> Result<(), LoError> {
        if mode == LoCommand::Inspect {
            let file_path = fm.get_file_path(file_index);
            stdout_writeln(format!(
                "{{ \"type\": \"file\", \
                    \"index\": {file_index}, \
                    \"path\": \"{file_path}\" }}, ",
            ));
        }

        let tokens = Lexer::lex(file_index, &file_contents)?;
        let ast = Parser::parse(tokens)?;

        // pass 0: parse all included files (recursive)
        for expr in &ast.exprs {
            let TopLevelExpr::Include(include) = expr else {
                continue;
            };

            let included_file = fm.include_file(&include.file_path.unescape(), &include.loc)?;

            if let Some(file_contents) = included_file.file_contents {
                parse_file_tree(mode, fm, asts, included_file.file_index, file_contents)?;
            }

            if mode == LoCommand::Inspect {
                let source_index = file_index;
                let source_range = RangeDisplay(&include.loc);
                let target_index = included_file.file_index;
                let target_range = "1:1-1:1";

                stdout_writeln(format!(
                    "{{ \"type\": \"info\", \
                        \"link\": \"{target_index}/{target_range}\", \
                        \"loc\": \"{source_index}/{source_range}\" }}, ",
                ));
            }
        }

        asts.push(ast);

        Ok(())
    }
}
