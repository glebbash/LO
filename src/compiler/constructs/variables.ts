import { SExpr } from "../../parser/parser.ts";
import {
  expectArgsLength,
  expectArgsLengthAtLeast,
  expectSymbol,
} from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { LLVMValue } from "../../llvm/llvm-c.ts";
import { getType } from "../types.ts";
import { buildValue } from "./mod.ts";

export function buildDef(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
  const { llvm } = ctx;

  const [name, type] = expectArgsLength(2, args, command);
  expectSymbol(name);

  const place = llvm.buildAlloca(ctx.builder, getType(type, ctx));
  defineValue(ctx, name, place);

  return place;
}

export function buildLet(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
  const { llvm } = ctx;

  const [name, expr] = expectArgsLength(2, args, command);
  expectSymbol(name);

  const value = buildValue(expr, ctx);

  const place = llvm.buildAlloca(ctx.builder, llvm.typeOf(value));
  defineValue(ctx, name, value);

  return llvm.buildStore(ctx.builder, value, place);
}

export function buildGet(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
  const { llvm } = ctx;

  const [sourcePtrExpr, ...indices] = expectArgsLengthAtLeast(1, args, command);

  const sourcePointer = buildValue(sourcePtrExpr, ctx);
  const indicesValues = indices.map((index) => buildValue(index, ctx));

  const elementPointer = llvm.buildGEP(
    ctx.builder,
    sourcePointer,
    indicesValues,
  );

  return llvm.buildLoad(ctx.builder, elementPointer);
}

export function buildSet(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
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

  const elementPointer = llvm.buildGEP(
    ctx.builder,
    sourcePointer,
    indicesValues,
  );

  return llvm.buildStore(ctx.builder, value, elementPointer);
}

export function buildConstantAccess(
  name: string,
  ctx: ModuleContext,
): LLVMValue {
  const constant = ctx.values[name];

  if (!constant) {
    throw new Error(`Constant is not defined ${name}`);
  }

  return constant;
}

export function defineValue(
  ctx: ModuleContext,
  name: string,
  value: LLVMValue,
): void {
  if (ctx.values[name]) {
    throw new Error(`Constant ${name} is already defined`);
  }

  ctx.values[name] = value;
}
