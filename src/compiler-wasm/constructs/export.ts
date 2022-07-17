import { SExpr } from "../../parser/parser.ts";
import {
  expectArgsLengthAtLeast,
  expectSymbol,
} from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";

export function buildExport(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
) {
  const [fnName, externalFnNameE] = expectArgsLengthAtLeast(
    1,
    args,
    command,
  );

  expectSymbol(fnName);

  let externalFnName = fnName;
  if (externalFnNameE !== undefined) {
    expectSymbol(externalFnNameE);
    externalFnName = externalFnNameE;
  }

  ctx.module.addFunctionExport(fnName, externalFnName);
}
