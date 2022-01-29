import { panic } from 'panic-fn';

import { SExpr } from '../parser/parser';

const STRING_START = ['"'];
const NUMBER_START = '0123456789'.split('');

const checkSymbol = (expr: SExpr) =>
  typeof expr !== 'string'
    ? 'Symbol expected, found: list'
    : STRING_START.includes(expr[0])
    ? 'Symbol expected, found: string'
    : NUMBER_START.includes(expr[0])
    ? 'Symbol expected, found: number'
    : true;

const checkString = (expr: SExpr) =>
  typeof expr !== 'string'
    ? 'String expected, found: list'
    : NUMBER_START.includes(expr[0])
    ? 'String expected, found: number'
    : !STRING_START.includes(expr[0])
    ? 'String expected, found: symbol'
    : true;

const checkList = (expr: SExpr) =>
  typeof expr === 'string' ? 'List expected, found: atom' : true;

export function isSymbol(expr: SExpr): expr is string {
  return checkSymbol(expr) === true;
}

export function expectSymbol(expr: SExpr): asserts expr is string {
  assert(checkSymbol, expr);
}

export function isString(expr: SExpr): expr is string {
  return checkString(expr) === true;
}

export function expectString(expr: SExpr): asserts expr is string {
  assert(checkString, expr);
}

export function expectNumber(expr: SExpr): asserts expr is string {
  if (typeof expr !== 'string') {
    panic('Number expected, found: list');
  }

  if (STRING_START.includes(expr[0])) {
    panic('Number expected, found: string');
  }

  if (!NUMBER_START.includes(expr[0])) {
    panic('Number expected, found: symbol');
  }
}

export function isList(expr: SExpr): expr is SExpr[] {
  return checkList(expr) === true;
}

export function expectList(expr: SExpr): asserts expr is SExpr[] {
  assert(checkList, expr);
}

export function expectArgsLength(
  argsLength: number,
  args: SExpr[],
  command: string,
): SExpr[] {
  if (args.length !== argsLength) {
    panic(
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
    panic(
      `Command ${command} expects at least ${argsLength} argument(s) but ${args.length} was given`,
    );
  }

  return args;
}

function assert(check: (expr: SExpr) => true | string, expr: SExpr) {
  const err = check(expr);
  err !== true && panic(`${err}, checking ${expr}`);
}
