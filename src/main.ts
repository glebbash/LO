import { parse } from "https://deno.land/std@0.149.0/flags/mod.ts";

import { mainLLVM2 } from "./main-llvm.ts";
import { mainWasm } from "./main-wasm.ts";

function main() {
  const parsed = parse(Deno.args);

  switch (parsed.target) {
    case undefined:
      return mainLLVM2(parsed);
    case "wasm":
      return mainWasm(parsed);
    default:
      throw new Error(`Unknown target: ${parsed.target}`);
  }
}

main();
