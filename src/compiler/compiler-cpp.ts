import { readFile } from 'fs/promises';
import {
  BasicBlock,
  CallInst,
  Constant,
  Function as LLVMFunction,
  FunctionType,
  IRBuilder,
  LLVMContext,
  Module,
  ReturnInst,
  Type,
  UndefValue,
  Value,
  verifyFunction,
  verifyModule,
} from 'llvm-bindings';
import { panic } from 'panic-fn';
import tempy from 'tempy';

import { SExpr } from '../parser/parser';
import {
  expectArgsLength,
  expectArgsLengthAtLeast,
  expectI32,
  expectList,
  expectNumber,
  expectString,
  expectSymbol,
  isString,
  isSymbol,
} from './assertions';

// TODO: add expression types and values in parser: symbol | string | number | list
// TODO: pass expression locations to compiler for better error messages

type ModuleContext = {
  context: LLVMContext;
  builder: IRBuilder;
  module: Module;
  values: Record<string, Value>;
};

type FunctionContext = ModuleContext & { fn: LLVMFunction };

export async function compile(exprs: SExpr[]): Promise<string> {
  const context = new LLVMContext();

  const module = buildModule(context, exprs);

  return tempy.file.task((tmpFile) => {
    module.print(tmpFile);
    return readFile(tmpFile, { encoding: 'utf-8' });
  });
}

function buildModule(context: LLVMContext, exprs: SExpr[]): Module {
  const ctx: ModuleContext = {
    context,
    builder: new IRBuilder(context),
    module: new Module('main', context),
    values: {},
  };

  ctx.values['true'] = ctx.builder.getTrue();
  ctx.values['false'] = ctx.builder.getFalse();

  for (const expr of exprs) {
    buildValueInModuleContext(expr, ctx);
  }

  if (verifyModule(ctx.module)) {
    panic(`Verifying module failed: ${ctx.module.getName()}`);
  }

  return ctx.module;
}

function buildValueInModuleContext(expr: SExpr, ctx: ModuleContext): Value {
  const [command, ...args] = expr;
  expectSymbol(command);

  if (command === 'llvm/target-triple') {
    const [targetTriple] = expectArgsLength(1, args, command);
    expectString(targetTriple);

    ctx.module.setTargetTriple(getStringValue(targetTriple));

    return buildVoid(ctx);
  }

  if (command === 'external-fn') {
    const [fnName, argTypes, returnType] = expectArgsLength(3, args, command);
    expectSymbol(fnName);
    expectList(argTypes);
    expectSymbol(returnType);

    ctx.module.getOrInsertFunction(
      fnName,
      FunctionType.get(
        getType(returnType, ctx.builder),
        argTypes.map((argType) => {
          expectSymbol(argType);

          return getType(argType, ctx.builder);
        }),
        false,
      ),
    );

    return buildVoid(ctx);
  }

  if (command === 'fn') {
    return buildFunction(command, args, ctx);
  }

  return buildValue(expr, ctx);
}

function buildFunction(
  command: string,
  args: SExpr[],
  moduleCtx: ModuleContext,
): LLVMFunction {
  const [fnName, argTypes, returnType, ...exprs] = expectArgsLengthAtLeast(
    3,
    args,
    command,
  );
  expectSymbol(fnName);
  expectList(argTypes);
  expectSymbol(returnType);

  const fnType = FunctionType.get(
    getType(returnType, moduleCtx.builder),
    argTypes.map((argType) => {
      expectSymbol(argType);

      return getType(argType, ctx.builder);
    }),
    false,
  );

  const ctx: FunctionContext = {
    ...moduleCtx,
    fn: LLVMFunction.Create(
      fnType,
      LLVMFunction.LinkageTypes.ExternalLinkage,
      fnName,
      moduleCtx.module,
    ),
  };

  ctx.builder.SetInsertPoint(BasicBlock.Create(ctx.context, 'entry', ctx.fn));

  const values = exprs.map((expr) => buildValueInFunctionContext(expr, ctx));
  insertImplicitReturnOfLastValue(values, ctx);

  if (verifyFunction(ctx.fn)) {
    panic(`Function verification failed: ${ctx.fn.getName()}`);
  }

  return ctx.fn;
}

function insertImplicitReturnOfLastValue(
  values: Value[],
  ctx: FunctionContext,
): ReturnInst {
  const lastValue = values.at(-1);

  if (lastValue instanceof ReturnInst) {
    return lastValue;
  }

  const returnValue = lastValue ?? buildVoid(ctx);

  // TODO: add check for return type (`returnValue.getType()` throws 'TypeError: Illegal invocation')
  // if (!Type.isSameType(ctx.fn.getReturnType(), returnValue.getType())) {
  //   panic(
  //     `Function ${ctx.fn.getName()} must return ${ctx.fn.getReturnType()} but ${returnValue.getType()} was found`,
  //   );
  // }

  return ctx.builder.CreateRet(returnValue);
}

function buildValueInFunctionContext(expr: SExpr, ctx: FunctionContext): Value {
  const [command, ...args] = expr;
  expectSymbol(command);

  if (command === 'llvm/ret') {
    const [returnExpr] = expectArgsLength(1, args, command);

    return ctx.builder.CreateRet(buildValue(returnExpr, ctx));
  }

  return buildValue(expr, ctx);
}

function buildValue(expr: SExpr, ctx: ModuleContext): Value {
  if (isSymbol(expr)) {
    return buildConstant(expr, ctx);
  }

  if (isString(expr)) {
    return buildString(expr, ctx);
  }

  expectList(expr);
  const [command, ...args] = expr;
  expectSymbol(command);

  if (command === 'i32') {
    const [value] = expectArgsLength(1, args, command);
    expectNumber(value);

    const i32Value = getNumberValue(value);
    expectI32(i32Value);

    return ctx.builder.getInt32(i32Value);
  }

  return buildFunctionCall(command, args, ctx);
}

function buildConstant(name: string, ctx: ModuleContext): Value {
  const constant = ctx.values[name];

  if (!constant) {
    panic(`Constant is not defined ${name}`);
  }

  return constant;
}

function buildString(expr: string, ctx: ModuleContext): Constant {
  return ctx.builder.CreateGlobalStringPtr(getStringValue(expr), 'str');
}

function buildFunctionCall(
  fnName: string,
  args: SExpr[],
  ctx: ModuleContext,
): CallInst {
  const callee = ctx.module.getFunction(fnName);

  if (!callee) {
    panic(`Function ${fnName} is not defined`);
  }

  return ctx.builder.CreateCall(
    callee,
    args.map((arg) => buildValue(arg, ctx)),
    'i',
  );
}

function buildVoid(ctx: ModuleContext): UndefValue {
  return UndefValue.get(ctx.builder.getVoidTy());
}

function getType(typeName: string, builder: IRBuilder): Type {
  switch (typeName) {
    case 'i32':
      return builder.getInt32Ty();
    case '&i8':
      return builder.getInt8PtrTy();
    default:
      panic(`Unknown type: ${typeName}`);
  }
}

function getStringValue(value: string): string {
  return value
    .slice(1, -1)
    .replace(/\\"/g, '"')
    .replace(/\\n/g, '\n')
    .replace(/\\\\/g, '\\');
}

function getNumberValue(value: string): number {
  return parseFloat(value.replace(/_/g, ''));
}
