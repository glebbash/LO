import { LLVM } from "../../llvm-c-14/llvm-c/mod.ts";
import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength } from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { toCString } from "../utils.ts";
import { buildValue } from "./mod.ts";

export function buildAdd(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const [lhs, rhs] = expectArgsLength(2, args, command);

  return llvm.BuildAdd(
    ctx.builder,
    buildValue(lhs, ctx),
    buildValue(rhs, ctx),
    toCString(command),
  );
}
