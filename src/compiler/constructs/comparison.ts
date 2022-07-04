import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength } from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { LLVMIntPredicate, LLVMValue } from "../../llvm/llvm-c.ts";
import { buildValue } from "./mod.ts";

export function buildLess(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
  const { llvm } = ctx;

  const [lhs, rhs] = expectArgsLength(2, args, command);

  const res = llvm.buildICmp(
    ctx.builder,
    LLVMIntPredicate.LLVMIntSLT,
    buildValue(lhs, ctx),
    buildValue(rhs, ctx),
  );

  return res;
}
