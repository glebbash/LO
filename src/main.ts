import { compileFile } from "./compiler/compiler.ts";

async function main() {
  const args = Deno.args;

  const mode = args.includes("-r") ? "interpret" : "compile";
  const inputFile = getArg(args, "src") ?? "examples/hello-world.lole";
  const outputIRFile = getArg(args, "out-ir") ?? "output.ll";
  const outputBinaryFile = getArg(args, "out") ?? "output";

  const llvmIR = compileFile(inputFile);

  if (mode === "compile") {
    await Deno.writeTextFile(outputIRFile, llvmIR);
    await compileIR(outputIRFile, outputBinaryFile);
  } else {
    await interpretIR(llvmIR);
  }
}

function getArg(args: string[], name: string): string | undefined {
  const argumentStart = `--${name}=`;

  return args
    .find((a) => a.startsWith(argumentStart))
    ?.slice(argumentStart.length);
}

async function compileIR(llvmIRFile: string, outputBinaryFile: string) {
  await Deno.run({
    cmd: [
      "clang-13",
      "-O3",
      "-o",
      outputBinaryFile,
      llvmIRFile,
      "-Wno-override-module",
    ],
  }).status();
}

async function interpretIR(llvmIR: string) {
  const lli = Deno.run({ cmd: ["lli-13"], stdin: "piped" });
  lli.stdin?.write(new TextEncoder().encode(llvmIR));
  lli.stdin.close();
  await lli.status();
}

main();
