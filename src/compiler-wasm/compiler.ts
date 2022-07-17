import Context from "https://deno.land/std@0.147.0/wasi/snapshot_preview1.ts";
import { binaryen as wasm } from "./binaryen.ts";
import { SExpr } from "../parser/parser.ts";
import { buildValueInModuleContext } from "./constructs/mod.ts";

export type WasmModule = InstanceType<typeof wasm.Module>;

export type FunctionDef = {
  name: string;
  params: number;
  result: number;
};

export type ModuleContext = {
  wasm: typeof wasm;
  module: WasmModule;
  functionDefs: Record<string, FunctionDef>;
};

export function buildModule(exprs: SExpr[]): WasmModule {
  const mod = new wasm.Module();
  const ctx: ModuleContext = { module: mod, wasm, functionDefs: {} };

  for (const expr of exprs) {
    buildValueInModuleContext(expr, ctx);
  }

  mod.autoDrop();

  // Optimize using default passes and levels
  mod.optimize();

  if (!mod.validate()) {
    throw new Error("Validation error");
  }

  return mod;
}

export function emitWAT(module: WasmModule): string {
  return module.emitText();
}

export function emitWasm(module: WasmModule): Uint8Array {
  return module.emitBinary();
}

export async function interpret(module: WasmModule) {
  const wasm = module.emitBinary();

  const context = new Context({
    args: Deno.args,
    env: Deno.env.toObject(),
  });

  const compiled = await WebAssembly.compile(wasm);
  const instance = await WebAssembly.instantiate(compiled, {
    "wasi_snapshot_preview1": context.exports,
  });

  context.start(instance);
}
