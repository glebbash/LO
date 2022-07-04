import { compile } from "./compiler/compiler.ts";
import { expandFile } from "./expand/expand.ts";
import { compileIR, interpretIR } from "./compiler/utils.ts";

async function main() {
  const args = Deno.args;

  const inputFile = getArg(args, "src") ?? "examples/hello-world.lole";
  const llvmIR = compile(expandFile(inputFile));

  const outputIRFile = getArg(args, "ir");
  if (outputIRFile !== undefined) {
    await Deno.writeTextFile(outputIRFile, llvmIR);
  }

  const mode = args.includes("-r") ? "interpret" : "compile";
  if (mode === "interpret") {
    await interpretIR(llvmIR);
    return;
  }

  const outputBinaryFile = getArg(args, "out") ?? "output";
  await compileIR(llvmIR, outputBinaryFile);
}

function getArg(args: string[], name: string): string | undefined {
  const argumentStart = `--${name}=`;

  return args
    .find((a) => a.startsWith(argumentStart))
    ?.slice(argumentStart.length);
}

main();
