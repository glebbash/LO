export async function getFunctionsFromSharedLib(
  exposedSymbolsFile: string,
  libPrefix?: string,
): Promise<string[]> {
  const output = await Deno.readTextFile(exposedSymbolsFile);

  const lines = output.split("\n");
  const usefulLines = lines.slice(4, -1);

  const functionsOfInterest = libPrefix
    ? usefulLines.filter((def) => getName(def).startsWith(libPrefix))
    : usefulLines;

  const allFunctions = functionsOfInterest
    .filter((s) => s.includes(" FUNC "))
    .filter((s) => !s.includes(".localalias"))
    .map(getName);

  return Array.from(new Set(allFunctions)).sort();
}

function getName(line: string): string {
  return line.slice(
    "   641: 000000000146c580   666 FUNC    GLOBAL DEFAULT   13 ".length,
  )!;
}
