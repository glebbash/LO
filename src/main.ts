import llvm from 'llvm-bindings';

function main(): void {
  const context = new llvm.LLVMContext();
  const module = new llvm.Module('demo', context);
  const builder = new llvm.IRBuilder(context);

  const returnType = builder.getInt32Ty();
  const paramTypes = [builder.getInt32Ty(), builder.getInt32Ty()];
  const functionType = llvm.FunctionType.get(returnType, paramTypes, false);
  const func = llvm.Function.Create(
    functionType,
    llvm.Function.LinkageTypes.ExternalLinkage,
    'add',
    module,
  );

  const entryBB = llvm.BasicBlock.Create(context, 'entry', func);
  builder.SetInsertPoint(entryBB);
  const a = func.getArg(0);
  const b = func.getArg(1);
  const result = builder.CreateAdd(a, b);
  builder.CreateRet(result);

  if (llvm.verifyFunction(func)) {
    console.error('Verifying function failed');
    return;
  }
  if (llvm.verifyModule(module)) {
    console.error('Verifying module failed');
    return;
  }
  module.print();
}

main();
