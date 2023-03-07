// @ts-check

import { test } from "node:test";
import assert from "node:assert";
import { readFile } from "node:fs/promises";

const COMPILER_PATH = "./target/wasm32-unknown-unknown/release/lole_lisp.wasm";

const compiler = await loadWasm(await readFile(COMPILER_PATH));

test("compiles 42 example", async () => {
    const { ok, output } = await compile(
        compiler,
        await readFile("./examples/42.lole")
    );

    assert.strictEqual(ok, 1);

    const program = await loadWasm(output);
    const result = program.main();

    assert.strictEqual(result, 42);
});

test("compiles factorial example", async () => {
    const { ok, output } = await compile(
        compiler,
        await readFile("./examples/factorial.lole")
    );

    assert.strictEqual(ok, 1);

    const program = await loadWasm(output);
    const result = program.factorial(5);

    assert.strictEqual(result, 120);
});

// utils

/**
 * @param {BufferSource} data
 * @returns {Promise<any>}
 */
async function loadWasm(data) {
    const mod = await WebAssembly.instantiate(data);
    return mod.instance.exports;
}

/**
 * @param {any} compiler
 * @param {Buffer} source
 */
async function compile(compiler, source) {
    const inSize = source.byteLength;
    const inPtr = compiler.mem_alloc(inSize);

    const input = new Uint8Array(compiler.memory.buffer, inPtr, inSize);
    input.set(source);

    const [ok, outPtr, outSize] = compiler.compile(inPtr, inSize);

    const output = new Uint8Array(outSize);
    output.set(new Uint8Array(compiler.memory.buffer, outPtr, outSize));

    compiler.mem_free(inPtr, inSize);
    compiler.mem_free(outPtr, outSize);

    return { ok, output };
}
