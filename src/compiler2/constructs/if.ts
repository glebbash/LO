import { LLVM } from "../../llvm-c-14/llvm-c/mod.ts";
import { SExpr } from "../../parser/parser.ts";
import { expectArgsLength } from "../../s-expr/assertions.ts";
import { ModuleContext } from "../compiler.ts";
import { buildArrayPtr, toCString } from "../utils.ts";
import { buildValue } from "./mod.ts";

export function buildIf(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const [cond, ifTrue, ifFalse] = expectArgsLength(3, args, command);

  const fn = llvm.GetBasicBlockParent(llvm.GetInsertBlock(ctx.builder));

  let ifTrueBlock = llvm.AppendBasicBlockInContext(
    ctx.context,
    fn,
    toCString(""),
  );
  let ifFalseBlock = llvm.CreateBasicBlockInContext(ctx.context, toCString(""));
  const mergeBlock = llvm.CreateBasicBlockInContext(ctx.context, toCString(""));

  const condValue = buildValue(cond, ctx);
  llvm.BuildCondBr(ctx.builder, condValue, ifTrueBlock, ifFalseBlock);

  llvm.PositionBuilderAtEnd(ctx.builder, ifTrueBlock);
  const ifTrueValue = buildValue(ifTrue, ctx);
  llvm.BuildBr(ctx.builder, mergeBlock);
  ifTrueBlock = llvm.GetInsertBlock(ctx.builder);

  llvm.InsertExistingBasicBlockAfterInsertBlock(ctx.builder, ifFalseBlock);
  llvm.PositionBuilderAtEnd(ctx.builder, ifFalseBlock);
  const ifFalseValue = buildValue(ifFalse, ctx);
  llvm.BuildBr(ctx.builder, mergeBlock);
  ifFalseBlock = llvm.GetInsertBlock(ctx.builder);

  llvm.InsertExistingBasicBlockAfterInsertBlock(ctx.builder, mergeBlock);
  llvm.PositionBuilderAtEnd(ctx.builder, mergeBlock);

  const phi = llvm.BuildPhi(
    ctx.builder,
    llvm.TypeOf(ifTrueValue),
    toCString(""),
  );
  llvm.AddIncoming(
    phi,
    buildArrayPtr([ifTrueValue]),
    buildArrayPtr([ifTrueBlock]),
    1,
  );
  llvm.AddIncoming(
    phi,
    buildArrayPtr([ifFalseValue]),
    buildArrayPtr([ifFalseBlock]),
    1,
  );

  return phi;
}
