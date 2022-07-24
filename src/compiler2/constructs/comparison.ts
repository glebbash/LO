import { LLVM } from "../../llvm-c-14/llvm-c/mod.ts";
import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength } from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { toCString } from "../utils.ts";
import { buildValue } from "./mod.ts";

export function buildLess(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const [lhs, rhs] = expectArgsLength(2, args, command);

  return llvm.BuildICmp(
    ctx.builder,
    LLVM.IntPredicate.LLVMIntSLT,
    buildValue(lhs, ctx),
    buildValue(rhs, ctx),
    toCString(""),
  );
}
