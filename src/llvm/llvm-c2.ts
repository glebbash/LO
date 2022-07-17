import * as functions from "./llvm-c/functions.ts";
import { LLVM_SYMBOLS } from "./llvm-symbols.ts";

type FunctionTypes = typeof functions;
type NonInlinedFunctions = {
  [key in keyof FunctionTypes]: FunctionTypes[key] extends { type: "inline" }
    ? never
    : (key extends (typeof LLVM_SYMBOLS)[number] ? FunctionTypes[key] : never);
};

const nonInlinedFunctions: NonInlinedFunctions = Object.fromEntries(
  Object.entries(functions).filter(([name, func]) =>
    !("type" in func) && LLVM_SYMBOLS.includes(name as never)
  ),
) as never;

export function loadLibLLVM(path = "./src/llvm/libLLVM-15git.so") {
  const lib = Deno.dlopen(path, nonInlinedFunctions);

  return { ...lib.symbols, close: () => lib.close() };
}
