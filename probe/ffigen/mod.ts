import { generateBindings } from "./generate-bindings.ts";
import { getFunctionsFromSharedLib } from "./get-functions-from-shared-lib.ts";

if (import.meta.main) {
  const [allSymbolsFile, exposedSymbolsFile, outputFolder] = Deno.args;

  const exposedFunctions = await getFunctionsFromSharedLib(
    exposedSymbolsFile,
  );

  await generateBindings(allSymbolsFile, exposedFunctions, outputFolder);
}
