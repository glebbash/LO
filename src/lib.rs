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

static USAGE: &str = "\
Usage: lo <file> [<mode>] [--v1|--v2]
  Where <mode> is either:
    --compile (default if not provided)
    --inspect
    --pretty-print
    --eval (experimental)
    --eval-wasm (experimental)\
";

mod wasi_api {
    use crate::{
        ast::*, codegen::*, core::*, lexer::*, parser, parser_v2::*, printer::*, wasm_eval::*,
        wasm_parser::*, USAGE,
    };
    use alloc::{format, rc::Rc, string::String, vec::Vec};

    #[no_mangle]
    pub extern "C" fn _start() {
        let err = start().err();

        stdout_disable_bufferring();

        if let Some(err_message) = err {
            stderr_write(err_message);
            stderr_write("\n");
            proc_exit(1);
        }
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

        let mut is_v2 = false;
        let mut compiler_mode = CompilerMode::Compile;

        if let Some(compiler_mode_arg) = args.get(2) {
            match compiler_mode_arg {
                "--compile" => {
                    compiler_mode = CompilerMode::Compile;
                }
                "--inspect" => {
                    // is_v2 = true;
                    compiler_mode = CompilerMode::Inspect;
                }
                "--pretty-print" => {
                    compiler_mode = CompilerMode::PrettyPrint;
                }
                "--eval" => {
                    compiler_mode = CompilerMode::Eval;
                }
                "--eval-wasm" => {
                    compiler_mode = CompilerMode::EvalWasm;
                }
                unknown_mode => {
                    return Err(format!("Unknown compiler mode: {unknown_mode}\n{}", USAGE));
                }
            }
        }

        if let Some(version_arg) = args.get(3) {
            match version_arg {
                "--v1" => {
                    is_v2 = false;
                }
                "--v2" => {
                    is_v2 = true;
                }
                unknown_version => {
                    return Err(format!("Unknown version: {unknown_version}\n{}", USAGE));
                }
            }
        }

        if compiler_mode == CompilerMode::PrettyPrint {
            let mut fm = FileManager::default();
            let (file_index, file_contents) = fm
                .include_file(file_name, &LoLocation::internal())
                .map_err(|err| err.to_string(&fm))?;

            let tokens = Lexer::lex(file_index, &file_contents.unwrap())
                .map_err(|err| err.to_string(&fm))?;
            let ast = ParserV2::parse(tokens).map_err(|err| err.to_string(&fm))?;

            stdout_enable_bufferring();
            Printer::print(Rc::new(ast));

            return Ok(());
        }

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
            let mut codegen = CodeGen::new(compiler_mode);

            let (file_index, file_contents) = codegen
                .fm
                .include_file(file_name, &LoLocation::internal())
                .map_err(|err| err.to_string(&codegen.fm))?;

            let mut asts = Vec::new();
            parse_file_tree(
                compiler_mode,
                &mut codegen.fm,
                &mut asts,
                file_index,
                file_contents.unwrap(),
            )
            .map_err(|err| err.to_string(&codegen.fm))?;

            for ast in asts {
                codegen
                    .process_file(ast)
                    .map_err(|err| err.to_string(&codegen.fm))?;
            }
            codegen.errors.print_all(&codegen.fm)?;

            codegen
                .generate()
                .map_err(|err| err.to_string(&codegen.fm))?
        } else {
            let ctx = &mut parser::init(compiler_mode);
            parser::parse_file(ctx, file_name, &LoLocation::internal())
                .map_err(|err| err.to_string(&ctx.fm))?;
            parser::finalize(ctx).map_err(|err| err.to_string(&ctx.fm))?;

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

    fn parse_file_tree(
        mode: CompilerMode,
        fm: &mut FileManager,
        asts: &mut Vec<AST>,
        file_index: u32,
        file_contents: String,
    ) -> Result<(), LoError> {
        if mode == CompilerMode::Inspect {
            let file_path = fm.get_file_path(file_index);
            stdout_writeln(format!(
                "{{ \"type\": \"file\", \
                    \"index\": {file_index}, \
                    \"path\": \"{file_path}\" }}, ",
            ));
        }

        let tokens = Lexer::lex(file_index, &file_contents)?;
        let ast = ParserV2::parse(tokens)?;

        for expr in &ast.exprs {
            let TopLevelExpr::Include(include) = expr else {
                continue;
            };

            let (target_file_index, file_contents) =
                fm.include_file(&include.file_path.unescape(), &include.loc)?;

            if let Some(file_contents) = file_contents {
                parse_file_tree(mode, fm, asts, target_file_index, file_contents)?;
            }

            if mode == CompilerMode::Inspect {
                let source_index = file_index;
                let source_range = RangeDisplay(&include.loc);
                let target_index = target_file_index;
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
