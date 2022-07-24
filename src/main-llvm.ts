import { parse } from "https://deno.land/std@0.149.0/flags/mod.ts";

import {
  buildLLVMIR,
  compileIR,
  compileToModule,
  interpret,
} from "./compiler/compiler.ts";
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

  const llvm = loadLibLLVM();
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
}
