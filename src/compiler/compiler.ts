import { SExpr } from "../parser/parser.ts";
import { buildValueInModuleContext } from "./constructs/mod.ts";
import {
  LibLLVM,
  LLVMContext,
  LLVMIRBuilder,
  LLVMModule,
  LLVMType,
  LLVMValue,
  loadLibLLVM,
} from "../llvm/llvm-c.ts";
import { defineDefaultTypes } from "./types.ts";

export type ModuleContext = {
  llvm: LibLLVM;
  context: LLVMContext;
  builder: LLVMIRBuilder;
  module: LLVMModule;
  moduleName: string;
  values: Record<string, LLVMValue>;
  types: Record<string, LLVMType>;
  dispose: () => void;
};

export const VERIFICATION_ENABLED = false;

export function compileToModule(
  exprs: SExpr[],
  llvm = loadLibLLVM(),
): ModuleContext {
  const ctx = createContext("main", llvm);

  for (const expr of exprs) {
    buildValueInModuleContext(expr, ctx);
  }

  verifyModule(ctx);

  return ctx;
}

export function interpret(moduleCtx: ModuleContext) {
  const { llvm } = moduleCtx;

  llvm.linkInMCJIT();
  llvm.initializeX86TargetInfo();
  llvm.initializeX86Target();
  llvm.initializeX86TargetMC();
  llvm.initializeX86AsmPrinter();

  const { ok, message, engine } = llvm.createExecutionEngineForModule(
    moduleCtx.module,
  );

  if (!ok) {
    console.error("Error during JIT compilation:", message);
    Deno.exit(1);
  }

  const fn = llvm.getFunctionAddress(engine, "main", {
    parameters: [],
    result: "i32",
  });

  fn.call();

  // TODO: check if this is necessary
  // llvm.removeModule(engine, moduleCtx.module);
  llvm.disposeExecutionEngine(engine);
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

function createContext(moduleName: string, llvm: LibLLVM): ModuleContext {
  const context = llvm.contextCreate();
  const builder = llvm.createBuilderInContext(context);
  const module = llvm.moduleCreateWithNameInContext(moduleName, context);

  const ctx: ModuleContext = {
    llvm,
    context,
    builder,
    moduleName,
    module,
    values: {},
    types: {},
    dispose: () => {
      llvm.disposeBuilder(builder);
      llvm.disposeModule(module);
      llvm.contextDispose(context);
    },
  };

  defineDefaultTypes(ctx);

  return ctx;
}

export function buildLLVMIR(ctx: ModuleContext): string {
  const { llvm } = ctx;

  const message = llvm.printModuleToString(ctx.module);
  const llvmIR = message.stringValue();
  llvm.disposeMessage(message);

  return llvmIR;
}

function verifyModule(ctx: ModuleContext) {
  const { llvm } = ctx;

  if (VERIFICATION_ENABLED) {
    const res = llvm.verifyModule(ctx.module);

    if (!res.ok) {
      console.error(res.message);
      throw new Error(`Verifying module failed: ${ctx.moduleName}`);
    }
  }
}
