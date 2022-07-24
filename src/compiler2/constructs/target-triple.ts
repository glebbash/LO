import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength, expectString } from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { getStringValue } from "../../s-expr/transformers.ts";
import { buildVoid } from "./void.ts";
import { LLVM } from "../../llvm-c-14/llvm-c/mod.ts";
import { toCString } from "../utils.ts";

export function buildTargetTriple(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const [targetTriple] = expectArgsLength(1, args, command);
  expectString(targetTriple);

  llvm.SetTarget(ctx.module, toCString(getStringValue(targetTriple)));

  return buildVoid(ctx);
}
