import { LLVM } from "../../llvm-c-14/llvm-c/mod.ts";
import { SExpr } from "../../parser/parser.ts";
import {
  expectArgsLength,
  expectArgsLengthAtLeast,
  expectList,
  expectSymbol,
} from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { defineType, getType } from "../types.ts";
import { BOOL_FALSE, BOOL_TRUE, buildArrayPtr, toCString } from "../utils.ts";
import { defineValue } from "./variables.ts";
import { buildVoid } from "./void.ts";

export function buildStruct(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const [structName, ...fields] = expectArgsLengthAtLeast(2, args, command);
  expectSymbol(structName);

  if (ctx.types[structName]) {
    throw new Error(`Redefinition of type ${structName}`);
  }

  const fieldTypes = fields.map((p, index) => {
    expectList(p);
    if (p.length !== 2) {
      throw new Error("Struct field definitions must have name and type only");
    }

    const [fieldName, fieldTypeExpr] = p;
    expectSymbol(fieldName);

    defineValue(
      ctx,
      `${structName}/${fieldName}`,
      llvm.ConstInt(
        llvm.Int32TypeInContext(ctx.context),
        BigInt(index),
        BOOL_FALSE,
      ),
    );

    return getType(fieldTypeExpr, ctx);
  });

  const structType = llvm.StructCreateNamed(ctx.context, toCString(structName));
  llvm.StructSetBody(
    structType,
    buildArrayPtr(fieldTypes),
    fieldTypes.length,
    BOOL_TRUE,
  );
  defineType(ctx, structName, structType);

  return buildVoid(ctx);
}

export function buildNew(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const [structName] = expectArgsLength(1, args, command);
  expectSymbol(structName);

  const structType = ctx.types[structName];
  if (!structName) {
    throw new Error(`Struct ${structName} is not defined`);
  }

  return llvm.BuildAlloca(ctx.builder, structType, toCString(""));
}
