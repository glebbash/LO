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
};

export const VERIFICATION_ENABLED = false;

export function compile(
  exprs: SExpr[],
  llvm = loadLibLLVM(),
): string {
  const ctx = compileToModule(exprs, llvm);
  const llvmIR = buildLLVMIR(ctx);

  disposeContext(ctx);

  return llvmIR;
}

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
  };

  defineDefaultTypes(ctx);

  return ctx;
}

export function disposeContext(ctx: ModuleContext): void {
  const { llvm } = ctx;

  llvm.disposeModule(ctx.module);
  llvm.disposeBuilder(ctx.builder);
  llvm.contextDispose(ctx.context);
  llvm.close();
}

function buildLLVMIR(ctx: ModuleContext): string {
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
