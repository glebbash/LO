import { parse } from "https://deno.land/std@0.149.0/flags/mod.ts";

import {
  buildModule,
  emitWasm,
  emitWAT,
  interpret,
} from "./compiler-wasm/compiler.ts";
import { expandFile } from "./expand-2/expand.ts";

if (import.meta.main) {
  mainWasm(parse(Deno.args));
}

export async function mainWasm(args: ReturnType<typeof parse>) {
  const inputFile = (args._[0] as string) ?? args.src;
  if (inputFile === undefined) {
    throw new Error("No input file specified");
  }

  const exprs = expandFile(inputFile);

  const module = buildModule(exprs);

  const outputWATFile = args.wat;
  if (outputWATFile !== undefined) {
    const wat = emitWAT(module);
    await Deno.writeTextFile(outputWATFile, wat);
  }

  const mode = args.includes("-r") ? "interpret" : "compile";
  if (mode === "interpret") {
    await interpret(module);
    return;
  }

  const outputBinaryFile = args.out ?? "output.wasm";
  const wasm = emitWasm(module);
  await Deno.writeFile(outputBinaryFile, wasm);
}
