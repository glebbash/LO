import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength } from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { LLVMValue } from "../../llvm/llvm-c.ts";
import { getType } from "../types.ts";

export function buildNullPtr(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
  const { llvm } = ctx;

  const [typeExpr] = expectArgsLength(1, args, command);

  const type = getType(typeExpr, ctx);
  return llvm.constPointerNull(type);
}
