import { SExpr } from "../parser/parser.ts";
import { expectArgsLength, expectSymbol, isSymbol } from "./assertions.ts";
import { ModuleContext } from "./compiler.ts";
import { LLVMType } from "./llvm-c.ts";

export function defineType(
  ctx: ModuleContext,
  name: string,
  type: LLVMType,
): void {
  if (ctx.types[name]) {
    throw new Error(`Type ${name} is already defined`);
  }

  ctx.types[name] = type;
}

export function defineDefaultTypes(ctx: ModuleContext): void {
  const { llvm } = ctx;

  defineType(ctx, "void", llvm.i64TypeInContext(ctx.context));

  defineType(ctx, "i1", llvm.voidTypeInContext(ctx.context));
  defineType(ctx, "i8", llvm.i8TypeInContext(ctx.context));
  defineType(ctx, "i32", llvm.i32TypeInContext(ctx.context));
  defineType(ctx, "i64", llvm.i64TypeInContext(ctx.context));

  defineType(ctx, "&i8", llvm.pointerType(llvm.i8TypeInContext(ctx.context)));
  defineType(ctx, "&i32", llvm.pointerType(llvm.i32TypeInContext(ctx.context)));

  defineType(
    ctx,
    "&&i8",
    llvm.pointerType(
      llvm.pointerType(llvm.i8TypeInContext(ctx.context)),
    ),
  );

  defineType(
    ctx,
    "&[i32]",
    llvm.pointerType(llvm.arrayType(llvm.i32TypeInContext(ctx.context), 0)),
  );
  defineType(
    ctx,
    "&[&i8]",
    llvm.pointerType(
      llvm.arrayType(llvm.pointerType(llvm.i8TypeInContext(ctx.context)), 0),
    ),
  );
  defineType(
    ctx,
    "&[i8]",
    llvm.pointerType(
      llvm.arrayType(llvm.i8TypeInContext(ctx.context), 0),
    ),
  );
}

export function getType(typeExpr: SExpr, ctx: ModuleContext): LLVMType {
  if (!isSymbol(typeExpr)) {
    return getTypeConstruct(typeExpr, ctx);
  }

  const type = ctx.types[typeExpr];
  if (!type) {
    throw new Error(`Unknown type: ${typeExpr}`);
  }

  return type;
}

export function getTypeConstruct(
  typeExpr: SExpr[],
  ctx: ModuleContext,
): LLVMType {
  const [command, ...args] = typeExpr;
  expectSymbol(command);

  switch (command) {
    case "&":
      return buildPtrType(command, args, ctx);
    case "[]":
      return buildArrayType(command, args, ctx);
    default:
      throw new Error(`Unknown type: ${typeExpr}`);
  }
}

export function buildPtrType(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMType {
  const { llvm } = ctx;

  const [typeExpr] = expectArgsLength(1, args, command);

  return llvm.pointerType(getType(typeExpr, ctx));
}

export function buildArrayType(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMType {
  const { llvm } = ctx;

  const [typeExpr] = expectArgsLength(1, args, command);

  return llvm.arrayType(getType(typeExpr, ctx), 0);
}
