#!/usr/bin/env -S node --no-warnings --experimental-network-imports
// @ts-check

import { WASI } from "node:wasi";
import process from "node:process";
import { test } from "node:test";
import assert from "node:assert";
import fs from "node:fs/promises";
import crypto from "node:crypto";
// @ts-ignore
import { m } from "https://unpkg.com/multiline-str@1.0.4/esm/mod.js?module";

const COMPILER_PATH = "lo.wasm";
const TMP_DIR = "tmp";

const COMMANDS = {
    compile: compileCommand,
    run: runCommand, // compile + execute
    runWasi: runWasiCommand, // run arbitrary .wasm file with WASI
    debugWasi: debugWasiCommand,
    test: testCommand,
};

main();

function main() {
    process.chdir(import.meta.dirname);

    const command = process.argv[2];
    const args = process.argv.slice(3);

    if (!(command in COMMANDS)) {
        console.log("Invalid command:", command);
        process.exit(1);
    }

    COMMANDS[command](args);
}

async function compileCommand() {
    let compilerArgs = process.argv.slice(3);

    return runWASI(await fs.readFile(COMPILER_PATH), {
        preopens: { ".": "." },
        args: ["lo", ...compilerArgs],
        returnOnExit: false,
    });
}

async function runCommand() {
    let compilerArgs = process.argv.slice(3);
    let programArgs = [];

    let programArgsStart = compilerArgs.indexOf("--");
    if (programArgsStart !== -1) {
        programArgs = compilerArgs.slice(programArgsStart + 1);
        compilerArgs = compilerArgs.slice(0, programArgsStart);
    }

    const program = await runWithTmpFile(async (stdout, stdoutFile) => {
        const exitCode = /** @type {unknown} */ (
            await runWASI(await fs.readFile(COMPILER_PATH), {
                stdout: stdout.fd,
                preopens: { ".": "." },
                args: ["lo", ...compilerArgs],
            })
        );

        if (exitCode !== 0) {
            throw new Error("Compilation failed, see compiler error above");
        }

        return fs.readFile(stdoutFile);
    }).catch((err) => {
        console.error(err.message);
        process.exit(1);
    });

    await runWASI(program, {
        preopens: { ".": "." },
        returnOnExit: false,
        args: ["main.lo", ...programArgs],
    });
}

/** @param {string[]} args */
async function runWasiCommand(args) {
    const filePath = new URL(args[0], import.meta.url);
    const input = await fs.readFile(filePath);
    await runWASI(input, {
        args,
        env: process.env,
        preopens: { ".": "." },
    });
}

/**
 * Start an http server that runs provided WASI module for debugging with Dev Tools
 * @param {string[]} args
 */
async function debugWasiCommand(args) {
    const filePath = new URL(args[0], import.meta.url);

    const http = await import("http");
    http.createServer(async (req, res) => {
        if (req.method === "GET" && req.url === "/") {
            res.setHeader("Content-Type", "text/html");
            res.end(`
                <script type="module">
                    import { init, WASI } from "https://esm.sh/@wasmer/wasi@1.2.2";

                    const compilerModule = await WebAssembly.compile(
                        await fetch('./index.wasm').then((r) => r.arrayBuffer())
                    );

                    await init();
                    const wasi = new WASI({});
                    await wasi.instantiate(compilerModule, {});
                    const exitCode = wasi.start();
                    console.log({
                        exitCode,
                        out: wasi.getStdoutString(),
                        err: wasi.getStderrString(),
                    });
                </script>
            `);
            return;
        }

        if (req.method === "GET" && req.url === "/index.wasm") {
            const wasmFile = await fs.readFile(filePath);
            res.setHeader("Content-Type", "application/wasm");
            res.end(wasmFile);
            return;
        }

        res.statusCode = 404;
        res.end("Not Found");
    }).listen(6969, () => {
        console.log("Debug server running at http://localhost:6969/");
    });
}

