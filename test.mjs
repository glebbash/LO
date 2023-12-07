// @ts-check

import { test } from "node:test";
import assert from "node:assert";
import { open, readFile, unlink, writeFile } from "node:fs/promises";
import { WASI } from "node:wasi";
import { randomUUID } from "node:crypto";

const COMPILER_PATH = "./target/wasm32-unknown-unknown/release/lole_lisp.wasm";

const compile = await loadCompilerWithWasiAPI(COMPILER_PATH);

test("ffi, file and stdin inputs all work the same", async () => {
    const compileFuncAPI = await loadCompilerWithFuncAPI(COMPILER_PATH);
    const compileMockedStdinAPI = await loadCompilerWithWasiAPI(
        COMPILER_PATH,
        true
    );

    const output1 = await compile("./examples/test/42.lole");
    const output2 = await compileFuncAPI("./examples/test/42.lole");
    const output3 = await compileMockedStdinAPI("./examples/test/42.lole");

    assert.deepStrictEqual(output1.buffer, output2.buffer);
    assert.deepStrictEqual(output2.buffer, output3.buffer);
});

test("compiles 42", async () => {
    const output = await compile("./examples/test/42.lole");

    const program = await loadWasm(output);
    const result = program.main();

    assert.strictEqual(result, 42);
});

test("compiles 42 (v2)", async () => {
    const output = await compile("./examples/test/42.lole2");

    const program = await loadWasm(output);
    const result = program.main();

    assert.strictEqual(result, 42);
});

test("compiles factorial (v2)", async () => {
    const output = await compile("./examples/test/factorial.lole2");

    const program = await loadWasm(output);
    const result = program.factorial(5);

    assert.strictEqual(result, 120);
});

test("compiles factorial", async () => {
    const output = await compile("./examples/test/factorial.lole");

    const program = await loadWasm(output);
    const result = program.factorial(5);

    assert.strictEqual(result, 120);
});

test("compiles locals", async () => {
    const output = await compile("./examples/test/locals.lole");

    const program = await loadWasm(output);
    assert.deepEqual(program.sub(5, 3), 2);
});

test("compiles locals (v2)", async () => {
    const output = await compile("./examples/test/locals.lole2");

    const program = await loadWasm(output);
    assert.deepEqual(program.sub(5, 3), 2);
});

test("compiles import", async () => {
    const output = await compile("./examples/test/import.lole");

    const logs = [];
    const program = await loadWasm(output, {
        utils: { debug: (x) => logs.push(x) },
    });

    program.main();
    assert.deepEqual(logs, [1, 2, 3]);
});

test("compiles import (v2)", async () => {
    const output = await compile("./examples/test/import.lole2");

    const logs = [];
    const program = await loadWasm(output, {
        utils: { debug: (x) => logs.push(x) },
    });

    program.main();
    assert.deepEqual(logs, [1, 2, 3]);
});

test("compiles globals", async () => {
    const output = await compile("./examples/test/globals.lole");

    const program = await loadWasm(output);
    const result = program.main();

    assert.strictEqual(result, 69);
});

test("compiles globals (v2)", async () => {
    const output = await compile("./examples/test/globals.lole2");

    const program = await loadWasm(output);
    const result = program.main();

    assert.strictEqual(result, 69);
});

test("compiles methods (v2)", async () => {
    const output = await compile("./examples/test/methods.lole2");

    const program = await loadWasm(output);
    const result = program.main();

    assert.strictEqual(result, 25);
});

test("compiles struct", async () => {
    const output = await compile("./examples/test/struct.lole");

    const program = await loadWasm(output);
    const result = program.main();

    assert.strictEqual(result, 13);
});

test("compiles struct-ref", async () => {
    const output = await compile("./examples/test/struct-ref.lole");

    const program = await loadWasm(output);
    const result = program.main();

    assert.strictEqual(result, 3);
});

