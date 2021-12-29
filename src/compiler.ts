import { readFile } from 'fs/promises';
import {
  BasicBlock,
  Function as LLVMFunction,
  FunctionType,
  IRBuilder,
  LLVMContext,
  Module,
  Type,
  Value,
  verifyFunction,
  verifyModule,
} from 'llvm-bindings';
import { panic } from 'panic-fn';
import tempy from 'tempy';

import { SExpr } from './parser';

// TODO: pass expression locations to compiler for better error messages

const STRING_START = ['"'];
const NUMBER_START = '0123456789'.split('');

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
    const [command, ...args] = expr;
    expectSymbol(command);

    if (command === 'llvm/target-triple') {
      const [targetTriple] = expectArgsLength(1, args, command);
      expectString(targetTriple);

      ctx.module.setTargetTriple(getStringValue(targetTriple));
    } else if (command === 'llvm/extern-fn') {
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
    } else if (command === 'llvm/define-string-ptr') {
      const [stringName, stringContent] = expectArgsLength(2, args, command);
      expectSymbol(stringName);
      expectString(stringContent);

      ctx.values[stringName] = ctx.builder.CreateGlobalStringPtr(
        getStringValue(stringContent),
        stringName,
        0,
        ctx.module,
      );
    } else if (command === 'llvm/fn') {
      const [fnName, argTypes, returnType, ...exprs] = expectArgsLengthAtLeast(
        3,
        args,
        command,
      );
      expectSymbol(fnName);
      expectList(argTypes);
      expectSymbol(returnType);

      const fnType = FunctionType.get(
        getType(returnType, ctx.builder),
        argTypes.map((argType) => {
          expectSymbol(argType);

          return getType(argType, ctx.builder);
        }),
        false,
      );

      buildFunction(fnName, fnType, exprs, ctx);
    } else {
      panic(`Unknown command: ${command}`);
    }
  }

  if (verifyModule(ctx.module)) {
    panic(`Verifying module failed: ${ctx.module.getName()}`);
  }

  return ctx.module;
}

function buildFunction(
  fnName: string,
  fnType: FunctionType,
  exprs: SExpr[],
  moduleContext: ModuleContext,
) {
  const ctx: FunctionContext = {
    ...moduleContext,
    fn: LLVMFunction.Create(
      fnType,
      LLVMFunction.LinkageTypes.ExternalLinkage,
      fnName,
      moduleContext.module,
    ),
  };

  for (const expr of exprs) {
    const [command, ...args] = expr;
    expectSymbol(command);

    if (command === 'llvm/insert-point') {
      const [entryName] = expectArgsLength(1, args, command);
      expectString(entryName);

      ctx.builder.SetInsertPoint(
        BasicBlock.Create(ctx.context, getStringValue(entryName), ctx.fn),
      );
    } else if (command === 'llvm/call') {
      const [fnName, argValues] = expectArgsLength(2, args, command);
      expectSymbol(fnName);
      expectList(argValues);

      const callee = ctx.module.getFunction(fnName);

      if (!callee) {
        panic(`Function ${fnName} is not defined`);
      }

      ctx.builder.CreateCall(
        callee,
        argValues.map((arg) => buildValue(arg, ctx)),
      );
    } else if (command === 'llvm/ret') {
      const [returnExpr] = expectArgsLength(1, args, command);

      ctx.builder.CreateRet(buildValue(returnExpr, ctx));
    } else {
      panic(`Unknown command: ${command}`);
    }
  }

  if (verifyFunction(ctx.fn)) {
    panic(`Function verification failed: ${ctx.fn.getName()}`);
  }
}

function buildValue(expr: SExpr, ctx: ModuleContext): Value {
  if (isSymbol(expr)) {
    const constant = ctx.values[expr];

    if (!constant) {
      panic(`Constant is not defined ${expr}`);
    }

    return constant;
  }

  expectList(expr);

  const [command, ...args] = expr;

  if (command === 'i32') {
    const [value] = expectArgsLength(1, args, command);
    expectNumber(value);

    const i32Value = getNumberValue(value);
    expectI32(i32Value);

    return ctx.builder.getInt32(i32Value);
  }

  panic(`Unknown command: ${command}`);
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

/*
  TODO: merge assertions and type predicates
  Would be nice to have it like this:
  isSymbol(expr: SExpr): expr is string
  expect(isSymbol): asserts expr is string
 */
function isSymbol(expr: SExpr): expr is string {
  if (typeof expr !== 'string') {
    return false;
  }

  if (STRING_START.includes(expr[0])) {
    return false;
  }

  if (NUMBER_START.includes(expr[0])) {
    return false;
  }

  return true;
}

function expectSymbol(expr: SExpr): asserts expr is string {
  if (typeof expr !== 'string') {
    panic('Symbol expected, found: list');
  }

  if (STRING_START.includes(expr[0])) {
    panic('Symbol expected, found: string');
  }

  if (NUMBER_START.includes(expr[0])) {
    panic('Symbol expected, found: number');
  }
}

function expectString(expr: SExpr): asserts expr is string {
  if (typeof expr !== 'string') {
    panic('Symbol expected, found: list');
  }

  if (NUMBER_START.includes(expr[0])) {
    panic('String expected, found: number');
  }

  if (!STRING_START.includes(expr[0])) {
    panic('String expected, found: symbol');
  }
}

function expectNumber(expr: SExpr): asserts expr is string {
  if (typeof expr !== 'string') {
    panic('Number expected, found: list');
  }

  if (STRING_START.includes(expr[0])) {
    panic('Number expected, found: string');
  }

  if (!NUMBER_START.includes(expr[0])) {
    panic('Number expected, found: symbol');
  }
}

function expectList(expr: SExpr): asserts expr is SExpr[] {
  if (typeof expr === 'string') {
    panic('List expected, found: atom');
  }
}

// TODO: add bound checks
function expectI32(value: number) {
  if (!Number.isInteger(value)) {
    panic(`i32 can not hold ${value}`);
  }
}

function expectArgsLength(
  argsLength: number,
  args: SExpr[],
  command: string,
): SExpr[] {
  if (args.length !== argsLength) {
    panic(
      `Command ${command} expects ${argsLength} argument(s) but ${args.length} was given`,
    );
  }

  return args;
}

function expectArgsLengthAtLeast(
  argsLength: number,
  args: SExpr[],
  command: string,
): SExpr[] {
  if (args.length < argsLength) {
    panic(
      `Command ${command} expects at least ${argsLength} argument(s) but ${args.length} was given`,
    );
  }

  return args;
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
