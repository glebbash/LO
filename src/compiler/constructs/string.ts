import { ModuleContext } from "../compiler.ts";
import { LLVMValue } from "../../llvm/llvm-c.ts";
import { getStringValue } from "../transformers.ts";

export function buildString(expr: string, ctx: ModuleContext): LLVMValue {
  const { llvm } = ctx;

  return llvm.buildGlobalStringPtr(ctx.builder, getStringValue(expr));
}
