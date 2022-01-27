import { readFile } from 'fs/promises';
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
import {
  LibLLVM,
  LLVMContext,
  LLVMIRBuilder,
  LLVMModule,
  LLVMType,
  LLVMValue,
  loadLibLLVM,
} from './llvm-c';
import { getNumberValue, getStringValue } from './transformers';

// TODO: add expression types and values in parser: symbol | string | number | list
// TODO: pass expression locations to compiler for better error messages

type CodeGenContext = {
  llvm: LibLLVM;
  context: LLVMContext;
};

type ModuleContext = CodeGenContext & {
  builder: LLVMIRBuilder;
  module: LLVMModule;
  values: Record<string, LLVMValue>;
};

type FunctionContext = ModuleContext & { fn: LLVMValue };

export async function compile(
  exprs: SExpr[],
  llvm = loadLibLLVM(),
): Promise<string> {
  const ctx: CodeGenContext = {
    llvm,
    context: llvm.contextCreate(),
  };

  const module = buildModule(ctx, exprs);

  const llvmIRFile = await tempy.file.task((tmpFile) => {
    llvm.printModuleToFile(module, tmpFile);
    return readFile(tmpFile, { encoding: 'utf-8' });
  });

  llvm.disposeModule(module);
  llvm.contextDispose(ctx.context);

  return llvmIRFile;
}

function buildModule(parentCtx: CodeGenContext, exprs: SExpr[]): LLVMModule {
  const { llvm } = parentCtx;

  const moduleName = 'main';
  const ctx: ModuleContext = {
    ...parentCtx,
    builder: llvm.createBuilderInContext(parentCtx.context),
    module: llvm.moduleCreateWithNameInContext(moduleName, parentCtx.context),
    values: {},
  };

  for (const expr of exprs) {
    buildValueInModuleContext(expr, ctx);
  }

  const res = llvm.verifyModule(ctx.module);

  if (!res.ok) {
    console.error(res.message);
    panic(`Verifying module failed: ${moduleName}`);
  }

  llvm.disposeBuilder(ctx.builder);

  return ctx.module;
}

function buildValueInModuleContext(expr: SExpr, ctx: ModuleContext): LLVMValue {
  const { llvm } = ctx;

  const [command, ...args] = expr;
  expectSymbol(command);

  if (command === 'llvm/target-triple') {
    const [targetTriple] = expectArgsLength(1, args, command);
    expectString(targetTriple);

    llvm.setTarget(ctx.module, getStringValue(targetTriple));

    return buildVoid(ctx);
  }

  if (command === 'external-fn') {
    const [fnName, argTypes, returnType] = expectArgsLength(3, args, command);
    expectSymbol(fnName);
    expectList(argTypes);
    expectSymbol(returnType);

    llvm.addFunction(
      ctx.module,
      fnName,
      llvm.functionType(
        getType(returnType, ctx),
        argTypes.map((argType) => {
          expectSymbol(argType);

          return getType(argType, ctx);
        }),
      ),
    );

    return buildVoid(ctx);
  }

  if (command === 'fn') {
    return buildFunction(command, args, ctx);
  }

  // return buildValue(expr, ctx);

  panic('//TODO: implement this');
}

function buildFunction(
  command: string,
  args: SExpr[],
  moduleCtx: ModuleContext,
): LLVMValue {
  const { llvm } = moduleCtx;

  const [fnName, argTypes, returnType, ...exprs] = expectArgsLengthAtLeast(
    3,
    args,
    command,
  );
  expectSymbol(fnName);
  expectList(argTypes);
  expectSymbol(returnType);

  const fnType = llvm.functionType(
    getType(returnType, moduleCtx),
    argTypes.map((argType) => {
      expectSymbol(argType);

      return getType(argType, moduleCtx);
    }),
  );

  const ctx: FunctionContext = {
    ...moduleCtx,
    // TODO: check if and how LLVMFunction.LinkageTypes.ExternalLinkage should be added
    fn: llvm.addFunction(moduleCtx.module, fnName, fnType),
  };

  const entry = llvm.appendBasicBlockInContext(ctx.context, ctx.fn, 'entry');
  llvm.positionBuilderAtEnd(ctx.builder, entry);

  const values = exprs.map((expr) => buildValueInFunctionContext(expr, ctx));
  insertImplicitReturnOfLastValue(values, ctx);

  if (!llvm.verifyFunction(ctx.fn).ok) {
    panic(`Function verification failed: ${fnName}`);
  }

  return ctx.fn;
}

