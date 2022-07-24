import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength, expectNumber } from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { getNumberValue } from "../../s-expr/transformers.ts";
import { LLVM } from "../../../ffigen/llvm-c/mod.ts";
import { BOOL_FALSE, value as valueOf } from "../utils.ts";

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
    // TODO(ffigen): fix this
    valueOf(i8Value) as never,
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
    // TODO(ffigen): fix this
    valueOf(i32Value) as never,
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
    // TODO(ffigen): fix this
    valueOf(i64Value) as never,
    BOOL_FALSE,
  );
}
