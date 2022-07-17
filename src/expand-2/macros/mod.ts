import { SExpr } from "../../parser/parser.ts";
import { ExpandContext, expandExpr, ResultAndContext } from "../expand.ts";
import { fnMacro } from "./fn.macro.ts";
import { includeMacro } from "./include.macro.ts";
import { unfoldMacro } from "./unfold.macro.ts";
import { valueMacro } from "./value.macro.ts";

export const DEFAULT_MACROS: Record<string, Macro> = {
  "#include": includeMacro,
  "#def": valueMacro,
  "#fn": fnMacro,
  "#unfold": unfoldMacro,
};

export type Macro<C extends ExpandContext = ExpandContext> = {
  type: "value" | "function";
  expand: (
    ctx: C,
    command: string,
    args: SExpr[],
  ) => ResultAndContext;
};

export function buildMacroFunction(expand: Macro["expand"]): Macro {
  return {
    type: "function",
    expand,
  };
}

export function buildMacroValue(expr: SExpr): Macro {
  return {
    type: "value",
    expand: (ctx) => expandExpr({ expr, ctx }),
  };
}
