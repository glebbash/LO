import { compile } from "./compiler/compiler.ts";
import { expandFile } from "./expand/expand.ts";

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

async function compileIR(llvmIR: string, outputBinaryFile: string) {
  const clang = Deno.run({
    cmd: [
      "clang-13",
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

async function interpretIR(llvmIR: string) {
  const lli = Deno.run({ cmd: ["lli-13"], stdin: "piped" });
  lli.stdin?.write(new TextEncoder().encode(llvmIR));
  lli.stdin.close();
  await lli.status();
}

main();
