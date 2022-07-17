import { SafeDynamicLibrary } from "./ffigen-output/utils.ts";
import * as functions from "./ffigen-output/functions.ts";

export function loadLibLLVM(path = "./src/llvm/libLLVM-15git.so") {
  const lib = Deno.dlopen(path, functions) as SafeDynamicLibrary<
    typeof functions
  >;

  return { ...lib.symbols, close: () => lib.close() };
}
