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

test("compiles import", async () => {
    const output = await compile(
        compiler,
        await readFile("./examples/import.lole")
    );

    const logs = [];
    const program = await loadWasm(output, {
        utils: { debug: (x) => logs.push(x) },
    });

    program.main();
    assert.deepEqual(logs, [1, 2, 3]);
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

    const parser = await loadWasm(output);

    {
        const text = new TextEncoder().encode("   hello\nawdawdfad\naxwadada");
        const data = storeData(
            parser.memory,
            parser.alloc(text.byteLength),
            text
        );

        assert.deepEqual(parser.char_at(data.ptr, data.size, 3), [1, 104]);
        assert.deepEqual(parser.char_at(data.ptr, data.size, 10000), [0, 0]);

        assert.equal(parser.skip_space(data.ptr, data.size, 0), 3);

        assert.deepEqual(parser.char_at(data.ptr, data.size, 20), [1, 120]);
        assert.deepEqual(
            parser.index_to_position(data.ptr, data.size, 20),
            [3, 2]
        );
    }

    {
        const [ok, index, expr_ref] = await parseExpr(parser, "abc");
        assert.deepEqual([ok, index], [1, 3]);

        const mem = parser.memory.buffer;

        const [expr_type, atom_ref] = u32s(mem, expr_ref, 2);
        assert.equal(expr_type, 0); // atom

        const [len, cap, item_size, chars_ref] = u32s(mem, atom_ref, 4);
        assert.deepEqual([len, cap, item_size], [3, 6, 1]);

        const chars = new Uint8Array(mem, chars_ref, len);
        assert.deepEqual(chars, new TextEncoder().encode("abc"));
    }

    {
        const [ok, index, expr_ref] = await parseExpr(parser, "(a)");
        assert.deepEqual([ok, index], [1, 3]);

        const mem = parser.memory.buffer;

        const [expr_type, atom_ref] = u32s(mem, expr_ref, 2);
        assert.equal(expr_type, 1); // list

        const [len, cap, item_size, _exprs_ref] = u32s(mem, atom_ref, 4);
        assert.deepEqual([len, cap, item_size], [1, 6, 8]);
    }

    {
        const res = await parseAll(
            parser,
            await readFile("examples/parser.lole", { encoding: "utf8" })
        );
        assert.deepEqual(res, [1, 9616, 9781]);
    }

    async function parseAll(parser, text) {
        const bytes = new TextEncoder().encode(text);
        const data_ref = parser.alloc(bytes.byteLength);
        const data = storeData(parser.memory, data_ref, bytes);
        return await parser.parse(data.ptr, data.size);
    }

    async function parseExpr(parser, text) {
        const bytes = new TextEncoder().encode(text);
        const data_ref = parser.alloc(bytes.byteLength);
        const data = storeData(parser.memory, data_ref, bytes);
        return await parser.parse_expr(data.ptr, data.size);
    }
});

// utils

/**
 * @param {BufferSource} data
 * @param {WebAssembly.Imports} [imports]
 * @returns {Promise<any>}
 */
async function loadWasm(data, imports) {
    const mod = await WebAssembly.instantiate(data, imports);
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

/**
 * @param {ArrayBufferLike} buff
 * @param {number} offset
 * @param {number} length
 */
function u32s(buff, offset, length) {
    return new Uint32Array(buff.slice(offset), 0, length);
}
