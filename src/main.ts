import * as child_process from 'child_process';
import { readFile } from 'fs/promises';
import { promisify } from 'util';

import { compile } from './compiler/compiler';
import { loadLibLLVM } from './compiler/llvm-c';
import { parse } from './parser/parser';

const exec = promisify(child_process.exec);

// TODO: consider moving to ESM modules (again): https://gist.github.com/sindresorhus/a39789f98801d908bbc7ff3ecc99d99c

async function main() {
  const args = process.argv;

  const mode = args.includes('-r') ? 'interpret' : 'compile';
  const inputFile = getArg(args, 'src') ?? 'examples/hello-world.lole';
  const outputIRFile = getArg(args, 'out-ir') ?? 'output.ll';
  const outputBinaryFile = getArg(args, 'out') ?? 'output';

  const inputFileContent = await readFile(inputFile, { encoding: 'utf-8' });
  const exprs = parse(inputFileContent);

  const llvm = loadLibLLVM();
  compile(exprs, outputIRFile, llvm);

  if (mode === 'compile') {
    await compileIR(outputIRFile, outputBinaryFile);
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

async function compileIR(llvmIRFile: string, outputBinaryFile: string) {
  await run(
    `clang-13 -O3 -o ${outputBinaryFile} ${llvmIRFile} -Wno-override-module`,
  );
}

async function interpretIR(llvmIRFile: string) {
  await run(`lli-13 ${llvmIRFile}`);
}

async function run(command: string): Promise<void> {
  const res = await exec(command);

  if (res.stderr) process.stderr.write(res.stderr);
  if (res.stdout) process.stdout.write(res.stdout);
}

main();
