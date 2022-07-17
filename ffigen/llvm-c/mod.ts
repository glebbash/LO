import { SafeDynamicLibrary } from "./safe-ffi.ts";
import * as functions from "./functions.ts";

export type LibLLVM = ReturnType<typeof loadLibLLVM>;

export function loadLibLLVM(path: string) {
  const lib = Deno.dlopen(path, functions) as SafeDynamicLibrary<
    typeof functions
  >;

  return { ...lib.symbols, close: () => lib.close() };
}