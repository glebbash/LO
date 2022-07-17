import { SExpr } from "../../parser/parser.ts";
import {
  expectSymbol,
  isNumber,
  isString,
  isSymbol,
} from "../../s-expr/assertions.ts";
import { getNumberValue } from "../../s-expr/transformers.ts";
import { ModuleContext } from "../compiler.ts";
import { buildCall } from "./call.ts";
import { buildDrop } from "./drop.ts";
import { buildExport } from "./export.ts";
import { buildFunctionDef } from "./function-def.ts";
import { buildFunction } from "./function.ts";
import { buildImport } from "./import.ts";
import { buildMemory } from "./memory.ts";
import { buildI32, buildI32Store } from "./numbers.ts";

export function buildValueInModuleContext(expr: SExpr, ctx: ModuleContext) {
  if (isSymbol(expr) || isString(expr) || isNumber(expr)) {
    throw new Error(`Unexpected literal: ${expr}`);
  }

  const [command, ...args] = expr;
  expectSymbol(command);

  switch (command) {
    case "import":
      return buildImport(command, args, ctx);
    case "export":
      return buildExport(command, args, ctx);
    case "fn":
      return buildFunction(command, args, ctx);
    case "::":
      return buildFunctionDef(command, args, ctx);
    case "memory":
      return buildMemory(command, args, ctx);
    default:
      throw new Error("Unknown construct: " + command);
  }
}

export function buildValueInFunctionContext(
  expr: SExpr,
  ctx: ModuleContext,
): number {
  if (isNumber(expr)) {
    return ctx.module.i32.const(getNumberValue(expr));
  }

  if (isSymbol(expr)) {
    throw new Error("Not implemented");
  }

  const [command, ...args] = expr;
  expectSymbol(command);

  switch (command) {
    case "i32/const":
      return buildI32(command, args, ctx);
    case "i32/store":
      return buildI32Store(command, args, ctx);
    case "drop":
      return buildDrop(command, args, ctx);
    default:
      return buildCall("call", expr, ctx);
  }
}
