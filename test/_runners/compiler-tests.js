// @ts-check

// Behaviour tests: compile the programs in `test/`, run them and check results,
//   output and diagnostics.

import { describe, it } from "node:test";
import assert from "node:assert";
import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { WASI, loadLoCompiler, loadWasm, runWASI } from "./utils.js";

if (process.argv[1] === fileURLToPath(import.meta.url)) {
    const args = process.argv.slice(2);
    const compilerPath = args.find((a) => !a.startsWith("--"));
    if (compilerPath === undefined) {
        console.error("Usage: node compiler-tests.js <lo.wasm> [--skip-fmt]");
        process.exit(1);
    }

    const absCompilerPath = path.resolve(compilerPath);

    process.chdir(path.join(import.meta.dirname, "../.."));

    await runCompilerTests(absCompilerPath, {
        skipFmt: args.includes("--skip-fmt"),
    });
}

/**
 * Registers the test suite for the compiler binary at `compilerPath`.
 * Expects the current directory to be the project root.
 * @param {string} compilerPath
 * @param {{ skipFmt?: boolean }} [options]
 */
export async function runCompilerTests(compilerPath, { skipFmt = false } = {}) {
    const lo = await loadLoCompiler(await fs.readFile(compilerPath));
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

    it("compiles lo-cli-slim.lo (the compiler itself) into a working compiler", async () => {
        const output = await compile("./test/lo-cli-slim.lo");

        const stdout = new WASI.VirtualFD();
        const exitCode = await runWASI(output, {
            stdout,
            preopens: { ".": "." },
            args: ["lo", "format", "./test/42.lo"],
        });

        assert.strictEqual(exitCode, 0);
        assert.strictEqual(
            stdout.flushAndReadUtf8(),
            await fs.readFile("./test/42.lo", "utf8"),
        );
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
        if (skipFmt) {
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
