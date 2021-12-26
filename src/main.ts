import * as child_process from 'child_process';
import { unlink } from 'fs/promises';
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
import { promisify } from 'util';

const exec = promisify(child_process.exec);

async function main(mode: 'compile' | 'interpret' = 'compile') {
  const context = new LLVMContext();

  const module = buildMainModule(context);

  const llvmIRFile = 'output.ll';
  module.print(llvmIRFile);

  if (mode === 'compile') {
    await compileLlvmIR(llvmIRFile, 'output');
    await unlink(llvmIRFile);
  } else {
    await interpretLlvmIR(llvmIRFile);
  }
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

async function compileLlvmIR(inputFile: string, outputFile = 'output') {
  const res = await exec(`clang-13 -O3 -o ${outputFile} ${inputFile}`);

  if (res.stderr) console.error(res.stderr.slice(0, -1));
  if (res.stdout) console.log(res.stdout.slice(0, -1));
}

async function interpretLlvmIR(inputFile: string) {
  const res = await exec(`lli-13 ${inputFile}`);

  if (res.stderr) console.error(res.stderr.slice(0, -1));
  if (res.stdout) console.log(res.stdout.slice(0, -1));
}

main('interpret');
