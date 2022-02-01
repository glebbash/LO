import {
  expectArgsLength,
  expectSymbol,
  isList,
  isSymbol,
} from "../compiler/assertions.ts";
import { SExpr } from "../parser/parser.ts";

type ExpandContext = { aliases: Record<string, SExpr>; level: number };

type ExprInContext = {
  expr: SExpr;
  ctx: ExpandContext;
};

type ResultInContext = {
  result: SExpr[];
  ctx: ExpandContext;
};

export function expand(exprs: SExpr[]): SExpr[] {
  return expandExpr({ expr: exprs, ctx: { aliases: {}, level: 0 } })
    .result
    .flat();
}

function expandExpr(
  exprInContext: ExprInContext,
): ResultInContext {
  const { expr, ctx } = exprInContext;
  if (isSymbol(expr)) {
    if (!ctx.aliases[expr]) {
      return { result: [expr], ctx };
    }

    return expandExpr({ expr: ctx.aliases[expr], ctx });
  }

  if (isList(expr)) {
    const [command, ...args] = expr;

    if (ctx.level === 1 && command === "alias") {
      const [name, value] = expectArgsLength(2, args, command);
      expectSymbol(name);

      return {
        result: [],
        ctx: { ...ctx, aliases: { ...ctx.aliases, [name]: value } },
      };
    }

    const res = expr.reduce(
      (acc, expr) => {
        const { ctx, result } = expandExpr({ ctx: acc.ctx, expr });
        return { ctx, result: [...acc.result, result] };
      },
      { result: [] as SExpr[], ctx: { ...ctx, level: ctx.level + 1 } },
    );

    return {
      result: [res.result.flat()],
      ctx: res.ctx,
    };
  }

  return { result: [expr], ctx };
}
