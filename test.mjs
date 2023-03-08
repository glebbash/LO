// @ts-check

import { test } from "node:test";
import assert from "node:assert";
import { readFile } from "node:fs/promises";

const COMPILER_PATH = "./target/wasm32-unknown-unknown/release/lole_lisp.wasm";

const compiler = await loadWasm(await readFile(COMPILER_PATH));

test("compiles 42 example", async () => {
    const output = await compile(
        compiler,
        await readFile("./examples/42.lole")
    );

    const program = await loadWasm(output);
    const result = program.main();

    assert.strictEqual(result, 42);
});

test("compiles factorial example", async () => {
    const output = await compile(
        compiler,
        await readFile("./examples/factorial.lole")
    );

    const program = await loadWasm(output);
    const result = program.factorial(5);

    assert.strictEqual(result, 120);
});

test("compiles locals", async () => {
    const output = await compile(
        compiler,
        await readFile("./examples/locals.lole")
    );

    const program = await loadWasm(output);
    assert.deepEqual(program.add(2, 2), 4);
});

test("compiles parser", async () => {
    const output = await compile(
        compiler,
        await readFile("./examples/parser.lole")
    );

    const program = await loadWasm(output);

    const data = storeData(
        program.memory,
        0,
        new TextEncoder().encode("hello")
    );

    assert.deepEqual(program.char_at(data.ptr, data.size, 0), [1, 104]);
    assert.deepEqual(program.char_at(data.ptr, data.size, 5), [0, 0]);
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
    const src = storeData(
        compiler.memory,
        compiler.mem_alloc(source.byteLength),
        source
    );

    const [ok, outPtr, outSize] = compiler.compile(src.ptr, src.size);

    const output = new Uint8Array(outSize);
    output.set(new Uint8Array(compiler.memory.buffer, outPtr, outSize));

    compiler.mem_free(src.ptr, src.size);
    compiler.mem_free(outPtr, outSize);

    if (!ok) {
        throw new Error(new TextDecoder().decode(output));
    }

    return output;
}

/**
 * @param {{buffer: ArrayBufferLike;}} memory
 * @param {number} ptr
 * @param {Uint8Array} data
 */
function storeData(memory, ptr, data) {
    const region = { ptr, size: data.byteLength };

    new Uint8Array(memory.buffer, region.ptr, region.size).set(data);

    return region;
}
