#!/usr/bin/env -S node
// @ts-check

import process from "node:process";
import { describe, it, test } from "node:test";
import assert from "node:assert";
import fs from "node:fs/promises";

// main

const COMPILER_PATH = "lo.wasm";
const INITIAL_COMPILER_PATH = "initial-compiler/lo.wasm";

setTimeout(main, 0);

async function main() {
    process.chdir(import.meta.dirname);

    if (["build", "b"].includes(process.argv[2])) {
        const output = new WASI.VirtualFD();
        const exitCode = await runWASI(await fs.readFile(COMPILER_PATH), {
            args: ["lo", "compile", "src/0-cli.lo"],
            stdout: output,
            preopens: { ".": "." },
        });
        if (exitCode !== 0) {
            process.exit(exitCode ?? 1);
        }
        await fs.writeFile(COMPILER_PATH, output.flushAndRead());

        process.argv.splice(2, 1); // remove build arg
        if (process.argv.length == 2) {
            return; // stop if this was the only command
        }
    }

    if (process.argv[2] === "test") {
        return commandTest();
    }

    if (process.argv[2] === "run") {
        return commandRun(process.argv.slice(3));
    }

    if (process.argv[2] === "debugWasi") {
        return commandDebugWasi(process.argv.slice(3));
    }

    if (process.argv[2] === "runWasi") {
        const filePath = new URL(process.argv[3], import.meta.url);
        const input = await fs.readFile(filePath);
        return runWASI(input, {
            args: process.argv.slice(3),
            env: process.env,
            preopens: { ".": "." },
        });
    }

    return runWASI(await fs.readFile(COMPILER_PATH), {
        preopens: { ".": "." },
        args: ["lo", ...process.argv.slice(2)],
        returnOnExit: false,
    });
}

/** @param {string[]} compilerArgs */
async function commandRun(compilerArgs) {
    /** @type {string[]} */
    let programArgs = [];

    const programArgsStart = compilerArgs.indexOf("--");
    if (programArgsStart !== -1) {
        programArgs = compilerArgs.slice(programArgsStart + 1);
        compilerArgs = compilerArgs.slice(0, programArgsStart);
    }

    const stdout = new WASI.VirtualFD();
    const exitCode = await runWASI(await fs.readFile(COMPILER_PATH), {
        stdout,
        preopens: { ".": "." },
        args: ["lo", "compile", ...compilerArgs],
    });

    if (exitCode !== 0) {
        console.error("Compilation failed, see compiler error(s) above");
        process.exit(1);
    }

    const program = stdout.flushAndRead();
    await runWASI(program, {
        preopens: { ".": "." },
        returnOnExit: false,
        args: ["main.lo", ...programArgs],
    });
}

/**
 * Start an http server that runs provided WASI module for debugging with Dev Tools
 * @param {string[]} args
 */
