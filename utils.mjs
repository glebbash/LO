#!/usr/bin/env -S node --no-warnings --experimental-network-imports
// @ts-check

import { WASI } from "node:wasi";
import process from "node:process";
import { test, describe, it } from "node:test";
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
    const v1 = await loadCompilerWithWasiAPI(await fs.readFile(COMPILER_PATH));
    const v2 = await loadCompilerWithWasiAPI(await fs.readFile(COMPILER_PATH), {
        buildArgs: (fileName) => ["lo", fileName ?? "-i", "--compile-v2"],
    });
    const vS = await loadCompilerWithWasiAPI(await v1("examples/lo.lo"));

    // NOTE: commenting/uncommenting this prevents random segfaults
    await v1("./examples/test/42.lo");

    testCompilers("42.lo", { v1, v2, vS }, async (compile) => {
        const output = await compile("./examples/test/42.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 42);
    });

    testCompilers("add.lo", { v1, v2 }, async (compile) => {
        const output = await compile("./examples/test/add.lo");

        const program = await loadWasm(output);

        assert.strictEqual(program.add(2, 3), 5);
    });

    testCompilers("else-if.lo", { v1, v2 }, async (compile) => {
        const output = await compile("examples/test/else-if.lo");

        const program = await loadWasm(output);

        assert.strictEqual(program.else_if_test(4), 0);
        assert.strictEqual(program.else_if_test(9), 1);
        assert.strictEqual(program.else_if_test(11), 2);
    });

    testCompilers("factorial.lo", { v1, v2 }, async (compile) => {
        const output = await compile("./examples/test/factorial.lo");

        const program = await loadWasm(output);
        const result = program.factorial(5);

        assert.strictEqual(result, 120);
    });

    testCompilers("hello-world-raw.lo", { v1, vS }, async (compile) => {
        const program = await compile(
            "./examples/test/demos/hello-world-raw.lo"
        );

        const output = await runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, { stdout: stdout.fd });
            return fs.readFile(stdoutFile, { encoding: "utf-8" });
        });

        assert.strictEqual(output, "Hello World!\n");
    });

    testCompilers("include.lo", { v1, v2 }, async (compile) => {
        const output = await compile("./examples/test/include.lo");

        const program = await loadWasm(output);

        assert.strictEqual(program.main(), 120);
    });

    testCompilers("hex-and-shifts.lo", { v1, v2 }, async (compile) => {
        const output = await compile("./examples/test/hex-and-shifts.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 31);
    });

    testCompilers("compiles locals", { v1 }, async (compile) => {
        const output = await compile("./examples/test/locals.lo");

        const program = await loadWasm(output);
        assert.deepEqual(program.sub(5, 3), 2);
    });

    testCompilers("compiles import", { v1 }, async (compile) => {
        const output = await compile("./examples/test/import.lo");

        const logs = [];
        const program = await loadWasm(output, {
            utils: { debug: (x) => logs.push(x) },
        });

        program.main();
        assert.deepEqual(logs, [1, 2, 3]);
    });

    testCompilers("compiles globals", { v1 }, async (compile) => {
        const output = await compile("./examples/test/globals.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 69);
    });

    testCompilers("compiles methods", { v1 }, async (compile) => {
        const output = await compile("./examples/test/methods.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 1);
    });

    testCompilers("compiles nesting", { v1 }, async (compile) => {
        const output = await compile("./examples/test/nesting.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 16);
    });

    testCompilers("compiles struct", { v1 }, async (compile) => {
        const output = await compile("./examples/test/struct.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 13);
    });

    testCompilers("compiles nested-if-break", { v1 }, async (compile) => {
        const output = await compile("./examples/test/nested-if-break.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 1);
    });

    testCompilers("compiles struct-ref", { v1 }, async (compile) => {
        const output = await compile("./examples/test/struct-ref.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 3);
    });

    testCompilers("compiles macro", { v1 }, async (compile) => {
        const output = await compile("./examples/test/macro.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 16);
    });

    testCompilers("compiles wasi", { v1 }, async (compile) => {
        const output = await compile("./examples/lib/wasi.lo");

        const wasi = new WASI({ version: "preview1" });
        const wasm = await WebAssembly.compile(output);
        // @ts-ignore
        await WebAssembly.instantiate(wasm, wasi.getImportObject());
    });

    testCompilers("compiles std", { v1 }, async (compile) => {
        const output = await compile("./examples/test/std.test.lo");

        await loadWasm(output);
    });

    testCompilers("compiles vec", { v1 }, async (compile) => {
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
    });

    testCompilers("compiles hello world", { v1 }, async (compile) => {
        const program = await compile("./examples/test/demos/hello-world.lo");

        const output = await runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, { stdout: stdout.fd });
            return fs.readFile(stdoutFile, { encoding: "utf-8" });
        });

        assert.strictEqual(output, "Hello World!\n");
    });

    testCompilers("compiles echo", { v1 }, async (compile) => {
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

    testCompilers("compiles args", { v1 }, async (compile) => {
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

    testCompilers("compiles cat", { v1 }, async (compile) => {
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

    testCompilers("compiles string-pooling", { v1 }, async (compile) => {
        const program = await compile("./examples/test/string-pooling.lo");

        const output = await runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, { stdout: stdout.fd });
            return fs.readFile(stdoutFile, { encoding: "utf-8" });
        });

        assert.strictEqual(output, "14\n");
    });

    testCompilers("compiles tracing", { v1 }, async (compile) => {
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

    testCompilers("compiles struct-in-struct", { v1 }, async (compile) => {
        const program = await compile("./examples/test/struct-in-struct.lo");

        const output = await runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, { stdout: stdout.fd });
            return fs.readFile(stdoutFile, { encoding: "utf-8" });
        });

        assert.strictEqual(output, "3\n3\n3\n3\n3\n3\n3\n");
    });

    testCompilers("compiles for-loop", { v1 }, async (compile) => {
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

    testCompilers("compiles heap-alloc", { v1 }, async (compile) => {
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

    testCompilers("compiles defer", { v1 }, async (compile) => {
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

    testCompilers("compiles errors", { v1 }, async (compile) => {
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

    describe("<stdin> input", async () => {
        const v1 = await loadCompilerWithWasiAPI(
            await fs.readFile(COMPILER_PATH),
            { mockStdin: true }
        );

        const v2 = await loadCompilerWithWasiAPI(
            await fs.readFile(COMPILER_PATH),
            { mockStdin: true, buildArgs: () => ["lo", "-i", "--compile-v2"] }
        );

        testCompilers("42.lo", { v1, v2 }, async (compile) => {
            const output = await compile("./examples/test/42.lo");

            const program = await loadWasm(output);
            const result = program.main();

            assert.strictEqual(result, 42);
        });
    });

    describe("aoc", async () => {
        testCompilers("2020 day 1", { v1 }, async (compile) => {
            const part1 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/1.lo"
            );
            assert.strictEqual(part1, "157059\n");

            const part2 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/1-part2.lo"
            );
            assert.strictEqual(part2, "165080960\n");
        });

        testCompilers("2020 day 2", { v1 }, async (compile) => {
            const part1 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/2.lo"
            );
            assert.strictEqual(part1, "560\n");

            const part2 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/2-part2.lo"
            );
            assert.strictEqual(part2, "303\n");
        });

        testCompilers("2020 day 3", { v1 }, async (compile) => {
            const part1 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/3.lo"
            );
            assert.strictEqual(part1, "151\n");

            const part2 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/3-part2.lo"
            );
            assert.strictEqual(part2, "7540141059\n");
        });

        testCompilers("2020 day 4", { v1 }, async (compile) => {
            const part1 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/4.lo"
            );
            assert.strictEqual(part1, "264\n");

            const part2 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/4-part2.lo"
            );
            assert.strictEqual(part2, "224\n");
        });

        testCompilers("2020 day 5", { v1 }, async (compile) => {
            const part1 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/5.lo"
            );
            assert.strictEqual(part1, "947\n");

            const part2 = await runAoc(
                compile,
                "./examples/test/demos/aoc2020/5-part2.lo"
            );
            assert.strictEqual(part2, "636\n");
        });

        testCompilers("2023 day 1", { v1 }, async (compile) => {
            const part1 = await runAoc(
                compile,
                "./examples/test/demos/aoc2023/1.lo"
            );
            assert.strictEqual(part1, "54450\n");

            const part2 = await runAoc(
                compile,
                "./examples/test/demos/aoc2023/1-part2.lo"
            );
            assert.strictEqual(part2, "54265\n");
        });

        /**
         * @param {Compile} compile
         * @param {string} path
         */
        async function runAoc(compile, path) {
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
    });

    testCompilers(
        "compiler reports multiple errors in multiple-compiler-errors.lo",
        { v2 },
        async (compile) => {
            try {
                await compile("./examples/test/multiple-compiler-errors.lo");
            } catch (err) {
                assert.strictEqual(
                    err.message,
                    m`
                    examples/test/multiple-compiler-errors.lo:3:14 - Duplicate function parameter name: a
                    examples/test/multiple-compiler-errors.lo:4:14 - Duplicate function parameter name: b
                    examples/test/multiple-compiler-errors.lo:5:14 - Duplicate function parameter name: c


                    `
                );
            }
        }
    );

    testCompilers("lexer.test.lo (vS unit test)", { v1 }, async (compile) => {
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
            examples/test/lexer.test.input.txt:16:2 - 0 : "Lorem ipsum \\t dolor", value = Lorem ipsum \t dolor
            examples/test/lexer.test.input.txt:18:1 - 3 : awdawd

            `
        );
    });

    /**
     * @param {string} testName
     * @param {Record<string, Compile>} compilers
     * @param {(compile: Compile) => Promise<void>} testFn
     */
    function testCompilers(testName, compilers, testFn) {
        for (const [compilerName, compile] of Object.entries(compilers)) {
            test(`${testName} (${compilerName})`, () => testFn(compile));
        }
    }

    /** @typedef {(sourcePath: string) => Promise<Buffer>} Compile */

    /**
     * @param {Buffer} compilerWasmBinary
     * @returns {Promise<Compile>}
     */
    async function loadCompilerWithWasiAPI(
        compilerWasmBinary,
        {
            mockStdin = false,
            buildArgs = (fileName) => ["lo", fileName ?? "-i"],
        } = {}
    ) {
        const mod = await WebAssembly.compile(compilerWasmBinary);

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
                        args: buildArgs(fileName),
                        preopens: { ".": "." },
                    });

                    // @ts-ignore
                    const instance = await WebAssembly.instantiate(mod, {
                        ...wasi.getImportObject(),
                        ...{ console },
                    });

                    try {
                        const exitCode = /** @type {unknown} */ (
                            wasi.start(instance)
                        );

                        if (exitCode ?? 0 !== 0) {
                            throw new Error(
                                await fs.readFile(stderrFile, "utf-8")
                            );
                        }

                        return fs.readFile(stdoutFile);
                    } catch (err) {
                        const errorMessage = await fs.readFile(
                            stderrFile,
                            "utf-8"
                        );
                        if (errorMessage !== "") {
                            if (
                                err instanceof WebAssembly.RuntimeError &&
                                err.message.includes("unreachable")
                            ) {
                                err.message = errorMessage;
                            } else {
                                throw new Error(errorMessage);
                            }
                        }

                        throw err;
                    }
                })
            );

        /**
         * @param {string} sourcePath
         */
        return async (sourcePath) => {
            if (!mockStdin) {
                return compile(sourcePath);
            }

            return runWithTmpFile(async (stdin, stdinFile) => {
                await fs.writeFile(stdinFile, await fs.readFile(sourcePath));

                return compile(undefined, stdin.fd);
            });
        };
    }
}

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
