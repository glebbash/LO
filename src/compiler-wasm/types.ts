import { SExpr } from "../parser/parser.ts";
import { isNumber, isString, isSymbol } from "../s-expr/assertions.ts";
import { ModuleContext } from "./compiler.ts";

export function buildType(expr: SExpr, ctx: ModuleContext): number {
  if (isSymbol(expr)) {
    switch (expr) {
      case "i32":
        return ctx.wasm.i32;
      case "i64":
        return ctx.wasm.i64;
      case "void":
        return ctx.wasm.none;
      default:
        throw new Error("Unknown type: " + expr);
    }
  }

  if (isString(expr) || isNumber(expr)) {
    throw new Error("Invalid type: " + expr);
  }

  throw new Error("Not implemented: complex types");
}