async function commandDebugWasi(args) {
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

async function commandTest() {
    const lo = await loadLoCompiler(await fs.readFile(COMPILER_PATH));
    const compile = (/** @type {string} */ fileName) =>
        lo(["compile", fileName]);

    it("compiles 42.lo", async () => {
        const output = await compile("./test/42.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 42);
    });

    it("compiles hex-and-shifts.lo", async () => {
        const output = await compile("./test/hex-and-shifts.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 31);
    });

    it("compiles add.lo", async () => {
        const output = await compile("./test/add.lo");

        const program = await loadWasm(output);

        assert.strictEqual(program.add(2, 3), 5);
    });

    it("compiles locals.lo", async () => {
        const output = await compile("./test/locals.lo");

        const program = await loadWasm(output);
        assert.deepEqual(program.sub(5, 3), 2);
    });

    it("compiles short-circuit.lo", async () => {
        const output = await compile("./test/short-circuit.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 12);
    });

    it("compiles factorial.lo", async () => {
        const output = await compile("./test/factorial.lo");

        const program = await loadWasm(output);
        const result = program.factorial(5);

        assert.strictEqual(result, 120);
    });

    it("compiles nested-fn.lo", async () => {
        const output = await compile("./test/nested-fn.lo");

        const program = await loadWasm(output);
        assert.strictEqual(program.main(), 120);
    });

    it("compiles include.lo", async () => {
        const output = await compile("./test/include.lo");

        const program = await loadWasm(output);

        assert.strictEqual(program.main(), 120);
    });

    it("compiles else-if.lo", async () => {
        const output = await compile("./test/else-if.lo");

        const program = await loadWasm(output);

        assert.strictEqual(program.main(), 13);
    });

    it("compiles import.lo", async () => {
        const output = await compile("./test/import.lo");

        /** @type {unknown[]} */
        const logs = [];
        const program = await loadWasm(output, {
            utils: { debug: (/** @type {unknown} */ x) => logs.push(x) },
        });

        program.main();
        assert.deepEqual(logs, [1, 2, 3]);
    });

    it("compiles hello-world-raw.lo", async () => {
        const program = await compile("./test/demos/hello-world-raw.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(output, "Hello World!\n");
    });

    it("compiles globals.lo", async () => {
        const output = await compile("./test/globals.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 69);
    });

    it("compiles while-loop.lo", async () => {
        const output = await compile("./test/while-loop.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 120);
    });

    it("compiles for-loop.lo", async () => {
        const output = await compile("./test/for-loop.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 138);
    });

    it("compiles for-each-loop.lo", async () => {
        const output = await compile("./test/for-each-loop.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 9);
    });

    it("compiles methods.lo", async () => {
        const output = await compile("./test/methods.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 1);
    });

    it("compiles nested-if-break.lo", async () => {
        const output = await compile("./test/nested-if-break.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 1);
    });

    it("compiles struct.lo", async () => {
        const output = await compile("./test/struct.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 13);
    });

    it("compiles auto-forward-decl.lo", async () => {
        const output = await compile("./test/auto-forward-decl.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 42);
    });

    it("compiles struct-value-field-access.lo", async () => {
        const output = await compile("./test/struct-value-field-access.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 18);
    });

    it("compiles decl-nesting.lo", async () => {
        const output = await compile("./test/decl-nesting.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 16);
    });

    it("compiles struct-ref.lo", async () => {
        const output = await compile("./test/struct-ref.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 3);
    });

    it("compiles inline-fn.lo", async () => {
        const output = await compile("./test/inline-fn.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 16);
    });

    it("compiles wasi.lo", async () => {
        const output = await compile("./lib/wasi.lo");

        const wasi = new WASI({
            version: "preview1",
            sysCalls: await WASI.NodeSysCalls(),
        });
        const wasm = await WebAssembly.compile(output);
        await WebAssembly.instantiate(wasm, wasi.getImportObject());
    });

    it("compiles string-pooling.lo", async () => {
        const output = await compile("./test/string-pooling.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 13);
    });

    it("compiles zst-noop.lo", async () => {
        const output = await compile("./test/zst-noop.lo");

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, undefined);
    });

    it("compiles std.lo", async () => {
        const output = await compile("./lib/std.lo");

        await loadWasm(output);
    });

    it("compiles vec.lo", async () => {
        const output = await compile("./test/vec.test.lo");
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
         * @param {WebAssembly.Memory} memory
         * @param {number} ptr
         * @param {Uint8Array} data
         */
        function storeData(memory, ptr, data) {
            const region = { ptr, size: data.byteLength };

            new Uint8Array(memory.buffer, region.ptr, region.size).set(data);

            return region;
        }
    });

    it("compiles hello-world.lo", async () => {
        const program = await compile("./test/demos/hello-world.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(output, "Hello World!\n");
    });

    it("compiles echo.lo", async () => {
        const program = await compile("./test/demos/echo.lo");

        const stdin = new WASI.VirtualFD();
        stdin.write(new TextEncoder().encode("abc"));
        stdin.flush();

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdin, stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(output, "abc");
    });

    it("compiles args.test.lo", async () => {
        const program = await compile("./test/args.test.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout, args: ["123", "456", "789"] });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(output, "123\n456\n789\n");
    });

    it("compiles cat.lo", async () => {
        const program = await compile("./test/demos/cat.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, {
            stdout,
            args: ["cat.lo", "test/42.lo"],
            preopens: { ".": "." },
        });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(output, await fs.readFile("test/42.lo", "utf-8"));
    });

    it("compiles tracing.lo", async () => {
        const program = await compile("./test/tracing.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(output, "test/tracing.lo:4:5 - hello there\n");
    });

    it("compiles struct-in-struct.lo", async () => {
        const program = await compile("./test/struct-in-struct.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(output, "3\n3\n3\n3\n3\n3\n3\n");
    });

    it("compiles heap-alloc.lo", async () => {
        const program = await compile("./test/heap-alloc.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(
            output,
            m`
            p1 = 1048604
            p2 = 1048604
            p3 = 1048620

            `,
        );
    });

    it("compiles defer.lo", async () => {
        const program = await compile("./test/defer.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(
            output,
            m`
            defer(inner_fn): loop (iteration #1)
            defer(inner_fn): loop (iteration #2)
            defer(inner_fn): loop (iteration #3)
            defer(inner_fn): top level
            defer(main): 3
            defer(main): 2
            defer(main): 1

            `,
        );
    });

    it("compiles errors.lo", async () => {
        const program = await compile("./test/errors.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(
            output,
            m`
            10 / 5 = 2, remainder = 0
            10 / 3 = 3, remainder = 1
            10 / 0 is undefined

            `,
        );
    });

    it("compiles do-with.lo", async () => {
        const program = await compile("./test/do-with.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(
            output,
            m`
            123456789
            -------------
            { "type": "file", "index": 1, "path": "some_path.lo" },

            `,
        );
    });

    it("compiles enums.lo", async () => {
        const program = await compile("./test/enums.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(
            output,
            m`
            +a

            `,
        );
    });

    it("compiles if-match-chain.lo", async () => {
        const program = await compile("./test/if-match-chain.lo");

        const stdout = new WASI.VirtualFD();
        await runWASI(program, { stdout });
        const output = stdout.flushAndReadUtf8();

        assert.strictEqual(
            output,
            m`
            going left 5 steps
            going right 3 steps

            `,
        );
    });

    it("compiles kitchen-sink.lo", async () => {
        const program = await compile("./test/kitchen-sink.lo");
        const exports = await loadWasm(
            /** @type {Uint8Array<ArrayBuffer>} */ (program),
        );

        assert.strictEqual(exports.main(), 0);
    });

    it("compiles 42.lo with <stdin> input", async () => {
        const stdin = new WASI.VirtualFD();
        stdin.write(await fs.readFile("./test/42.lo"));
        stdin.flush();

        const output = await lo(["compile", "-i"], { stdin });

        const program = await loadWasm(output);
        const result = program.main();

        assert.strictEqual(result, 42);
    });

    describe("aoc", () => {
        it("compiles 2020 day 1", async () => {
            const part1 = await runAoc(compile, "./test/demos/aoc2020/1.lo");
            assert.strictEqual(part1, "157059\n");

            const part2 = await runAoc(
                compile,
                "./test/demos/aoc2020/1-part2.lo",
            );
            assert.strictEqual(part2, "165080960\n");
        });

        it("compiles 2020 day 2", async () => {
            const part1 = await runAoc(compile, "./test/demos/aoc2020/2.lo");
            assert.strictEqual(part1, "560\n");

            const part2 = await runAoc(
                compile,
                "./test/demos/aoc2020/2-part2.lo",
            );
            assert.strictEqual(part2, "303\n");
        });

        it("compiles 2020 day 3", async () => {
            const part1 = await runAoc(compile, "./test/demos/aoc2020/3.lo");
            assert.strictEqual(part1, "151\n");

            const part2 = await runAoc(
                compile,
                "./test/demos/aoc2020/3-part2.lo",
            );
            assert.strictEqual(part2, "7540141059\n");
        });

        it("compiles 2020 day 4", async () => {
            const part1 = await runAoc(compile, "./test/demos/aoc2020/4.lo");
            assert.strictEqual(part1, "264\n");

            const part2 = await runAoc(
                compile,
                "./test/demos/aoc2020/4-part2.lo",
            );
            assert.strictEqual(part2, "224\n");
        });

        it("compiles 2020 day 5", async () => {
            const part1 = await runAoc(compile, "./test/demos/aoc2020/5.lo");
            assert.strictEqual(part1, "947\n");

            const part2 = await runAoc(
                compile,
                "./test/demos/aoc2020/5-part2.lo",
            );
            assert.strictEqual(part2, "636\n");
        });

        it("compiles 2023 day 1", async () => {
            const part1 = await runAoc(compile, "./test/demos/aoc2023/1.lo");
            assert.strictEqual(part1, "54450\n");

            const part2 = await runAoc(
                compile,
                "./test/demos/aoc2023/1-part2.lo",
            );
            assert.strictEqual(part2, "54265\n");
        });

        /**
         * @param {(sourcePath: string) => Promise<Uint8Array<ArrayBuffer>>} compile
         * @param {string} path
         */
        async function runAoc(compile, path) {
            const program = await compile(path);

            const stdout = new WASI.VirtualFD();
            const exitCode = await runWASI(program, {
                stdout,
                preopens: { ".": "." },
            });

            if (exitCode !== 0) {
                throw new Error(`Process exited with error code: ${exitCode}`);
            }

            return stdout.flushAndReadUtf8();
        }
    });

    describe("wasm4", () => {
        const wasm4Imports = {
            env: new Proxy(
                { memory: new WebAssembly.Memory({ initial: 1 }) },
                {
                    get: (target, prop) =>
                        target[/** @type {keyof typeof target} */ (prop)] ??
                        (() => void 0),
                },
            ),
        };

        it("compiles blink.lo", async () => {
            const output = await compile("./test/demos/wasm4/src/blink.lo");
            await loadWasm(output, wasm4Imports);
        });

        it("compiles dark-maze.lo", async () => {
            const output = await compile("./test/demos/wasm4/src/dark-maze.lo");
            await loadWasm(output, wasm4Imports);
        });

        it("compiles slasher.lo", async () => {
            const output = await compile("./test/demos/wasm4/src/slasher.lo");
            await loadWasm(output, wasm4Imports);
        });
    });

    describe("error reporting", () => {
        it("reports multiple errors in fault-tolerance.lo", async () => {
            try {
                await compile("./test/compile-errors/fault-tolerance.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/fault-tolerance.lo:10:4 - Cannot redefine main, previously defined at test/compile-errors/fault-tolerance.lo:2:4
                    ERROR: test/compile-errors/fault-tolerance.lo:2:17 - Cannot redefine a, previously defined at test/compile-errors/fault-tolerance.lo:2:9
                    ERROR: test/compile-errors/fault-tolerance.lo:5:9 - Cannot redefine x, previously defined at test/compile-errors/fault-tolerance.lo:3:9
                    ERROR: test/compile-errors/fault-tolerance.lo:13:9 - Cannot redefine x, previously defined at test/compile-errors/fault-tolerance.lo:11:9

                    `,
                );
            }
        });

        it("resolves types defined out of order and reports errors in case of unsizable types", async () => {
            try {
                await compile("./test/compile-errors/out-of-order-defs.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/out-of-order-defs.lo:10:10 - Cannot determite size of recursive type
                    ERROR: test/compile-errors/out-of-order-defs.lo:13:11 - Cannot determite size of recursive type
                    ERROR: test/compile-errors/out-of-order-defs.lo:22:8 - Cannot determite size of recursive type

                    `,
                );
            }
        });

        it("reports expression not allowed in const context", async () => {
            try {
                await compile("./test/compile-errors/const-context.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/const-context.lo:6:9 - Expression not allowed in const context
                    ERROR: test/compile-errors/const-context.lo:9:9 - Expression not allowed in const context

                    `,
                );
            }
        });

        it("reports incompatible operator for enum payload comparison in enum-payload-compare.lo", async () => {
            try {
                await compile("./test/compile-errors/enum-payload-compare.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/enum-payload-compare.lo:11:15 - Operator \`==\` is incompatible with operands of type Shape

                    `,
                );
            }
        });

        it("reports forbidden memory operations in forbid-memory.lo", async () => {
            try {
                await compile("./test/compile-errors/forbid-memory.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/forbid-memory.lo:4:2 - Cannot both forbid and define a memory, forbidden at test/compile-errors/forbid-memory.lo:1:2
                    ERROR: test/compile-errors/forbid-memory.lo:14:13 - Memory operations forbidden, see test/compile-errors/forbid-memory.lo:1:2
                    ERROR: test/compile-errors/forbid-memory.lo:17:13 - Memory operations forbidden, see test/compile-errors/forbid-memory.lo:1:2
                    ERROR: test/compile-errors/forbid-memory.lo:20:15 - Memory operations forbidden, see test/compile-errors/forbid-memory.lo:1:2
                    ERROR: test/compile-errors/forbid-memory.lo:23:7 - Memory operations forbidden, see test/compile-errors/forbid-memory.lo:1:2
                    ERROR: test/compile-errors/forbid-memory.lo:23:5 - Memory operations forbidden, see test/compile-errors/forbid-memory.lo:1:2
                    ERROR: test/compile-errors/forbid-memory.lo:29:14 - Memory operations forbidden, see test/compile-errors/forbid-memory.lo:1:2

                    `,
                );
            }
        });

        it("reports redefinition across includes in include-redefinition.lo", async () => {
            try {
                await compile("./test/compile-errors/include-redefinition.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/include-redefinition.lo:4:6 - Cannot redefine Point, already defined at test/compile-errors/lib/point.lo:1:6

                    `,
                );
            }
        });

        it("reports incompatible operator in incompatible-operator.lo", async () => {
            try {
                await compile("./test/compile-errors/incompatible-operator.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/incompatible-operator.lo:11:15 - Operator \`==\` is incompatible with operands of type Point

                    `,
                );
            }
        });

        it("reports invalid infer call arity in infer-arity.lo", async () => {
            try {
                await compile("./test/compile-errors/infer-arity.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/infer-arity.lo:1:21 - Invalid \`infer\` call, expected 1 named type argument

                    `,
                );
            }
        });

        it("reports invalid infer call for non-named type in infer-not-named.lo", async () => {
            try {
                await compile("./test/compile-errors/infer-not-named.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/infer-not-named.lo:1:21 - Invalid \`infer\` call, expected 1 named type argument

                    `,
                );
            }
        });

        it("reports invalid inline fn args in inline-fn-args.lo", async () => {
            try {
                await compile("./test/compile-errors/inline-fn-args.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/inline-fn-args.lo:7:13 - Invalid inline fn args, expected [u32], got [bool]

                    `,
                );
            }
        });

        it("reports invalid inline fn arity in inline-fn-arity.lo", async () => {
            try {
                await compile("./test/compile-errors/inline-fn-arity.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/inline-fn-arity.lo:7:13 - Invalid number of inline fn args, expected 1, got 2

                    `,
                );
            }
        });

        it("reports invalid inline fn type args in inline-fn-type-args.lo", async () => {
            try {
                await compile("./test/compile-errors/inline-fn-type-args.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/inline-fn-type-args.lo:7:13 - Invalid number of type args, expected 1, got 2

                    `,
                );
            }
        });

        it("reports invalid assign target in invalid-assign-target.lo", async () => {
            try {
                await compile("./test/compile-errors/invalid-assign-target.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/invalid-assign-target.lo:7:5 - Invalid lhs

                    `,
                );
            }
        });

        it("reports invalid condition type in invalid-condition-type.lo", async () => {
            try {
                await compile(
                    "./test/compile-errors/invalid-condition-type.lo",
                );
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/invalid-condition-type.lo:3:8 - Invalid condition type: u32, expected: bool

                    `,
                );
            }
        });

        it("reports malformed struct literal errors in malformed-struct-literal.lo", async () => {
            try {
                await compile(
                    "./test/compile-errors/malformed-struct-literal.lo",
                );
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/malformed-struct-literal.lo:8:13 - Missing struct fields: y, z
                    ERROR: test/compile-errors/malformed-struct-literal.lo:9:13 - Missing struct fields: z
                    ERROR: test/compile-errors/malformed-struct-literal.lo:10:29 - Unexpected struct field name, expecting: \`y\`
                    ERROR: test/compile-errors/malformed-struct-literal.lo:10:13 - Missing struct fields: y
                    ERROR: test/compile-errors/malformed-struct-literal.lo:11:23 - Unexpected struct field name, expecting: \`x\`
                    ERROR: test/compile-errors/malformed-struct-literal.lo:11:29 - Unexpected struct field name, expecting: \`y\`
                    ERROR: test/compile-errors/malformed-struct-literal.lo:11:13 - Missing struct fields: x
                    ERROR: test/compile-errors/malformed-struct-literal.lo:12:23 - Unexpected struct field name, expecting: \`x\`
                    ERROR: test/compile-errors/malformed-struct-literal.lo:12:29 - Unexpected struct field name, expecting: \`y\`
                    ERROR: test/compile-errors/malformed-struct-literal.lo:12:35 - Unexpected struct field name, expecting: \`z\`
                    ERROR: test/compile-errors/malformed-struct-literal.lo:13:41 - Excess field values

                    `,
                );
            }
        });

        it("reports mismatched array item type in mismatch-array-item.lo", async () => {
            try {
                await compile("./test/compile-errors/mismatch-array-item.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/mismatch-array-item.lo:3:22 - Unexpected array element type: bool, expected: u32

                    `,
                );
            }
        });

        it("reports mismatched assignment type in mismatch-assign.lo", async () => {
            try {
                await compile("./test/compile-errors/mismatch-assign.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/mismatch-assign.lo:5:7 - Cannot assign bool to variable of type u32

                    `,
                );
            }
        });

        it("reports mismatched enum payload type in mismatch-enum-payload.lo", async () => {
            try {
                await compile("./test/compile-errors/mismatch-enum-payload.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/mismatch-enum-payload.lo:8:13 - Invalid enum payload: bool, expected: u32

                    `,
                );
            }
        });

        it("reports mismatched function arguments in mismatch-fn-args.lo", async () => {
            try {
                await compile("./test/compile-errors/mismatch-fn-args.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/mismatch-fn-args.lo:7:13 - Invalid function arguments for function add: [u32, bool], expected [u32, u32]
                    ERROR: test/compile-errors/mismatch-fn-args.lo:10:13 - Invalid function arguments for function add: [u32], expected [u32, u32]

                    `,
                );
            }
        });

        it("reports mismatched result literal types in mismatch-result-literal.lo", async () => {
            try {
                await compile(
                    "./test/compile-errors/mismatch-result-literal.lo",
                );
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/mismatch-result-literal.lo:3:12 - Cannot create result, Ok type mismatch. Got bool, expected: u32
                    ERROR: test/compile-errors/mismatch-result-literal.lo:8:12 - Cannot create result, Err type mismatch. Got u32, expected: bool
                    ERROR: test/compile-errors/mismatch-result-literal.lo:6:4 - Control reaches end of non-void function

                    `,
                );
            }
        });

        it("reports mismatched return type in mismatch-return.lo", async () => {
            try {
                await compile("./test/compile-errors/mismatch-return.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/mismatch-return.lo:3:5 - Invalid return type: bool, expected: u32

                    `,
                );
            }
        });

        it("reports mismatched struct field type in mismatch-struct-field.lo", async () => {
            try {
                await compile("./test/compile-errors/mismatch-struct-field.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/mismatch-struct-field.lo:8:30 - Invalid type for struct field Point.y, expected: u32, got: bool

                    `,
                );
            }
        });

        it("reports missing return in missing-return.lo", async () => {
            try {
                await compile("./test/compile-errors/missing-return.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/missing-return.lo:2:4 - Control reaches end of non-void function

                    `,
                );
            }
        });

        it("reports non-void expression in non-void-expression.lo", async () => {
            try {
                await compile("./test/compile-errors/non-void-expression.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/non-void-expression.lo:8:9 - Non void expression in block. Use \`let _ = <expr>\` to ignore expression result.

                    `,
                );
            }
        });

        it("reports redefined global in redefine-global.lo", async () => {
            try {
                await compile("./test/compile-errors/redefine-global.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/redefine-global.lo:4:5 - Cannot redefine limit, previously defined at test/compile-errors/redefine-global.lo:1:5

                    `,
                );
            }
        });

        it("reports invalid @unreachable() arguments in unreachable-args.lo", async () => {
            try {
                await compile("./test/compile-errors/unreachable-args.lo");
            } catch (err) {
                assert.strictEqual(
                    /** @type {Error} */ (err).message,
                    m`
                    ERROR: test/compile-errors/unreachable-args.lo:3:6 - Invalid arguments for \`@unreachable(): never\`

                    `,
                );
            }
        });

        it("reports unreachable expression warnings in unreachable-expression.lo", async () => {
            const stderr = new WASI.VirtualFD();
            await lo(
                ["compile", "./test/compile-errors/unreachable-expression.lo"],
                { stderr },
            );

            assert.strictEqual(
                stderr.flushAndReadUtf8(),
                m`
                WARNING: test/compile-errors/unreachable-expression.lo:5:5 - Unreachable expression
                WARNING: test/compile-errors/unreachable-expression.lo:6:5 - Unreachable expression

                `,
            );
        });
    });

    describe("inspect", () => {
        it("emits partial diagnostics and multiple errors in fault-tolerance.lo", async () => {
            const stdout = new WASI.VirtualFD();
            try {
                await lo(
                    ["inspect", "./test/compile-errors/fault-tolerance.lo"],
                    { stdout },
                );
            } catch (err) {
                assert.strictEqual(/** @type {Error} */ (err).message, "");

                assert.strictEqual(
                    stdout.flushAndReadUtf8(),
                    m`
                    [
                    { "type": "file", "index": 1, "path": "test/compile-errors/fault-tolerance.lo" },
                    { "type": "message", "content": "Cannot redefine main, previously defined at test/compile-errors/fault-tolerance.lo:2:4", "severity": "error", "loc": "1/10:4-10:8" },
                    { "type": "info", "hover": "let a: u32", "loc": "1/2:9-2:10" },
                    { "type": "info", "hover": "let a: u32", "loc": "1/2:17-2:18" },
                    { "type": "message", "content": "Cannot redefine a, previously defined at test/compile-errors/fault-tolerance.lo:2:9", "severity": "error", "loc": "1/2:17-2:18" },
                    { "type": "info", "hover": "let x: u32", "loc": "1/3:9-3:10" },
                    { "type": "info", "hover": "let x: u32", "loc": "1/5:9-5:10" },
                    { "type": "message", "content": "Cannot redefine x, previously defined at test/compile-errors/fault-tolerance.lo:3:9", "severity": "error", "loc": "1/5:9-5:10" },
                    { "type": "info", "link": "1/3:9-3:10", "hover": "let x: u32", "loc": "1/6:13-6:14" },
                    { "type": "info", "hover": "let y: u32", "loc": "1/6:9-6:10" },
                    { "type": "info", "hover": "let x: u32", "loc": "1/11:9-11:10" },
                    { "type": "info", "hover": "let x: u32", "loc": "1/13:9-13:10" },
                    { "type": "message", "content": "Cannot redefine x, previously defined at test/compile-errors/fault-tolerance.lo:11:9", "severity": "error", "loc": "1/13:9-13:10" },
                    { "type": "info", "link": "1/11:9-11:10", "hover": "let x: u32", "loc": "1/14:13-14:14" },
                    { "type": "info", "hover": "let y: u32", "loc": "1/14:9-14:10" },
                    { "type": "end" }
                    ]

                    `,
                );
            }
        });

        it("handles struct literal errors", async () => {
            const stdout = new WASI.VirtualFD();
            try {
                await lo(
                    [
                        "inspect",
                        "./test/compile-errors/malformed-struct-literal.lo",
                    ],
                    { stdout },
                );
            } catch (err) {
                assert.strictEqual(/** @type {Error} */ (err).message, "");

                assert.strictEqual(
                    stdout.flushAndReadUtf8(),
                    m`
                    [
                    { "type": "file", "index": 1, "path": "test/compile-errors/malformed-struct-literal.lo" },
                    { "type": "info", "hover": "Point3D.x: u32", "loc": "1/2:5-2:6" },
                    { "type": "info", "hover": "Point3D.y: u32", "loc": "1/3:5-3:6" },
                    { "type": "info", "hover": "Point3D.z: u32", "loc": "1/4:5-4:6" },
                    { "type": "info", "link": "1/1:6-1:13", "hover": "struct { ... }", "loc": "1/8:13-8:20" },
                    { "type": "info", "link": "1/2:5-2:6", "hover": "Point3D.x: u32", "loc": "1/8:23-8:24" },
                    { "type": "message", "content": "Missing struct fields: y, z", "severity": "error", "loc": "1/8:13-8:20" },
                    { "type": "info", "hover": "let _: Point3D", "loc": "1/8:9-8:10" },
                    { "type": "info", "link": "1/1:6-1:13", "hover": "struct { ... }", "loc": "1/9:13-9:20" },
                    { "type": "info", "link": "1/2:5-2:6", "hover": "Point3D.x: u32", "loc": "1/9:23-9:24" },
                    { "type": "info", "link": "1/3:5-3:6", "hover": "Point3D.y: u32", "loc": "1/9:29-9:30" },
                    { "type": "message", "content": "Missing struct fields: z", "severity": "error", "loc": "1/9:13-9:20" },
                    { "type": "info", "hover": "let _: Point3D", "loc": "1/9:9-9:10" },
                    { "type": "info", "link": "1/1:6-1:13", "hover": "struct { ... }", "loc": "1/10:13-10:20" },
                    { "type": "info", "link": "1/2:5-2:6", "hover": "Point3D.x: u32", "loc": "1/10:23-10:24" },
                    { "type": "message", "content": "Unexpected struct field name, expecting: \`y\`", "severity": "error", "loc": "1/10:29-10:30" },
                    { "type": "info", "link": "1/3:5-3:6", "hover": "Point3D.y: u32", "loc": "1/10:29-10:30" },
                    { "type": "message", "content": "Missing struct fields: y", "severity": "error", "loc": "1/10:13-10:20" },
                    { "type": "info", "hover": "let _: Point3D", "loc": "1/10:9-10:10" },
                    { "type": "info", "link": "1/1:6-1:13", "hover": "struct { ... }", "loc": "1/11:13-11:20" },
                    { "type": "message", "content": "Unexpected struct field name, expecting: \`x\`", "severity": "error", "loc": "1/11:23-11:24" },
                    { "type": "info", "link": "1/2:5-2:6", "hover": "Point3D.x: u32", "loc": "1/11:23-11:24" },
                    { "type": "message", "content": "Unexpected struct field name, expecting: \`y\`", "severity": "error", "loc": "1/11:29-11:30" },
                    { "type": "info", "link": "1/3:5-3:6", "hover": "Point3D.y: u32", "loc": "1/11:29-11:30" },
                    { "type": "message", "content": "Missing struct fields: x", "severity": "error", "loc": "1/11:13-11:20" },
                    { "type": "info", "hover": "let _: Point3D", "loc": "1/11:9-11:10" },
                    { "type": "info", "link": "1/1:6-1:13", "hover": "struct { ... }", "loc": "1/12:13-12:20" },
                    { "type": "message", "content": "Unexpected struct field name, expecting: \`x\`", "severity": "error", "loc": "1/12:23-12:24" },
                    { "type": "info", "link": "1/2:5-2:6", "hover": "Point3D.x: u32", "loc": "1/12:23-12:24" },
                    { "type": "message", "content": "Unexpected struct field name, expecting: \`y\`", "severity": "error", "loc": "1/12:29-12:30" },
                    { "type": "info", "link": "1/3:5-3:6", "hover": "Point3D.y: u32", "loc": "1/12:29-12:30" },
                    { "type": "message", "content": "Unexpected struct field name, expecting: \`z\`", "severity": "error", "loc": "1/12:35-12:36" },
                    { "type": "info", "link": "1/4:5-4:6", "hover": "Point3D.z: u32", "loc": "1/12:35-12:36" },
                    { "type": "info", "hover": "let _: Point3D", "loc": "1/12:9-12:10" },
                    { "type": "info", "link": "1/1:6-1:13", "hover": "struct { ... }", "loc": "1/13:13-13:20" },
                    { "type": "info", "link": "1/2:5-2:6", "hover": "Point3D.x: u32", "loc": "1/13:23-13:24" },
                    { "type": "info", "link": "1/3:5-3:6", "hover": "Point3D.y: u32", "loc": "1/13:29-13:30" },
                    { "type": "info", "link": "1/4:5-4:6", "hover": "Point3D.z: u32", "loc": "1/13:35-13:36" },
                    { "type": "message", "content": "Excess field values", "severity": "error", "loc": "1/13:41-13:42" },
                    { "type": "info", "hover": "let _: Point3D", "loc": "1/13:9-13:10" },
                    { "type": "end" }
                    ]

                    `,
                );
            }
        });

        it("handles memory forbids", async () => {
            const stdout = new WASI.VirtualFD();
            try {
                await lo(
                    ["inspect", "./test/compile-errors/forbid-memory.lo"],
                    { stdout },
                );
            } catch (err) {
                assert.strictEqual(/** @type {Error} */ (err).message, "");

                assert.strictEqual(
                    stdout.flushAndReadUtf8(),
                    m`
                    [
                    { "type": "file", "index": 1, "path": "test/compile-errors/forbid-memory.lo" },
                    { "type": "info", "hover": "S.x: u32", "loc": "1/7:5-7:6" },
                    { "type": "message", "content": "Cannot both forbid and define a memory, forbidden at test/compile-errors/forbid-memory.lo:1:2", "severity": "error", "loc": "1/4:2-4:12" },
                    { "type": "info", "link": "1/6:6-6:7", "hover": "struct S { ... }", "loc": "1/11:19-11:20" },
                    { "type": "info", "hover": "let p: &S", "loc": "1/11:9-11:10" },
                    { "type": "message", "content": "Memory operations forbidden, see test/compile-errors/forbid-memory.lo:1:2", "severity": "error", "loc": "1/14:13-14:26" },
                    { "type": "info", "hover": "let _: seg(u8)", "loc": "1/14:9-14:10" },
                    { "type": "message", "content": "Memory operations forbidden, see test/compile-errors/forbid-memory.lo:1:2", "severity": "error", "loc": "1/17:13-17:27" },
                    { "type": "info", "hover": "let _: seg(u32)", "loc": "1/17:9-17:10" },
                    { "type": "info", "link": "1/11:9-11:10", "hover": "let p: &S", "loc": "1/20:13-20:14" },
                    { "type": "message", "content": "Memory operations forbidden, see test/compile-errors/forbid-memory.lo:1:2", "severity": "error", "loc": "1/20:15-20:16" },
                    { "type": "info", "link": "1/7:5-7:6", "hover": "&S.x: u32", "loc": "1/20:15-20:16" },
                    { "type": "info", "hover": "let _: u32", "loc": "1/20:9-20:10" },
                    { "type": "info", "link": "1/11:9-11:10", "hover": "let p: &S", "loc": "1/23:5-23:6" },
                    { "type": "message", "content": "Memory operations forbidden, see test/compile-errors/forbid-memory.lo:1:2", "severity": "error", "loc": "1/23:7-23:8" },
                    { "type": "info", "link": "1/7:5-7:6", "hover": "&S.x: u32", "loc": "1/23:7-23:8" },
                    { "type": "message", "content": "Memory operations forbidden, see test/compile-errors/forbid-memory.lo:1:2", "severity": "error", "loc": "1/23:5-23:8" },
                    { "type": "info", "link": "1/11:9-11:10", "hover": "let p: &S", "loc": "1/26:14-26:15" },
                    { "type": "info", "link": "1/7:5-7:6", "hover": "&S.x: u32", "loc": "1/26:16-26:17" },
                    { "type": "info", "hover": "let _: &u32", "loc": "1/26:9-26:10" },
                    { "type": "message", "content": "Memory operations forbidden, see test/compile-errors/forbid-memory.lo:1:2", "severity": "error", "loc": "1/29:14-29:30" },
                    { "type": "info", "hover": "<deref>: void", "loc": "1/29:14-29:15" },
                    { "type": "info", "hover": "let x2: void", "loc": "1/29:9-29:11" },
                    { "type": "end" }
                    ]

                    `,
                );
            }
        });
    });

    const formattedFiles = await fs
        .readdir(".", { recursive: true })
        .then((files) => files.filter((f) => !f.startsWith("docs")))
        .then((files) => files.filter((f) => f.endsWith(".lo")));

    describe("formatter", () => {
        if (process.argv.includes("--skip-fmt")) {
            return;
        }

        const format = async (fileName = "-i") => {
            const output = await lo(["format", fileName]);
            return new TextDecoder().decode(output);
        };

        for (const fileName of formattedFiles) {
            it(`formats ${fileName}`, async () => {
                const formatted = await format(fileName);
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

    test("self-hosted matches initial", async () => {
        const loInitial = await loadLoCompiler(
            await fs.readFile(INITIAL_COMPILER_PATH),
        );

        describe("formatter", () => {
            const format = async (fileName = "-i") => {
                const output = await lo(["format", fileName]);
                return new TextDecoder().decode(output);
            };

            for (const fileName of formattedFiles) {
                it(`formats ${fileName}`, async () => {
                    const formatted = await format(fileName);
                    const expected = await fs.readFile(fileName, "utf8");

                    assert.strictEqual(formatted, expected);
                });
            }
        });

        const TESTED_SAMPLES = [
            "test/42.lo",
            "test/hex-and-shifts.lo",
            "test/add.lo",
            "test/locals.lo",
            "test/short-circuit.lo",
            "test/factorial.lo",
            "test/nested-fn.lo",
            "test/include.lo",
            "test/else-if.lo",
            "test/import.lo",
            "test/demos/hello-world-raw.lo",
            "test/globals.lo",
            "test/while-loop.lo",
            "test/for-loop.lo",
            "test/for-each-loop.lo",
            "test/methods.lo",
            "test/nested-if-break.lo",
            "test/struct.lo",
            "test/auto-forward-decl.lo",
            "test/struct-value-field-access.lo",
            "test/decl-nesting.lo",
            "test/struct-ref.lo",
            "test/inline-fn.lo",
            "lib/wasi.lo",
            "test/string-pooling.lo",
            "test/zst-noop.lo",
            "lib/std.lo",
            "test/vec.test.lo",
            "test/demos/hello-world.lo",
            "test/demos/echo.lo",
            "test/args.test.lo",
            "test/demos/cat.lo",
            "test/tracing.lo",
            "test/struct-in-struct.lo",
            "test/heap-alloc.lo",
            "test/defer.lo",
            "test/errors.lo",
            "test/do-with.lo",
            "test/enums.lo",
            "test/special-field-access.lo",
            "test/if-match-chain.lo",
            "test/demos/aoc2020/1.lo",
            "test/demos/aoc2020/1-part2.lo",
            "test/demos/aoc2020/2.lo",
            "test/demos/aoc2020/2-part2.lo",
            "test/demos/aoc2020/3.lo",
            "test/demos/aoc2020/3-part2.lo",
            "test/demos/aoc2020/4.lo",
            "test/demos/aoc2020/4-part2.lo",
            "test/demos/aoc2020/5.lo",
            "test/demos/aoc2020/5-part2.lo",
            "test/demos/aoc2023/1.lo",
            "test/demos/aoc2023/1-part2.lo",
            "test/demos/wasm4/src/blink.lo",
            "test/demos/wasm4/src/dark-maze.lo",
            "test/demos/wasm4/src/slasher.lo",

            // extras
            "test/inline-let.lo",
            "test/wide-for-range.lo",
            "test/const-data.lo",
            "test/include-alias.lo",
            "test/backslashes.lo",
            "test/double-backslashes.lo",
            "test/expr-pipe.lo",
            "test/itemof.lo",
            "test/demos/vscode_wasm_issue_161.lo",
            "test/kitchen-sink.lo",

            // compiler itself
            "src/0-cli.lo",
        ];

        const TESTED_ERROR_SAMPLES = [
            "out-of-order-defs",
            "const-context",
            "enum-payload-compare",
            "fault-tolerance",
            "forbid-memory",
            "include-redefinition",
            "incompatible-operator",
            "infer-arity",
            "infer-not-named",
            "inline-fn-args",
            "inline-fn-arity",
            "inline-fn-type-args",
            "invalid-assign-target",
            "invalid-condition-type",
            "malformed-struct-literal",
            "mismatch-array-item",
            "mismatch-assign",
            "mismatch-enum-payload",
            "mismatch-fn-args",
            "mismatch-result-literal",
            "mismatch-return",
            "mismatch-struct-field",
            "missing-return",
            "non-void-expression",
            "redefine-global",
            "unreachable-args",
            "unreachable-expression",
        ];

        describe("compiler matches initial", () => {
            for (const fileName of TESTED_SAMPLES) {
                it(`compiles ${fileName}`, async () => {
                    const compiled1 = await lo(["compile", fileName]);
                    const compiled2 = await loInitial(["compile", fileName]);

                    assert.deepStrictEqual(compiled1, compiled2);
                });
            }
        });

        it("compiles 42.lo with <stdin> input", async () => {
            const stdin = new WASI.VirtualFD();
            stdin.write(await fs.readFile("./test/42.lo"));
            stdin.flush();

            const output = await lo(["compile", "-i"], { stdin });

            const program = await loadWasm(output);
            const result = program.main();

            assert.strictEqual(result, 42);
        });

        describe("compiler errors match initial", () => {
            /**
             * @param {typeof loInitial} compiler
             * @param {string} fileName
             * @returns {Promise<string>}
             */
            const outcome = async (compiler, fileName) => {
                const stderr = new WASI.VirtualFD();
                try {
                    const output = await compiler(["compile", fileName], {
                        stderr,
                    });
                    const warnings = stderr.flushAndReadUtf8();
                    return `ok: ${Buffer.from(output).toString("hex")}\n${warnings}`;
                } catch (err) {
                    return `error: ${/** @type {Error} */ (err).message}`;
                }
            };

            for (const file of TESTED_ERROR_SAMPLES) {
                const fileName = `test/compile-errors/${file}.lo`;
                it(`reports the same for ${fileName}`, async () => {
                    assert.strictEqual(
                        await outcome(lo, fileName),
                        await outcome(loInitial, fileName),
                    );
                });
            }
        });

        describe("compiler inspections match initial", () => {
            for (const fileName of TESTED_SAMPLES) {
                it(`inspects ${fileName}`, async () => {
                    const inspect1 = await lo(["inspect", fileName]);
                    const inspect2 = await loInitial(["inspect", fileName]);

                    assert.strictEqual(
                        new TextDecoder().decode(inspect1),
                        new TextDecoder().decode(inspect2),
                    );
                });
            }
        });

        describe("compiler error inspections match initial", () => {
            /**
             * @param {typeof loInitial} compiler
             * @param {string} fileName
             * @returns {Promise<string>}
             */
            const inspectOutput = async (compiler, fileName) => {
                const stdout = new WASI.VirtualFD();
                try {
                    await compiler(["inspect", fileName], { stdout });
                } catch {
                    // error is just generic exit 1 here, check stdout for more info
                }
                return new TextDecoder().decode(stdout.flushAndRead());
            };

            for (const file of TESTED_ERROR_SAMPLES) {
                const fileName = `test/compile-errors/${file}.lo`;

                it(`inspects ${fileName}`, async () => {
                    assert.strictEqual(
                        await inspectOutput(lo, fileName),
                        await inspectOutput(loInitial, fileName),
                    );
                });
            }
        });
    });
}

// utils

/** @param {Uint8Array} compilerWasmBinary */
async function loadLoCompiler(compilerWasmBinary) {
    const mod = await WebAssembly.compile(
        /** @type {Uint8Array<ArrayBuffer>} */ (compilerWasmBinary),
    );

    return async (
        args = ["help"],
        {
            stdin = new WASI.VirtualFD(),
            stdout = new WASI.VirtualFD(),
            stderr = new WASI.VirtualFD(),
        } = {},
    ) => {
        const wasi = new WASI({
            version: "preview1",
            stdin,
            stdout,
            stderr,
            args: ["lo", ...args],
            preopens: { ".": "." },
            sysCalls: await WASI.NodeSysCalls(),
        });

        const instance = await WebAssembly.instantiate(mod, {
            ...wasi.getImportObject(),
            ...{ console: { ...console } },
        });

        try {
            const exitCode = wasi.start(instance);

            if (exitCode ?? 0 !== 0) {
                throw new Error(stderr.flushAndReadUtf8());
            }

            return stdout.flushAndRead();
        } catch (err) {
            const errorMessage = stderr.flushAndReadUtf8();
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
    };
}

/**
 * @param {Uint8Array<ArrayBuffer>} data
 * @param {WebAssembly.Imports} [imports]
 */
async function loadWasm(data, imports) {
    const mod = await WebAssembly.instantiate(data, imports);
    return /** @type {any} */ (mod.instance.exports);
}

/**
 * @param {Uint8Array} data
 * @param {Omit<WASIOptions, "version" | "sysCalls">} [wasiOptions]
 * @param {Record<string, unknown>} [additionalImports]
 */
async function runWASI(data, wasiOptions, additionalImports = {}) {
    const wasi = new WASI({
        version: "preview1",
        sysCalls: await WASI.NodeSysCalls(),
        ...wasiOptions,
    });

    const wasm = await WebAssembly.compile(
        /** @type {Uint8Array<ArrayBuffer>} */ (data),
    );
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
                    memory.buffer.slice(0, 8),
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
 * @param {TemplateStringsArray} strings
 * @param {...unknown} args
 */
function m(strings, ...args) {
    const INDENT_PLACEHOLDER = "```";

    // interpolate
    const stringLines = strings
        .map(
            (str, index) =>
                str.replace(/\\\n[ \t]*/g, "").replace(/\\`/g, "`") +
                String(args[index] ?? "").replace(
                    /\n/g,
                    `\n${INDENT_PLACEHOLDER}`,
                ),
        )
        .join("")
        .split("\n")
        .slice(1);

    const lastLineIndentation = /** @type {string} */ (stringLines.pop())
        .length;

    // dedent
    return stringLines
        .map((str) =>
            str
                .split(INDENT_PLACEHOLDER)
                .join(" ".repeat(lastLineIndentation))
                .slice(lastLineIndentation),
        )
        .join("\n");
}

// WASI shim

/**
 * @typedef {{
 *   version: "preview1",
 *   stdin?: number | FD,
 *   stdout?: number | FD,
 *   stderr?: number | FD,
 *   args?: string[],
 *   env?: Record<string, string | undefined>,
 *   preopens?: Record<string, string>,
 *   returnOnExit?: boolean,
 *   trace?: boolean,
 *   sysCalls: WASISysCalls,
 * }} WASIOptions
 */

/**
 * @typedef {{
 *   processExit(exitCode: number): never,
 *   pathOpen(path: string, mode: "r" | "w" | "wx"): number,
 *   fdRead(fd: number, buffer: Uint8Array): number,
 *   fdWrite(fd: number, buffer: Uint8Array): number,
 *   fdClose(fd: number): void,
 *   listDir(path: string): Array<{ name: string; isDirectory: boolean }>,
 *   mkdir(path: string): void,
 * }} WASISysCalls
 */

/**
 * @typedef {{
 *   getPreopenDirName(): string,
 *   openChild(
 *     fileName: string,
 *     mode: "r" | "w" | "wx",
 *   ): { code: number, fd?: FD },
 *   createChildDir(dirName: string): number,
 *   read(buffer: Uint8Array): { code: number, nread: number },
 *   write(buffer: Uint8Array): { code: number, nwritten: number },
 *   close(): number,
 *   listDir(): {
 *     code: number,
 *     entries?: Array<{ name: string, isDirectory: boolean }>,
 *   },
 * }} FD
 */

const WASI_ERRNO_SUCCESS = 0;
const WASI_ERRNO_BADF = 8;
const WASI_ERRNO_NOENT = 2;
const WASI_ERRNO_EXIST = 20;
const WASI_ERRNO_INVAL = 28;

const WASI_FILETYPE_DIRECTORY = 3;
const WASI_FILETYPE_REGULAR_FILE = 4;

/** @implements {FD} */
class SysFD {
    /** @type {number | null} */
    sysFileFd;
    /** @type {string} */
    sysDirName;
    /** @type {WASISysCalls} */
    sysCalls;

    /**
     * @param {number | null} sysFileFd
     * @param {string} sysDirName
     * @param {WASISysCalls} sysCalls
     */
    constructor(sysFileFd, sysDirName, sysCalls) {
        this.sysFileFd = sysFileFd;
        this.sysDirName = sysDirName;
        this.sysCalls = sysCalls;
    }

    getPreopenDirName() {
        return this.sysDirName;
    }

    /**
     * @param {string} fileName
     * @param {"r" | "w" | "wx"} mode
     */
    openChild(fileName, mode) {
        const fullPath = `${this.sysDirName}/${fileName}`;
        try {
            const sysFd = this.sysCalls.pathOpen(fullPath, mode);
            const fd = new SysFD(sysFd, fullPath, this.sysCalls);

            return { code: WASI_ERRNO_SUCCESS, fd };
        } catch (err) {
            if (/** @type {{ code?: string }} */ (err)?.code === "EEXIST") {
                return { code: WASI_ERRNO_EXIST, fd: undefined };
            }
            return { code: WASI_ERRNO_NOENT, fd: undefined };
        }
    }

    /** @param {string} dirName */
    createChildDir(dirName) {
        const fullPath = `${this.sysDirName}/${dirName}`;
        try {
            this.sysCalls.mkdir(fullPath);
            return WASI_ERRNO_SUCCESS;
        } catch (err) {
            if (/** @type {{ code?: string }} */ (err)?.code === "EEXIST") {
                return WASI_ERRNO_EXIST;
            }
            return WASI_ERRNO_NOENT;
        }
    }

    listDir() {
        try {
            return {
                code: WASI_ERRNO_SUCCESS,
                entries: this.sysCalls.listDir(this.sysDirName),
            };
        } catch {
            return { code: WASI_ERRNO_NOENT, entries: undefined };
        }
    }

    /** @param {Uint8Array} buffer */
    read(buffer) {
        if (this.sysFileFd === null) {
            return { code: WASI_ERRNO_INVAL, nread: 0 };
        }

        try {
            const bytesRead = this.sysCalls.fdRead(this.sysFileFd, buffer);
            return { code: WASI_ERRNO_SUCCESS, nread: bytesRead };
        } catch {
            return { code: WASI_ERRNO_INVAL, nread: 0 };
        }
    }

    /** @param {Uint8Array} buffer */
    write(buffer) {
        if (this.sysFileFd === null) {
            return { code: WASI_ERRNO_INVAL, nwritten: 0 };
        }

        try {
            const bytesRead = this.sysCalls.fdWrite(this.sysFileFd, buffer);
            return { code: WASI_ERRNO_SUCCESS, nwritten: bytesRead };
        } catch {
            return { code: WASI_ERRNO_INVAL, nwritten: 0 };
        }
    }

    close() {
        if (this.sysFileFd === null || this.sysFileFd < 3) {
            return WASI_ERRNO_SUCCESS;
        }

        try {
            this.sysCalls.fdClose(this.sysFileFd);
            return WASI_ERRNO_SUCCESS;
        } catch {
            return WASI_ERRNO_INVAL;
        }
    }
}

/** @implements {FD} */
class VirtualFD {
    /** @type {Uint8Array<ArrayBuffer>} */
    contents;
    /** @type {number} */
    readCursor;
    /** @type {Uint8Array[]} */
    writes;

    constructor() {
        this.contents = new Uint8Array();
        this.readCursor = 0;
        this.writes = [];
    }

    /** @param {Uint8Array} buffer */
    read(buffer) {
        if (this.readCursor >= this.contents.length - 1) {
            return { code: WASI_ERRNO_SUCCESS, nread: 0 };
        }

        const bytesRead = Math.min(
            buffer.length,
            this.contents.length - this.readCursor,
        );
        buffer.set(
            this.contents.subarray(
                this.readCursor,
                this.readCursor + bytesRead,
            ),
        );

        this.readCursor += bytesRead;

        return { code: WASI_ERRNO_SUCCESS, nread: bytesRead };
    }

    /** @param {Uint8Array} buffer */
    write(buffer) {
        this.writes.push(buffer.slice());
        return { code: WASI_ERRNO_SUCCESS, nwritten: buffer.length };
    }

    flushAndRead() {
        this.flush();
        return this.contents;
    }

    flushAndReadUtf8() {
        this.flush();
        return new TextDecoder().decode(this.contents);
    }

    flush() {
        if (this.writes.length === 0) {
            return;
        }

        const newSize = this.writes.reduce((acc, w) => acc + w.length, 0);
        this.contents = new Uint8Array(newSize);

        let offset = 0;
        for (let i = 0; i < this.writes.length; i++) {
            const write = this.writes[i];
            this.contents.set(write, offset);
            offset += write.length;
        }

        this.writes = [];
    }

    close() {
        return WASI_ERRNO_SUCCESS;
    }

    /** @returns {never} */
    getPreopenDirName() {
        throw new Error("Not supported");
    }

    /** @returns {never} */
    openChild() {
        throw new Error("Not supported");
    }

    /** @returns {never} */
    createChildDir() {
        throw new Error("Not supported");
    }

    /** @returns {never} */
    listDir() {
        throw new Error("Not supported");
    }
}

export class WASI {
    static FD = SysFD;
    static VirtualFD = VirtualFD;

    /** @type {WASIOptions} */
    options;
    /** @type {string[]} */
    args;
    /** @type {number} */
    nextFd;
    /** @type {WebAssembly.Memory} */
    memory;
    /** @type {boolean} */
    returnOnExit;
    /** @type {WASISysCalls} */
    sysCalls;
    /** @type {FD[]} */
    fds;
    /** @type {number} */
    preopenCount;

    /** @param {WASIOptions} options */
    constructor(options) {
        this.options = options;
        this.args = this.options.args ?? [];
        this.nextFd = 3;
        this.memory = /** @type {never} */ (void 0);
        this.returnOnExit = this.options.returnOnExit ?? true;
        this.sysCalls = this.options.sysCalls;
        this.fds = [];

        if (typeof this.options.stdin === "object") {
            this.fds.push(this.options.stdin);
        } else {
            this.fds.push(
                new SysFD(this.options.stdin ?? 0, "<stdin>", this.sysCalls),
            );
        }

        if (typeof this.options.stdout === "object") {
            this.fds.push(this.options.stdout);
        } else {
            this.fds.push(
                new SysFD(this.options.stdout ?? 1, "<stdout>", this.sysCalls),
            );
        }

        if (typeof this.options.stderr === "object") {
            this.fds.push(this.options.stderr);
        } else {
            this.fds.push(
                new SysFD(this.options.stderr ?? 2, "<stderr>", this.sysCalls),
            );
        }

        for (const path of Object.values(this.options.preopens ?? {})) {
            this.fds.push(new SysFD(null, path, this.sysCalls));
        }

        this.preopenCount = this.fds.length;
    }

    /** @returns {Promise<WASISysCalls>} */
    static async NodeSysCalls() {
        const fs = await import("node:fs");
        const process = await import("node:process");

        return {
            processExit: (exitCode) => process.exit(exitCode),
            pathOpen: (path, mode) => fs.openSync(path, mode),
            fdRead: (fd, buffer) => fs.readSync(fd, buffer),
            fdWrite: (fd, buffer) => fs.writeSync(fd, buffer),
            mkdir: (path) => fs.mkdirSync(path),
            fdClose: (fd) => {
                try {
                    fs.closeSync(fd);
                } catch (err) {
                    console.error(err);
                    process.exit(1);
                }
            },
            listDir: (path) => [
                { name: ".", isDirectory: true },
                { name: "..", isDirectory: true },
                ...fs
                    .readdirSync(path, { withFileTypes: true })
                    .map((entry) => ({
                        name: entry.name,
                        isDirectory: entry.isDirectory(),
                    }))
                    .sort((a, b) =>
                        a.name < b.name ? -1 : a.name > b.name ? 1 : 0,
                    ),
            ],
        };
    }

    /**
     * @param {WebAssembly.Instance} instance
     * @returns {number}
     */
    start(instance) {
        this.memory = /** @type {WebAssembly.Memory} */ (
            instance.exports.memory
        );

        let exitCode = 0;

        try {
            if (typeof instance.exports._start !== "function") {
                throw new Error("_start function is not exported");
            }

            instance.exports._start();
        } catch (err) {
            if (!(err instanceof ProcExitError)) {
                throw err;
            }

            exitCode = err.exitCode;
        } finally {
            for (let fd = this.preopenCount; fd < this.fds.length; fd++) {
                this.fds[fd].close();
            }
        }

        if (!this.returnOnExit) {
            this.sysCalls.processExit(exitCode);
        }

        return exitCode;
    }

    /** @returns {WebAssembly.Imports} */
    getImportObject() {
        const imports = this.#getImportObject();
        if (!this.options.trace) {
            return imports;
        }

        for (const [fnName, fn] of Object.entries(
            imports.wasi_snapshot_preview1,
        )) {
            imports.wasi_snapshot_preview1[fnName] = (
                /** @type {unknown[]} */ ...args
            ) => {
                console.log(`[WASI] ${fnName}(${args.join(", ")})`);

                // @ts-ignore: fn is a function
                return fn(...args);
            };
        }

        return imports;
    }

    /** @returns {WebAssembly.Imports} */
    #getImportObject() {
        return {
            wasi_snapshot_preview1: {
                /**
                 * @type {(
                 *   dirfd: number,
                 *   _dirflags: number,
                 *   path_ptr: number,
                 *   path_len: number,
                 *   oflags: number,
                 *   _fs_rights_base: number,
                 *   _fs_rights_inheriting: number,
                 *   _fdflags: number,
                 *   fd_ptr: number,
                 * ) => number}
                 */
                path_open: (
                    dirfd,
                    _dirflags,
                    path_ptr,
                    path_len,
                    oflags,
                    _fs_rights_base,
                    _fs_rights_inheriting,
                    _fdflags,
                    fd_ptr,
                ) => {
                    const dir = this.fds[dirfd];
                    if (dir === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    const pathBytes = new Uint8Array(
                        this.memory.buffer,
                        path_ptr,
                        path_len,
                    );
                    const path = new TextDecoder()
                        .decode(pathBytes)
                        .replace(/\0/g, "");

                    const OFLAGS_CREAT = 1;
                    const OFLAGS_EXCL = 4;
                    const mode =
                        (oflags & OFLAGS_CREAT) !== 0 &&
                        (oflags & OFLAGS_EXCL) !== 0
                            ? "wx"
                            : "r";

                    const { code: err, fd: childFile } = dir.openChild(
                        path,
                        mode,
                    );
                    if (childFile !== undefined) {
                        this.fds.push(childFile);

                        const childFd = this.fds.length - 1;
                        const memory = new DataView(this.memory.buffer);
                        memory.setUint32(fd_ptr, childFd, true);
                    }

                    return err;
                },
                /**
                 * @type {(
                 *   dirfd: number,
                 *   path_ptr: number,
                 *   path_len: number,
                 * ) => number}
                 */
                path_create_directory: (dirfd, path_ptr, path_len) => {
                    const dir = this.fds[dirfd];
                    if (dir === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    const pathBytes = new Uint8Array(
                        this.memory.buffer,
                        path_ptr,
                        path_len,
                    );
                    const path = new TextDecoder()
                        .decode(pathBytes)
                        .replace(/\0/g, "");

                    return dir.createChildDir(path);
                },
                /**
                 * @type {(
                 *   fd: number,
                 *   iovs_ptr: number,
                 *   iovs_len: number,
                 *   nread_ptr: number,
                 * ) => number}
                 */
                fd_read: (fd, iovs_ptr, iovs_len, nread_ptr) => {
                    const file = this.fds[fd];
                    if (file === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    let totalBytesRead = 0;

                    const memory = new DataView(this.memory.buffer);

                    for (let i = 0; i < iovs_len; i++) {
                        const baseOffset = iovs_ptr + i * 8;
                        const bufPtr = memory.getUint32(baseOffset, true);
                        const bufLen = memory.getUint32(baseOffset + 4, true);

                        const buffer = new Uint8Array(
                            this.memory.buffer,
                            bufPtr,
                            bufLen,
                        );

                        const { code, nread } = file.read(buffer);
                        if (code != WASI_ERRNO_SUCCESS) {
                            return code;
                        }

                        totalBytesRead += nread;

                        if (nread < bufLen) {
                            break;
                        }
                    }

                    memory.setUint32(nread_ptr, totalBytesRead, true);
                    return WASI_ERRNO_SUCCESS;
                },
                /**
                 * @type {(
                 *   fd: number,
                 *   iovs_ptr: number,
                 *   iovs_len: number,
                 *   nwritten_ptr: number,
                 * ) => number}
                 */
                fd_write: (fd, iovs_ptr, iovs_len, nwritten_ptr) => {
                    const file = this.fds[fd];
                    if (file === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    let totalBytesWritten = 0;

                    const memory = new DataView(this.memory.buffer);

                    for (let i = 0; i < iovs_len; i++) {
                        const baseOffset = iovs_ptr + i * 8;
                        const bufPtr = memory.getUint32(baseOffset, true);
                        const bufLen = memory.getUint32(baseOffset + 4, true);

                        const buffer = new Uint8Array(
                            this.memory.buffer,
                            bufPtr,
                            bufLen,
                        );

                        const { code, nwritten } = file.write(buffer);
                        if (code != WASI_ERRNO_SUCCESS) {
                            return code;
                        }
                        totalBytesWritten += nwritten;

                        if (nwritten < bufLen) {
                            break;
                        }
                    }

                    memory.setUint32(nwritten_ptr, totalBytesWritten, true);
                    return WASI_ERRNO_SUCCESS;
                },
                fd_seek: () => {
                    throw new Error("Not Implemented");
                },
                /** @type {(fd: number) => number} */
                fd_close: (fd) => {
                    // don't allow closing preopens
                    if (fd < this.preopenCount) {
                        return WASI_ERRNO_INVAL;
                    }

                    const file = this.fds[fd];
                    if (file === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    const code = file.close();

                    if (code === WASI_ERRNO_SUCCESS) {
                        this.fds.splice(fd, 1);
                    }

                    return code;
                },
                /**
                 * @type {(
                 *   argc_ptr: number,
                 *   argv_buf_size_ptr: number,
                 * ) => number}
                 */
                args_sizes_get: (argc_ptr, argv_buf_size_ptr) => {
                    const memory = new DataView(this.memory.buffer);

                    const argc = this.args.length;
                    memory.setUint32(argc_ptr, argc, true);

                    const argvBufferSize = this.args.reduce(
                        (acc, arg) => acc + arg.length + 1,
                        0,
                    );
                    memory.setUint32(argv_buf_size_ptr, argvBufferSize, true);

                    return WASI_ERRNO_SUCCESS;
                },
                /** @type {(argv_ptr: number, argv_buf_ptr: number) => number} */
                args_get: (argv_ptr, argv_buf_ptr) => {
                    const memory = new DataView(this.memory.buffer);

                    const encoder = new TextEncoder();
                    let bufferOffset = argv_buf_ptr;

                    for (const [i, arg] of Object.entries(this.args)) {
                        const argBytes = encoder.encode(arg + "\0");
                        new Uint8Array(
                            this.memory.buffer,
                            bufferOffset,
                            argBytes.length,
                        ).set(argBytes);
                        memory.setUint32(
                            argv_ptr + Number(i) * 4,
                            bufferOffset,
                            true,
                        );
                        bufferOffset += argBytes.length;
                    }

                    return WASI_ERRNO_SUCCESS;
                },
                /** @type {(exit_code: number) => never} */
                proc_exit: (exit_code) => {
                    throw new ProcExitError(exit_code);
                },
                /** @type {(fd: number, buf_ptr: number) => number} */
                fd_prestat_get: (fd, buf_ptr) => {
                    // don't allow touching non-preopens
                    if (fd >= this.preopenCount) {
                        return WASI_ERRNO_INVAL;
                    }

                    const dir = this.fds[fd];
                    if (dir === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    const memory = new DataView(this.memory.buffer);

                    memory.setUint8(buf_ptr, 0); // dir

                    const dirName = dir.getPreopenDirName();
                    memory.setUint32(buf_ptr + 4, dirName.length, true);

                    return WASI_ERRNO_SUCCESS;
                },
                /**
                 * @type {(
                 *   fd: number,
                 *   path_ptr: number,
                 *   path_len: number,
                 * ) => number}
                 */
                fd_prestat_dir_name: (fd, path_ptr, path_len) => {
                    // don't allow touching non-preopens
                    if (fd >= this.preopenCount) {
                        return WASI_ERRNO_INVAL;
                    }

                    const dir = this.fds[fd];
                    if (dir === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    const preopenDirName = dir.getPreopenDirName();

                    const bytesToWrite = Math.min(
                        path_len,
                        preopenDirName.length,
                    );

                    const encoder = new TextEncoder();
                    const dirBytes = encoder.encode(preopenDirName);
                    new Uint8Array(
                        this.memory.buffer,
                        path_ptr,
                        bytesToWrite,
                    ).set(dirBytes.subarray(0, bytesToWrite));

                    return WASI_ERRNO_SUCCESS;
                },
                /** @type {(fd: number, buf_ptr: number) => number} */
                fd_fdstat_get: (fd, buf_ptr) => {
                    const memory = new DataView(this.memory.buffer);

                    if (fd === 0 || fd === 1 || fd === 2) {
                        memory.setUint8(buf_ptr, WASI_FILETYPE_REGULAR_FILE);
                    } else if (fd >= 3 && this.options.preopens !== undefined) {
                        memory.setUint8(buf_ptr, WASI_FILETYPE_DIRECTORY);
                    } else {
                        return WASI_ERRNO_BADF;
                    }

                    const fd_flags = 0;
                    memory.setUint16(buf_ptr + 2, fd_flags, true);

                    const rights_base = 0n;
                    memory.setBigUint64(buf_ptr + 8, rights_base, true);

                    const rights_inheriting = 0n;
                    memory.setBigUint64(buf_ptr + 16, rights_inheriting, true);

                    return WASI_ERRNO_SUCCESS;
                },
                /**
                 * @type {(
                 *   clock_id: number,
                 *   _precision: number,
                 *   time: number,
                 * ) => number}
                 */
                clock_time_get: (clock_id, _precision, time) => {
                    if (clock_id !== 1) {
                        return WASI_ERRNO_INVAL;
                    }

                    const now = BigInt(
                        Math.floor(performance.now() * 1_000_000),
                    );
                    const memory = new DataView(this.memory.buffer);
                    memory.setBigUint64(time, now, true);
                    return WASI_ERRNO_SUCCESS;
                },
                /**
                 * @type {(
                 *   fd: number,
                 *   buf_ptr: number,
                 *   buf_len: number,
                 *   cookie: bigint,
                 *   bufused_ptr: number,
                 * ) => number}
                 */
                fd_readdir: (fd, buf_ptr, buf_len, cookie, bufused_ptr) => {
                    const WASI_DIRENT_SIZE = 24; // d_next: u64, d_ino: u64, d_namlen: u32, d_type: u8 (+ padding)

                    const dir = this.fds[fd];
                    if (dir === undefined) {
                        return WASI_ERRNO_BADF;
                    }

                    const { code, entries } = dir.listDir();
                    if (code !== WASI_ERRNO_SUCCESS || entries === undefined) {
                        return code;
                    }

                    // the directory listing is re-read on every call,
                    // a plain index into it is used as the dirent cookie
                    const startIndex = Number(cookie);

                    const memory = new DataView(this.memory.buffer);
                    const bufBytes = new Uint8Array(this.memory.buffer);
                    const encoder = new TextEncoder();

                    let bufused = 0;
                    for (let i = startIndex; i < entries.length; i++) {
                        const entry = entries[i];
                        const nameBytes = encoder.encode(entry.name);
                        const entrySize = WASI_DIRENT_SIZE + nameBytes.length;
                        const direntType = entry.isDirectory
                            ? WASI_FILETYPE_DIRECTORY
                            : WASI_FILETYPE_REGULAR_FILE;

                        const dirent = new Uint8Array(entrySize);
                        const direntView = new DataView(dirent.buffer);
                        direntView.setBigUint64(0, BigInt(i + 1), true); // d_next
                        direntView.setBigUint64(8, 0n, true); // d_ino
                        direntView.setUint32(16, nameBytes.length, true); // d_namlen
                        direntView.setUint8(20, direntType); // d_type
                        dirent.set(nameBytes, WASI_DIRENT_SIZE);

                        // per spec, a trailing entry that doesn't fully fit is truncated
                        const remaining = buf_len - bufused;
                        const bytesToCopy = Math.min(remaining, entrySize);
                        bufBytes.set(
                            dirent.subarray(0, bytesToCopy),
                            buf_ptr + bufused,
                        );
                        bufused += bytesToCopy;

                        if (bytesToCopy < entrySize) {
                            break;
                        }
                    }

                    memory.setUint32(bufused_ptr, bufused, true);
                    return WASI_ERRNO_SUCCESS;
                },
            },
        };
    }
}

class ProcExitError extends Error {
    /** @type {number} */
    exitCode;

    /** @param {number} exitCode */
    constructor(exitCode) {
        super();
        this.exitCode = exitCode;
    }
}
