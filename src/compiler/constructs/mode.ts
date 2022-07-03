import { SExpr } from "../../parser/parser.ts";
import {
  expectList,
  expectSymbol,
  isList,
  isNumber,
  isString,
  isSymbol,
} from "../assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { LLVMValue } from "../../llvm/llvm-c.ts";
import { buildAdd } from "./arithmetic.ts";
import { buildArray } from "./array.ts";
import { buildCast } from "./cast.ts";
import { buildLess } from "./comparison.ts";
import { buildExternalFn, buildFn, buildFunctionCall } from "./function.ts";
import { buildIf } from "./if.ts";
import { buildNullPtr } from "./null-ptr.ts";
import { buildI32, buildI64, buildI8 } from "./numbers.ts";
import { buildString } from "./string.ts";
import { buildNew, buildStruct } from "./struct.ts";
import { buildTargetTriple } from "./target-triple.ts";
import {
  buildConstantAccess,
  buildDef,
  buildGet,
  buildLet,
  buildSet,
} from "./variables.ts";

export function buildValue(expr: SExpr, ctx: ModuleContext): LLVMValue {
  if (isSymbol(expr)) {
    return buildConstantAccess(expr, ctx);
  }

  if (isString(expr)) {
    return buildString(expr, ctx);
  }

  return buildConstruct(expr, ctx);
}

export function buildValueInFunctionContext(
  expr: SExpr,
  ctx: ModuleContext,
): LLVMValue {
  if (!isList(expr)) {
    return buildValue(expr, ctx);
  }

  const [command, ...args] = expr;
  expectSymbol(command);

  switch (command) {
    case "def":
      return buildDef(command, args, ctx);
    case "let":
      return buildLet(command, args, ctx);
    default:
      return buildValue(expr, ctx);
  }
}

export function buildValueInModuleContext(
  expr: SExpr,
  ctx: ModuleContext,
): LLVMValue {
  const [command, ...args] = expr;
  expectSymbol(command);

  switch (command) {
    case "llvm/target-triple":
      return buildTargetTriple(command, args, ctx);
    case "external-fn":
      return buildExternalFn(command, args, ctx);
    case "fn":
      return buildFn(command, args, ctx);
    case "struct":
      return buildStruct(command, args, ctx);
    default:
      throw new Error("Only functions and externs are allowed at top level");
  }
}

export function buildConstruct(expr: SExpr, ctx: ModuleContext): LLVMValue {
  if (isNumber(expr)) {
    return buildI32("i32", [expr], ctx);
  }

  expectList(expr);

  const [command, ...args] = expr;
  expectSymbol(command);

  switch (command) {
    case "i8":
      return buildI8(command, args, ctx);
    case "i32":
      return buildI32(command, args, ctx);
    case "i64":
      return buildI64(command, args, ctx);
    case "+":
      return buildAdd(command, args, ctx);
    case "<":
      return buildLess(command, args, ctx);
    case "if":
      return buildIf(command, args, ctx);
    case "nullptr":
      return buildNullPtr(command, args, ctx);
    case "array":
      return buildArray(command, args, ctx);
    case "new":
      return buildNew(command, args, ctx);
    case "get":
      return buildGet(command, args, ctx);
    case "set":
      return buildSet(command, args, ctx);
    case "cast":
      return buildCast(command, args, ctx);
    default:
      return buildFunctionCall(command, args, ctx);
  }
}
