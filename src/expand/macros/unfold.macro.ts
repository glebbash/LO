import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength } from "../../s-expr/assertions.ts";
import { expandExpr } from "../expand.ts";
import { buildMacroFunction } from "./mod.ts";

export const unfoldMacro = buildMacroFunction((ctx, command, args) => {
  const [listToUnfold] = expectArgsLength(1, args, command);

  const expanded = expandExpr({ expr: listToUnfold, ctx });

  return {
    result: expanded.result[0] as SExpr[],
    ctx: expanded.ctx,
  };
});