test("compiles vec", async () => {
    const output = await compile("./examples/test/vec.test.lole");

    const lib = await loadWasm(output);

    const vec = lib.vec_new(4, 1);

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

test("compiles hello world", async () => {
    const program = await compile("./examples/hello-world.lole");

    const output = await runWithTmpFile(async (stdout, stdoutFile) => {
        await runWASI(program, { stdout: stdout.fd });
        return readFile(stdoutFile, { encoding: "utf-8" });
    });

    assert.strictEqual(output, "Hello World!\n");
});

test("compiles echo", async () => {
    const program = await compile("./examples/echo.lole");

    const output = await runWithTmpFile(async (stdin, stdinFile) => {
        await writeFile(stdinFile, "abc");
        return runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, { stdin: stdin.fd, stdout: stdout.fd });
            return readFile(stdoutFile, { encoding: "utf-8" });
        });
    });

    assert.strictEqual(output, "abc");
});

test("compiles args", async () => {
    const program = await compile("./examples/test/args.test.lole");

    const output = await runWithTmpFile(async (stdout, stdoutFile) => {
        await runWASI(program, {
            stdout: stdout.fd,
            args: ["123", "456", "789"],
        });
        return readFile(stdoutFile, { encoding: "utf-8" });
    });

    assert.strictEqual(output, "123\n456\n789\n");
});

test("compiles cat", async () => {
    const program = await compile("./examples/cat.lole");

    const output = await runWithTmpFile(async (stdout, stdoutFile) => {
        await runWASI(program, {
            stdout: stdout.fd,
            args: ["args.lole", "examples/test/42.lole"],
            preopens: { ".": "." },
        });
        return readFile(stdoutFile, { encoding: "utf-8" });
    });

    assert.strictEqual(
        output,
        await readFile("examples/test/42.lole", "utf-8")
    );
});

test("compiles string-pooling", async () => {
    const program = await compile("./examples/test/string-pooling.lole");

    const output = await runWithTmpFile(async (stdout, stdoutFile) => {
        await runWASI(program, { stdout: stdout.fd });
        return readFile(stdoutFile, { encoding: "utf-8" });
    });

    assert.strictEqual(output, "108\n");
});

test("compiles struct-in-struct", async () => {
    const program = await compile("./examples/test/struct-in-struct.lole");

    const output = await runWithTmpFile(async (stdout, stdoutFile) => {
        await runWASI(program, { stdout: stdout.fd });
        return readFile(stdoutFile, { encoding: "utf-8" });
    });

    assert.strictEqual(output, "3\n3\n3\n3\n3\n3\n3\n");
});

test("compiles nested-if-break", async () => {
    const program = await compile("./examples/test/nested-if-break.lole");

    const output = await runWithTmpFile(async (stdout, stdoutFile) => {
        await runWASI(program, { stdout: stdout.fd });
        return readFile(stdoutFile, { encoding: "utf-8" });
    });

    assert.strictEqual(output, "all good\n");
});

test("compiles heap-alloc", async () => {
    const program = await compile("./examples/test/heap-alloc.lole");

    const output = await runWithTmpFile(async (stdout, stdoutFile) => {
        await runWASI(program, { stdout: stdout.fd });
        return readFile(stdoutFile, { encoding: "utf-8" });
    });

    assert.strictEqual(
        output,
        dropPadding(`
            Heap/TOTAL_ALLOCATED = 1048576
            p = (Heap/alloc 1) // 1048589
            (Heap/free p)
            p = (Heap/alloc 1) // 1048589
            p = (Heap/alloc 1) // 1048606
        `)
    );
});

test("compiles defer", async () => {
    const program = await compile("./examples/test/defer.lole");

    const output = await runWithTmpFile(async (stdout, stdoutFile) => {
        await runWASI(program, { stdout: stdout.fd });
        return readFile(stdoutFile, { encoding: "utf-8" });
    });

    assert.strictEqual(
        output,
        dropPadding(`
            defer(scope1): 3
            defer(scope1): 2
            defer(scope1): 1
            -------------
            defer(scope2): 2
            defer(scope2): 1
            -------------
            defer(scope2): 3
            defer(scope2): 2
            defer(scope2): 1
            -------------
            defer(return): 3
            defer(return): 2
            defer(return): 1
            `)
    );
});

