import { SExpr } from "../../parser/parser.ts";
import {
  expectArgsLengthAtLeast,
  expectSymbol,
} from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { buildValueInFunctionContext } from "./mod.ts";

export function buildCall(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): number {
  const [fnName, ...argsList] = expectArgsLengthAtLeast(
    1,
    args,
    command,
  );

  expectSymbol(fnName);

  if (!ctx.functionDefs[fnName]) {
    throw new Error(`Type for function '${fnName}' is not defined`);
  }

  const fnType = ctx.functionDefs[fnName];

  return ctx.module.call(
    fnName,
    argsList.map((arg) => buildValueInFunctionContext(arg, ctx)),
    fnType.result,
  );
}
