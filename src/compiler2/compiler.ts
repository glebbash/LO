import { LLVM } from "../../ffigen/llvm-c/mod.ts";
import { Pointer } from "../../ffigen/llvm-c/safe-ffi.ts";
import { SExpr } from "../parser/parser.ts";
// import { buildValueInModuleContext } from "./constructs/mod.ts";
// import { defineDefaultTypes } from "./types.ts";

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

  // TODO: implement
  // for (const expr of exprs) {
  //   buildValueInModuleContext(expr, ctx);
  // }

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

  // TODO: implement this
  // const moduleCtx = compileToModule(expandFile(inputFile), llvm);

  // const { ok, message, engine } = llvm.createExecutionEngineForModule(
  //   moduleCtx.module,
  // );

  // if (!ok) {
  //   console.error("Error during JIT compilation:", message);
  //   Deno.exit(1);
  // }

  // const fn = llvm.GetFunctionAddress(engine, "main", {
  //   parameters: [],
  //   result: "i32",
  // });

  // fn.call();

  // llvm.DisposeExecutionEngine(engine);
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

  // TODO: implement
  // defineDefaultTypes(ctx);

  return ctx;
}

export function buildLLVMIR(ctx: ModuleContext): string {
  const { llvm } = ctx;

  const messageRef = llvm.PrintModuleToString(ctx.module);
  const llvmIR = readCString(messageRef);
  // TODO(ffigen): update type casts here
  llvm.DisposeMessage(messageRef as unknown as Pointer<"Message">);

  return llvmIR;
}

function verifyModule(ctx: ModuleContext) {
  const { llvm } = ctx;

  if (VERIFICATION_ENABLED) {
    const messageRef = allocPtr<"OutMessage">();
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

function allocPtr<T extends string>(): Pointer<T> {
  return Deno.UnsafePointer.of(new BigUint64Array(1)) as Pointer<T>;
}

function readCString(message: bigint) {
  return new Deno.UnsafePointerView(message).getCString();
}

// TODO(ffigen): update type casts here
function toCString<T extends string>(str: string): Pointer<T> {
  return Deno.UnsafePointer.of(
    Uint8Array.from(toCharCodes(str + "\0")),
  ) as Pointer<T>;
}

function toCharCodes(str: string): number[] {
  return [...str].map((_, i) => str.charCodeAt(i));
}
