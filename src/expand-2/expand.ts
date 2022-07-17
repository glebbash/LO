import { isList, isSymbol } from "../s-expr/assertions.ts";
import { SExpr } from "../parser/parser.ts";
import { DEFAULT_MACROS, Macro } from "./macros/mod.ts";

export type ExpandContext = {
  path: string;
  level: number;
  macros: Record<string, Macro>;
};

export type ResultAndContext = {
  result: SExpr[];
  ctx: ExpandContext;
};

export function expandFile(filePath: string): SExpr[] {
  return expandExprs([["#include", `"${filePath}"`]]);
}

export function expandExprs(exprs: SExpr[]): SExpr[] {
  // TODO: figure out how to handle initializing ctx including data from macros
  const ctx: ExpandContext = {
    level: 0,
    path: ".",
    macros: { ...DEFAULT_MACROS },
  };
  return expandExpr({ ctx, expr: exprs }).result.flat();
}

export function expandExpr(
  exprInContext: {
    expr: SExpr;
    ctx: ExpandContext;
  },
): ResultAndContext {
  const { expr, ctx } = exprInContext;

  if (isList(expr)) {
    const [command, ...args] = expr;

    if (isSymbol(command)) {
      const macro = ctx.macros[command];

      if (macro?.type === "function") {
        return macro.expand(ctx, command, args);
      }
    }

    const nestedCtx: ExpandContext = { ...ctx, level: ctx.level + 1 };
    const res = expandList(nestedCtx, expr);

    return {
      result: [res.result.flat()],
      ctx: res.ctx,
    };
  }

  if (isSymbol(expr)) {
    const macro = ctx.macros[expr];

    if (macro) {
      if (macro.type !== "value") {
        throw new Error(`Macro ${expr} is not a value`);
      }

      return macro.expand(ctx, expr, []);
    }
  }

  return { result: [expr], ctx };
}

export function expandList(
  ctx: ExpandContext,
  exprs: SExpr[],
): ResultAndContext {
  return exprs.reduce(
    (acc, expr) => {
      const { ctx, result } = expandExpr({ ctx: acc.ctx, expr });
      return { ctx, result: [...acc.result, result] };
    },
    { result: [] as SExpr[], ctx },
  );
}
