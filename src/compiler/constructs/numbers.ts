import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength, expectNumber } from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { getNumberValue } from "../../s-expr/transformers.ts";
import { BOOL_FALSE } from "../utils.ts";
import { LLVM } from "../../llvm-c-14/llvm-c/mod.ts";

export function buildI8(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const [value] = expectArgsLength(1, args, command);
  expectNumber(value);

  const i8Value = getNumberValue(value);

  return llvm.ConstInt(
    llvm.Int8TypeInContext(ctx.context),
    BigInt(i8Value),
    BOOL_FALSE,
  );
}

export function buildI32(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const [value] = expectArgsLength(1, args, command);
  expectNumber(value);

  const i32Value = getNumberValue(value);

  return llvm.ConstInt(
    llvm.Int32TypeInContext(ctx.context),
    BigInt(i32Value),
    BOOL_FALSE,
  );
}

export function buildI64(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const [value] = expectArgsLength(1, args, command);
  expectNumber(value);

  const i64Value = getNumberValue(value);

  return llvm.ConstInt(
    llvm.Int64TypeInContext(ctx.context),
    BigInt(i64Value),
    BOOL_FALSE,
  );
}
