import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength, expectNumber } from "../../s-expr/assertions.ts";
import { getNumberValue } from "../../s-expr/transformers.ts";
import { ModuleContext } from "../compiler.ts";
import { buildValueInFunctionContext } from "./mod.ts";

export function buildI32(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): number {
  const [value] = expectArgsLength(
    1,
    args,
    command,
  );

  expectNumber(value);

  return ctx.module.i32.const(getNumberValue(value));
}

export function buildI32Store(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): number {
  const [addrExpr, valueExpr] = expectArgsLength(
    2,
    args,
    command,
  );

  const addr = buildValueInFunctionContext(addrExpr, ctx);
  const value = buildValueInFunctionContext(valueExpr, ctx);

  return ctx.module.i32.store(0, 4, addr, value);
}
