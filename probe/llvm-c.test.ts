import { loadLibLLVM } from "./llvm-c.ts";

const llvm = loadLibLLVM();

const ctx = llvm.LLVMContextCreate();
const builder = llvm.LLVMCreateBuilderInContext(ctx);

console.log({ ctx, builder });

llvm.LLVMDisposeBuilder(builder);
llvm.LLVMContextDispose(ctx);
