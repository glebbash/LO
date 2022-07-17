import { LLVM } from "./types.ts";
import { LLVM_SYMBOLS } from "./symbols.ts";

export type LLVM = typeof LLVM;

export function loadLLVM(path: string): LLVM {
  const lib = Deno.dlopen(path, LLVM_SYMBOLS);

  return { ...lib.symbols, close: () => lib.close() } as never;
}
