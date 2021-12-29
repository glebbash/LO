import {
  BasicBlock,
  Function as LLVMFunction,
  FunctionCallee,
  FunctionType,
  IRBuilder,
  LLVMContext,
  Module,
  verifyFunction,
  verifyModule,
} from 'llvm-bindings';

import { SExpr } from './parser';

export function compile(exprs: SExpr[], outputIRFile: string) {
  const context = new LLVMContext();

  const module = buildMainModule(context);

  module.print(outputIRFile);
}

function buildMainModule(context: LLVMContext): Module {
  const module = new Module('top', context);
  const builder = new IRBuilder(context);

  module.setTargetTriple('x86_64-pc-linux-gnu');

  buildMainFunction(context, module, builder);

  if (verifyModule(module)) {
    console.error('Verifying module failed');
    process.exit(1);
  }

  return module;
}

function buildMainFunction(
  context: LLVMContext,
  module: Module,
  builder: IRBuilder,
): LLVMFunction {
  const funcType = FunctionType.get(builder.getInt32Ty(), false);
  const mainFunc = LLVMFunction.Create(
    funcType,
    LLVMFunction.LinkageTypes.ExternalLinkage,
    'main',
    module,
  );

  const entry = BasicBlock.Create(context, 'entrypoint', mainFunc);
  builder.SetInsertPoint(entry);

  // string constant
  const helloWorldStr = builder.CreateGlobalStringPtr('Hello World!');

  const putsFunc = defineExternPuts(builder, module);
  builder.CreateCall(putsFunc, [helloWorldStr]);

  // return zero
  builder.CreateRet(builder.getInt32(0));

  if (verifyFunction(mainFunc)) {
    console.error('Verifying function failed');
    process.exit(1);
  }

  return mainFunc;
}

function defineExternPuts(builder: IRBuilder, module: Module): FunctionCallee {
  const putsArgs = [builder.getInt8PtrTy()];
  const putsType = FunctionType.get(builder.getInt32Ty(), putsArgs, false);
  return module.getOrInsertFunction('puts', putsType);
}
