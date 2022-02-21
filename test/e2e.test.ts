import { assertEquals } from "https://deno.land/std@0.123.0/testing/asserts.ts";
import { m } from "https://raw.githubusercontent.com/glebbash/multiline-str/master/src/multiline-str.ts";

const expectedResults = [
  {
    program: "examples/cmd-args.lole",
    output: m`
      Program name: -

      `,
  },
  {
    program: "examples/fn.lole",
    output: m`
      Number: 4

      `,
  },
  {
    program: "examples/get_set.lole",
    output: m`
      Pointer value: 1
      Pointer value: 2
      Pointer value in function: 2
      Bob's id is 1
      First element: 1
      First element with helper: 1
      Bottom-right matrix element: 9

      `,
  },
  {
    program: "examples/hello-world.lole",
    output: m`
      Hello World!

      `,
  },
  {
    program: "examples/nested-if.lole",
    output: m`
      cmp: f

      `,
  },
  {
    program: "examples/parse-file.lole",
    output: m`
      (include "./lib/file.lole")

      (external-fn puts ((& i8)) void)
      (external-fn strtok ((& i8) (& i8)) (& i8))

      (fn main () i32
        (let source (read_file "examples/parse-file.lole"))
        (puts source)

        (puts "----------------------------")

        (puts "first line:")
        (let first_line (strtok source "\\n"))
        (puts first_line)

        (puts "second line:")
        (let second_line (strtok (nullptr (& i8)) "\\n"))
        (puts second_line)

        0
      )

      ----------------------------
      first line:
      (include "./lib/file.lole")
      second line:
      (external-fn puts ((& i8)) void)

      `,
  },
  {
    program: "examples/rec.lole",
    output: m`
      1
      2
      3
      4
      5
      6
      7
      8
      9
      10

      `,
  },
  {
    program: "examples/struct.lole",
    output: m`
      1
      2
      3

      `,
  },
];

for (const { program, output } of expectedResults) {
  Deno.test(`it executes ${program}`, async () => {
    const programOutput = await runProgram(program);
    assertEquals(programOutput, output);
  });
}

async function runProgram(path: string): Promise<string> {
  const program = Deno.run({
    cmd: [
      "deno",
      "run",
      "--allow-all",
      "--unstable",
      "src/main.ts",
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
