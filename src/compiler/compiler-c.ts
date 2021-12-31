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
  LLVMFunction,
  LLVMIRBuilder,
  LLVMModule,
  LLVMType,
  LLVMValue,
} from './llvm-c';
import { getStringValue } from './transformers';

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

type FunctionContext = ModuleContext & { fn: LLVMFunction };

export async function compile(llvm: LibLLVM, exprs: SExpr[]): Promise<string> {
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

  // if (command === 'fn') {
  //   return buildFunction(command, args, ctx);
  // }

  // return buildValue(expr, ctx);

  panic('//TODO: implement this');
}

// function buildFunction(
//   command: string,
//   args: SExpr[],
//   moduleCtx: ModuleContext,
// ): LLVMFunction {
//   const [fnName, argTypes, returnType, ...exprs] = expectArgsLengthAtLeast(
//     3,
//     args,
//     command,
//   );
//   expectSymbol(fnName);
//   expectList(argTypes);
//   expectSymbol(returnType);

//   const fnType = FunctionType.get(
//     getType(returnType, moduleCtx.builder),
//     argTypes.map((argType) => {
//       expectSymbol(argType);

//       return getType(argType, ctx.builder);
//     }),
//     false,
//   );

//   const ctx: FunctionContext = {
//     ...moduleCtx,
//     fn: LLVMFunction.Create(
//       fnType,
//       LLVMFunction.LinkageTypes.ExternalLinkage,
//       fnName,
//       moduleCtx.module,
//     ),
//   };

//   ctx.builder.SetInsertPoint(BasicBlock.Create(ctx.context, 'entry', ctx.fn));

//   const values = exprs.map((expr) => buildValueInFunctionContext(expr, ctx));
//   insertImplicitReturnOfLastValue(values, ctx);

//   if (verifyFunction(ctx.fn)) {
//     panic(`Function verification failed: ${ctx.fn.getName()}`);
//   }

//   return ctx.fn;
// }

// function insertImplicitReturnOfLastValue(
//   values: LLVMValue[],
//   ctx: FunctionContext,
// ): ReturnInst {
//   const lastValue = values.at(-1);

//   if (lastValue instanceof ReturnInst) {
//     return lastValue;
//   }

//   const returnValue = lastValue ?? buildVoid(ctx);

//   // TODO: add check for return type (`returnValue.getType()` throws 'TypeError: Illegal invocation')
//   // if (!Type.isSameType(ctx.fn.getReturnType(), returnValue.getType())) {
//   //   panic(
//   //     `Function ${ctx.fn.getName()} must return ${ctx.fn.getReturnType()} but ${returnValue.getType()} was found`,
//   //   );
//   // }

//   return ctx.builder.CreateRet(returnValue);
// }

// function buildValueInFunctionContext(
//   expr: SExpr,
//   ctx: FunctionContext,
// ): LLVMValue {
//   const [command, ...args] = expr;
//   expectSymbol(command);

//   if (command === 'llvm/ret') {
//     const [returnExpr] = expectArgsLength(1, args, command);

//     return ctx.builder.CreateRet(buildValue(returnExpr, ctx));
//   }

//   return buildValue(expr, ctx);
// }

// function buildValue(expr: SExpr, ctx: ModuleContext): LLVMValue {
//   if (isSymbol(expr)) {
//     return buildConstant(expr, ctx);
//   }

//   if (isString(expr)) {
//     return buildString(expr, ctx);
//   }

//   expectList(expr);
//   const [command, ...args] = expr;
//   expectSymbol(command);

//   if (command === 'i32') {
//     const [value] = expectArgsLength(1, args, command);
//     expectNumber(value);

//     const i32Value = getNumberValue(value);
//     expectI32(i32Value);

//     return ctx.builder.getInt32(i32Value);
//   }

//   return buildFunctionCall(command, args, ctx);
// }

// function buildConstant(name: string, ctx: ModuleContext): LLVMValue {
//   const constant = ctx.values[name];

//   if (!constant) {
//     panic(`Constant is not defined ${name}`);
//   }

//   return constant;
// }

// function buildString(expr: string, ctx: ModuleContext): Constant {
//   return ctx.builder.CreateGlobalStringPtr(getStringValue(expr));
// }

// function buildFunctionCall(
//   fnName: string,
//   args: SExpr[],
//   ctx: ModuleContext,
// ): CallInst {
//   const callee = ctx.module.getFunction(fnName);

//   if (!callee) {
//     panic(`Function ${fnName} is not defined`);
//   }

//   return ctx.builder.CreateCall(
//     callee,
//     args.map((arg) => buildValue(arg, ctx)),
//   );
// }

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
    default:
      panic(`Unknown type: ${typeName}`);
  }
}
