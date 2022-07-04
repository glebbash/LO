import { SExpr } from "../../parser/parser.ts";
import {
  expectArgsLength,
  expectArgsLengthAtLeast,
  expectList,
  expectSymbol,
} from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { LLVMValue } from "../../llvm/llvm-c.ts";
import { defineType, getType } from "../types.ts";
import { defineValue } from "./variables.ts";
import { buildVoid } from "./void.ts";

export function buildStruct(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
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
      llvm.constInt(
        llvm.i32TypeInContext(ctx.context),
        index,
      ),
    );

    return getType(fieldTypeExpr, ctx);
  });

  const structType = llvm.structCreateNamed(ctx.context, structName);
  llvm.structSetBody(structType, fieldTypes);
  defineType(ctx, structName, structType);

  return buildVoid(ctx);
}

export function buildNew(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
  const { llvm } = ctx;

  const [structName] = expectArgsLength(1, args, command);
  expectSymbol(structName);

  const structType = ctx.types[structName];
  if (!structName) {
    throw new Error(`Struct ${structName} is not defined`);
  }

  return llvm.buildAlloca(ctx.builder, structType);
}
