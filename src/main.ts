import * as child_process from 'child_process';
import {
  APInt,
  BasicBlock,
  ConstantInt,
  Function,
  FunctionType,
  IRBuilder,
  LLVMContext,
  Module,
  verifyFunction,
  verifyModule,
} from 'llvm-bindings';
import { promisify } from 'util';

const exec = promisify(child_process.exec);

async function main() {
  const context = new LLVMContext();
  const module = new Module('top', context);
  const builder = new IRBuilder(context);

  module.setTargetTriple('x86_64-pc-linux-gnu');

  // create main function
  const funcType = FunctionType.get(builder.getInt32Ty(), false);
  const mainFunc = Function.Create(
    funcType,
    Function.LinkageTypes.ExternalLinkage,
    'main',
    module,
  );
  const entry = BasicBlock.Create(context, 'entrypoint', mainFunc);
  builder.SetInsertPoint(entry);

  // // string constant
  const helloWorldStr = builder.CreateGlobalStringPtr('Hello World!');

  // create "puts" function
  const putsArgs = [builder.getInt8PtrTy()];
  const putsType = FunctionType.get(builder.getInt32Ty(), putsArgs, false);
  const putsFunc = module.getOrInsertFunction('puts', putsType);

  // invoke it
  builder.CreateCall(putsFunc, [helloWorldStr]);

  // return zero
  builder.CreateRet(ConstantInt.get(context, new APInt(32, 0)));

  if (verifyFunction(mainFunc)) {
    console.error('Verifying function failed');
    return;
  }

  if (verifyModule(module)) {
    console.error('Verifying module failed');
    return;
  }

  module.print('output.ll');

  await compileLlvmIR('output.ll', 'output');
}

async function compileLlvmIR(inputFile: string, outputFile = 'output') {
  await exec(`clang-13 -O3 -o ${outputFile} ${inputFile}`);
}

main();
