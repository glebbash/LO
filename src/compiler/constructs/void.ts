import { ModuleContext } from "../compiler.ts";
import { LLVMValue } from "../llvm-c.ts";

export function buildVoid(ctx: ModuleContext): LLVMValue {
  const { llvm } = ctx;

  return llvm.getUndef(llvm.voidTypeInContext(ctx.context));
}
