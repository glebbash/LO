import { LLVM } from "../../llvm-c-14/llvm-c/mod.ts";
import { SExpr } from "../../parser/parser.ts";
import {
  expectArgsLength,
  expectArgsLengthAtLeast,
  expectSymbol,
} from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { getType } from "../types.ts";
import { buildArrayPtr, toCString } from "../utils.ts";
import { buildValue } from "./mod.ts";

export function buildDef(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const [name, type] = expectArgsLength(2, args, command);
  expectSymbol(name);

  const place = llvm.BuildAlloca(
    ctx.builder,
    getType(type, ctx),
    toCString(""),
  );
  defineValue(ctx, name, place);

  return place;
}

export function buildLet(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const [name, expr] = expectArgsLength(2, args, command);
  expectSymbol(name);

  const value = buildValue(expr, ctx);

  const place = llvm.BuildAlloca(
    ctx.builder,
    llvm.TypeOf(value),
    toCString(""),
  );
  defineValue(ctx, name, value);

  return llvm.BuildStore(ctx.builder, value, place);
}

export function buildGet(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const [sourcePtrExpr, ...indices] = expectArgsLengthAtLeast(1, args, command);

  const sourcePointer = buildValue(sourcePtrExpr, ctx);
  const indicesValues = indices.map((index) => buildValue(index, ctx));

  const elementPointer = llvm.BuildGEP(
    ctx.builder,
    sourcePointer,
    buildArrayPtr(indicesValues),
    indicesValues.length,
    toCString(""),
  );

  return llvm.BuildLoad(ctx.builder, elementPointer, toCString(""));
}

export function buildSet(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const [sourcePtrExpr, ...indicesAndValue] = expectArgsLengthAtLeast(
    2,
    args,
    command,
  );

  const sourcePointer = buildValue(sourcePtrExpr, ctx);
  const value = buildValue(indicesAndValue[indicesAndValue.length - 1], ctx);
  const indicesValues = indicesAndValue
    .slice(0, -1)
    .map((index) => buildValue(index, ctx));

  const elementPointer = llvm.BuildGEP(
    ctx.builder,
    sourcePointer,
    buildArrayPtr(indicesValues),
    indicesValues.length,
    toCString(""),
  );

  return llvm.BuildStore(ctx.builder, value, elementPointer);
}

export function buildConstantAccess(
  name: string,
  ctx: ModuleContext,
): LLVM.ValueRef {
  const constant = ctx.values[name];

  if (!constant) {
    throw new Error(`Constant is not defined ${name}`);
  }

  return constant;
}

export function defineValue(
  ctx: ModuleContext,
  name: string,
  value: LLVM.ValueRef,
): void {
  if (ctx.values[name]) {
    throw new Error(`Constant ${name} is already defined`);
  }

  ctx.values[name] = value;
}
