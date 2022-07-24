import { ModuleContext } from "../compiler.ts";
import { getStringValue } from "../../s-expr/transformers.ts";
import { nullPtr, toCString } from "../utils.ts";
import { LLVM } from "../../../ffigen/llvm-c/mod.ts";

export function buildString(expr: string, ctx: ModuleContext): LLVM.ValueRef {
  const { llvm } = ctx;

  return llvm.BuildGlobalStringPtr(
    ctx.builder,
    toCString(getStringValue(expr)),
    nullPtr(),
  );
}