async function testCommand() {
    const compile = await loadCompilerWithWasiAPI(COMPILER_PATH);

    test("file and stdin inputs all work the same", async () => {
        const compileMockedStdinAPI = await loadCompilerWithWasiAPI(
            COMPILER_PATH,
            true
        );

        const output1 = await compile("./examples/test/42.lo");
        const output2 = await compileMockedStdinAPI("./examples/test/42.lo");

        assert.deepStrictEqual(output1.buffer, output2.buffer);
    });

    test("compiles 42", async () => {
        const output = await compile("./examples/test/42.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 42);
    });

    test("compiles factorial", async () => {
        const output = await compile("./examples/test/factorial.lo");

        const program = await loadWasm(output);
        const result = program.factorial(5);

        assert.strictEqual(result, 120);
    });

    test("compiles hex-and-shifts", async () => {
        const output = await compile("./examples/test/hex-and-shifts.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 31);
    });

    test("compiles locals", async () => {
        const output = await compile("./examples/test/locals.lo");

        const program = await loadWasm(output);
        assert.deepEqual(program.sub(5, 3), 2);
    });

    test("compiles import", async () => {
        const output = await compile("./examples/test/import.lo");

        const logs = [];
        const program = await loadWasm(output, {
            utils: { debug: (x) => logs.push(x) },
        });

        program.main();
        assert.deepEqual(logs, [1, 2, 3]);
    });

    test("compiles globals", async () => {
        const output = await compile("./examples/test/globals.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 69);
    });

    test("compiles methods", async () => {
        const output = await compile("./examples/test/methods.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 1);
    });

    test("compiles nesting", async () => {
        const output = await compile("./examples/test/nesting.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 16);
    });

    test("compiles struct", async () => {
        const output = await compile("./examples/test/struct.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 13);
    });

    test("compiles nested-if-break", async () => {
        const output = await compile("./examples/test/nested-if-break.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 1);
    });

    test("compiles struct-ref", async () => {
        const output = await compile("./examples/test/struct-ref.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 3);
    });

    test("compiles macro", async () => {
        const output = await compile("./examples/test/macro.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 16);
    });

    test("compiles wasi", async () => {
        const output = await compile("./examples/lib/wasi.lo");

        const wasi = new WASI({ version: "preview1" });
        const wasm = await WebAssembly.compile(output);
        // @ts-ignore
        await WebAssembly.instantiate(wasm, wasi.getImportObject());
    });

    test("compiles std", async () => {
        const output = await compile("./examples/test/std.test.lo");

        await loadWasm(output);
    });

    test("compiles vec", async () => {
        const output = await compile("./examples/test/vec.test.lo");
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
        const program = await compile("./examples/test/demos/hello-world.lo");

        const output = await runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, { stdout: stdout.fd });
            return fs.readFile(stdoutFile, { encoding: "utf-8" });
        });

        assert.strictEqual(output, "Hello World!\n");
    });

    test("compiles hello world (raw)", async () => {
        const program = await compile(
            "./examples/test/demos/hello-world-raw.lo"
        );

        const output = await runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, { stdout: stdout.fd });
            return fs.readFile(stdoutFile, { encoding: "utf-8" });
        });

        assert.strictEqual(output, "Hello World!\n");
    });

    test("compiles echo", async () => {
        const program = await compile("./examples/test/demos/echo.lo");

        const output = await runWithTmpFile(async (stdin, stdinFile) => {
            await fs.writeFile(stdinFile, "abc");
            return runWithTmpFile(async (stdout, stdoutFile) => {
                await runWASI(program, { stdin: stdin.fd, stdout: stdout.fd });
                return fs.readFile(stdoutFile, { encoding: "utf-8" });
            });
        });

        assert.strictEqual(output, "abc");
    });

    test("compiles args", async () => {
        const program = await compile("./examples/test/args.test.lo");

        const output = await runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, {
                stdout: stdout.fd,
                args: ["123", "456", "789"],
            });
            return fs.readFile(stdoutFile, { encoding: "utf-8" });
        });

        assert.strictEqual(output, "123\n456\n789\n");
    });

    test("compiles cat", async () => {
        const program = await compile("./examples/test/demos/cat.lo");

        const output = await runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, {
                stdout: stdout.fd,
                args: ["args.lo", "examples/test/42.lo"],
                preopens: { ".": "." },
            });
            return fs.readFile(stdoutFile, { encoding: "utf-8" });
        });

        assert.strictEqual(
            output,
            await fs.readFile("examples/test/42.lo", "utf-8")
        );
    });

    test("compiles string-pooling", async () => {
        const program = await compile("./examples/test/string-pooling.lo");

        const output = await runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, { stdout: stdout.fd });
            return fs.readFile(stdoutFile, { encoding: "utf-8" });
        });

        assert.strictEqual(output, "14\n");
    });

    test("compiles tracing", async () => {
        const program = await compile("./examples/test/tracing.lo");

        const output = await runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, { stdout: stdout.fd });
            return fs.readFile(stdoutFile, { encoding: "utf-8" });
        });

        assert.strictEqual(
            output,
            "examples/test/tracing.lo:4:10 - hello there\n"
        );
    });

    test("compiles struct-in-struct", async () => {
        const program = await compile("./examples/test/struct-in-struct.lo");

        const output = await runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, { stdout: stdout.fd });
            return fs.readFile(stdoutFile, { encoding: "utf-8" });
        });

        assert.strictEqual(output, "3\n3\n3\n3\n3\n3\n3\n");
    });

    test("compiles for-loop", async () => {
        const program = await compile("./examples/test/for-loop.lo");

        const output = await runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, { stdout: stdout.fd });
            return fs.readFile(stdoutFile, { encoding: "utf-8" });
        });

        assert.strictEqual(
            output,
            [
                "0 1 2 3 4 5 6 7 8 9 ",
                "0 1 2 3 4 5 6 7 ",
                "0 1 2 3 5 6 7 8 9 ",
                "0 1 2 3 5 6 7 ",
            ]
                .map((x) => x + "\n")
                .join("") // using this instead of multiline because trailing spaces are removed when formatting
        );
    });

    test("compiles heap-alloc", async () => {
        const program = await compile("./examples/test/heap-alloc.lo");

        const output = await runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, { stdout: stdout.fd });
            return fs.readFile(stdoutFile, { encoding: "utf-8" });
        });

        assert.strictEqual(
            output,
            m`
            p1 = 1048597
            p2 = 1048597
            p3 = 1048606

            `
        );
    });

    test("compiles defer", async () => {
        const program = await compile("./examples/test/defer.lo");

        const output = await runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, { stdout: stdout.fd });
            return fs.readFile(stdoutFile, { encoding: "utf-8" });
        });

        assert.strictEqual(
            output,
            m`
            defer(inner_fn): 3
            defer(inner_fn): 2
            defer(inner_fn): 1
            defer(return): 3
            defer(return): 2
            defer(return): 1

            `
        );
    });

    test("compiles errors", async () => {
        const program = await compile("./examples/test/errors.lo");

        const output = await runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, { stdout: stdout.fd });
            return fs.readFile(stdoutFile, { encoding: "utf-8" });
        });

        assert.strictEqual(
            output,
            m`
            10 / 5 = 2, remainder = 0
            10 / 3 = 3, remainder = 1
            10 / 0 is undefined

            `
        );
    });

    {
        test("aoc 2020 day 1", async () => {
            const part1 = await runAoc("./examples/test/demos/aoc2020/1.lo");
            assert.strictEqual(part1, "157059\n");

            const part2 = await runAoc(
                "./examples/test/demos/aoc2020/1-part2.lo"
            );
            assert.strictEqual(part2, "165080960\n");
        });

        test("aoc 2020 day 2", async () => {
            const part1 = await runAoc("./examples/test/demos/aoc2020/2.lo");
            assert.strictEqual(part1, "560\n");

            const part2 = await runAoc(
                "./examples/test/demos/aoc2020/2-part2.lo"
            );
            assert.strictEqual(part2, "303\n");
        });

        test("aoc 2020 day 3", async () => {
            const part1 = await runAoc("./examples/test/demos/aoc2020/3.lo");
            assert.strictEqual(part1, "151\n");

            const part2 = await runAoc(
                "./examples/test/demos/aoc2020/3-part2.lo"
            );
            assert.strictEqual(part2, "7540141059\n");
        });

        test("aoc 2020 day 4", async () => {
            const part1 = await runAoc("./examples/test/demos/aoc2020/4.lo");
            assert.strictEqual(part1, "264\n");

            const part2 = await runAoc(
                "./examples/test/demos/aoc2020/4-part2.lo"
            );
            assert.strictEqual(part2, "224\n");
        });

        test("aoc 2020 day 5", async () => {
            const part1 = await runAoc("./examples/test/demos/aoc2020/5.lo");
            assert.strictEqual(part1, "947\n");

            const part2 = await runAoc(
                "./examples/test/demos/aoc2020/5-part2.lo"
            );
            assert.strictEqual(part2, "636\n");
        });

        async function runAoc(path) {
            const program = await compile(path);

            return await runWithTmpFile(async (stdout, stdoutFile) => {
                const exitCode = await runWASI(program, {
                    stdout: stdout.fd,
                    preopens: { ".": "." },
                });

                if (exitCode !== 0) {
                    throw new Error(
                        `Process exited with error code: ${exitCode}`
                    );
                }

                return fs.readFile(stdoutFile, { encoding: "utf-8" });
            });
        }
    }

    // lo.lo
    {
        test("compiles lexer.test.lo", async () => {
            const program = await compile("./examples/test/lexer.test.lo");

            const output = await runWithTmpFile(async (stdout, stdoutFile) => {
                await runWASI(program, {
                    stdout: stdout.fd,
                    preopens: { ".": "." },
                });

                return fs.readFile(stdoutFile, { encoding: "utf-8" });
            });

            assert.strictEqual(
                output,
                m`
                examples/test/lexer.test.input.txt:1:1 - 3 : a
                examples/test/lexer.test.input.txt:2:2 - 3 : b
                examples/test/lexer.test.input.txt:3:3 - 3 : c
                examples/test/lexer.test.input.txt:4:4 - 3 : d
                examples/test/lexer.test.input.txt:7:5 - 3 : a
                examples/test/lexer.test.input.txt:7:7 - 5 : +
                examples/test/lexer.test.input.txt:7:9 - 3 : b
                examples/test/lexer.test.input.txt:8:5 - 3 : b
                examples/test/lexer.test.input.txt:8:7 - 5 : +=
                examples/test/lexer.test.input.txt:8:10 - 2 : 0xAF, value = 175
                examples/test/lexer.test.input.txt:9:5 - 3 : f
                examples/test/lexer.test.input.txt:9:7 - 5 : /=
                examples/test/lexer.test.input.txt:9:10 - 2 : 25, value = 25
                examples/test/lexer.test.input.txt:9:12 - 5 : .
                examples/test/lexer.test.input.txt:9:13 - 2 : 6, value = 6
                examples/test/lexer.test.input.txt:10:5 - 3 : x
                examples/test/lexer.test.input.txt:10:7 - 5 : <<=
                examples/test/lexer.test.input.txt:10:11 - 3 : 'a'
                examples/test/lexer.test.input.txt:11:5 - 2 : 0_123_456, value = 123456
                examples/test/lexer.test.input.txt:12:3 - 3 : a
                examples/test/lexer.test.input.txt:12:5 - 4 : (
                examples/test/lexer.test.input.txt:12:6 - 3 : a
                examples/test/lexer.test.input.txt:12:7 - 4 : )
                examples/test/lexer.test.input.txt:12:8 - 3 : dwawd
                examples/test/lexer.test.input.txt:12:13 - 4 : ;
                examples/test/lexer.test.input.txt:13:3 - 3 : b
                examples/test/lexer.test.input.txt:13:16 - 3 : awdawd
                examples/test/lexer.test.input.txt:14:3 - 3 : c
                examples/test/lexer.test.input.txt:15:2 - 3 : dawd
                examples/test/lexer.test.input.txt:16:2 - 0 : "John doe went fucking crazy trimming the \\t bushes", value = John doe went fucking crazy trimming the \t bushes
                examples/test/lexer.test.input.txt:18:1 - 3 : awdawd

                `
            );
        });

        /**
         * NOTE: this test is pretty slow.
         * When multiple of these kind of tests will be run it would make sense to use WABT.js:
         * `const wabt = await import("https://unpkg.com/wabt@1.0.36/index.js");`
         */
        test("compiles 42.lo using lo.lo", async () => {
            const { exec } = await import("node:child_process");
            const { promisify } = await import("node:util");

            const { stdout, stderr } = await promisify(exec)(
                "./utils.mjs run examples/lo.lo -- examples/test/42.lo | wasm2wat -"
            );
            if (stderr) {
                throw new Error(stderr);
            }

            assert.equal(
                stdout,
                m`
                (module
                  (type (;0;) (func (result i32)))
                  (func (;0;) (type 0) (result i32)
                    i32.const 42
                    return)
                  (export "main" (func 0)))

                `
            );
        });

        test("compiles hello-world-raw.lo using lo.lo", async () => {
            const { exec } = await import("node:child_process");
            const { promisify } = await import("node:util");

            const { stdout, stderr } = await promisify(exec)(
                "./utils.mjs run examples/lo.lo -- examples/test/demos/hello-world-raw.lo | wasm2wat -"
            );
            if (stderr) {
                throw new Error(stderr);
            }

            assert.equal(
                stdout,
                m`
                (module
                  (type (;0;) (func (param i32 i32 i32 i32) (result i32)))
                  (type (;1;) (func))
                  (import "wasi_snapshot_preview1" "fd_write" (func (;0;) (type 0)))
                  (func (;1;) (type 1)
                    i32.const 4
                    i32.const 12
                    i32.store align=1
                    i32.const 8
                    i32.const 13
                    i32.store align=1
                    i32.const 1
                    i32.const 4
                    i32.const 1
                    i32.const 0
                    call 0
                    drop)
                  (memory (;0;) 1)
                  (export "memory" (memory 0))
                  (export "_start" (func 1))
                  (data (;0;) (i32.const 12) "Hello World!\\0a"))

                `
            );
        });
    }
}

