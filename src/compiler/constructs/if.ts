import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength } from "../assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { LLVMValue } from "../../llvm/llvm-c.ts";
import { buildValue } from "./mod.ts";

export function buildIf(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
  const { llvm } = ctx;

  const [cond, ifTrue, ifFalse] = expectArgsLength(3, args, command);

  const fn = llvm.getBasicBlockParent(llvm.getInsertBlock(ctx.builder));

  let ifTrueBlock = llvm.appendBasicBlockInContext(ctx.context, fn);
  let ifFalseBlock = llvm.createBasicBlockInContext(ctx.context);
  const mergeBlock = llvm.createBasicBlockInContext(ctx.context);

  const condValue = buildValue(cond, ctx);
  llvm.buildCondBr(ctx.builder, condValue, ifTrueBlock, ifFalseBlock);

  llvm.positionBuilderAtEnd(ctx.builder, ifTrueBlock);
  const ifTrueValue = buildValue(ifTrue, ctx);
  llvm.buildBr(ctx.builder, mergeBlock);
  ifTrueBlock = llvm.getInsertBlock(ctx.builder);

  llvm.insertExistingBasicBlockAfterInsertBlock(ctx.builder, ifFalseBlock);
  llvm.positionBuilderAtEnd(ctx.builder, ifFalseBlock);
  const ifFalseValue = buildValue(ifFalse, ctx);
  llvm.buildBr(ctx.builder, mergeBlock);
  ifFalseBlock = llvm.getInsertBlock(ctx.builder);

  llvm.insertExistingBasicBlockAfterInsertBlock(ctx.builder, mergeBlock);
  llvm.positionBuilderAtEnd(ctx.builder, mergeBlock);

  const phi = llvm.buildPhi(ctx.builder, llvm.typeOf(ifTrueValue));
  llvm.addIncoming(phi, [ifTrueValue], [ifTrueBlock]);
  llvm.addIncoming(phi, [ifFalseValue], [ifFalseBlock]);

  return phi;
}
