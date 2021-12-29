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

import { SExpr } from './parser';

const STRING_START = ['"'];
const NUMBER_START = '0123456789'.split('');

export function compile(exprs: SExpr[], outputIRFile: string) {
  const context = new LLVMContext();

  const module = buildMainModule(context, exprs);

  module.print(outputIRFile);
}

function buildMainModule(context: LLVMContext, exprs: SExpr[]): Module {
  const module = new Module('top', context);
  const builder = new IRBuilder(context);

  const values: Record<string, Value> = {};

  for (const expr of exprs) {
    const [command, ...args] = expr;
    expectSymbol(command);

    if (command === 'llvm/target-triple') {
      const [targetTriple] = expectArgsLength(1, args, command);
      expectString(targetTriple);

      module.setTargetTriple(getStringValue(targetTriple));
    } else if (command === 'llvm/extern-fn') {
      const [fnName, argTypes, returnType] = expectArgsLength(3, args, command);
      expectSymbol(fnName);
      expectList(argTypes);
      expectSymbol(returnType);

      module.getOrInsertFunction(
        fnName,
        FunctionType.get(
          getType(returnType, builder),
          argTypes.map((argType) => {
            expectSymbol(argType);

            return getType(argType, builder);
          }),
          false,
        ),
      );
    } else if (command === 'llvm/define-string-ptr') {
      const [stringName, stringContent] = expectArgsLength(2, args, command);
      expectSymbol(stringName);
      expectString(stringContent);

      values[stringName] = builder.CreateGlobalStringPtr(
        getStringValue(stringContent),
        stringName,
        0,
        module,
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
        getType(returnType, builder),
        argTypes.map((argType) => {
          expectSymbol(argType);

          return getType(argType, builder);
        }),
        false,
      );

      const fn = LLVMFunction.Create(
        fnType,
        LLVMFunction.LinkageTypes.ExternalLinkage,
        fnName,
        module,
      );

      for (const expr of exprs) {
        const [command, ...args] = expr;
        expectSymbol(command);

        if (command === 'llvm/insert-point') {
          const [entryName] = expectArgsLength(1, args, command);
          expectString(entryName);

          builder.SetInsertPoint(
            BasicBlock.Create(context, getStringValue(entryName), fn),
          );
        } else if (command === 'llvm/define-string-ptr') {
          const [stringName, stringContent] = expectArgsLength(
            2,
            args,
            command,
          );
          expectSymbol(stringName);
          expectString(stringContent);

          values[stringName] = builder.CreateGlobalStringPtr(
            getStringValue(stringContent),
            stringName,
            0,
            module,
          );
        } else if (command === 'llvm/call') {
          const [fnName, argValues] = expectArgsLength(2, args, command);
          expectSymbol(fnName);
          expectList(argValues);

          const callee = module.getFunction(fnName);

          if (!callee) {
            panic(`Function ${fnName} is not defined`);
          }

          builder.CreateCall(
            callee,
            argValues.map((arg) => {
              expectSymbol(arg);

              const constant = values[arg];

              if (!constant) {
                panic(`Constant is not defined ${arg}`);
              }

              return constant;
            }),
          );
        } else if (command === 'llvm/ret') {
          const [returnExpr] = expectArgsLength(1, args, command);
          returnExpr;

          // TODO: implement returning values other than 0

          builder.CreateRet(builder.getInt32(0));
        } else {
          panic(`Unknown command: ${command}`);
        }
      }

      if (verifyFunction(fn)) {
        panic(`Function verification failed: ${fnName}`);
      }
    } else {
      panic(`Unknown command: ${command}`);
    }
  }

  if (verifyModule(module)) {
    panic(`Verifying module failed: ${module.getName()}`);
  }

  return module;
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

function expectList(expr: SExpr): asserts expr is SExpr[] {
  if (typeof expr === 'string') {
    panic('List expected, found: atom');
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
    .replace(/\\n/, '\n')
    .replace(/\\\\/, '\\');
}
