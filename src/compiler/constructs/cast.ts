import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength } from "../assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { LLVMValue } from "../../llvm/llvm-c.ts";
import { getType } from "../types.ts";
import { buildValue } from "./mod.ts";

export function buildCast(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
  const { llvm } = ctx;

  const [valueExpr, typeExpr] = expectArgsLength(2, args, command);

  return llvm.buildBitCast(
    ctx.builder,
    buildValue(valueExpr, ctx),
    getType(typeExpr, ctx),
  );
}
