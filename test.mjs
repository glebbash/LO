// @ts-check

import { test } from "node:test";
import assert from "node:assert";
import { readFile } from "node:fs/promises";

const COMPILER_PATH = "./target/wasm32-unknown-unknown/release/lole_lisp.wasm";

const compiler = await loadWasm(await readFile(COMPILER_PATH));

test("compiles 42", async () => {
    const output = await compile(
        compiler,
        await readFile("./examples/42.lole")
    );

    const program = await loadWasm(output);
    const result = program.main();

    assert.strictEqual(result, 42);
});

test("compiles factorial", async () => {
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
    assert.deepEqual(program.sub(5, 3), 2);
});

test("compiles struct", async () => {
    const output = await compile(
        compiler,
        await readFile("./examples/struct.lole")
    );

    const program = await loadWasm(output);
    const result = program.main();

    assert.strictEqual(result, 13);
});

test("compiles globals", async () => {
    const output = await compile(
        compiler,
        await readFile("./examples/globals.lole")
    );

    const program = await loadWasm(output);
    const result = program.main();

    assert.strictEqual(result, 69);
});

test("compiles struct-ref", async () => {
    const output = await compile(
        compiler,
        await readFile("./examples/struct-ref.lole")
    );

    const program = await loadWasm(output);
    const result = program.main();

    assert.strictEqual(result, 3);
});

test("compiles enums", async () => {
    const output = await compile(
        compiler,
        await readFile("./examples/enums.lole")
    );

    const program = await loadWasm(output);
    const result = program.main();

    assert.strictEqual(result, 1);
});

test("compiles vec", async () => {
    const output = await compile(
        compiler,
        await readFile("./examples/vec.lole")
    );

    const lib = await loadWasm(output);

    const vec = lib.vec_new(4, 1);
    assert.deepEqual(vec, 0);

    lib.vec_push_u8(vec, 1);

    lib.vec_push_u8(vec, 3);
    lib.vec_push_u8(vec, 2);
    lib.vec_swap(vec, 1, 2);

    storeData(lib.memory, 1000, new Uint8Array([4, 5]));
    lib.vec_push_all(vec, 1000, 2);

    storeData(lib.memory, 1000, new Uint8Array([6]));
    lib.vec_push_all(vec, 1000, 1);

    lib.vec_push_u8(vec, 7);

    assert.strictEqual(lib.vec_get_u8(vec, 0), 1);
    assert.strictEqual(lib.vec_get_u8(vec, 1), 2);
    assert.strictEqual(lib.vec_get_u8(vec, 2), 3);
    assert.strictEqual(lib.vec_get_u8(vec, 3), 4);
    assert.strictEqual(lib.vec_get_u8(vec, 4), 5);
    assert.strictEqual(lib.vec_get_u8(vec, 5), 6);
    assert.strictEqual(lib.vec_get_u8(vec, 6), 7);
    assert.strictEqual(lib.vec_len(vec), 7);
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
        new TextEncoder().encode("   hello\nawdawdfad\naxwadada")
    );

    assert.deepEqual(program.char_at(data.ptr, data.size, 3), [1, 104]);
    assert.deepEqual(program.char_at(data.ptr, data.size, 10000), [0, 0]);

    assert.equal(program.skip_space(data.ptr, data.size, 0), 3);

    assert.deepEqual(program.char_at(data.ptr, data.size, 20), [1, 120]);
    assert.deepEqual(
        program.index_to_position(data.ptr, data.size, 20),
        [3, 2]
    );
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
