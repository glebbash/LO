import { SExpr } from "../../parser/parser.ts";
import {
  expectArgsLength,
  expectArgsLengthAtLeast,
  expectList,
  expectSymbol,
} from "../assertions.ts";
import { ModuleContext, VERIFICATION_ENABLED } from "../compiler.ts";
import { LLVMValue } from "../llvm-c.ts";
import { getType } from "../types.ts";
import { buildValue, buildValueInFunctionContext } from "./mod.ts";
import { buildVoid } from "./void.ts";

export function buildFn(
  command: string,
  args: SExpr[],
  moduleCtx: ModuleContext,
): LLVMValue {
  const { llvm } = moduleCtx;

  const [fnName, params, returnTypeExpr, ...exprs] = expectArgsLengthAtLeast(
    3,
    args,
    command,
  );
  expectSymbol(fnName);
  expectList(params);

  const paramInfos = params.map((paramInfo) => {
    expectList(paramInfo);

    if (paramInfo.length !== 2) {
      throw new Error("Arguments in argument list must have name and type");
    }

    const [paramName, paramTypeExpr] = paramInfo;
    expectSymbol(paramName);

    return { name: paramName, type: getType(paramTypeExpr, moduleCtx) };
  });

  const paramTypes = paramInfos.map((info) => info.type);
  const paramNames = paramInfos.map((info) => info.name);

  if (new Set(paramNames).size !== paramNames.length) {
    throw new Error("Parameter names must be unique");
  }

  const fnType = llvm.functionType(
    getType(returnTypeExpr, moduleCtx),
    paramTypes,
  );

  const ctx: ModuleContext = {
    ...moduleCtx,
    values: { ...moduleCtx.values },
  };

  const fn = llvm.addFunction(moduleCtx.module, fnName, fnType);

  for (let index = 0; index < paramNames.length; index++) {
    const paramName = paramNames[index];
    ctx.values[paramName] = llvm.getParam(fn, index);
  }

  const entry = llvm.appendBasicBlockInContext(ctx.context, fn, "entry");
  llvm.positionBuilderAtEnd(ctx.builder, entry);

  const values = exprs.map((expr) => buildValueInFunctionContext(expr, ctx));
  insertImplicitReturnOfLastValue(values, ctx);

  if (VERIFICATION_ENABLED && !llvm.verifyFunction(fn).ok) {
    throw new Error(`Function verification failed: ${fnName}`);
  }

  return fn;
}

export function buildExternalFn(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
  const { llvm } = ctx;

  const [fnName, argTypes, returnTypeExpr] = expectArgsLength(3, args, command);
  expectSymbol(fnName);
  expectList(argTypes);

  llvm.addFunction(
    ctx.module,
    fnName,
    llvm.functionType(
      getType(returnTypeExpr, ctx),
      argTypes.map((argTypeExpr) => getType(argTypeExpr, ctx)),
    ),
  );

  return buildVoid(ctx);
}

export function buildFunctionCall(
  fnName: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
  const { llvm } = ctx;

  const callee = llvm.getNamedFunction(ctx.module, fnName);

  if (callee.isNull()) {
    throw new Error(`Function ${fnName} is not defined`);
  }

  return llvm.buildCall(
    ctx.builder,
    callee,
    args.map((arg) => buildValue(arg, ctx)),
  );
}

function insertImplicitReturnOfLastValue(
  values: LLVMValue[],
  ctx: ModuleContext,
): LLVMValue {
  const { llvm } = ctx;

  const returnValue = values.at(-1) ?? buildVoid(ctx);
  return llvm.buildRet(ctx.builder, returnValue);
}
