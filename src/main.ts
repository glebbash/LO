import { compileFile } from "./compiler/compiler.ts";

async function main() {
  const args = Deno.args;

  const mode = args.includes("-r") ? "interpret" : "compile";
  const inputFile = getArg(args, "src") ?? "examples/hello-world.lole";
  const outputIRFile = getArg(args, "out-ir") ?? "output.ll";
  const outputBinaryFile = getArg(args, "out") ?? "output";

  const llvmIR = compileFile(inputFile);

  await Deno.writeTextFile(outputIRFile, llvmIR);

  if (mode === "compile") {
    await compileIR(outputIRFile, outputBinaryFile);
  } else {
    await interpretIR(outputIRFile);
  }
}

function getArg(args: string[], name: string): string | undefined {
  const argumentStart = `--${name}=`;

  return args
    .find((a) => a.startsWith(argumentStart))
    ?.slice(argumentStart.length);
}

async function compileIR(llvmIRFile: string, outputBinaryFile: string) {
  await run(
    [
      "clang-13",
      "-O3",
      "-o",
      outputBinaryFile,
      llvmIRFile,
      "-Wno-override-module",
    ],
  );
}

async function interpretIR(llvmIRFile: string) {
  await run(["lli-13", llvmIRFile]);
}

async function run(cmd: string[]): Promise<void> {
  await Deno.run({ cmd }).status();
}

main();
