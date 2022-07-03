import { SExpr } from "../../parser/parser.ts";
import { expectArgsLengthAtLeast } from "../assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { LLVMValue } from "../../llvm/llvm-c.ts";
import { buildValue } from "./mod.ts";

export function buildArray(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
  const { llvm } = ctx;

  const valueExprs = expectArgsLengthAtLeast(1, args, command);

  const values = valueExprs.map((expr) => buildValue(expr, ctx));
  const [firstValue, ...otherValues] = values;

  const elementType = llvm.typeOf(firstValue);
  const arrayType = llvm.arrayType(elementType, valueExprs.length);
  const array = llvm.buildAlloca(ctx.builder, arrayType);

  const zero = llvm.constInt(llvm.i32TypeInContext(ctx.context), 0);
  const firstElementPointer = llvm.buildGEP(ctx.builder, array, [zero, zero]);
  llvm.buildStore(ctx.builder, firstValue, firstElementPointer);

  let elementPointer = firstElementPointer;
  for (let index = 0; index < otherValues.length; index++) {
    const value = otherValues[index];
    elementPointer = llvm.buildGEP(ctx.builder, elementPointer, [
      llvm.constInt(llvm.i32TypeInContext(ctx.context), 1),
    ]);
    llvm.buildStore(ctx.builder, value, elementPointer);
  }

  return llvm.buildBitCast(
    ctx.builder,
    array,
    llvm.pointerType(llvm.arrayType(elementType, 0)),
  );
}
