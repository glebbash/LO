import { LLVMValue } from "../../llvm/llvm-c.ts";
import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength } from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { buildValue } from "./mod.ts";

export function buildPrintType(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
  const { llvm } = ctx;

  const [valueExpr] = expectArgsLength(1, args, command);
  const value = buildValue(valueExpr, ctx);

  const type = llvm.typeOf(value);
  const typeName = llvm.printTypeToString(type).stringValue();
  console.log(typeName);

  return value;
}
