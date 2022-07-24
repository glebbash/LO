import { LLVM } from "./types.ts";
import { LLVM_SYMBOLS } from "./symbols.ts";

export function loadLLVM(path: string): typeof LLVM {
  const lib = Deno.dlopen(path, LLVM_SYMBOLS);

  return { ...lib.symbols, close: () => lib.close() } as never;
}

export { LLVM } from "./types.ts";
