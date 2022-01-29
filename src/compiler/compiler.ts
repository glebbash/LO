import { panic } from 'panic-fn';

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

const VERIFICATION_ENABLED = false;

export function compile(
  exprs: SExpr[],
  outputIRFile: string,
  llvm = loadLibLLVM(),
) {
  const ctx: CodeGenContext = {
    llvm,
    context: llvm.contextCreate(),
  };

  const module = buildModule(ctx, exprs);
  llvm.printModuleToFile(module, outputIRFile);

  llvm.disposeModule(module);
  llvm.contextDispose(ctx.context);
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

  if (VERIFICATION_ENABLED) {
    const res = llvm.verifyModule(ctx.module);

    if (!res.ok) {
      console.error(res.message);
      panic(`Verifying module failed: ${moduleName}`);
    }
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

  if (VERIFICATION_ENABLED && !llvm.verifyFunction(ctx.fn).ok) {
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
  const [command, ...args] = expr;
  expectSymbol(command);

  if (command === 'let') {
    const [name, expr] = expectArgsLength(2, args, command);
    expectSymbol(name);

    if (ctx.values[name]) {
      panic(`Constant ${name} is already defined`);
    }

    const value = buildValue(expr, ctx);
    ctx.values[name] = value;
    return value;
  }

  return buildValue(expr, ctx);
}

function buildValue(expr: SExpr, ctx: ModuleContext): LLVMValue {
  const { llvm } = ctx;

  if (isSymbol(expr)) {
    return buildConstantAccess(expr, ctx);
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

  if (command === 'nullptr') {
    const [typeName] = expectArgsLength(1, args, command);
    expectSymbol(typeName);

    const type = getType(typeName, ctx);
    return llvm.constPointerNull(type);
  }

  if (command === 'array') {
    const [elementTypeName, valueExprs] = expectArgsLength(2, args, command);
    expectSymbol(elementTypeName);
    expectList(valueExprs);

    const elementType = getType(elementTypeName, ctx);
    const arrayType = llvm.arrayType(elementType, valueExprs.length);
    const array = llvm.buildAlloca(ctx.builder, arrayType);

    if (valueExprs.length === 0) {
      return array; // TODO: check if this should be null pointer
    }

    const values = valueExprs.map((expr) => buildValue(expr, ctx));
    const [firstValue, ...otherValues] = values;

    const zero = llvm.constInt(llvm.i32TypeInContext(ctx.context), 0);
    const firstElementPointer = llvm.buildGEP(ctx.builder, array, [zero, zero]);
    llvm.buildStore(ctx.builder, firstValue, firstElementPointer);

    let elementPointer = firstElementPointer;
    for (let index = 0; index < otherValues.length; index++) {
      const value = otherValues[index];
      elementPointer = llvm.buildGEP(ctx.builder, elementPointer, [
        llvm.constInt(llvm.i32TypeInContext(ctx.context), 1),
      ]);
      llvm.buildStore(ctx.builder, value, elementPointer);
    }

    return firstElementPointer;
  }

  return buildFunctionCall(command, args, ctx);
}

function buildConstantAccess(name: string, ctx: ModuleContext): LLVMValue {
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
    case '&&i8':
      return llvm.pointerType(
        llvm.pointerType(llvm.i8TypeInContext(ctx.context)),
      );
    case 'void':
      return llvm.voidTypeInContext(ctx.context);
    default:
      panic(`Unknown type: ${typeName}`);
  }
}
