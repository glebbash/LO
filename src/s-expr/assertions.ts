import { SExpr } from "../parser/parser.ts";

const STRING_START = ['"'];
const NUMBER_START = "0123456789".split("");

type SExprType = "string" | "number" | "symbol" | "list";

export function isString(expr: SExpr): expr is string {
  return getType(expr) === "string";
}

export function isNumber(expr: SExpr): expr is string {
  return getType(expr) === "number";
}

export function isSymbol(expr: SExpr): expr is string {
  return getType(expr) === "symbol";
}

export function isList(expr: SExpr): expr is SExpr[] {
  return getType(expr) === "list";
}

export function expectString(expr: SExpr): asserts expr is string {
  assert("string", expr);
}

export function expectNumber(expr: SExpr): asserts expr is string {
  assert("number", expr);
}

export function expectSymbol(expr: SExpr): asserts expr is string {
  assert("symbol", expr);
}

export function expectList(expr: SExpr): asserts expr is SExpr[] {
  assert("list", expr);
}

export function expectArgsLength(
  argsLength: number,
  args: SExpr[],
  command: string,
): SExpr[] {
  if (args.length !== argsLength) {
    throw new Error(
      `Command ${command} expects ${argsLength} argument(s) but ${args.length} was given`,
    );
  }

  return args;
}

export function expectArgsLengthAtLeast(
  argsLength: number,
  args: SExpr[],
  command: string,
): SExpr[] {
  if (args.length < argsLength) {
    throw new Error(
      `Command ${command} expects at least ${argsLength} argument(s) but ${args.length} was given`,
    );
  }

  return args;
}

function assert(expectedType: SExprType, expr: SExpr) {
  const exprType = getType(expr);

  if (exprType !== expectedType) {
    throw new Error(
      `${expectedType} expected but ${exprType} was found, checking ${expr}`,
    );
  }
}

function getType(expr: SExpr): SExprType {
  if (typeof expr !== "string") {
    return "list";
  }

  if (STRING_START.includes(expr[0])) {
    return "string";
  }

  if (NUMBER_START.includes(expr[0])) {
    return "number";
  }

  return "symbol";
}
