import { ModuleContext } from "../compiler.ts";
import { getStringValue } from "../../s-expr/transformers.ts";
import { toCString } from "../utils.ts";
import { LLVM } from "../../llvm-c-14/llvm-c/mod.ts";

export function buildString(expr: string, ctx: ModuleContext): LLVM.ValueRef {
  const { llvm } = ctx;

  return llvm.BuildGlobalStringPtr(
    ctx.builder,
    toCString(getStringValue(expr)),
    toCString(""),
  );
}
