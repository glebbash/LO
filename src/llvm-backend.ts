export async function compileIR(llvmIR: string, outputBinaryFile: string) {
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

export async function interpretIR(llvmIR: string) {
  const lli = Deno.run({ cmd: ["lli-14"], stdin: "piped" });
  lli.stdin?.write(new TextEncoder().encode(llvmIR));
  lli.stdin.close();
  await lli.status();
}
