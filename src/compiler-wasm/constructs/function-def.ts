import { SExpr } from "../../parser/parser.ts";
import {
  expectArgsLength,
  expectList,
  expectSymbol,
} from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { buildType } from "../types.ts";

export function buildFunctionDef(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
) {
  const [fnName, paramTypes, resultType] = expectArgsLength(
    3,
    args,
    command,
  );

  expectSymbol(fnName);
  expectList(paramTypes);

  if (ctx.functionDefs[fnName]) {
    throw new Error(`Function ${fnName} already defined`);
  }

  const paramsWasmType = ctx.wasm.createType(
    paramTypes.map((type) => buildType(type, ctx)),
  );
  const resultWasmType = buildType(resultType, ctx);

  ctx.functionDefs[fnName] = {
    name: fnName,
    params: paramsWasmType,
    result: resultWasmType,
  };
}
