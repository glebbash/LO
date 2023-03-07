import { readFileSync } from 'fs'
import { TextEncoder } from 'util';

const COMPILER_PATH = '../target/wasm32-unknown-unknown/release/lole_lisp.wasm';
const SOURCE_PATH = './parser.lole';

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
const [ok, outData, outSize] = compiler.compile(data, size);
const outBuff = new Uint8Array(compiler.memory.buffer, outData, outSize);

if (!ok) {
    throw new Error(new TextDecoder().decode(outBuff));
}

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

// store data in memory
const str = "hello";
const strData = new TextEncoder().encode(str);
const strPtr = 0;
const strSize = strData.length;
const strBuff = new Uint8Array(program.memory.buffer, strPtr, strSize);
strBuff.set(strData);

// run the program
const result = program.char_at(strPtr, strSize, 0);
console.log(result);
