const data = Deno.readTextFileSync("symbols.txt");
const lines = data.split("\n");
const usefulLines = lines.slice(4, -1);

const publicSymbols = usefulLines.filter((l) => !getName(l).startsWith("_"));

function getName(line: string): string {
  return line.slice(
    "   641: 000000000146c580   666 FUNC    GLOBAL DEFAULT   13 ".length,
  )!;
}

const allFunctions = publicSymbols
  .filter((s) => getName(s).startsWith("LLVM"))
  .filter((s) => s.includes(" FUNC "))
  .filter((s) => !s.includes(".localalias"))
  .map((s) => getName(s).replace("@@LLVM_15", ""));

const uniqueFunctions = Array.from(new Set(allFunctions)).sort();

Deno.writeTextFileSync(
  "llvm-c-functions.ts",
  `export const LLVM_C_FUNCTIONS = ${
    JSON.stringify(uniqueFunctions, null, 2)
  };`,
);
