import { dirname, resolve } from "https://deno.land/std@0.123.0/path/mod.ts";
import {
  expectArgsLength,
  expectString,
  expectSymbol,
  isList,
  isSymbol,
} from "../s-expr/assertions.ts";
import { getStringValue } from "../s-expr/transformers.ts";
import { parse, SExpr } from "../parser/parser.ts";

type ExpandContext = {
  path: string;
  aliases: Record<string, SExpr>;
  level: number;
};

type ExprInContext = {
  expr: SExpr;
  ctx: ExpandContext;
};

type ResultInContext = {
  result: SExpr[];
  ctx: ExpandContext;
};

export function expandFile(filePath: string): SExpr[] {
  return expandExprs([["include", `"${filePath}"`]]);
}

export function expandExprs(exprs: SExpr[]): SExpr[] {
  const ctx: ExpandContext = { aliases: {}, level: 0, path: "." };
  return expandExpr({ ctx, expr: exprs }).result.flat();
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

    if (command === "alias") {
      return processAlias(ctx, command, args);
    }

    if (command === "include") {
      return processInclude(ctx, command, args);
    }

    const nestedCtx: ExpandContext = { ...ctx, level: ctx.level + 1 };
    const res = expandList(nestedCtx, expr);

    return {
      result: [res.result.flat()],
      ctx: res.ctx,
    };
  }

  return { result: [expr], ctx };
}

function processAlias(
  ctx: ExpandContext,
  command: string,
  args: SExpr[],
): ResultInContext {
  const [name, value] = expectArgsLength(2, args, command);
  expectSymbol(name);

  return {
    result: [],
    ctx: { ...ctx, aliases: { ...ctx.aliases, [name]: value } },
  };
}

function processInclude(
  ctx: ExpandContext,
  command: string,
  args: SExpr[],
): ResultInContext {
  const [fileNameStr] = expectArgsLength(1, args, command);
  expectString(fileNameStr);

  if (ctx.level !== 1) {
    throw new Error("include can only be used at top level");
  }

  const filePath = getStringValue(fileNameStr);
  const fullFilePath = resolve(ctx.path, filePath);

  const fileContent = Deno.readTextFileSync(fullFilePath);
  const exprs = parse(fileContent);

  const path = dirname(fullFilePath);
  const fileCtx: ExpandContext = { ...ctx, path };
  const res = expandList(fileCtx, exprs);

  return {
    result: res.result.flat(),
    ctx: { ...ctx, aliases: res.ctx.aliases },
  };
}

function expandList(ctx: ExpandContext, exprs: SExpr[]): ResultInContext {
  return exprs.reduce(
    (acc, expr) => {
      const { ctx, result } = expandExpr({ ctx: acc.ctx, expr });
      return { ctx, result: [...acc.result, result] };
    },
    { result: [] as SExpr[], ctx },
  );
}
