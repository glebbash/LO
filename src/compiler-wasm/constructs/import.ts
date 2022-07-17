import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength, expectSymbol } from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";

export function buildImport(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
) {
  const [fnName, fromKeyword, moduleName] = expectArgsLength(
    3,
    args,
    command,
  );

  expectSymbol(fnName);
  expectSymbol(fromKeyword);
  expectSymbol(moduleName);

  if (fromKeyword !== ":from") {
    throw new Error(`Expected :from keyword, got ${fromKeyword}`);
  }

  if (!ctx.functionDefs[fnName]) {
    throw new Error(`Type for function '${fnName}' is not defined`);
  }

  const fnType = ctx.functionDefs[fnName];

  ctx.module.addFunctionImport(
    fnName,
    moduleName,
    fnName,
    fnType.params,
    fnType.result,
  );
}
