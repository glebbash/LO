import { LLVM } from "../llvm-c-14/llvm-c/mod.ts";
import { Pointer } from "../llvm-c-14/llvm-c/safe-ffi.ts";
import { SExpr } from "../parser/parser.ts";
import { buildValueInModuleContext } from "./constructs/mod.ts";
import { defineDefaultTypes } from "./types.ts";
import { allocPtr, derefRef, readCString, toCString } from "./utils.ts";

export type ModuleContext = {
  llvm: typeof LLVM;
  context: LLVM.ContextRef;
  builder: LLVM.BuilderRef;
  module: LLVM.ModuleRef;
  moduleName: string;
  values: Record<string, LLVM.ValueRef>;
  types: Record<string, LLVM.TypeRef>;
  dispose: () => void;
};

export const VERIFICATION_ENABLED = false;

export function compileToModule(
  exprs: SExpr[],
  llvm: typeof LLVM,
): ModuleContext {
  const ctx = createContext("main", llvm);

  for (const expr of exprs) {
    buildValueInModuleContext(expr, ctx);
  }

  verifyModule(ctx);

  return ctx;
}

export async function compileIR(llvmIR: string, outputBinaryFile: string) {
  const clang = Deno.run({
    cmd: [
      "clang-14",
      "-O3",
      "-o",
      outputBinaryFile,
      "-Wno-override-module",
      "-x",
      "ir",
      "-",
    ],
    stdin: "piped",
  });
  clang.stdin?.write(new TextEncoder().encode(llvmIR));
  clang.stdin.close();
  await clang.status();
}

export function interpret(ctx: ModuleContext) {
  const { llvm } = ctx;

  llvm.LinkInMCJIT();
  llvm.InitializeX86TargetInfo();
  llvm.InitializeX86Target();
  llvm.InitializeX86TargetMC();
  llvm.InitializeX86AsmPrinter();

  const engineRefRef = allocPtr<LLVM.ExecutionEngineRef>();
  const errorRef = allocPtr<Pointer<number>>();
  const res = llvm.CreateExecutionEngineForModule(
    engineRefRef,
    ctx.module,
    errorRef,
  );

  if (res !== 0) {
    throw new Error(
      "Failed to create execution engine: " + readCString(errorRef),
    );
  }

  const engineRef = derefRef(engineRefRef);

  const fnPtr = llvm.GetFunctionAddress(engineRef, toCString("main"));

  const fn = new Deno.UnsafeFnPointer(fnPtr, {
    parameters: [],
    result: "i32",
  });

  fn.call();

  llvm.DisposeExecutionEngine(engineRef);
}

function createContext(moduleName: string, llvm: typeof LLVM): ModuleContext {
  const context = llvm.ContextCreate();
  const builder = llvm.CreateBuilderInContext(context);
  const module = llvm.ModuleCreateWithNameInContext(
    toCString(moduleName),
    context,
  );

  const ctx: ModuleContext = {
    llvm,
    context,
    builder,
    moduleName,
    module,
    values: {},
    types: {},
    dispose: () => {
      llvm.DisposeBuilder(builder);
      llvm.DisposeModule(module);
      llvm.ContextDispose(context);
    },
  };

  defineDefaultTypes(ctx);

  return ctx;
}

export function buildLLVMIR(ctx: ModuleContext): string {
  const { llvm } = ctx;

  const messageRef = llvm.PrintModuleToString(ctx.module);
  const llvmIR = readCString(messageRef);
  llvm.DisposeMessage(messageRef);

  return llvmIR;
}

function verifyModule(ctx: ModuleContext) {
  const { llvm } = ctx;

  if (VERIFICATION_ENABLED) {
    const messageRef = allocPtr<Pointer<number>>();
    const res = llvm.VerifyModule(
      ctx.module,
      llvm.VerifierFailureAction.LLVMReturnStatusAction,
      messageRef,
    );

    if (res !== 0) {
      console.error(readCString(messageRef));
      throw new Error(`Verifying module failed: ${ctx.moduleName}`);
    }
  }
}
