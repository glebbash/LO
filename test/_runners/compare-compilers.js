// @ts-check

// Checks that two compiler binaries produce the same output for the sample
//   programs (binaries, errors and inspections).

import { describe, it } from "node:test";
import assert from "node:assert";
import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { WASI, loadLoCompiler } from "./utils.js";

if (process.argv[1] === fileURLToPath(import.meta.url)) {
    const [a, b] = process.argv.slice(2);
    if (a === undefined || b === undefined) {
        console.error("Usage: node compare-compilers.js <a.wasm> <b.wasm>");
        process.exit(1);
    }
    const [absA, absB] = [path.resolve(a), path.resolve(b)];
    process.chdir(path.join(import.meta.dirname, "../.."));
    await compareCompilers(absA, absB);
}

/**
 * Registers the tests comparing two compiler binaries.
 * Expects the current directory to be the project root.
 * @param {string} compilerPath
 * @param {string} otherCompilerPath
 */
export async function compareCompilers(compilerPath, otherCompilerPath) {
    const lo = await loadLoCompiler(await fs.readFile(compilerPath));
    const loOther = await loadLoCompiler(await fs.readFile(otherCompilerPath));

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

        // compiler itself, except for @embed_files functionality
        "test/lo-cli-slim.lo",
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
                const compiled2 = await loOther(["compile", fileName]);

                assert.deepStrictEqual(compiled1, compiled2);
            });
        }
    });

    describe("compiler errors match other compiler", () => {
        /**
         * @param {typeof loOther} compiler
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
                    await outcome(loOther, fileName),
                );
            });
        }
    });

    describe("compiler inspections match other compiler", () => {
        for (const fileName of TESTED_SAMPLES) {
            it(`inspects ${fileName}`, async () => {
                const inspect1 = await lo(["inspect", fileName]);
                const inspect2 = await loOther(["inspect", fileName]);

                assert.strictEqual(
                    new TextDecoder().decode(inspect1),
                    new TextDecoder().decode(inspect2),
                );
            });
        }
    });

    describe("compiler error inspections match other compiler", () => {
        /**
         * @param {typeof loOther} compiler
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
                    await inspectOutput(loOther, fileName),
                );
            });
        }
    });
}
