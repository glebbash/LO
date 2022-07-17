import { loadLLVM } from "./llvm-c/mod.ts";

const llvm = loadLLVM("./input/libLLVM-15git.so");

const ctx = llvm.ContextCreate();

console.log(ctx);

llvm.ContextDispose(ctx);
