import { LLVM } from "../../llvm-c-14/llvm-c/mod.ts";
import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength } from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { getType } from "../types.ts";

export function buildNullPtr(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const [typeExpr] = expectArgsLength(1, args, command);

  const type = getType(typeExpr, ctx);
  return llvm.ConstPointerNull(type);
}
