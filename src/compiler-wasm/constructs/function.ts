import { SExpr } from "../../parser/parser.ts";
import {
  expectArgsLengthAtLeast,
  expectList,
  expectSymbol,
} from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { buildValueInFunctionContext } from "./mod.ts";

export function buildFunction(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): number {
  const [fnName, argsList, ...body] = expectArgsLengthAtLeast(
    2,
    args,
    command,
  );

  expectSymbol(fnName);
  expectList(argsList);

  if (!ctx.functionDefs[fnName]) {
    throw new Error(`Type for function '${fnName}' is not defined`);
  }

  const fnType = ctx.functionDefs[fnName];

  return ctx.module.addFunction(
    fnName,
    fnType.params,
    fnType.result,
    // TODO: Add support for local variables
    [],
    ctx.module.block(
      null,
      body.map((expr) => buildValueInFunctionContext(expr, ctx)),
    ),
  );
}
