import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength, expectNumber } from "../assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { LLVMValue } from "../llvm-c.ts";
import { getNumberValue } from "../transformers.ts";

export function buildI8(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
  const { llvm } = ctx;

  const [value] = expectArgsLength(1, args, command);
  expectNumber(value);

  const i8Value = getNumberValue(value);

  return llvm.constInt(llvm.i8TypeInContext(ctx.context), i8Value);
}

export function buildI32(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
  const { llvm } = ctx;

  const [value] = expectArgsLength(1, args, command);
  expectNumber(value);

  const i32Value = getNumberValue(value);

  return llvm.constInt(llvm.i32TypeInContext(ctx.context), i32Value);
}

export function buildI64(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
  const { llvm } = ctx;

  const [value] = expectArgsLength(1, args, command);
  expectNumber(value);

  const i64Value = getNumberValue(value);

  return llvm.constInt(llvm.i64TypeInContext(ctx.context), i64Value);
}
