import { assertEquals } from "https://deno.land/std@0.123.0/testing/asserts.ts";

const exampleProgramsDir = "examples/wasm/";
const expectedOutputsDir = "examples/wasm/_expected_outputs";

for await (const test of Deno.readDir(expectedOutputsDir)) {
  if (!test.isFile) continue;

  const expectedOutputFile = `${expectedOutputsDir}/${test.name}`;
  const program = exampleProgramsDir + test.name.slice(0, -".txt".length);

  Deno.test(`it executes ${program}`, async () => {
    const programOutput = await runProgram(program);
    const expectedOutput = await Deno.readTextFile(expectedOutputFile);
    assertEquals(programOutput, expectedOutput);
  });
}

async function runProgram(path: string): Promise<string> {
  const program = Deno.run({
    cmd: [
      "deno",
      "run",
      "-A",
      "--unstable",
      "src/main-wasm.ts",
      `--src=${path}`,
      "-r",
    ],
    stdout: "piped",
  });

  await program.status();

  const output = await program.output();
  program.close();

  return new TextDecoder().decode(output);
}
