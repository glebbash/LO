import { compile, compileToModule } from "./compiler/compiler.ts";
import { expandFile } from "./expand/expand.ts";
import { loadLibLLVM } from "./llvm/llvm-c.ts";

async function main() {
  const args = Deno.args;
  const inputFile = getArg(args, "src") ?? "examples/hello-world.lole";

  const mode = args.includes("-r") ? "interpret" : "compile";
  if (mode === "interpret") {
    interpret(inputFile);
    return;
  }

  const outputIRFile = getArg(args, "ir");
  const llvmIR = compile(expandFile(inputFile));

  if (outputIRFile !== undefined) {
    await Deno.writeTextFile(outputIRFile, llvmIR);
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

main();
