import { parse } from "https://deno.land/std@0.149.0/flags/mod.ts";

import {
  buildLLVMIR,
  compileIR,
  compileToModule,
  interpret,
} from "./compiler/compiler.ts";
import { expandFile } from "./expand/expand.ts";
import { loadLLVM } from "./llvm-c-14/llvm-c/mod.ts";

const LLVM_PATH = "/usr/lib/llvm-14/lib/libLLVM.so";

if (import.meta.main) {
  mainLLVM2(parse(Deno.args));
}

export async function mainLLVM2(args: ReturnType<typeof parse>) {
  const inputFile = (args._[0] as string) ?? args.src;
  if (inputFile === undefined) {
    throw new Error("No input file specified");
  }

  const llvm = loadLLVM(LLVM_PATH);

  const module = compileToModule(expandFile(inputFile), llvm);

  const mode = args.r ? "interpret" : "compile";
  if (mode === "interpret") {
    interpret(module);
    return;
  }

  const llvmIR = buildLLVMIR(module);

  const outputIRFile = args.ir;
  if (outputIRFile !== undefined) {
    await Deno.writeTextFile(outputIRFile, llvmIR);
  }

  const outputBinaryFile = args.out ?? "output";
  await compileIR(llvmIR, outputBinaryFile);

  module.dispose();
  llvm.close();
}
