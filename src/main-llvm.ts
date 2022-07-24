import { parse } from "https://deno.land/std@0.149.0/flags/mod.ts";

import { buildIR, compileToModule } from "./compiler/compiler.ts";
import { expandFile } from "./expand/expand.ts";
import { loadLibLLVM } from "./llvm/llvm-c.ts";

if (import.meta.main) {
  mainLLVM(parse(Deno.args));
}

export async function mainLLVM(args: ReturnType<typeof parse>) {
  const inputFile = (args._[0] as string) ?? args.src;
  if (inputFile === undefined) {
    throw new Error("No input file specified");
  }

  const mode = args.r ? "interpret" : "compile";
  if (mode === "interpret") {
    interpret(inputFile);
    return;
  }

  const outputIRFile = args.ir;
  const llvmIR = buildIR(expandFile(inputFile));

  if (outputIRFile !== undefined) {
    await Deno.writeTextFile(outputIRFile, llvmIR);
  }

  const outputBinaryFile = args.out ?? "output";
  await compileIR(llvmIR, outputBinaryFile);
}

async function compileIR(llvmIR: string, outputBinaryFile: string) {
  const clang = Deno.run({
    cmd: [
      "clang-14",
      "-O3",
      "-o",
      outputBinaryFile,
      "-Wno-override-module",
      "-x",
      "ir",
      "-",
    ],
    stdin: "piped",
  });
  clang.stdin?.write(new TextEncoder().encode(llvmIR));
  clang.stdin.close();
  await clang.status();
}

function interpret(inputFile: string) {
  const llvm = loadLibLLVM();

  llvm.linkInMCJIT();
  llvm.initializeX86TargetInfo();
  llvm.initializeX86Target();
  llvm.initializeX86TargetMC();
  llvm.initializeX86AsmPrinter();

  const moduleCtx = compileToModule(expandFile(inputFile), llvm);

  const { ok, message, engine } = llvm.createExecutionEngineForModule(
    moduleCtx.module,
  );

  if (!ok) {
    console.error("Error during JIT compilation:", message);
    Deno.exit(1);
  }

  const fn = llvm.getFunctionAddress(engine, "main", {
    parameters: [],
    result: "i32",
  });

  fn.call();

  // llvm.removeModule(engine, moduleCtx.module);
  llvm.disposeExecutionEngine(engine);
  // disposeContext(moduleCtx);
}
