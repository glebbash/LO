import { LLVM } from "../../llvm-c-14/llvm-c/mod.ts";
import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength } from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { getType } from "../types.ts";
import { toCString } from "../utils.ts";
import { buildValue } from "./mod.ts";

export function buildCast(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const [valueExpr, typeExpr] = expectArgsLength(2, args, command);

  return llvm.BuildBitCast(
    ctx.builder,
    buildValue(valueExpr, ctx),
    getType(typeExpr, ctx),
    toCString(""),
  );
}
