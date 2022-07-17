import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength, expectString } from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { LLVMValue } from "../../llvm/llvm-c.ts";
import { getStringValue } from "../../s-expr/transformers.ts";
import { buildVoid } from "./void.ts";

export function buildTargetTriple(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
  const { llvm } = ctx;

  const [targetTriple] = expectArgsLength(1, args, command);
  expectString(targetTriple);

  llvm.setTarget(ctx.module, getStringValue(targetTriple));

  return buildVoid(ctx);
}
