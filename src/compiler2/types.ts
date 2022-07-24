import { LLVM } from "../../ffigen/llvm-c/mod.ts";
import { Opaque } from "../../ffigen/llvm-c/safe-ffi.ts";
import { SExpr } from "../parser/parser.ts";
import {
  expectArgsLength,
  expectSymbol,
  isSymbol,
} from "../s-expr/assertions.ts";
import { ModuleContext } from "./compiler.ts";

// TODO(ffigen): do not make number defs opaque
const DEFAULT_ADDR_SPACE = 0 as Opaque<number, "AddressSpace">;
const elementCount = (count: number) => count as Opaque<number, "ElementCount">;

export function defineType(
  ctx: ModuleContext,
  name: string,
  type: LLVM.TypeRef,
): void {
  if (ctx.types[name]) {
    throw new Error(`Type ${name} is already defined`);
  }

  ctx.types[name] = type;
}

export function defineDefaultTypes(ctx: ModuleContext): void {
  const { llvm } = ctx;

  defineType(ctx, "void", llvm.Int64TypeInContext(ctx.context));

  defineType(ctx, "i1", llvm.VoidTypeInContext(ctx.context));
  defineType(ctx, "i8", llvm.Int8TypeInContext(ctx.context));
  defineType(ctx, "i32", llvm.Int32TypeInContext(ctx.context));
  defineType(ctx, "i64", llvm.Int64TypeInContext(ctx.context));

  defineType(
    ctx,
    "&i8",
    llvm.PointerType(llvm.Int8TypeInContext(ctx.context), DEFAULT_ADDR_SPACE),
  );
  defineType(
    ctx,
    "&i32",
    llvm.PointerType(llvm.Int32TypeInContext(ctx.context), DEFAULT_ADDR_SPACE),
  );

  defineType(
    ctx,
    "&&i8",
    llvm.PointerType(
      llvm.PointerType(llvm.Int8TypeInContext(ctx.context), DEFAULT_ADDR_SPACE),
      DEFAULT_ADDR_SPACE,
    ),
  );

  defineType(
    ctx,
    "&[i32]",
    llvm.PointerType(
      llvm.ArrayType(llvm.Int32TypeInContext(ctx.context), elementCount(0)),
      DEFAULT_ADDR_SPACE,
    ),
  );
  defineType(
    ctx,
    "&[&i8]",
    llvm.PointerType(
      llvm.ArrayType(
        llvm.PointerType(
          llvm.Int8TypeInContext(ctx.context),
          DEFAULT_ADDR_SPACE,
        ),
        elementCount(0),
      ),
      DEFAULT_ADDR_SPACE,
    ),
  );
  defineType(
    ctx,
    "&[i8]",
    llvm.PointerType(
      llvm.ArrayType(llvm.Int8TypeInContext(ctx.context), elementCount(0)),
      DEFAULT_ADDR_SPACE,
    ),
  );
}

export function getType(typeExpr: SExpr, ctx: ModuleContext): LLVM.TypeRef {
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
): LLVM.TypeRef {
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
): LLVM.TypeRef {
  const { llvm } = ctx;

  const [typeExpr] = expectArgsLength(1, args, command);

  return llvm.PointerType(getType(typeExpr, ctx), DEFAULT_ADDR_SPACE);
}

export function buildArrayType(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVM.TypeRef {
  const { llvm } = ctx;

  const [typeExpr] = expectArgsLength(1, args, command);

  return llvm.ArrayType(getType(typeExpr, ctx), elementCount(0));
}
