#!/usr/bin/env -S node --no-warnings --experimental-network-imports
// @ts-check

import { WASI } from "./wasi-shim.mjs";
import process from "node:process";
import { test, describe } from "node:test";
import assert from "node:assert";
import fs from "node:fs/promises";
import crypto from "node:crypto";
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
    process.chdir(/** @type {never} */ (import.meta.dirname));

    const command = process.argv[2];
    const args = process.argv.slice(3);

    if (!(command in COMMANDS)) {
        console.log("Invalid command:", command);
        process.exit(1);
    }

    COMMANDS[/** @type {keyof typeof COMMANDS} */ (command)](args);
}

async function compileCommand() {
    const compilerArgs = process.argv.slice(3);

    return runWASI(await fs.readFile(COMPILER_PATH), {
        preopens: { ".": "." },
        args: ["lo", ...compilerArgs],
        returnOnExit: false,
    });
}

async function runCommand() {
    let compilerArgs = process.argv.slice(3);
    /** @type {string[]} */
    let programArgs = [];

    const programArgsStart = compilerArgs.indexOf("--");
    if (programArgsStart !== -1) {
        programArgs = compilerArgs.slice(programArgsStart + 1);
        compilerArgs = compilerArgs.slice(0, programArgsStart);
    }

    const program = await runWithTmpFile(async (stdout, stdoutFile) => {
        const exitCode = await runWASI(await fs.readFile(COMPILER_PATH), {
            stdout: stdout.fd,
            preopens: { ".": "." },
            args: ["lo", ...compilerArgs],
        });

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

    const http = await import("node:http");
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

    testCompilers("compiles 42.lo", { v1, v2, vS }, async (compile) => {
        const output = await compile("./examples/test/42.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 42);
    });

    testCompilers("compiles add.lo", { v1, v2 }, async (compile) => {
        const output = await compile("./examples/test/add.lo");

        const program = await loadWasm(output);

        assert.strictEqual(program.add(2, 3), 5);
    });

    testCompilers("compiles factorial.lo", { v1, v2 }, async (compile) => {
        const output = await compile("./examples/test/factorial.lo");

        const program = await loadWasm(output);
        const result = program.factorial(5);

        assert.strictEqual(result, 120);
    });

    testCompilers("compiles include.lo", { v1, v2 }, async (compile) => {
        const output = await compile("./examples/test/include.lo");

        const program = await loadWasm(output);

        assert.strictEqual(program.main(), 120);
    });

    testCompilers("compiles hex-and-shifts.lo", { v1, v2 }, async (compile) => {
        const output = await compile("./examples/test/hex-and-shifts.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 31);
    });

    testCompilers("compiles else-if.lo", { v1, v2 }, async (compile) => {
        const output = await compile("examples/test/else-if.lo");

        const program = await loadWasm(output);

        assert.strictEqual(program.main(), 13);
    });

    testCompilers("compiles import.lo", { v1, v2 }, async (compile) => {
        const output = await compile("./examples/test/import.lo");

        /** @type {unknown[]} */
        const logs = [];
        const program = await loadWasm(output, {
            utils: { debug: (/** @type {unknown} */ x) => logs.push(x) },
        });

        program.main();
        assert.deepEqual(logs, [1, 2, 3]);
    });

    testCompilers(
        "compiles hello-world-raw.lo",
        { v1, v2, vS },
        async (compile) => {
            const program = await compile(
                "./examples/test/demos/hello-world-raw.lo"
            );

            const output = await runWithTmpFile(async (stdout, stdoutFile) => {
                await runWASI(program, { stdout: stdout.fd });
                return fs.readFile(stdoutFile, { encoding: "utf-8" });
            });

            assert.strictEqual(output, "Hello World!\n");
        }
    );

    testCompilers("compiles locals.lo", { v1, v2 }, async (compile) => {
        const output = await compile("./examples/test/locals.lo");

        const program = await loadWasm(output);
        assert.deepEqual(program.sub(5, 3), 2);
    });

    testCompilers("compiles globals.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/globals.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 69);
    });

    testCompilers("compiles loop.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/loop.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 120);
    });

    testCompilers("compiles for-loop.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/for-loop.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 138);
    });

    testCompilers("compiles methods.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/methods.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 1);
    });

    testCompilers("compiles decl-nesting.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/decl-nesting.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 16);
    });

    testCompilers("compiles struct.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/struct.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 13);
    });

    testCompilers("compiles nested-if-break.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/nested-if-break.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 1);
    });

    testCompilers("compiles struct-ref.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/struct-ref.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 3);
    });

    testCompilers("compiles macro.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/macro.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 16);
    });

    testCompilers("compiles wasi.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/lib/wasi.lo");

        const wasi = new WASI({ version: "preview1" });
        const wasm = await WebAssembly.compile(output);
        await WebAssembly.instantiate(wasm, wasi.getImportObject());
    });

    testCompilers("compiles std.lo", { v1 }, async (compile) => {
        const output = await compile("./examples/test/std.test.lo");

        await loadWasm(output);
    });

    testCompilers("compiles vec.lo", { v1 }, async (compile) => {
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

    testCompilers("compiles hello-world.lo", { v1 }, async (compile) => {
        const program = await compile("./examples/test/demos/hello-world.lo");

        const output = await runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, { stdout: stdout.fd });
            return fs.readFile(stdoutFile, { encoding: "utf-8" });
        });

        assert.strictEqual(output, "Hello World!\n");
    });

    testCompilers("compiles echo.lo", { v1 }, async (compile) => {
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

    testCompilers("compiles args.test.lo", { v1 }, async (compile) => {
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

    testCompilers("compiles cat.lo", { v1 }, async (compile) => {
        const program = await compile("./examples/test/demos/cat.lo");

        const output = await runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, {
                stdout: stdout.fd,
                args: ["cat.lo", "examples/test/42.lo"],
                preopens: { ".": "." },
            });
            return fs.readFile(stdoutFile, { encoding: "utf-8" });
        });

        assert.strictEqual(
            output,
            await fs.readFile("examples/test/42.lo", "utf-8")
        );
    });

    testCompilers("compiles string-pooling.lo", { v1 }, async (compile) => {
        const program = await compile("./examples/test/string-pooling.lo");

        const output = await runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, { stdout: stdout.fd });
            return fs.readFile(stdoutFile, { encoding: "utf-8" });
        });

        assert.strictEqual(output, "14\n");
    });

    testCompilers("compiles tracing.lo", { v1 }, async (compile) => {
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

    testCompilers("compiles struct-in-struct.lo", { v1 }, async (compile) => {
        const program = await compile("./examples/test/struct-in-struct.lo");

        const output = await runWithTmpFile(async (stdout, stdoutFile) => {
            await runWASI(program, { stdout: stdout.fd });
            return fs.readFile(stdoutFile, { encoding: "utf-8" });
        });

        assert.strictEqual(output, "3\n3\n3\n3\n3\n3\n3\n");
    });

    testCompilers("compiles heap-alloc.lo", { v1 }, async (compile) => {
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

    testCompilers("compiles defer.lo", { v1 }, async (compile) => {
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

    testCompilers("compiles errors.lo", { v1 }, async (compile) => {
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

        testCompilers("compiles 42.lo", { v1, v2 }, async (compile) => {
            const output = await compile("./examples/test/42.lo");

            const program = await loadWasm(output);
            const result = program.main();

            assert.strictEqual(result, 42);
        });
    });

    describe("aoc", () => {
        testCompilers("compiles 2020 day 1", { v1 }, async (compile) => {
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

        testCompilers("compiles 2020 day 2", { v1 }, async (compile) => {
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

        testCompilers("compiles 2020 day 3", { v1 }, async (compile) => {
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

        testCompilers("compiles 2020 day 4", { v1 }, async (compile) => {
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

        testCompilers("compiles 2020 day 5", { v1 }, async (compile) => {
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

        testCompilers("compiles 2023 day 1", { v1 }, async (compile) => {
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
                    examples/test/multiple-compiler-errors.lo:2:14 - Duplicate function parameter name: a
                    examples/test/multiple-compiler-errors.lo:6:14 - Duplicate function parameter name: b
                    examples/test/multiple-compiler-errors.lo:10:14 - Duplicate function parameter name: c


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

    describe("formatter", async () => {
        const format = await loadCompilerWithWasiAPI(
            await fs.readFile(COMPILER_PATH),
            {
                buildArgs: (fileName) => [
                    "lo",
                    fileName ?? "-i",
                    "--pretty-print",
                ],
            }
        );

        const formattedFiles = [
            "examples/lib/args.lo",
            "examples/lib/cli.lo",
            "examples/lib/debug.lo",
            "examples/lib/fs.lo",
            "examples/lib/int_parser.lo",
            "examples/lib/lo/core.lo",
            "examples/lib/lo/ir.lo",
            "examples/lib/lo/lexer.lo",
            "examples/lib/lo/parser.lo",
            "examples/lib/lo/utils.lo",
            "examples/lib/lo/wasm.lo",
            "examples/lib/print.lo",
            "examples/lib/std.lo",
            "examples/lib/str_cutter.lo",
            "examples/lib/string_map.lo",
            "examples/lib/wasi.lo",
            "examples/lo.lo",
            "examples/test/42.lo",
            "examples/test/add.lo",
            "examples/test/args.test.lo",
            "examples/test/decl-nesting.lo",
            "examples/test/defer.lo",
            "examples/test/demos/aoc2020/1-part2.lo",
            "examples/test/demos/aoc2020/1.lo",
            "examples/test/demos/aoc2020/2-part2.lo",
            "examples/test/demos/aoc2020/2.lo",
            "examples/test/demos/aoc2020/3-part2.lo",
            "examples/test/demos/aoc2020/3.lo",
            "examples/test/demos/aoc2020/4-part2.lo",
            "examples/test/demos/aoc2020/4.lo",
            "examples/test/demos/aoc2020/5-part2.lo",
            "examples/test/demos/aoc2020/5.lo",
            "examples/test/demos/aoc2023/1-part2.lo",
            "examples/test/demos/aoc2023/1.lo",
            "examples/test/demos/cat.lo",
            "examples/test/demos/echo.lo",
            "examples/test/demos/hello-world-raw.lo",
            "examples/test/demos/hello-world.lo",
            "examples/test/demos/vscode_wasm_issue_161.lo",
            "examples/test/demos/wasm4/src/blink.lo",
            "examples/test/demos/wasm4/src/dark-maze.lo",
            "examples/test/demos/wasm4/src/lib/wasm4.lo",
            "examples/test/demos/wasm4/src/slasher.lo",
            "examples/test/else-if.lo",
            "examples/test/errors.lo",
            "examples/test/factorial.lo",
            "examples/test/for-loop.lo",
            "examples/test/globals.lo",
            "examples/test/heap-alloc.lo",
            "examples/test/hex-and-shifts.lo",
            "examples/test/import.lo",
            "examples/test/include.lo",
            "examples/test/lexer.test.lo",
            "examples/test/locals.lo",
            "examples/test/loop.lo",
            "examples/test/macro.lo",
            "examples/test/methods.lo",
            "examples/test/multiple-compiler-errors.lo",
            "examples/test/nested-if-break.lo",
            "examples/test/std.test.lo",
            "examples/test/string-pooling.lo",
            "examples/test/struct-in-struct.lo",
            "examples/test/struct-ref.lo",
            "examples/test/struct.lo",
            "examples/test/tracing.lo",
            "examples/test/vec.test.lo",
        ];

        for (const fileName of formattedFiles) {
            test(`formats ${fileName}`, async () => {
                const formatted = (await format(fileName)).toString();
                const expected = await fs.readFile(fileName, "utf8");

                // formatting the file to update snapshot
                await fs.writeFile(fileName, formatted);

                assert.strictEqual(formatted, expected);
            });
        }
    });

    if (process.argv.includes("--fast")) {
        return;
    }

    describe("interpreter", async () => {
        const interpret = await loadCompilerWithWasiAPI(
            await fs.readFile(COMPILER_PATH),
            { buildArgs: (fileName) => ["lo", fileName ?? "-i", "--eval"] }
        );

        test("interprets 42.lo", async () => {
            const res = await interpret("examples/test/42.lo");
            assert.strictEqual(
                res.toString("utf-8"),
                "result of `main` is: 42\n"
            );
        });

        test("interprets include.lo", async () => {
            const res = await interpret("examples/test/include.lo");
            assert.strictEqual(
                res.toString("utf-8"),
                "result of `main` is: 120\n"
            );
        });

        test("interprets else-if.lo", async () => {
            const res = await interpret("examples/test/else-if.lo");
            assert.strictEqual(
                res.toString("utf-8"),
                "result of `main` is: 13\n"
            );
        });

        test("interprets globals.lo", async () => {
            const res = await interpret("examples/test/globals.lo");
            assert.strictEqual(
                res.toString("utf-8"),
                "result of `main` is: 69\n"
            );
        });

        test("interprets hex-and-shifts.lo", async () => {
            const res = await interpret("examples/test/hex-and-shifts.lo");
            assert.strictEqual(
                res.toString("utf-8"),
                "result of `main` is: 31\n"
            );
        });

        test("interprets loop.lo", async () => {
            const res = await interpret("./examples/test/loop.lo");
            assert.strictEqual(
                res.toString("utf-8"),
                "result of `main` is: 120\n"
            );
        });

        test("interprets for-loop.lo", async () => {
            const res = await interpret("./examples/test/for-loop.lo");
            assert.strictEqual(
                res.toString("utf-8"),
                "result of `main` is: 138\n"
            );
        });

        test("interprets methods.lo", async () => {
            const res = await interpret("./examples/test/methods.lo");
            assert.strictEqual(
                res.toString("utf-8"),
                "result of `main` is: 1\n"
            );
        });

        test("interprets decl-nesting.lo", async () => {
            const res = await interpret("./examples/test/decl-nesting.lo");
            assert.strictEqual(
                res.toString("utf-8"),
                "result of `main` is: 16\n"
            );
        });

        test("interprets struct.lo", async () => {
            const res = await interpret("./examples/test/struct.lo");
            assert.strictEqual(
                res.toString("utf-8"),
                "result of `main` is: 13\n"
            );
        });

        test("interprets nested-if-break.lo", async () => {
            const res = await interpret("./examples/test/nested-if-break.lo");
            assert.strictEqual(
                res.toString("utf-8"),
                "result of `main` is: 1\n"
            );
        });

        test("interprets struct-ref.lo", async () => {
            const res = await interpret("./examples/test/struct-ref.lo");
            assert.strictEqual(
                res.toString("utf-8"),
                "result of `main` is: 3\n"
            );
        });

        test("interprets macro.lo", async () => {
            const res = await interpret("./examples/test/macro.lo");
            assert.strictEqual(
                res.toString("utf-8"),
                "result of `main` is: 16\n"
            );
        });

        test("interprets import.lo", async () => {
            const res = await interpret("./examples/test/import.lo");
            // NOTE: can't really see debug output because it's on stderr and that is ignored on exit = 0
            assert.strictEqual(
                res.toString("utf-8"),
                "result of `main` is: \n"
            );
        });

        // wasi

        test("interprets hello-world-raw.lo", async () => {
            const res = await interpret(
                "./examples/test/demos/hello-world-raw.lo"
            );
            assert.strictEqual(res.toString("utf-8"), "Hello World!\n");
        });

        test("interprets hello-world.lo", async () => {
            const res = await interpret("./examples/test/demos/hello-world.lo");
            assert.strictEqual(res.toString("utf-8"), "Hello World!\n");
        });

        test("interprets echo.lo", async () => {
            const res = await interpret("./examples/test/demos/echo.lo");
            // NOTE: no stdin provided so no output
            assert.strictEqual(res.toString("utf-8"), "");
        });

        test("interprets args.test.lo", async () => {
            const res = await interpret("./examples/test/args.test.lo");
            assert.strictEqual(
                res.toString("utf-8"),
                m`
                lo
                ./examples/test/args.test.lo
                --eval

                `
            );
        });

        // TODO: provide a way to pass own args to --eval
        test("interprets cat.lo", async () => {
            try {
                await interpret("./examples/test/demos/cat.lo");
            } catch (err) {
                assert.strictEqual(err.message, "Usage cat.lo <file>");
            }
        });

        test("interprets string-pooling.lo", async () => {
            const res = await interpret("./examples/test/string-pooling.lo");
            assert.strictEqual(res.toString("utf-8"), "14\n");
        });

        test("interprets tracing.lo", async () => {
            const res = await interpret("./examples/test/tracing.lo");
            assert.strictEqual(
                res.toString("utf-8"),
                "examples/test/tracing.lo:4:10 - hello there\n"
            );
        });

        test("interprets struct-in-struct.lo", async () => {
            const res = await interpret("./examples/test/struct-in-struct.lo");
            assert.strictEqual(res.toString("utf-8"), "3\n3\n3\n3\n3\n3\n3\n");
        });

        test("interprets heap-alloc.lo", async () => {
            const res = await interpret("./examples/test/heap-alloc.lo");
            assert.strictEqual(
                res.toString("utf-8"),
                m`
                p1 = 1048597
                p2 = 1048597
                p3 = 1048606

                `
            );
        });

        test("interprets defer.lo", async () => {
            const res = await interpret("./examples/test/defer.lo");
            assert.strictEqual(
                res.toString("utf-8"),
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

        test("interprets errors.lo", async () => {
            const res = await interpret("./examples/test/errors.lo");
            assert.strictEqual(
                res.toString("utf-8"),
                m`
                10 / 5 = 2, remainder = 0
                10 / 3 = 3, remainder = 1
                10 / 0 is undefined

                `
            );
        });

        // aoc

        // TODO: look into performance of part 2
        test("interprets aoc2020 day 1", async () => {
            const res1 = await interpret("./examples/test/demos/aoc2020/1.lo");
            assert.strictEqual(res1.toString("utf-8"), "157059\n");

            const res2 = await interpret(
                "./examples/test/demos/aoc2020/1-part2.lo"
            );
            assert.strictEqual(res2.toString("utf-8"), "165080960\n");
        });

        test("interprets aoc2020 day 2", async () => {
            const res1 = await interpret("./examples/test/demos/aoc2020/2.lo");
            assert.strictEqual(res1.toString("utf-8"), "560\n");

            const res2 = await interpret(
                "./examples/test/demos/aoc2020/2-part2.lo"
            );
            assert.strictEqual(res2.toString("utf-8"), "303\n");
        });

        test("interprets aoc2020 day 3", async () => {
            const res1 = await interpret("./examples/test/demos/aoc2020/3.lo");
            assert.strictEqual(res1.toString("utf-8"), "151\n");

            const res2 = await interpret(
                "./examples/test/demos/aoc2020/3-part2.lo"
            );
            assert.strictEqual(res2.toString("utf-8"), "7540141059\n");
        });

        test("interprets aoc2020 day 4", async () => {
            const res1 = await interpret("./examples/test/demos/aoc2020/4.lo");
            assert.strictEqual(res1.toString("utf-8"), "264\n");

            const res2 = await interpret(
                "./examples/test/demos/aoc2020/4-part2.lo"
            );
            assert.strictEqual(res2.toString("utf-8"), "224\n");
        });

        // TODO: look into performance of part 2
        test("interprets aoc2020 day 5", async () => {
            const res1 = await interpret("./examples/test/demos/aoc2020/5.lo");
            assert.strictEqual(res1.toString("utf-8"), "947\n");

            const res2 = await interpret(
                "./examples/test/demos/aoc2020/5-part2.lo"
            );
            assert.strictEqual(res2.toString("utf-8"), "636\n");
        });

        test("interprets aoc2023 day 1", async () => {
            const res1 = await interpret("./examples/test/demos/aoc2023/1.lo");
            assert.strictEqual(res1.toString("utf-8"), "54450\n");

            const res2 = await interpret(
                "./examples/test/demos/aoc2023/1-part2.lo"
            );
            assert.strictEqual(res2.toString("utf-8"), "54265\n");
        });
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

    /** @typedef {(sourcePath: string) => Promise<InstanceType<typeof global.Buffer>>} Compile */

    /**
     * @param {InstanceType<typeof global.Buffer>} compilerWasmBinary
     * @returns {Promise<Compile>}
     */
    async function loadCompilerWithWasiAPI(
        compilerWasmBinary,
        {
            mockStdin = false,
            buildArgs = (/** @type {string | undefined} */ fileName) => [
                "lo",
                fileName ?? "-i",
            ],
        } = {}
    ) {
        const mod = await WebAssembly.compile(compilerWasmBinary);

        /**
         * @param {string} [fileName]
         * @param {number} [stdinFd]
         */
        const compile = (fileName, stdinFd) =>
            runWithTmpFile((stderr, stderrFile) =>
                runWithTmpFile(async (stdout, stdoutFile) => {
                    const wasi = new WASI({
                        version: "preview1",
                        stdin: stdinFd,
                        stdout: stdout.fd,
                        stderr: stderr.fd,
                        args: buildArgs(fileName),
                        preopens: { ".": "." },
                    });

                    const instance = await WebAssembly.instantiate(mod, {
                        ...wasi.getImportObject(),
                        ...{ console: { ...console } },
                    });

                    try {
                        const exitCode = wasi.start(instance);

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

            return await runWithTmpFile(async (stdin, stdinFile) => {
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
 * @param {Omit<import("./wasi-shim.mjs").WASIOptions, 'version'>} [wasiOptions]
 * @returns {Promise<number>}
 */
async function runWASI(data, wasiOptions, additionalImports = {}) {
    const wasi = new WASI({ version: "preview1", ...wasiOptions });

    const wasm = await WebAssembly.compile(data);
    const instance = await WebAssembly.instantiate(wasm, {
        ...wasi.getImportObject(),
        ...{ console: { ...console } },
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
