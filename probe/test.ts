const data = Deno.readTextFileSync("symbols.txt");
const lines = data.split("\n");
const usefulLines = lines.slice(4, -1);

// const llvmLines = lines.slice(4, -1).filter(line => line.replace(/\s+/, " ").split(" ").at(-2).includes("LLVM"))

const publicSymbols = usefulLines.filter((l) => !getName(l).startsWith("_"));

// console.log(getName(lines[640]));
console.log(publicSymbols.filter((s) => getName(s).startsWith("LLVM")));
// console.log(publicSymbols.filter((s) => s.includes("llvm")));
// console.log(getName(usefulLines[0]));

function getName(line: string): string {
  return line.slice(
    "   641: 000000000146c580   666 FUNC    GLOBAL DEFAULT   13 ".length,
  )!;
}

Deno.writeTextFileSync(
  "llvmc-symbols.txt",
  publicSymbols.filter((s) => getName(s).startsWith("LLVM")).map((s) =>
    getName(s).replace("@@LLVM_14", "")
  ).sort().join("\n"),
);
