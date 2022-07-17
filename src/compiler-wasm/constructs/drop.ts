import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength } from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { buildValueInFunctionContext } from "./mod.ts";

export function buildDrop(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): number {
  const [valueExpr] = expectArgsLength(
    1,
    args,
    command,
  );

  const value = buildValueInFunctionContext(valueExpr, ctx);

  return ctx.module.drop(value);
}
