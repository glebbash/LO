import { LLVM } from "../../llvm-c-14/llvm-c/mod.ts";
import { SExpr } from "../../parser/parser.ts";
import {
  expectArgsLength,
  expectArgsLengthAtLeast,
  expectList,
  expectSymbol,
} from "../../s-expr/assertions.ts";
import { ModuleContext, VERIFICATION_ENABLED } from "../compiler.ts";
import { getType } from "../types.ts";
import {
  BOOL_FALSE,
  BOOL_TRUE,
  buildArrayPtr,
  NULL_PTR,
  nullPtr,
  toCString,
} from "../utils.ts";
import { buildValue, buildValueInFunctionContext } from "./mod.ts";
import { buildVoid } from "./void.ts";

export function buildFn(
  command: string,
  args: SExpr[],
  moduleCtx: ModuleContext,
): LLVM.ValueRef {
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

  const fnType = llvm.FunctionType(
    getType(returnTypeExpr, moduleCtx),
    buildArrayPtr(paramTypes),
    paramTypes.length,
    BOOL_FALSE,
  );

  const ctx: ModuleContext = {
    ...moduleCtx,
    values: { ...moduleCtx.values },
  };

  const fn = llvm.AddFunction(moduleCtx.module, toCString(fnName), fnType);

  for (let index = 0; index < paramNames.length; index++) {
    const paramName = paramNames[index];
    ctx.values[paramName] = llvm.GetParam(fn, index);
  }

  const entry = llvm.AppendBasicBlockInContext(
    ctx.context,
    fn,
    toCString("entry"),
  );
  llvm.PositionBuilderAtEnd(ctx.builder, entry);

  const values = exprs.map((expr) => buildValueInFunctionContext(expr, ctx));
  insertImplicitReturnOfLastValue(values, ctx);

  if (VERIFICATION_ENABLED) {
    const errorDetected = llvm.VerifyFunction(
      fn,
      llvm.VerifierFailureAction.LLVMReturnStatusAction,
    );

    if (errorDetected !== BOOL_FALSE) {
      throw new Error(`Verifying function failed: ${fnName}`);
    }
  }

  return fn;
}

export function buildExternalFn(
  command: string,
  args: SExpr[],
  ctx: ModuleContext,
  isVarArg = false,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const [fnName, argTypes, returnTypeExpr] = expectArgsLength(3, args, command);
  expectSymbol(fnName);
  expectList(argTypes);

  llvm.AddFunction(
    ctx.module,
    toCString(fnName),
    llvm.FunctionType(
      getType(returnTypeExpr, ctx),
      buildArrayPtr(argTypes.map((argTypeExpr) => getType(argTypeExpr, ctx))),
      argTypes.length,
      isVarArg ? BOOL_TRUE : BOOL_FALSE,
    ),
  );

  return buildVoid(ctx);
}

export function buildFunctionCall(
  fnName: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const callee = llvm.GetNamedFunction(ctx.module, toCString(fnName));

  if (callee === NULL_PTR) {
    throw new Error(`Function ${fnName} is not defined`);
  }

  return llvm.BuildCall(
    ctx.builder,
    callee,
    buildArrayPtr(args.map((arg) => buildValue(arg, ctx))),
    args.length,
    nullPtr(),
  );
}

function insertImplicitReturnOfLastValue(
  values: LLVM.ValueRef[],
  ctx: ModuleContext,
): LLVM.ValueRef {
  const { llvm } = ctx;

  const returnValue = values.at(-1) ?? buildVoid(ctx);
  return llvm.BuildRet(ctx.builder, returnValue);
}