function insertImplicitReturnOfLastValue(
  values: LLVMValue[],
  ctx: FunctionContext,
): LLVMValue {
  const { llvm } = ctx;

  const lastValue = values.at(-1);

  const returnValue = lastValue ?? buildVoid(ctx);

  // TODO: add check for return type (`returnValue.getType()` throws 'TypeError: Illegal invocation')
  // if (!Type.isSameType(ctx.fn.getReturnType(), returnValue.getType())) {
  //   panic(
  //     `Function ${ctx.fn.getName()} must return ${ctx.fn.getReturnType()} but ${returnValue.getType()} was found`,
  //   );
  // }

  return llvm.buildRet(ctx.builder, returnValue);
}

function buildValueInFunctionContext(
  expr: SExpr,
  ctx: FunctionContext,
): LLVMValue {
  const { llvm } = ctx;

  const [command, ...args] = expr;
  expectSymbol(command);

  if (command === 'llvm/ret') {
    const [returnExpr] = expectArgsLength(1, args, command);

    return llvm.buildRet(ctx.builder, buildValue(returnExpr, ctx));
  }

  if (command === 'let') {
    const [name, expr] = expectArgsLength(2, args, command);
    expectSymbol(name);

    const value = buildValue(expr, ctx);
    ctx.values[name] = value;
    return value;
  }

  return buildValue(expr, ctx);
}

function buildValue(expr: SExpr, ctx: ModuleContext): LLVMValue {
  const { llvm } = ctx;

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

    return llvm.constInt(llvm.i32TypeInContext(ctx.context), i32Value);
  }

  if (command === 'void') {
    expectArgsLength(0, args, command);

    return buildVoid(ctx);
  }

  return buildFunctionCall(command, args, ctx);
}

function buildConstant(name: string, ctx: ModuleContext): LLVMValue {
  const constant = ctx.values[name];

  if (!constant) {
    panic(`Constant is not defined ${name}`);
  }

  return constant;
}

function buildString(expr: string, ctx: ModuleContext): LLVMValue {
  const { llvm } = ctx;

  return llvm.buildGlobalStringPtr(ctx.builder, getStringValue(expr));
}

function buildFunctionCall(
  fnName: string,
  args: SExpr[],
  ctx: ModuleContext,
): LLVMValue {
  const { llvm } = ctx;

  const callee = llvm.getNamedFunction(ctx.module, fnName);

  if (callee.value.isNull()) {
    panic(`Function ${fnName} is not defined`);
  }

  return llvm.buildCall(
    ctx.builder,
    callee,
    args.map((arg) => buildValue(arg, ctx)),
  );
}

function buildVoid(ctx: ModuleContext): LLVMValue {
  const { llvm } = ctx;

  return llvm.getUndef(llvm.voidTypeInContext(ctx.context));
}

function getType(typeName: string, ctx: ModuleContext): LLVMType {
  const { llvm } = ctx;

  switch (typeName) {
    case 'i32':
      return llvm.i32TypeInContext(ctx.context);
    case '&i8':
      return llvm.pointerType(llvm.i8TypeInContext(ctx.context));
    case 'void':
      return llvm.voidTypeInContext(ctx.context);
    default:
      panic(`Unknown type: ${typeName}`);
  }
}