test("compiles minify", async () => {
    const testSource = `
        ; std + wasi
        (mod lib/cli)

        (fn main [] void (
            (puts "Hello World!\n")
        ))
        `;

    const program = await compile("./examples/minify.lole");

    const output = await runWithTmpFile(async (stdin, stdinFile) => {
        await writeFile(stdinFile, testSource);
        return runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, { stdin: stdin.fd, stdout: stdout.fd });
            return readFile(stdoutFile, { encoding: "utf-8" });
        });
    });

    assert.strictEqual(
        output,
        "(mod lib/cli) (fn main () void ((puts Hello World!\n)))\n"
    );
});

test("compiles minify (using file input)", async () => {
    const program = await compile("./examples/minify.lole");

    const output = await runWithTmpFile(async (stdout, stdoutFile) => {
        await runWASI(program, {
            stdout: stdout.fd,
            args: ["minify.lole", "test/42.lole"],
            preopens: { ".": "examples" },
        });
        return readFile(stdoutFile, { encoding: "utf-8" });
    });

    assert.strictEqual(
        output,
        "(export main :as main) (fn main () u32 ((return 42)))\n"
    );
});

// utils

/**
 * @param {string} compilerPath
 * @returns {Promise<(sourcePath: string) => Promise<Uint8Array>>}
 */
async function loadCompilerWithFuncAPI(compilerPath) {
    const compiler = await loadWasm(await readFile(compilerPath), {
        wasi_snapshot_preview1: new Proxy({}, { get: () => () => 0 }),
    });

    return async (sourcePath) => {
        const source = await readFile(sourcePath);
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
    };
}

/**
 * @param {string} compilerPath
 * @returns {Promise<(sourcePath: string) => Promise<Promise<Buffer>>>}
 */
async function loadCompilerWithWasiAPI(compilerPath, mockStdin = false) {
    const mod = await WebAssembly.compile(await readFile(compilerPath));

    /**
     * @param {string} [fileName]
     * @param {number} [stdinFd]
     */
    const compile = (fileName, stdinFd) =>
        runWithTmpFile(async (stderr, stderrFile) =>
            runWithTmpFile(async (stdout, stdoutFile) => {
                const wasi = new WASI({
                    // @ts-ignore
                    version: "preview1",
                    stdin: stdinFd,
                    stdout: stdout.fd,
                    stderr: stderr.fd,
                    args: [
                        "compiler.wasm",
                        ...(fileName !== undefined ? [fileName] : []),
                    ],
                    preopens: { ".": "examples" },
                });

                const instance = await WebAssembly.instantiate(
                    mod,
                    // @ts-ignore
                    wasi.getImportObject()
                );

                const exitCode = /** @type {unknown} */ (wasi.start(instance));

                if (exitCode ?? 0 !== 0) {
                    throw new Error(await readFile(stderrFile, "utf-8"));
                }

                return readFile(stdoutFile);
            })
        );

    /**
     * @param {string} sourcePath
     */
    return (sourcePath) => {
        if (!mockStdin) {
            sourcePath = sourcePath.slice("./examples/".length);
            return compile(sourcePath);
        }

        return runWithTmpFile(async (stdin, stdinFile) => {
            await writeFile(stdinFile, await readFile(sourcePath));

            return compile(undefined, stdin.fd);
        });
    };
}

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
 * @param {BufferSource} data
 * @param {import("node:wasi").WASIOptions} [wasiOptions]
 */
async function runWASI(data, wasiOptions) {
    // @ts-ignore
    const wasi = new WASI({ version: "preview1", ...wasiOptions });

    const wasm = await WebAssembly.compile(data);
    const instance = await WebAssembly.instantiate(
        wasm,
        // @ts-ignore
        wasi.getImportObject()
    );

    wasi.start(instance);
}

/**
 * @template T
 * @param {(file: import("node:fs/promises").FileHandle, fileName: string) => T} run
 */
async function runWithTmpFile(run) {
    const mockOutputFileName = `${randomUUID()}.tmp`;
    const mockOutputFile = await open(mockOutputFileName, "w+");

    try {
        return await run(mockOutputFile, mockOutputFileName);
    } finally {
        await mockOutputFile.close();
        await unlink(mockOutputFileName);
    }
}

function dropPadding(/** @type {string} */ str) {
    return str
        .slice(1)
        .split("\n")
        .map((s) => s.trimStart())
        .join("\n");
}
