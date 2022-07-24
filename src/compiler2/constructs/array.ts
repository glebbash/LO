import { LLVM } from "../../llvm-c-14/llvm-c/mod.ts";
import { SExpr } from "../../parser/parser.ts";
import { expectArgsLengthAtLeast } from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { DEFAULT_ADDR_SPACE } from "../types.ts";
import { BOOL_FALSE, buildArrayPtr, toCString } from "../utils.ts";
import { buildValue } from "./mod.ts";

export function buildArray(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const valueExprs = expectArgsLengthAtLeast(1, args, command);

  const values = valueExprs.map((expr) => buildValue(expr, ctx));
  const [firstValue, ...otherValues] = values;

  const elementType = llvm.TypeOf(firstValue);
  const arrayType = llvm.ArrayType(elementType, valueExprs.length);
  const array = llvm.BuildAlloca(ctx.builder, arrayType, toCString(""));

  const zero = llvm.ConstInt(
    llvm.Int32TypeInContext(ctx.context),
    0n,
    BOOL_FALSE,
  );

  const indices = [zero, zero];
  const firstElementPointer = llvm.BuildGEP(
    ctx.builder,
    array,
    buildArrayPtr(indices),
    indices.length,
    toCString(""),
  );
  llvm.BuildStore(ctx.builder, firstValue, firstElementPointer);

  let elementPointer = firstElementPointer;
  for (let index = 0; index < otherValues.length; index++) {
    const value = otherValues[index];
    const subIndices = [
      llvm.ConstInt(llvm.Int32TypeInContext(ctx.context), 1n, BOOL_FALSE),
    ];
    elementPointer = llvm.BuildGEP(
      ctx.builder,
      elementPointer,
      buildArrayPtr(subIndices),
      subIndices.length,
      toCString(""),
    );
    llvm.BuildStore(ctx.builder, value, elementPointer);
  }

  return llvm.BuildBitCast(
    ctx.builder,
    array,
    llvm.PointerType(llvm.ArrayType(elementType, 0), DEFAULT_ADDR_SPACE),
    toCString(""),
  );
}
