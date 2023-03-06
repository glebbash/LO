import { readFileSync } from 'fs'

const COMPILER_PATH = '../target/wasm32-unknown-unknown/release/lole_lisp.wasm';
const SOURCE_PATH = './42.lole';

// load compiler
const compilerData = readFileSync(COMPILER_PATH);
const compilerMod = await WebAssembly.instantiate(compilerData);
const compiler = compilerMod.instance.exports;

// allocate buffer in compiler
const source = readFileSync(SOURCE_PATH);
const size = source.byteLength;
const data = compiler.mem_alloc(size);

// copy into compiler's src buffer
const srcBuff = new Uint8Array(compiler.memory.buffer, data, size);
srcBuff.set(source);

// compile
const [outData, outSize] = compiler.compile(data, size);
const outBuff = new Uint8Array(compiler.memory.buffer, outData, outSize);

// copy out of compiler's out buffer
const programData = new Uint8Array(outSize);
programData.set(outBuff);

// cleanup
compiler.mem_free(outData, outSize);
compiler.mem_free(data, size);

/////////////////////////////////////////////

// load the compiled program
const programMod = await WebAssembly.instantiate(programData);
const program = programMod.instance.exports;

// run the program
const result = program.main();
console.log(result);
