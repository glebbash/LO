import {
  buildModule,
  emitWasm,
  emitWAT,
  interpret,
} from "./compiler-wasm/compiler.ts";
import { expandFile } from "./expand/expand.ts";

async function main() {
  const args = Deno.args;

  const inputFile = getArg(args, "src") ?? "examples/hello-world.lole";
  const exprs = expandFile(inputFile);

  const module = buildModule(exprs);

  const outputWATFile = getArg(args, "wat");
  if (outputWATFile !== undefined) {
    const wat = emitWAT(module);
    await Deno.writeTextFile(outputWATFile, wat);
  }

  const mode = args.includes("-r") ? "interpret" : "compile";
  if (mode === "interpret") {
    await interpret(module);
    return;
  }

  const outputBinaryFile = getArg(args, "out") ?? "output.wasm";
  const wasm = emitWasm(module);
  await Deno.writeFile(outputBinaryFile, wasm);
}

function getArg(args: string[], name: string): string | undefined {
  const argumentStart = `--${name}=`;

  return args
    .find((a) => a.startsWith(argumentStart))
    ?.slice(argumentStart.length);
}

main();