// utils

/**
 * @param {string} compilerPath
 * @returns {Promise<(sourcePath: string) => Promise<Promise<Buffer>>>}
 */
async function loadCompilerWithWasiAPI(compilerPath, mockStdin = false) {
    const mod = await WebAssembly.compile(await fs.readFile(compilerPath));

    /**
     * @param {string} [fileName]
     * @param {number} [stdinFd]
     */
    const compile = (fileName, stdinFd) =>
        runWithTmpFile(async (stderr, stderrFile) =>
            runWithTmpFile(async (stdout, stdoutFile) => {
                const wasi = new WASI({
                    version: "preview1",
                    stdin: stdinFd,
                    stdout: stdout.fd,
                    stderr: stderr.fd,
                    args: ["lo", fileName ?? "-i"],
                    preopens: { ".": "." },
                });

                // @ts-ignore
                const instance = await WebAssembly.instantiate(mod, {
                    ...wasi.getImportObject(),
                    ...{ console },
                });

                const exitCode = /** @type {unknown} */ (wasi.start(instance));

                if (exitCode ?? 0 !== 0) {
                    throw new Error(await fs.readFile(stderrFile, "utf-8"));
                }

                return fs.readFile(stdoutFile);
            })
        );

    /**
     * @param {string} sourcePath
     */
    return (sourcePath) => {
        if (!mockStdin) {
            return compile(sourcePath);
        }

        return runWithTmpFile(async (stdin, stdinFile) => {
            await fs.writeFile(stdinFile, await fs.readFile(sourcePath));

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
 * @param {Omit<import("node:wasi").WASIOptions, 'version'>} [wasiOptions]
 */
async function runWASI(data, wasiOptions, additionalImports = {}) {
    const wasi = new WASI({ version: "preview1", ...wasiOptions });

    const wasm = await WebAssembly.compile(data);
    // @ts-ignore
    const instance = await WebAssembly.instantiate(wasm, {
        ...wasi.getImportObject(),
        ...{ console },
        ...additionalImports,
    });

    try {
        return wasi.start(instance);
    } catch (err) {
        if (err instanceof WebAssembly.RuntimeError) {
            if (err.message.includes("unreachable")) {
                const memory = /** @type {WebAssembly.Memory} */ (
                    instance.exports.memory
                );
                const [errorIndicator, errorCode] = new Uint32Array(
                    memory.buffer.slice(0, 8)
                );

                if (errorIndicator === 69420) {
                    err.message = "Abort code " + errorCode;
                }
            }
        }
        throw err;
    }
}

/**
 * @template T
 * @param {(file: import("node:fs/promises").FileHandle, fileName: string) => T} run
 */
async function runWithTmpFile(run) {
    const fileName = `${TMP_DIR}/${crypto.randomUUID()}.tmp`;
    const fileHandle = await fs.open(fileName, "w+");

    try {
        return await run(fileHandle, fileName);
    } finally {
        await fileHandle.close();
        await fs.unlink(fileName);
    }
}
