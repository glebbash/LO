import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength } from "../assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { LLVMValue } from "../llvm-c.ts";
import { buildValue } from "./mod.ts";

export function buildAdd(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
  const { llvm } = ctx;

  const [lhs, rhs] = expectArgsLength(2, args, command);

  return llvm.buildAdd(ctx.builder, buildValue(lhs, ctx), buildValue(rhs, ctx));
}
