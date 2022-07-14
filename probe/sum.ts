import { loadLibLLVM } from "./src/llvm/llvm-c.ts";

const llvm = loadLibLLVM();

const mod = llvm.moduleCreateWithName("my_module");

const paramTypes = [llvm.i32Type(), llvm.i32Type()];
const retType = llvm.functionType(
  llvm.i32Type(),
  paramTypes,
  false,
);
const sum = llvm.addFunction(mod, "sum", retType);

const entry = llvm.appendBasicBlock(sum, "entry");

const builder = llvm.createBuilder();
llvm.positionBuilderAtEnd(builder, entry);
const tmp = llvm.buildAdd(
  builder,
  llvm.getParam(sum, 0),
  llvm.getParam(sum, 1),
  "tmp",
);
llvm.buildRet(builder, tmp);

const { ok: moduleCreated, message: moduleCreationErrorMessage } = llvm
  .verifyModule(mod);
if (!moduleCreated) {
  console.error(moduleCreationErrorMessage);
}

llvm.dumpModule(mod);

llvm.linkInMCJIT();
llvm.initializeX86Target();
llvm.initializeX86AsmPrinter();

const { ok, message, engine } = llvm.createExecutionEngineForModule(mod);
if (!ok) {
  console.error("Error during JIT compilation:", message);
  Deno.exit(1);
}

// const { ok: functionFound, fn } = llvm
//   .findFunction(engine, "sum");

// if (!functionFound) {
//   console.error("Function not found");
//   Deno.exit(1);
// }

// console.log(fn);

const fn = llvm.getFunctionAddress(
  engine,
  "sum",
  {
    parameters: ["i32", "i32"],
    result: "i32",
  } as const,
);

console.log(fn);

const res = fn.call(34, 35);
console.log(res);

llvm.disposeBuilder(builder);
llvm.disposeExecutionEngine(engine);
