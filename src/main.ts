import * as child_process from 'child_process';
import { readFile, unlink } from 'fs/promises';
import { promisify } from 'util';

import { compile } from './compiler';
import { parse } from './parser';

const exec = promisify(child_process.exec);

async function main() {
  const args = process.argv;

  const mode = args.includes('-r') ? 'interpret' : 'compile';
  const inputFile = getArg(args, 'src') ?? 'input.lll';
  const outputIRFile = getArg(args, 'out-ir') ?? 'output.ll';
  const outputBinaryFile = getArg(args, 'out') ?? 'output';

  const inputFileContent = await readFile(inputFile, { encoding: 'utf-8' });
  const exprs = parse(inputFileContent);

  compile(exprs, outputIRFile);

  if (mode === 'compile') {
    await compileIR(outputIRFile, outputBinaryFile);
    await unlink(outputIRFile);
  } else {
    await interpretIR(outputIRFile);
  }
}

function getArg(args: string[], name: string): string | undefined {
  const argumentStart = `--${name}=`;

  return args
    .find((a) => a.startsWith(argumentStart))
    ?.slice(argumentStart.length);
}

async function compileIR(inputFile: string, outputFile = 'output') {
  await run(`clang-13 -O3 -o ${outputFile} ${inputFile}`);
}

async function interpretIR(inputFile: string) {
  await run(`lli-13 ${inputFile}`);
}

async function run(command: string): Promise<void> {
  const res = await exec(command);

  if (res.stderr) console.error(res.stderr.slice(0, -1));
  if (res.stdout) console.log(res.stdout.slice(0, -1));
}

main();
