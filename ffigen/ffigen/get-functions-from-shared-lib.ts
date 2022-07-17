export async function getFunctionsFromSharedLib(
  exposedSymbolsFile: string,
): Promise<string[]> {
  const output = await Deno.readTextFile(exposedSymbolsFile);

  const lines = output.split("\n");
  const usefulLines = lines.slice(4, -1);

  const publicSymbols = usefulLines.filter((l) => !getName(l).startsWith("_"));

  const allFunctions = publicSymbols
    .filter((s) => getName(s).startsWith("LLVM"))
    .filter((s) => s.includes(" FUNC "))
    .filter((s) => !s.includes(".localalias"))
    .map((s) => getName(s).replace("@@LLVM_15", ""));

  return Array.from(new Set(allFunctions)).sort();
}

function getName(line: string): string {
  return line.slice(
    "   641: 000000000146c580   666 FUNC    GLOBAL DEFAULT   13 ".length,
  )!;
}
