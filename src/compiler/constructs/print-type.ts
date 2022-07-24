import { LLVM } from "../../llvm-c-14/llvm-c/mod.ts";
import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength } from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { readCString } from "../utils.ts";
import { buildValue } from "./mod.ts";

export function buildPrintType(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const [valueExpr] = expectArgsLength(1, args, command);
  const value = buildValue(valueExpr, ctx);

  const type = llvm.TypeOf(value);
  const typeNameRef = llvm.PrintTypeToString(type);
  console.log(readCString(typeNameRef));

  return value;
}
